---
name: grasshopper-grasshopper-gui
description: This skill encodes the grasshopper 8.0 surface of the Grasshopper.GUI namespace — 81 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GH_Animator, GH_Animator<T>, GH_AsynchFileData, GH_BezierSolver, GH_CanvasMouseEvent, GH_CanvasObjectMouseDownEventArgs, GH_ColourPicker, GH_ColourSwatchEventArgs, and 73 more types.
---

# Grasshopper.GUI

Auto-generated from vendor docs for grasshopper 8.0. 81 types in this namespace.

## GH_AnimationPhase (enum)

Enumerates the possible phases in which an animation can occur.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_AnimationPhase.htm)

### Values
- `Before` = `0` — Animation hasn't started yet.
- `During` = `1` — Animation currently in progress.
- `After` = `2` — Animation has finished.

## GH_Animator (class)

Provides static methods and constructors GH_Animator(Of T).

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_Animator.htm)

### Methods
#### `public static GH_Animator<decimal> DecimalAnimator(decimal A, decimal B)`



**Parameters:**
- `A` (System.Decimal)
- `B` (System.Decimal)

**Returns:** `GH_Animator<Decimal>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_DecimalAnimator.htm)

#### `public static GH_Animator<decimal> DecimalAnimator(decimal A, decimal B, DateTime time0, DateTime time1)`



**Parameters:**
- `A` (System.Decimal)
- `B` (System.Decimal)
- `time0` (System.DateTime)
- `time1` (System.DateTime)

**Returns:** `GH_Animator<Decimal>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_DecimalAnimator_1.htm)

#### `public static GH_Animator<decimal> DecimalAnimator(decimal A, decimal B, int duration)`



**Parameters:**
- `A` (System.Decimal)
- `B` (System.Decimal)
- `duration` (System.Int32)

**Returns:** `GH_Animator<Decimal>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_DecimalAnimator_2.htm)

#### `public static GH_Animator<double> DoubleAnimator(double A, double B)`



**Parameters:**
- `A` (System.Double)
- `B` (System.Double)

**Returns:** `GH_Animator<Double>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_DoubleAnimator.htm)

#### `public static GH_Animator<double> DoubleAnimator(double A, double B, DateTime time0, DateTime time1)`



**Parameters:**
- `A` (System.Double)
- `B` (System.Double)
- `time0` (System.DateTime)
- `time1` (System.DateTime)

**Returns:** `GH_Animator<Double>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_DoubleAnimator_1.htm)

#### `public static GH_Animator<double> DoubleAnimator(double A, double B, int duration)`



**Parameters:**
- `A` (System.Double)
- `B` (System.Double)
- `duration` (System.Int32)

**Returns:** `GH_Animator<Double>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_DoubleAnimator_2.htm)

#### `public static GH_Animator<int> IntegerAnimator(int A, int B)`



**Parameters:**
- `A` (System.Int32)
- `B` (System.Int32)

**Returns:** `GH_Animator<Int32>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_IntegerAnimator.htm)

#### `public static GH_Animator<int> IntegerAnimator(int A, int B, DateTime time0, DateTime time1)`



**Parameters:**
- `A` (System.Int32)
- `B` (System.Int32)
- `time0` (System.DateTime)
- `time1` (System.DateTime)

**Returns:** `GH_Animator<Int32>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_IntegerAnimator_1.htm)

#### `public static GH_Animator<int> IntegerAnimator(int A, int B, int duration)`



**Parameters:**
- `A` (System.Int32)
- `B` (System.Int32)
- `duration` (System.Int32)

**Returns:** `GH_Animator<Int32>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_IntegerAnimator_2.htm)

#### `public static decimal InterpolateDecimal(decimal A, decimal B, double factor)`

Utility interpolation delegate for System.Decimal values.

**Parameters:**
- `A` (System.Decimal)
- `B` (System.Decimal)
- `factor` (System.Double)

**Returns:** `Decimal` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_InterpolateDecimal.htm)

#### `public static double InterpolateDouble(double A, double B, double factor)`

Utility interpolation delegate for System.Double values.

**Parameters:**
- `A` (System.Double)
- `B` (System.Double)
- `factor` (System.Double)

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_InterpolateDouble.htm)

#### `public static int InterpolateInteger(int A, int B, double factor)`

Utility interpolation delegate for System.Int32 values.

**Parameters:**
- `A` (System.Int32)
- `B` (System.Int32)
- `factor` (System.Double)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_InterpolateInteger.htm)

#### `public static Point InterpolatePoint(Point A, Point B, double factor)`

Utility interpolation delegate for System.Drawing.Point values.

**Parameters:**
- `A` (System.Drawing.Point)
- `B` (System.Drawing.Point)
- `factor` (System.Double)

**Returns:** `Point` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_InterpolatePoint.htm)

#### `public static PointF InterpolatePointF(PointF A, PointF B, double factor)`

Utility interpolation delegate for System.Drawing.PointF values.

**Parameters:**
- `A` (System.Drawing.PointF)
- `B` (System.Drawing.PointF)
- `factor` (System.Double)

**Returns:** `PointF` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_InterpolatePointF.htm)

#### `public static Rectangle InterpolateRectangle(Rectangle A, Rectangle B, double factor)`

Utility interpolation delegate for System.Drawing.Rectangle values.

**Parameters:**
- `A` (System.Drawing.Rectangle)
- `B` (System.Drawing.Rectangle)
- `factor` (System.Double)

**Returns:** `Rectangle` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_InterpolateRectangle.htm)

#### `public static RectangleF InterpolateRectangleF(RectangleF A, RectangleF B, double factor)`

Utility interpolation delegate for System.Drawing.RectangleF values.

**Parameters:**
- `A` (System.Drawing.RectangleF)
- `B` (System.Drawing.RectangleF)
- `factor` (System.Double)

**Returns:** `RectangleF` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_InterpolateRectangleF.htm)

#### `public static float InterpolateSingle(float A, float B, double factor)`

Utility interpolation delegate for System.Single values.

**Parameters:**
- `A` (System.Single)
- `B` (System.Single)
- `factor` (System.Double)

**Returns:** `Single` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_InterpolateSingle.htm)

#### `public static GH_Animator<Point> PointAnimator(Point A, Point B)`



**Parameters:**
- `A` (System.Drawing.Point)
- `B` (System.Drawing.Point)

**Returns:** `GH_Animator<Point>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_PointAnimator.htm)

#### `public static GH_Animator<Point> PointAnimator(Point A, Point B, DateTime time0, DateTime time1)`



**Parameters:**
- `A` (System.Drawing.Point)
- `B` (System.Drawing.Point)
- `time0` (System.DateTime)
- `time1` (System.DateTime)

**Returns:** `GH_Animator<Point>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_PointAnimator_1.htm)

#### `public static GH_Animator<Point> PointAnimator(Point A, Point B, int duration)`



**Parameters:**
- `A` (System.Drawing.Point)
- `B` (System.Drawing.Point)
- `duration` (System.Int32)

**Returns:** `GH_Animator<Point>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_PointAnimator_2.htm)

#### `public static GH_Animator<PointF> PointFAnimator(PointF A, PointF B)`



**Parameters:**
- `A` (System.Drawing.PointF)
- `B` (System.Drawing.PointF)

**Returns:** `GH_Animator<PointF>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_PointFAnimator.htm)

#### `public static GH_Animator<PointF> PointFAnimator(PointF A, PointF B, DateTime time0, DateTime time1)`



**Parameters:**
- `A` (System.Drawing.PointF)
- `B` (System.Drawing.PointF)
- `time0` (System.DateTime)
- `time1` (System.DateTime)

**Returns:** `GH_Animator<PointF>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_PointFAnimator_1.htm)

#### `public static GH_Animator<PointF> PointFAnimator(PointF A, PointF B, int duration)`



**Parameters:**
- `A` (System.Drawing.PointF)
- `B` (System.Drawing.PointF)
- `duration` (System.Int32)

**Returns:** `GH_Animator<PointF>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_PointFAnimator_2.htm)

#### `public static GH_Animator<Rectangle> RectangleAnimator(Rectangle A, Rectangle B)`



**Parameters:**
- `A` (System.Drawing.Rectangle)
- `B` (System.Drawing.Rectangle)

**Returns:** `GH_Animator<Rectangle>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_RectangleAnimator.htm)

#### `public static GH_Animator<Rectangle> RectangleAnimator(Rectangle A, Rectangle B, DateTime time0, DateTime time1)`



**Parameters:**
- `A` (System.Drawing.Rectangle)
- `B` (System.Drawing.Rectangle)
- `time0` (System.DateTime)
- `time1` (System.DateTime)

**Returns:** `GH_Animator<Rectangle>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_RectangleAnimator_1.htm)

#### `public static GH_Animator<Rectangle> RectangleAnimator(Rectangle A, Rectangle B, int duration)`



**Parameters:**
- `A` (System.Drawing.Rectangle)
- `B` (System.Drawing.Rectangle)
- `duration` (System.Int32)

**Returns:** `GH_Animator<Rectangle>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_RectangleAnimator_2.htm)

#### `public static GH_Animator<RectangleF> RectangleFAnimator(RectangleF A, RectangleF B)`



**Parameters:**
- `A` (System.Drawing.RectangleF)
- `B` (System.Drawing.RectangleF)

**Returns:** `GH_Animator<RectangleF>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_RectangleFAnimator.htm)

#### `public static GH_Animator<RectangleF> RectangleFAnimator(RectangleF A, RectangleF B, DateTime time0, DateTime time1)`



**Parameters:**
- `A` (System.Drawing.RectangleF)
- `B` (System.Drawing.RectangleF)
- `time0` (System.DateTime)
- `time1` (System.DateTime)

**Returns:** `GH_Animator<RectangleF>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_RectangleFAnimator_1.htm)

#### `public static GH_Animator<RectangleF> RectangleFAnimator(RectangleF A, RectangleF B, int duration)`



**Parameters:**
- `A` (System.Drawing.RectangleF)
- `B` (System.Drawing.RectangleF)
- `duration` (System.Int32)

**Returns:** `GH_Animator<RectangleF>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_RectangleFAnimator_2.htm)

#### `public static GH_Animator<float> SingleAnimator(float A, float B)`



**Parameters:**
- `A` (System.Single)
- `B` (System.Single)

**Returns:** `GH_Animator<Single>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_SingleAnimator.htm)

#### `public static GH_Animator<float> SingleAnimator(float A, float B, DateTime time0, DateTime time1)`



**Parameters:**
- `A` (System.Single)
- `B` (System.Single)
- `time0` (System.DateTime)
- `time1` (System.DateTime)

**Returns:** `GH_Animator<Single>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_SingleAnimator_1.htm)

#### `public static GH_Animator<float> SingleAnimator(float A, float B, int duration)`



**Parameters:**
- `A` (System.Single)
- `B` (System.Single)
- `duration` (System.Int32)

**Returns:** `GH_Animator<Single>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_SingleAnimator_2.htm)

## GH_Animator<T> (class)

Provides methods for animating values.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_Animator_1.htm)

### Constructors
- `public GH_Animator(T A, T B, TimeSpan duration, GH_Animator<T>.Interpolate interpolator)` — Create a new instance of the generic animator.
- `public GH_Animator(T A, T B, DateTime startTime, DateTime endTime, GH_Animator<T>.Interpolate interpolator)` — Create a new instance of the generic animator.

### Methods
#### `public void AdjustAnimation(T newTarget)`

Adjust the animation.

**Parameters:**
- `newTarget` (T) — New B value.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_1_AdjustAnimation.htm)

#### `public void AdjustAnimation(T newTarget, DateTime newStartTime, DateTime newEndTime)`

Adjust the animation.

**Parameters:**
- `newTarget` (T) — New B value.
- `newStartTime` (System.DateTime)
- `newEndTime` (System.DateTime)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_1_AdjustAnimation_1.htm)

#### `public void AdjustAnimation(T newTarget, int duration)`

Adjust the animation.

**Parameters:**
- `newTarget` (T) — New B value.
- `duration` (System.Int32) — Duration of adjusted animation in milliseconds.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_1_AdjustAnimation_2.htm)

#### `public T CurrentValue()`

Gets the current animated value.

**Returns:** `T` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Animator_1_CurrentValue.htm)

### Properties
- `Interpolation` (GH_Interpolation, get/set) — Gets or sets the interpolation mode. Changing the mode during an animation will result in discontinuous motion.
- `Phase` (GH_AnimationPhase, get) — Gets the current animation phase.
- `ValueA` (T, get/set) — Gets or sets the start value of the animation. This value is set from within the constructor though you can change it at any time.
- `ValueB` (T, get/set) — Gets or sets the end value of the animation. This value is set from within the constructor though you can change it at any time.

## GH_Animator<T>.Interpolate (delegate)

Delegate for interpolating values.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_Animator_1_Interpolate.htm)

## GH_AsynchFileData (class)

This class provides asynchronous methods to keep tabs on a file.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_AsynchFileData.htm)

### Constructors
- `public GH_AsynchFileData(string filePath, bool harvestThumbnail, GH_AsynchFileData.FileChangedDelegate callback)` — Asynchronous constructor. File properties will be loaded asychronously.

### Methods
#### `public void Dispose()`

Releases all resources used by the GH_AsynchFileData

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_AsynchFileData_Dispose.htm)

#### `public void UpdateProperties()`

Updates all file properties on the thread on which this method is called.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_AsynchFileData_UpdateProperties.htm)

### Properties
- `Callback` (GH_AsynchFileData.FileChangedDelegate, get/set) — Gets or sets the callback delegate to be used when properties are updated.
- `FileCreated` (DateTime, get) — Gets the date at which the file was created.
- `FileExists` (Boolean, get) — Gets whether the file exists. Note that this property returns false if DataExists=False, regardless of whether the file exists.
- `FileLastChanged` (DateTime, get) — Gets the date at which the file was last changed.
- `FileSize` (Int64, get) — Gets the size of the file.
- `FileThumbnail` (Bitmap, get) — Gets the thumbnail bitmap associated with the file.
- `HarvestThumbnail` (Boolean, get/set) — Gets or sets whether the thumbnail ought to be parsed. At the moment this only makes sense for GH/GHX files.
- `IsFileData` (Boolean, get) — Gets whether the data has been retrieved at least once.
- `FilePath` (String, get) — Gets the location this instance is watching.
- `IsLocal` (Boolean, get) — Gets whether Grasshopper thinks the file it is watching is on a local machine.

## GH_AsynchFileData.FileChangedDelegate (delegate)

(No description provided in vendor docs for Grasshopper.GUI.GH_AsynchFileData.FileChangedDelegate.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_AsynchFileData_FileChangedDelegate.htm)

## GH_BezierSolver (class)

Class for evaluating four-point bezier curves as used in GDI+

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_BezierSolver.htm)

### Constructors
- `public GH_BezierSolver()` — Initializes a new instance of the GH_BezierSolver class

### Methods
#### `public static float AngleAt(ref PointF P0, ref PointF P1, ref PointF P2, ref PointF P3, float t)`

Evaluate the tangent angle of a Bezier span.

**Parameters:**
- `P0` (System.Drawing.PointF) — Start point of bezier.
- `P1` (System.Drawing.PointF) — Point for start tangent.
- `P2` (System.Drawing.PointF) — Point for end tangent.
- `P3` (System.Drawing.PointF) — End point of bezier.
- `t` (System.Single) — Unit parameter.

**Returns:** `Single` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_BezierSolver_AngleAt.htm)

#### `public static PointF PointAt(ref PointF P0, ref PointF P1, ref PointF P2, ref PointF P3, float t)`

Evaluate a Bezier span.

**Parameters:**
- `P0` (System.Drawing.PointF) — Start point of bezier.
- `P1` (System.Drawing.PointF) — Point for start tangent.
- `P2` (System.Drawing.PointF) — Point for end tangent.
- `P3` (System.Drawing.PointF) — End point of bezier.
- `t` (System.Single) — Unit parameter.

**Returns:** `PointF` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_BezierSolver_PointAt.htm)

#### `public static SizeF TangentAt(ref PointF P0, ref PointF P1, ref PointF P2, ref PointF P3, float t)`

Evaluate the tangent vector of a Bezier span.

**Parameters:**
- `P0` (System.Drawing.PointF) — Start point of bezier.
- `P1` (System.Drawing.PointF) — Point for start tangent.
- `P2` (System.Drawing.PointF) — Point for end tangent.
- `P3` (System.Drawing.PointF) — End point of bezier.
- `t` (System.Single) — Unit parameter.

**Returns:** `SizeF` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_BezierSolver_TangentAt.htm)

## GH_CanvasMouseEvent (class)

Class used in Canvas UI events.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_CanvasMouseEvent.htm)

### Constructors
- `public GH_CanvasMouseEvent()` — Blank constructor.
- `public GH_CanvasMouseEvent(GH_Viewport vp, MouseEventArgs e_args)` — Constructor.
- `public GH_CanvasMouseEvent(Point pt_control, PointF pt_canvas, MouseButtons mButton, int iClicks = 0, int iDelta = 0)` — Constructor.

### Properties
- `Button` (MouseButtons, get) — Gets which mouse button was pressed.
- `CanvasLocation` (PointF, get) — Gets the location of the cursor in canvas unit coordinates.
- `CanvasX` (Single, get) — Gets the x-coordinate of the cursor location in canvas unit coordinates.
- `CanvasY` (Single, get) — Gets the y-coordinate of the cursor location in canvas unit coordinates.
- `Clicks` (Int32, get) — Gets the number of times the mouse button was pressed and released.
- `ControlLocation` (Point, get) — Gets the location of the cursor in control pixel coordinates.
- `ControlX` (Int32, get) — Gets the x-coordinate of the cursor location in control pixel coordinates.
- `ControlY` (Int32, get) — Gets the y-coordinate of the cursor location in control pixel coordinates.
- `Delta` (Int32, get) — Gets a signed count of the number of detents the mouse wheel has rotated. A detent is one notch of the mouse wheel.
- `WinFormsEventArgs` (MouseEventArgs, get) — Downcast this data into a Windows.Forms.MouseEventArgs instance.

## GH_CanvasObjectMouseDownEventArgs (class)

Event arguments for the CanvasObjectMouseDown events.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_CanvasObjectMouseDownEventArgs.htm)

### Constructors
- `public GH_CanvasObjectMouseDownEventArgs(GH_Canvas sender, GH_RelevantObjectData object)` — Initializes a new instance of the GH_CanvasObjectMouseDownEventArgs class

### Properties
- `Canvas` (GH_Canvas, get) — 
- `Document` (GH_Document, get) — 
- `Object` (GH_RelevantObjectData, get) — 

## GH_ColourPicker (class)

Provides the standard Grasshopper Colour picker as a winforms Control.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_ColourPicker.htm)

### Constructors
- `public GH_ColourPicker()` — Initializes a new instance of the GH_ColourPicker class

### Methods
#### `protected override void Dispose(bool disposing)`

Releases the unmanaged resources used by the GH_ColourPicker and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_ColourPicker_Dispose.htm)

#### `protected override void OnPaint(PaintEventArgs e)`

(Overrides Control.OnPaint(PaintEventArgs).)

**Parameters:**
- `e` (System.Windows.Forms.PaintEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_ColourPicker_OnPaint.htm)

#### `protected override void OnPaintBackground(PaintEventArgs e)`

(Overrides ScrollableControl.OnPaintBackground(PaintEventArgs).)

**Parameters:**
- `e` (System.Windows.Forms.PaintEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_ColourPicker_OnPaintBackground.htm)

### Properties
- `AllowNumericInput` (Boolean, get/set) — Gets or sets whether this colour picker pops up a textbox on channel slider double clicks.
- `Colour` (Color, get/set) — Gets or sets the colour currently specified in the picker.
- `DesiredHeight` (Int32, get) — Gets the ideal height for this control given the current width and UI settings.
- `ShowAlphaSlider` (Boolean, get/set) — Gets or sets whether the alpha channel slider is included.
- `ShowChannelSliders` (Boolean, get/set) — Gets or sets whether the channel sliders are included.

### Events
#### `ColourChanged` (`Grasshopper.GUI.GH_ColourPicker.ColourChangedEventHandler`)

**Signature:** `public event GH_ColourPicker.ColourChangedEventHandler ColourChanged`

Raised whenever the value of the slider is changed via GUI interaction.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_GH_ColourPicker_ColourChanged.htm)

## GH_ColourPicker.ColourChangedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.GH_ColourPicker.ColourChangedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_ColourPicker_ColourChangedEventHandler.htm)

## GH_ColourSwatchEventArgs (class)

Event arguments used in the GH_ColourSwatchControl.ColourChanged event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_ColourSwatchEventArgs.htm)

### Constructors
- `public GH_ColourSwatchEventArgs(Color colour)` — Initializes a new instance of the GH_ColourSwatchEventArgs class

### Properties
- `Colour` (Color, get) — 

## GH_Constraint (enum)

Constraints used during UI dragging operations.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_Constraint.htm)

### Values
- `None` = `0` — No constraint is placed on drag direction.
- `Vertical` = `1` — Dragging is constrainted to the vertical direction.
- `Horizontal` = `2` — Dragging is constrainted to the horizontal direction.

## GH_CursorServer (class)

Class used to cache and set cursors.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_CursorServer.htm)

### Methods
#### `public bool AttachCursor(Control control, string name)`

Attach a cursor to a control.

**Parameters:**
- `control` (System.Windows.Forms.Control) — Control to assign cursor to.
- `name` (System.String) — Name of cursor to assign. This is the filename of the original cursor file minus the extension.

**Returns:** `Boolean` — True if cursor was assigned, or false if no matching cursor could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_CursorServer_AttachCursor.htm)

#### `public Cursor Cursor(string name)`

Gets a cursor from the cache.

**Parameters:**
- `name` (System.String) — Name of cursor to retrieve.

**Returns:** `Cursor` — The cursor that is associated with the given name or null if not cursor could be matched.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_CursorServer_Cursor.htm)

#### `public bool IsCursor(string name)`

Test for the existence of a cursor.

**Parameters:**
- `name` (System.String) — Cursor name to test for.

**Returns:** `Boolean` — True if the cursor is cached.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_CursorServer_IsCursor.htm)

#### `public int ReferenceCursors(string folder)`

Reference all the cursor files in a given directory.

**Parameters:**
- `folder` (System.String) — Directory to parse.

**Returns:** `Int32` — The number of cursors referenced. Use the AttachCursor() method to assign a referenced cursor to a control.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_CursorServer_ReferenceCursors.htm)

#### `public bool ResetCursor(Control control)`

Reset the cursor of the given control to default.

**Parameters:**
- `control` (System.Windows.Forms.Control) — Control to reset.

**Returns:** `Boolean` — True if control was successfully reset, false if iControl is a null reference.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_CursorServer_ResetCursor.htm)

### Properties
- `Count` (Int32, get) — Gets the number of cached cursors.

## GH_DebugLogForm (class)

(No description provided in vendor docs for Grasshopper.GUI.GH_DebugLogForm.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_DebugLogForm.htm)

### Constructors
- `public GH_DebugLogForm()` — Initializes a new instance of the GH_DebugLogForm class

### Methods
#### `protected override void Dispose(bool disposing)`

Releases the unmanaged resources used by the GH_DebugLogForm and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DebugLogForm_Dispose.htm)

## GH_DigitScroller (class)

Provides the standard Grasshopper Digit scroller as a winforms Control.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_DigitScroller.htm)

### Constructors
- `public GH_DigitScroller()` — Initializes a new instance of the GH_DigitScroller class

### Methods
#### `protected override void Input_Assign(string text)`

(Overrides GH_TextInputBaseControl.Input_Assign(String).)

**Parameters:**
- `text` (System.String)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DigitScroller_Input_Assign.htm)

#### `protected override bool Input_Parse(string text)`

(Overrides GH_TextInputBaseControl.Input_Parse(String).)

**Parameters:**
- `text` (System.String)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DigitScroller_Input_Parse.htm)

#### `protected override string Input_Supply()`

(Overrides GH_TextInputBaseControl.Input_Supply().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DigitScroller_Input_Supply.htm)

#### `protected override void OnPaint(PaintEventArgs e)`

(Overrides Control.OnPaint(PaintEventArgs).)

**Parameters:**
- `e` (System.Windows.Forms.PaintEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DigitScroller_OnPaint.htm)

#### `protected override void OnPaintBackground(PaintEventArgs e)`

(Overrides ScrollableControl.OnPaintBackground(PaintEventArgs).)

**Parameters:**
- `e` (System.Windows.Forms.PaintEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DigitScroller_OnPaintBackground.htm)

### Properties
- `AllowRadixDrag` (Boolean, get/set) — Gets or sets whether the radix symbol can be dragged.
- `AllowTextInput` (Boolean, get/set) — Gets or sets whether the control accepts text input override.
- `Amplify` (Boolean, get/set) — Gets or sets whether vertical mouse movement ought to be amplified.
- `DecimalPlaces` (Int32, get/set) — Gets or sets the number of decimal places.
- `DigitAlign` (GH_DigitAlign, get/set) — Gets or sets the digit align mode
- `Digits` (Int32, get/set) — Gets or sets the number of digits displayed in the scroller.
- `DigitScroller` (GH_DigitScrollerBase, get) — Gets the internal scroller object.
- `Epsilon` (Decimal, get) — Gets the smallest possible change with the current settings.
- `MaximumValue` (Decimal, get/set) — Gets or sets the upper bound of the scroll value.
- `MinimumValue` (Decimal, get/set) — Gets or sets the lower bound of the scroll value.
- `Prefix` (String, get/set) — Gets or sets the prefix text
- `Radix` (Int32, get/set) — Gets or sets the radix point index.
- `Suffix` (String, get/set) — Gets or sets the prefix text
- `Value` (Decimal, get/set) — Gets or sets the value of this scroller.

### Events
#### `ValueChanged` (`Grasshopper.GUI.GH_DigitScroller.ValueChangedEventHandler`)

**Signature:** `public event GH_DigitScroller.ValueChangedEventHandler ValueChanged`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_GH_DigitScroller_ValueChanged.htm)

## GH_DigitScroller.ValueChangedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.GH_DigitScroller.ValueChangedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_DigitScroller_ValueChangedEventHandler.htm)

## GH_DocumentEditor (class)

The Grasshopper main window.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_DocumentEditor.htm)

### Constructors
- `public GH_DocumentEditor()` — Initializes a new instance of the GH_DocumentEditor class

### Methods
#### `public void ClearStatusBar()`

Clear the Grasshopper status-bar.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_ClearStatusBar.htm)

#### `public void CloseForReal()`

Really, really close the window.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_CloseForReal.htm)

#### `public void CollapseForm()`

Collapses the form into the title bar area.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_CollapseForm.htm)

#### `public void DisableUI()`

Disable all modifier UI.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_DisableUI.htm)

#### `protected override void Dispose(bool disposing)`

Releases the unmanaged resources used by the GH_DocumentEditor and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_Dispose.htm)

#### `public void EnableUI()`

Enable all modifier UI.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_EnableUI.htm)

#### `public void ExpandForm()`

Expands the form if it is currenly collapsed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_ExpandForm.htm)

#### `public void FadeIn()`

Call this method if the prompt switches back to Grasshopper.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_FadeIn.htm)

#### `public void FadeOut()`

Call this method if the prompt switches to Rhino.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_FadeOut.htm)

#### `public void FixCanvasToolbarState()`

Enabled/Disables the appropriate buttons depending on whether there is a document loaded on the canvas.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_FixCanvasToolbarState.htm)

#### `public bool ScriptAccess_CloseAllDocuments()`

Closes all documents in the Document Queue.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_ScriptAccess_CloseAllDocuments.htm)

#### `public bool ScriptAccess_CloseDocument()`

Closes the currently active document and loads the next available document into the canvas. Changes will not be saved.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_ScriptAccess_CloseDocument.htm)

#### `public bool ScriptAccess_IsDocument()`

Returns true if there is an active document.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_ScriptAccess_IsDocument.htm)

#### `public bool ScriptAccess_IsDocumentModified()`

Gets the modified flag of the currently loaded document.

**Returns:** `Boolean` — True if the document has been modified, false if not or if there is no document.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_ScriptAccess_IsDocumentModified.htm)

#### `public bool ScriptAccess_NewDocument()`

Creates a new document and loads it into the Canvas.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_ScriptAccess_NewDocument.htm)

#### `public bool ScriptAccess_OpenDocument()`

Displays a command line interface for File Open... then loads that file into the Canvas.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_ScriptAccess_OpenDocument.htm)

#### `public bool ScriptAccess_OpenDocument(string filename)`

Loads a specific file off the hard disk into the Canvas.

**Parameters:**
- `filename` (System.String) — File to load.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_ScriptAccess_OpenDocument_1.htm)

#### `public bool ScriptAccess_SaveDocument()`

Saves the document currently loaded in the Canvas. If the document hasn't been saved before then a Save... dialog box will be shown.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_ScriptAccess_SaveDocument.htm)

#### `public bool ScriptAccess_SaveDocumentAs()`

Saves the currently active document to disk.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_ScriptAccess_SaveDocumentAs.htm)

#### `public bool ScriptAccess_SaveDocumentAs(string path)`

Saves the currently active document to disk.

**Parameters:**
- `path` (System.String) — Path of save destination.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_ScriptAccess_SaveDocumentAs_1.htm)

#### `public void SetStatusBarEvent(GH_RuntimeMessage event)`

Set a new message on the Grasshopper status bar.

**Parameters:**
- `event` (Grasshopper.Kernel.GH_RuntimeMessage) — Message to set

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_SetStatusBarEvent.htm)

#### `public void ToggleForm()`

Collapses the form if it's currently expanded or expands the form if it's currently collapsed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_ToggleForm.htm)

#### `protected override sealed void WndProc(ref Message m)`

WndProc is overridden in order to allow for title-bar double-click (un)folding.

**Parameters:**
- `m` (System.Windows.Forms.Message) — Proc message.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DocumentEditor_WndProc.htm)

### Properties
- `Collapsed` (Boolean, get) — Gets a value indicating whether or not the form is currently collapsed.
- `CreateParams` (CreateParams, get) — (Overrides Form.CreateParams.)
- `FormShepard` (GH_FormShepard, get) — Gets the form shepard for this window. Register your modeless dialogs with this shepard if you want to be popular.

### Events
#### `AggregateShortcutMenuItems` (`Grasshopper.GUI.GH_DocumentEditor.AggregateShortcutMenuItemsEventHandler`)

**Signature:** `public static event GH_DocumentEditor.AggregateShortcutMenuItemsEventHandler AggregateShortcutMenuItems`

This event is raised whenever a GH_DocumentEditor collects all shortcuttable menu items. If you added items to the menu and you want to be able to have custom shortcuts on these items, you must handle this event and insert your own items.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_GH_DocumentEditor_AggregateShortcutMenuItems.htm)

#### `EditorFoldStateChanged` (`Grasshopper.GUI.GH_DocumentEditor.EditorFoldStateChangedEventHandler`)

**Signature:** `public event GH_DocumentEditor.EditorFoldStateChangedEventHandler EditorFoldStateChanged`

This event is raised whenever the main windows folds or unfolds.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_GH_DocumentEditor_EditorFoldStateChanged.htm)

## GH_DocumentEditor.AggregateShortcutMenuItemsEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.GH_DocumentEditor.AggregateShortcutMenuItemsEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_DocumentEditor_AggregateShortcutMenuItemsEventHandler.htm)

## GH_DocumentEditor.EditorFoldStateChangedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.GH_DocumentEditor.EditorFoldStateChangedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_DocumentEditor_EditorFoldStateChangedEventHandler.htm)

## GH_DoubleBufferedPanel (class)

Derives from Windows.Forms.Panel but overrides certain flags that improve on-screen rendering. Use panel if you want to have a custom drawn control that is under the control of the owner form.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_DoubleBufferedPanel.htm)

### Constructors
- `public GH_DoubleBufferedPanel()` — Initializes a new instance of the GH_DoubleBufferedPanel class

## GH_DragInfo (class)

Contains data used for UI drag operations.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_DragInfo.htm)

### Constructors
- `public GH_DragInfo(Point drag_point, Rectangle initial_shape)` — Create a new DragInfo instance.

### Methods
#### `public void Drag(Point new_point)`

Call this method to adjust the drag data. Results will be automatically constrained and limited.

**Parameters:**
- `new_point` (System.Drawing.Point) — New location of cursor.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_DragInfo_Drag.htm)

### Properties
- `Box_Drag` (Rectangle, get) — Gets the dimensions of the box as a result of the dragging op.
- `Box_Start` (Rectangle, get) — Gets the dimensions of the drag box prior to dragging.
- `Constraint` (GH_Constraint, get/set) — Gets or sets the directional constraint for the drag operation.
- `Point_Drag` (Point, get) — Gets the cursor location at the current moment.
- `Point_Start` (Point, get) — Gets the cursor location at the start of the drag.
- `Region` (Rectangle, get/set) — Gets or sets the drag limits. If the region is Rectangle.Empty then the region is ignored. For valid results the Region should be larger than the dimensions of the drag box.

## GH_EtoUtil (class)

(No description provided in vendor docs for Grasshopper.GUI.GH_EtoUtil.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_EtoUtil.htm)

### Methods
#### `public static void CenterFormOnCursor(this Window F, bool limitToScreen)`

Positions a Form so that it is centered around the cursor

**Parameters:**
- `F` (Window) — The Form to adjust
- `limitToScreen` (System.Boolean) — If True, the Form won't be allowed to extend beyond the limits of the screen containing the cursor.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_EtoUtil_CenterFormOnCursor.htm)

#### `public static void CenterFormOnEditor(this Window F, bool limitToScreen)`

Positions a Form so that it is centered on the active editor. If the editor doesn't exist or is hidden, the form will be centered on the screen instead.

**Parameters:**
- `F` (Window) — The Form to adjust
- `limitToScreen` (System.Boolean) — If True, the Form won't be allowed to extend beyond the limits of the screen containing the cursor.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_EtoUtil_CenterFormOnEditor.htm)

#### `public static void CenterFormOnScreen(this Window F, bool limitToScreen)`

Positions a Form so that it is centered on the current screen. The current screen being defined as the screen containing the cursor.

**Parameters:**
- `F` (Window) — The Form to adjust
- `limitToScreen` (System.Boolean) — If True, the Form won't be allowed to extend beyond the limits of the active screen.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_EtoUtil_CenterFormOnScreen.htm)

#### `public static void CenterFormOnWindow(this Window F, Window parentWindow, bool limitToScreen)`

Positions a Form so that it is centered on the supplied window. If the window doesn't exist or is hidden, the form will be centered on the screen instead.

**Parameters:**
- `F` (Window) — The Form to adjust
- `parentWindow` (Window)
- `limitToScreen` (System.Boolean) — If True, the Form won't be allowed to extend beyond the limits of the screen containing the cursor.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_EtoUtil_CenterFormOnWindow.htm)

## GH_ExternalFileConflictDialog (class)

(No description provided in vendor docs for Grasshopper.GUI.GH_ExternalFileConflictDialog.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_ExternalFileConflictDialog.htm)

### Constructors
- `public GH_ExternalFileConflictDialog()` — Initializes a new instance of the GH_ExternalFileConflictDialog class

### Methods
#### `protected override void Dispose(bool disposing)`

Releases the unmanaged resources used by the GH_ExternalFileConflictDialog and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_ExternalFileConflictDialog_Dispose.htm)

### Properties
- `FilePathA` (String, get/set) — 
- `FilePathB` (String, get/set) — 
- `InfoLabel` (Label, get/set) — 
- `LoadFileCode` (Int32, get) — 

## GH_FadeAnimation (class)

Utility class for animating fade ZUI events on the canvas.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_FadeAnimation.htm)

### Constructors
- `public GH_FadeAnimation()` — Creates a new instance of the Animator class with default settings.
- `public GH_FadeAnimation(float threshold)` — Create a new instance of the Animator class with custom values for zoom threshold.
- `public GH_FadeAnimation(float threshold, int duration)` — Create a new instance of the Animator class with custom values for zoom threshold and duration.

### Methods
#### `public void Evaluate(GH_Canvas canvas)`

Reevaluate the fade animation parameters. You should call this method from within the Render method of your attributes. If an animation is currently in progress a redraw of the canvas will be scheduled, whether or not the attributes are actually visible on the canvas. You should therefore only call this method if your attributes are already drawing themselves.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas currently drawing in.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_FadeAnimation_Evaluate.htm)

#### `public void Evaluate(GH_Canvas canvas, bool scheduleRedraw)`

Reevaluate the fade animation parameters. You should call this method from within the Render method of your attributes. If an animation is currently in progress a redraw of the canvas will be scheduled, whether or not the attributes are actually visible on the canvas. You should therefore only call this method if your attributes are already drawing themselves.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas currently drawing in.
- `scheduleRedraw` (System.Boolean) — If true and the animator is active, a redraw will be scheduled.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_FadeAnimation_Evaluate_1.htm)

#### `public void Evaluate(GH_Canvas canvas, float value)`

Reevaluate the fade animation parameters. You should call this method from within the Render method of your attributes. If an animation is currently in progress a redraw of the canvas will be scheduled, whether or not the attributes are actually visible on the canvas. You should therefore only call this method if your attributes are already drawing themselves.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas currently drawing in.
- `value` (System.Single)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_FadeAnimation_Evaluate_2.htm)

#### `public void Evaluate(GH_Canvas canvas, float value, bool scheduleRedraw)`

Reevaluate the fade animation parameters. You should call this method from within the Render method of your attributes. If an animation is currently in progress a redraw of the canvas will be scheduled, whether or not the attributes are actually visible on the canvas. You should therefore only call this method if your attributes are already drawing themselves.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas currently drawing in.
- `value` (System.Single)
- `scheduleRedraw` (System.Boolean) — If true and the animator is active, a redraw will be scheduled.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_FadeAnimation_Evaluate_3.htm)

### Properties
- `Duration` (Int32, get/set) — Gets or sets the duration (in milliseconds) for ZUI animations. Negative durations are not allowed. The default is 350 milliseconds.
- `FadeAlpha` (Int32, get) — Gets the blend alpha byte for the current state. 0 means fully hidden, 255 means fully shown.
- `FadeFactor` (Double, get) — Gets the blend factor for the current state. 0.0 means fully hidden, 1.0 means fully shown.
- `IsFinished` (Boolean, get) — Gets a value indicating whether this animator has finished animating.
- `Phase` (GH_FadePhase, get) — Gets the current animation phase.
- `Threshold` (Single, get/set) — Gets or sets the threshold value for the fade animation trigger.
- `TriggerUpdate` (Boolean, get/set) — Gets or sets whether this animator triggers canvas updates.
- `Delay` (Int32, get) — Gets the number of milliseconds between redraws (25 milliseconds, resulting in ~40 fps)

## GH_FadePhase (enum)

Enumerates the possible states of a ZUI fade animation.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_FadePhase.htm)

### Values
- `Hidden` = `0` — Indicates the UI elements ought to be hidden.
- `Hiding` = `1` — Indicates the UI elements are in the process of being hidden.
- `Shown` = `2` — Indicates the UI elements ought to be shown.
- `Showing` = `3` — Indicates the UI elements are in the process of being shown.

## GH_FontControl (class)

Represents a control that offers both Font preview and Picking methods.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_FontControl.htm)

### Constructors
- `public GH_FontControl()` — Initializes a new instance of the GH_FontControl class

### Methods
#### `protected override void Dispose(bool disposing)`

Releases the unmanaged resources used by the GH_FontControl and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_FontControl_Dispose.htm)

#### `protected override void OnLayout(LayoutEventArgs e)`

Raises the Layout event.

**Parameters:**
- `e` (System.Windows.Forms.LayoutEventArgs) — A LayoutEventArgs that contains the event data.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_FontControl_OnLayout.htm)

### Properties
- `AllowFamilyOverride` (Boolean, get/set) — 
- `AllowSizeOverride` (Boolean, get/set) — 
- `AllowStyleOverride` (Boolean, get/set) — 

## GH_FontList (class)

Represents the installed Font collection. This control allows both navigation and selection of Font Families.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_FontList.htm)

### Constructors
- `public GH_FontList()` — Initializes a new instance of the GH_FontList class

### Methods
#### `public Font CreateFont()`

Create a font that best approximates the current settings.

**Returns:** `Font` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_FontList_CreateFont.htm)

#### `protected override void OnPaint(PaintEventArgs e)`

(Overrides Control.OnPaint(PaintEventArgs).)

**Parameters:**
- `e` (System.Windows.Forms.PaintEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_FontList_OnPaint.htm)

#### `protected override void OnPaintBackground(PaintEventArgs e)`

(Overrides ScrollableControl.OnPaintBackground(PaintEventArgs).)

**Parameters:**
- `e` (System.Windows.Forms.PaintEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_FontList_OnPaintBackground.htm)

#### `public void RecreateFontList()`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_FontList_RecreateFontList.htm)

### Properties
- `DisplayText` (String, get/set) — Gets or sets the placeholder string.
- `Filter` (String, get/set) — Gets or sets the filter string. You must call RecreateFontList() before the new font list is created.
- `FontFamily` (FontFamily, get/set) — Gets or sets the selected font family.
- `FontSize` (Single, get/set) — Gets or sets the font size.
- `FontStyle` (FontStyle, get/set) — Gets or sets the font style for all families.

## GH_FontPicker (class)

(No description provided in vendor docs for Grasshopper.GUI.GH_FontPicker.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_FontPicker.htm)

### Constructors
- `public GH_FontPicker()` — Initializes a new instance of the GH_FontPicker class

### Methods
#### `public static Form CreateFontPickerWindow(Font defaultFont)`

Create a new window containing a Font Picker object and OK, Cancel buttons. Picked fonts will be stored in the Tag field of the form.

**Parameters:**
- `defaultFont` (System.Drawing.Font)

**Returns:** `Form` — A window with a font picker.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_FontPicker_CreateFontPickerWindow.htm)

#### `protected override void Dispose(bool disposing)`

Releases the unmanaged resources used by the GH_FontPicker and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_FontPicker_Dispose.htm)

#### `public static Font ShowFontPickerWindow(Font font)`

Display the fontpicker in a window.

**Parameters:**
- `font` (System.Drawing.Font) — Default font.

**Returns:** `Font` — A newly picked font.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_FontPicker_ShowFontPickerWindow.htm)

#### `public static Font ShowFontPickerWindow(Font font, Form parent)`

Display the fontpicker in a window.

**Parameters:**
- `font` (System.Drawing.Font) — Default font.
- `parent` (System.Windows.Forms.Form) — Parent window.

**Returns:** `Font` — A newly picked font.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_FontPicker_ShowFontPickerWindow_1.htm)

### Properties
- `SelectedFont` (Font, get/set) — 

## GH_FontScroller (class)

Represents a panel with a scrolling collection of pangrams.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_FontScroller.htm)

### Constructors
- `public GH_FontScroller()` — Initializes a new instance of the GH_FontScroller class

### Methods
#### `protected override void OnPaint(PaintEventArgs e)`

(Overrides Control.OnPaint(PaintEventArgs).)

**Parameters:**
- `e` (System.Windows.Forms.PaintEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_FontScroller_OnPaint.htm)

#### `protected override void OnPaintBackground(PaintEventArgs e)`

(Overrides ScrollableControl.OnPaintBackground(PaintEventArgs).)

**Parameters:**
- `e` (System.Windows.Forms.PaintEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_FontScroller_OnPaintBackground.htm)

### Properties
- `PostPaintDelegate` (GH_FontScroller.PostPaint, get/set) — 

## GH_FontScroller.PostPaint (delegate)

(No description provided in vendor docs for Grasshopper.GUI.GH_FontScroller.PostPaint.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_FontScroller_PostPaint.htm)

## GH_FormShepard (class)

A Form shepard is used to herde a bunch of floating forms. Important forms that potentially spout child forms might choose to implement one.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_FormShepard.htm)

### Constructors
- `public GH_FormShepard(Form nOwner)` — Create a new form shepard.

### Methods
#### `public void RegisterForm(Form dialog)`

Add a sheep form to the flock. This form will be affected by the location and size of the main form.

**Parameters:**
- `dialog` (System.Windows.Forms.Form) — Form to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_FormShepard_RegisterForm.htm)

#### `public void UnregisterForm(Form dialog)`

Remove a sheep from the flock.

**Parameters:**
- `dialog` (System.Windows.Forms.Form) — Form to remove.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_FormShepard_UnregisterForm.htm)

### Properties
- `Sheep` (IEnumerable<Form>, get) — 

## GH_GDI_Util (class)

Contains some global function for filleting corners of GDI objects.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_GDI_Util.htm)

### Methods
#### `public static List<Point[]> BoxUnion(IEnumerable<Rectangle> rectangles)`

Create the outline of a set of boxes.

**Parameters:**
- `rectangles` (System.Collections.Generic.IEnumerable<Rectangle>) — Boxes to union.

**Returns:** `List<Point[]>` — A list of Point arrays, where each array describes the corners of each silhouette.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GDI_Util_BoxUnion.htm)

#### `public static GraphicsPath FilletBoxOutline(IEnumerable<Rectangle> boxes, int radius)`

Fillet all the corners of the combined outline of a set of rectangles.

**Parameters:**
- `boxes` (System.Collections.Generic.IEnumerable<Rectangle>) — Rectangles to outline.
- `radius` (System.Int32) — Radius of corner fillets.

**Returns:** `GraphicsPath` — GraphicsPath describing the filleted outline or null on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GDI_Util_FilletBoxOutline.htm)

#### `public static GraphicsPath FilletPolyline(PointF[] corners, float radius)`

Fillet all the corners of a freeform outline.

**Parameters:**
- `corners` (System.Drawing.PointF[]) — A list of corner points.
- `radius` (System.Single) — Fillet to apply to all corners.

**Returns:** `GraphicsPath` — A graphics path describing the filleted shape, or Nothing on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GDI_Util_FilletPolyline_1.htm)

#### `public static GraphicsPath FilletPolyline(Point[] corners, int radius)`

Fillet all the corners of a freeform outline.

**Parameters:**
- `corners` (System.Drawing.Point[]) — A list of corner points.
- `radius` (System.Int32) — Fillet to apply to all corners.

**Returns:** `GraphicsPath` — A graphics path describing the filleted shape, or Nothing on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GDI_Util_FilletPolyline.htm)

#### `public static GraphicsPath FilletRectangle(Rectangle box, int radius)`

Create a rectangle with filleted corners.

**Parameters:**
- `box` (System.Drawing.Rectangle) — Rectangle to fillet.
- `radius` (System.Int32) — Fillet radius, radius will be limited if too large.

**Returns:** `GraphicsPath` — GraphicsPath describing filletted rectangle.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GDI_Util_FilletRectangle.htm)

#### `public static GraphicsPath Freeform_Blob(IEnumerable<RectangleF> content, int padding, double accuracy)`

Create a meta-ball outline for a set of rectangles.

**Parameters:**
- `content` (System.Collections.Generic.IEnumerable<RectangleF>) — Rectangles to encompass.
- `padding` (System.Int32) — Number of units to add to each box.
- `accuracy` (System.Double) — Pixel accuracy for isosurface outlines. 3 is a good value for 1:1 UI.

**Returns:** `GraphicsPath` — A graphics path describing all iso-surfaces.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GDI_Util_Freeform_Blob.htm)

#### `public static GraphicsPath Freeform_Blob(IEnumerable<RectangleF> content, int padding, double accuracy, FieldSolver solver, double section)`

Create a meta-ball outline for a set of rectangles.

**Parameters:**
- `content` (System.Collections.Generic.IEnumerable<RectangleF>) — Rectangles to encompass.
- `padding` (System.Int32) — Number of units to add to each box.
- `accuracy` (System.Double) — Pixel accuracy for isosurface outlines. 3 is a good value for 1:1 UI.
- `solver` (FieldSolver) — The falloff solver to use. GH_Util.MetaBall.GH_Context contains a bunch of predefined solvers.
- `section` (System.Double) — Elevation of section plane.

**Returns:** `GraphicsPath` — A graphics path describing all iso-surfaces

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GDI_Util_Freeform_Blob_1.htm)

#### `public static Rectangle[] SimplifyBoxes(IEnumerable<Rectangle> boxes)`

Create a simplified collection of Rectangles that describe the same union space as the supplied rectangles.

**Parameters:**
- `boxes` (System.Collections.Generic.IEnumerable<Rectangle>) — Rectangles to simplify.

**Returns:** `Rectangle[]` — An ordered collection of non-intersecting rectangles or null on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GDI_Util_SimplifyBoxes.htm)

## GH_GeometricFont (class)

Exposes methods for drawing geometric characters.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_GeometricFont.htm)

### Methods
#### `public static void Draw0(Graphics graphics, Color colour, PointF position)`

Draw the geometric character '0' at the specified location.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `colour` (System.Drawing.Color) — Colour of character.
- `position` (System.Drawing.PointF) — Location of character.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GeometricFont_Draw0.htm)

#### `public static void Draw1(Graphics graphics, Color colour, PointF position)`

Draw the geometric character '1' at the specified location.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `colour` (System.Drawing.Color) — Colour of character.
- `position` (System.Drawing.PointF) — Location of character.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GeometricFont_Draw1.htm)

#### `public static void Draw2(Graphics graphics, Color colour, PointF position)`

Draw the geometric character '2' at the specified location.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `colour` (System.Drawing.Color) — Colour of character.
- `position` (System.Drawing.PointF) — Location of character.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GeometricFont_Draw2.htm)

#### `public static void Draw3(Graphics graphics, Color colour, PointF position)`

Draw the geometric character '3' at the specified location.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `colour` (System.Drawing.Color) — Colour of character.
- `position` (System.Drawing.PointF) — Location of character.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GeometricFont_Draw3.htm)

#### `public static void Draw4(Graphics graphics, Color colour, PointF position)`

Draw the geometric character '4' at the specified location.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `colour` (System.Drawing.Color) — Colour of character.
- `position` (System.Drawing.PointF) — Location of character.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GeometricFont_Draw4.htm)

#### `public static void Draw5(Graphics graphics, Color colour, PointF position)`

Draw the geometric character '5' at the specified location.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `colour` (System.Drawing.Color) — Colour of character.
- `position` (System.Drawing.PointF) — Location of character.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GeometricFont_Draw5.htm)

#### `public static void Draw6(Graphics graphics, Color colour, PointF position)`

Draw the geometric character '6' at the specified location.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `colour` (System.Drawing.Color) — Colour of character.
- `position` (System.Drawing.PointF) — Location of character.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GeometricFont_Draw6.htm)

#### `public static void Draw7(Graphics graphics, Color colour, PointF position)`

Draw the geometric character '7' at the specified location.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `colour` (System.Drawing.Color) — Colour of character.
- `position` (System.Drawing.PointF) — Location of character.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GeometricFont_Draw7.htm)

#### `public static void Draw8(Graphics graphics, Color colour, PointF position)`

Draw the geometric character '8' at the specified location.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `colour` (System.Drawing.Color) — Colour of character.
- `position` (System.Drawing.PointF) — Location of character.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GeometricFont_Draw8.htm)

#### `public static void Draw9(Graphics graphics, Color colour, PointF position)`

Draw the geometric character '9' at the specified location.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `colour` (System.Drawing.Color) — Colour of character.
- `position` (System.Drawing.PointF) — Location of character.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GeometricFont_Draw9.htm)

#### `public static void DrawBracketClose(Graphics graphics, Color colour, PointF position)`

Draw the geometric character '}' at the specified location.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `colour` (System.Drawing.Color) — Colour of character.
- `position` (System.Drawing.PointF) — Location of character.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GeometricFont_DrawBracketClose.htm)

#### `public static void DrawBracketOpen(Graphics graphics, Color colour, PointF position)`

Draw the geometric character '{' at the specified location.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `colour` (System.Drawing.Color) — Colour of character.
- `position` (System.Drawing.PointF) — Location of character.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GeometricFont_DrawBracketOpen.htm)

#### `public static void DrawDash(Graphics graphics, Color colour, PointF position)`

Draw the geometric character '-' at the specified location.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `colour` (System.Drawing.Color) — Colour of character.
- `position` (System.Drawing.PointF) — Location of character.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GeometricFont_DrawDash.htm)

#### `public static void DrawNumber(Graphics graphics, int number, Color colour, PointF position)`

Draw an integer at the specified location.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics to draw with.
- `number` (System.Int32) — Number to draw.
- `colour` (System.Drawing.Color) — Colour to draw with.
- `position` (System.Drawing.PointF) — Position for first character.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GeometricFont_DrawNumber.htm)

#### `public static void DrawNumber(Graphics graphics, long number, Color colour, PointF position)`

Draw an integer at the specified location.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics to draw with.
- `number` (System.Int64) — Number to draw.
- `colour` (System.Drawing.Color) — Colour to draw with.
- `position` (System.Drawing.PointF) — Position for first character.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GeometricFont_DrawNumber_1.htm)

#### `public static void DrawNumber(Graphics graphics, uint number, Color colour, PointF position)`

Draw an integer at the specified location.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics to draw with.
- `number` (System.UInt32) — Number to draw.
- `colour` (System.Drawing.Color) — Colour to draw with.
- `position` (System.Drawing.PointF) — Position for first character.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GeometricFont_DrawNumber_2.htm)

#### `public static void DrawNumber(Graphics graphics, ulong number, Color colour, PointF position)`

Draw an integer at the specified location.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics to draw with.
- `number` (System.UInt64) — Number to draw.
- `colour` (System.Drawing.Color) — Colour to draw with.
- `position` (System.Drawing.PointF) — Position for first character.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GeometricFont_DrawNumber_3.htm)

#### `public static void DrawPath(Graphics graphics, GH_Path path, Color colour, PointF position)`

Draw an entire path string at a specific location.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `path` (Grasshopper.Kernel.Data.GH_Path) — Path to draw.
- `colour` (System.Drawing.Color) — Colour of path.
- `position` (System.Drawing.PointF) — Location of opening bracket.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GeometricFont_DrawPath.htm)

#### `public static void DrawSemiColon(Graphics graphics, Color colour, PointF position)`

Draw the geometric character ';' at the specified location.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `colour` (System.Drawing.Color) — Colour of character.
- `position` (System.Drawing.PointF) — Location of character.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GeometricFont_DrawSemiColon.htm)

### Properties
- `Char0` (GraphicsPath, get) — Gets the default geometry for '0'. Caller is responsible for disposing the path.
- `Char1` (GraphicsPath, get) — Gets the default geometry for '1'. Caller is responsible for disposing the path.
- `Char2` (GraphicsPath, get) — Gets the default geometry for '2'. Caller is responsible for disposing the path.
- `Char3` (GraphicsPath, get) — Gets the default geometry for '3'. Caller is responsible for disposing the path.
- `Char4` (GraphicsPath, get) — Gets the default geometry for '4'. Caller is responsible for disposing the path.
- `Char5` (GraphicsPath, get) — Gets the default geometry for '5'. Caller is responsible for disposing the path.
- `Char6` (GraphicsPath, get) — Gets the default geometry for '6'. Caller is responsible for disposing the path.
- `Char7` (GraphicsPath, get) — Gets the default geometry for '7'. Caller is responsible for disposing the path.
- `Char8` (GraphicsPath, get) — Gets the default geometry for '8'. Caller is responsible for disposing the path.
- `Char9` (GraphicsPath, get) — Gets the default geometry for '9'. Caller is responsible for disposing the path.
- `CharBracketClose` (GraphicsPath, get) — Gets the default geometry for '}'. Caller is responsible for disposing the path.
- `CharBracketOpen` (GraphicsPath, get) — Gets the default geometry for '{'. Caller is responsible for disposing the path.
- `CharDash` (GraphicsPath, get) — Gets the default geometry for '-'. Caller is responsible for disposing the path.
- `CharSemiColon` (GraphicsPath, get) — Gets the default geometry for ';'. Caller is responsible for disposing the path.

## GH_GraphicsUtil (class)

Utility class with static (Shared) functions that help with colour, shape and text rendering.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_GraphicsUtil.htm)

### Methods
#### `public static void AppendArc(Point3d p0, Point3d p1, Vector3d dir, GraphicsPath path)`

Append an arc segment to a GraphicsPath. If the arc cannot be solved, a linear segment is appended instead.

**Parameters:**
- `p0` (Point3d) — Start point of arc.
- `p1` (Point3d) — End point of arc.
- `dir` (Vector3d) — Tangent direction at start of arc.
- `path` (System.Drawing.Drawing2D.GraphicsPath) — Path to append to.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_AppendArc.htm)

#### `public static void AppendArc(PointF p0, PointF p1, SizeF dir, GraphicsPath path)`

Append an arc segment to a GraphicsPath. If the arc cannot be solved, a linear segment is appended instead.

**Parameters:**
- `p0` (System.Drawing.PointF) — Start point of arc.
- `p1` (System.Drawing.PointF) — End point of arc.
- `dir` (System.Drawing.SizeF) — Tangent direction at start of arc.
- `path` (System.Drawing.Drawing2D.GraphicsPath) — Path to append to.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_AppendArc_1.htm)

#### `public static Color BlendColour(Color bottom, Color top)`

Overlays two colours. The Alpha component of the top colour controls the blend.

**Parameters:**
- `bottom` (System.Drawing.Color) — Base colour.
- `top` (System.Drawing.Color) — Overlay colour.

**Returns:** `Color` — The blended colour. Alpha channels are added together.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_BlendColour.htm)

#### `public static Color BlendColour(Color bottom, Color top, double factor)`

Overlays two colours.

**Parameters:**
- `bottom` (System.Drawing.Color) — Base colour.
- `top` (System.Drawing.Color) — Overlay colour.
- `factor` (System.Double) — Blend factor. 0.0 means the return colour will be identical to the bottom colour, 1.0 means the return colour will be identical to the top colour.

**Returns:** `Color` — The blended colour.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_BlendColour_1.htm)

#### `public static int BlendInteger(double t0, double t1, int a0, int a1, double t)`

Calculates a linear blend between two extremes. Although the inputs are unlimited integers, the output is an integer in the byte range (0-255).

**Parameters:**
- `t0` (System.Double) — Value of first extreme.
- `t1` (System.Double) — Value of second extreme.
- `a0` (System.Int32) — Integer value at first extreme.
- `a1` (System.Int32) — Integer value at second extreme.
- `t` (System.Double) — Interpolation parameter.

**Returns:** `Int32` — Interpolated value.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_BlendInteger.htm)

#### `public static PointF BoxClosestArcPoint(PointF pt, RectangleF box)`

Find the closest point on or in a box given a test point. Arc extension is implied.

**Parameters:**
- `pt` (System.Drawing.PointF) — Sample point.
- `box` (System.Drawing.RectangleF) — Box to evaluate.

**Returns:** `PointF` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_BoxClosestArcPoint.htm)

#### `public static Point BoxClosestPoint(Point pt, Rectangle box)`

Find the closest point on or in a box given a test point.

**Parameters:**
- `pt` (System.Drawing.Point) — Sample point.
- `box` (System.Drawing.Rectangle) — Box to evaluate.

**Returns:** `Point` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_BoxClosestPoint.htm)

#### `public static PointF BoxClosestPoint(PointF pt, RectangleF box)`

Find the closest point on or in a box given a test point.

**Parameters:**
- `pt` (System.Drawing.PointF) — Sample point.
- `box` (System.Drawing.RectangleF) — Box to evaluate.

**Returns:** `PointF` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_BoxClosestPoint_1.htm)

#### `public static Point BoxFurthestPoint(Point pt, Rectangle box)`

Find the furthest point on or in a box given a test point.

**Parameters:**
- `pt` (System.Drawing.Point) — Sample point.
- `box` (System.Drawing.Rectangle) — Box to evaluate.

**Returns:** `Point` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_BoxFurthestPoint.htm)

#### `public static PointF BoxFurthestPoint(PointF pt, RectangleF box)`

Find the furthest point on or in a box given a test point.

**Parameters:**
- `pt` (System.Drawing.PointF) — Sample point.
- `box` (System.Drawing.RectangleF) — Box to evaluate.

**Returns:** `PointF` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_BoxFurthestPoint_1.htm)

#### `public static GraphicsPath BoxViewCone(PointF pt, RectangleF box)`

Computes the graphicspath that represents the viewcone for a point and a rectangle. If the rectangle contains the point, null is returned.

**Parameters:**
- `pt` (System.Drawing.PointF)
- `box` (System.Drawing.RectangleF)

**Returns:** `GraphicsPath` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_BoxViewCone.htm)

#### `public static Color ColourARGB(double r, double g, double b)`

Create a colour from floating ARGB channels (0.0~1.0). Channels are clipped to valid ranges.

**Parameters:**
- `r` (System.Double)
- `g` (System.Double)
- `b` (System.Double)

**Returns:** `Color` — The colour implied by the RGB channel values.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_ColourARGB.htm)

#### `public static Color ColourARGB(int r, int g, int b)`

Create a colour from integer RGB channels. Channels are clipped to valid ranges.

**Parameters:**
- `r` (System.Int32)
- `g` (System.Int32)
- `b` (System.Int32)

**Returns:** `Color` — Returns the colour as implied by the RGB channel values.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_ColourARGB_1.htm)

#### `public static Color ColourARGB(int a, int r, int g, int b)`

Create a colour from integer ARGB channels. Channels are clipped to valid ranges.

**Parameters:**
- `a` (System.Int32)
- `r` (System.Int32)
- `g` (System.Int32)
- `b` (System.Int32)

**Returns:** `Color` — Returns the colour as implied by the ARGB channel values.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_ColourARGB_2.htm)

#### `public static Bitmap CreateColourIcon(int width, int height, Color colour)`

Create a colour swatch icon.

**Parameters:**
- `width` (System.Int32) — Width of icon.
- `height` (System.Int32) — Height of icon.
- `colour` (System.Drawing.Color) — Colour for swatch, if colour contains an alpha component, you should draw an alpha-grid behind the icon.

**Returns:** `Bitmap` — A bitmap with the requested icon colour.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_CreateColourIcon.htm)

#### `public static void DentHorizontal(Graphics g, Rectangle box)`

Draws a horizontal dent. A horizontal dent had dark lines along the top and bottom edges.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `box` (System.Drawing.Rectangle) — Dimensions of the dent.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_DentHorizontal.htm)

#### `public static void DentHorizontal(Graphics g, Rectangle box, int alphaEdge, int alphaFill)`

Draws a horizontal dent. A horizontal dent had dark lines along the top and bottom edges.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `box` (System.Drawing.Rectangle) — Dimensions of the dent.
- `alphaEdge` (System.Int32) — Transparency on the middle of the edges.
- `alphaFill` (System.Int32) — Transparency on the middle of the fill.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_DentHorizontal_1.htm)

#### `public static void DentVertical(Graphics g, Rectangle box)`

Draws a vertical dent. A vertical dent had dark lines along the left and right edges.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `box` (System.Drawing.Rectangle) — Dimensions of the dent.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_DentVertical.htm)

#### `public static void DentVertical(Graphics g, Rectangle box, int alphaEdge, int alphaFill)`

Draws a vertical dent. A vertical dent had dark lines along the left and right edges.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `box` (System.Drawing.Rectangle) — Dimensions of the dent.
- `alphaEdge` (System.Int32) — Transparency on the middle of the edges.
- `alphaFill` (System.Int32) — Transparency on the middle of the fill.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_DentVertical_1.htm)

#### `public static float Distance(PointF a, PointF b)`

Compute the distance between two points.

**Parameters:**
- `a` (System.Drawing.PointF) — First point.
- `b` (System.Drawing.PointF) — Second point.

**Returns:** `Single` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_Distance.htm)

#### `public static float DistanceS(PointF a, PointF b)`

Compute the squared distance between two points.

**Parameters:**
- `a` (System.Drawing.PointF) — First point.
- `b` (System.Drawing.PointF) — Second point.

**Returns:** `Single` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_DistanceS.htm)

#### `public static void EtchFadingHorizontal(Graphics g, int x0, int x1, int y, int alphaLight = 200, int alphaDark = 35)`

Draw an etched line with fading on the extremes.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `x0` (System.Int32) — Left x coordinate of etch.
- `x1` (System.Int32) — Right x coordinate of etch.
- `y` (System.Int32) — Y coordinate of etch.
- `alphaLight` (System.Int32) — Transparancy of bright etch line.
- `alphaDark` (System.Int32) — Transparency of dark etch line.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_EtchFadingHorizontal.htm)

#### `public static void EtchFadingVertical(Graphics g, int y0, int y1, int x, int alphaLight = 200, int alphaDark = 80)`

Draw an etched line with fading on the extremes.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `y0` (System.Int32) — Top y coordinate of etch.
- `y1` (System.Int32) — Bottom y coordinate of etch.
- `x` (System.Int32) — X coordinate of etch.
- `alphaLight` (System.Int32) — Transparancy of etch.
- `alphaDark` (System.Int32) — Transparancy of etch.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_EtchFadingVertical.htm)

#### `public static void EtchHorizontal(Graphics g, int x0, int x1, int y, int alphaLight = 200, int alphaDark = 35)`

Draws an etched line segment.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `x0` (System.Int32) — x component of start point.
- `x1` (System.Int32) — x component of end point.
- `y` (System.Int32) — y component of line segment.
- `alphaLight` (System.Int32) — Alpha level of etched lit line.
- `alphaDark` (System.Int32) — Alpha level of etched shadow line.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_EtchHorizontal.htm)

#### `public static void EtchHorizontal(Graphics g, float x0, float x1, float y, int alpha = 50)`

Draws an etched line segment.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `x0` (System.Single) — x component of start point.
- `x1` (System.Single) — x component of end point.
- `y` (System.Single) — y component of line segment.
- `alpha` (System.Int32) — Alpha level of etched line.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_EtchHorizontal_1.htm)

#### `public static void EtchVertical(Graphics g, int x, int y0, int y1, int alpha = 50)`

Draws an etched line segment.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `x` (System.Int32) — x component of line segment.
- `y0` (System.Int32) — x component of start point.
- `y1` (System.Int32) — y component of end point.
- `alpha` (System.Int32) — Alpha level of etched line.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_EtchVertical.htm)

#### `public static void EtchVertical(Graphics g, float x, float y0, float y1, int alpha = 50)`

Draws an etched line segment.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `x` (System.Single) — x component of line segment.
- `y0` (System.Single) — x component of start point.
- `y1` (System.Single) — y component of end point.
- `alpha` (System.Int32) — Alpha level of etched line.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_EtchVertical_1.htm)

#### `public static Color FadeColour(double t0, double t1, double t, Color col)`

Fade a colour to transparency by linearly blending a parameter between extremes.

**Parameters:**
- `t0` (System.Double) — First extreme. (represents transparency)
- `t1` (System.Double) — Second extreme. (represent unaltered colour)
- `t` (System.Double) — Blending factor between t0 and t1.
- `col` (System.Drawing.Color) — Colour to fade.

**Returns:** `Color` — The colour adjusted for blending.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_FadeColour.htm)

#### `public static Color ForegroundColour(Color col, int contrast = 30)`

Calculates a suitable foreground color. If the background is lighter than 50%, the foreground will be darker, otherwise the foreground will be lighter.

**Parameters:**
- `col` (System.Drawing.Color) — Foreground colour that works well on the given background colour.
- `contrast` (System.Int32) — Contrast amount (in 0~255 range)

**Returns:** `Color` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_ForegroundColour.htm)

#### `public static void Grid(Graphics g, RectangleF rec, int density)`

Draw a default grid.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `rec` (System.Drawing.RectangleF) — Grid rectangle.
- `density` (System.Int32) — Density in both directions.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_Grid.htm)

#### `public static void Grid(Graphics g, RectangleF rec, int densityX, int densityY)`

Draw a default grid.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `rec` (System.Drawing.RectangleF) — Grid rectangle.
- `densityX` (System.Int32) — Density in x direction.
- `densityY` (System.Int32) — Density in y direction.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_Grid_1.htm)

#### `public static void Grid(Graphics g, RectangleF rec, int densityX, int densityY, Color colorBg, Color colorWire)`

Draw a grid.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `rec` (System.Drawing.RectangleF) — Grid rectangle.
- `densityX` (System.Int32) — Density in x direction.
- `densityY` (System.Int32) — Density in y direction.
- `colorBg` (System.Drawing.Color) — Background colour override, the default is 30/255 white.
- `colorWire` (System.Drawing.Color) — Grid line colour override, the default is 40/255 black.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_Grid_2.htm)

#### `public static Brush HighlightBrush()`

Gets a new brush object that can be used to draw the content of hover and focus rectangles. Caller is responsible for disposing the brush.

**Returns:** `Brush` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_HighlightBrush.htm)

#### `public static Pen HighlightPen()`

Gets a new pen object that can be used to draw the edges of hover and focus rectangles. Caller is responsible for disposing the pen.

**Returns:** `Pen` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_HighlightPen.htm)

#### `public static Color InvertColour(Color col)`

Computes the inverted colour. Alpha remains the same.

**Parameters:**
- `col` (System.Drawing.Color)

**Returns:** `Color` — Colour where RGB channels are set to 255-C.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_InvertColour.htm)

#### `public static bool IsPointInEllipse(RectangleF ellipse, PointF point)`

Test whether a point is inside or outside an ellipse.

**Parameters:**
- `ellipse` (System.Drawing.RectangleF) — Rectangle describing ellipse.
- `point` (System.Drawing.PointF) — Point to test for inclusion.

**Returns:** `Boolean` — True if the point is in or on the ellipse.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_IsPointInEllipse.htm)

#### `public static void LimitInteger(ref int channel)`

Limits the given integer to the 0~255 range.

**Parameters:**
- `channel` (System.Int32)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_LimitInteger.htm)

#### `public static void LimitIntegers(ref int r, ref int g, ref int b)`

Limits RGB channels to the 0~255 range,

**Parameters:**
- `r` (System.Int32)
- `g` (System.Int32)
- `b` (System.Int32)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_LimitIntegers.htm)

#### `public static Color OffsetColour(Color col, int shift)`

Add a fixed value to all the channels in a colour.

**Remarks:** The alpha channel of the colour is not offset.

**Parameters:**
- `col` (System.Drawing.Color) — The colour to mutate.
- `shift` (System.Int32) — Offset value (-255 ~ +255).

**Returns:** `Color` — The offsetted colour.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_OffsetColour.htm)

#### `public static Color OffsetColour(Color col, int redShift, int greenShift, int blueShift)`

Add a fixed value to all the channels in a colour.

**Remarks:** The alpha channel of the colour is not offset.

**Parameters:**
- `col` (System.Drawing.Color) — The colour to mutate
- `redShift` (System.Int32) — Offset value for red channel (-255 ~ +255).
- `greenShift` (System.Int32) — Offset value for green channel (-255 ~ +255).
- `blueShift` (System.Int32) — Offset value for blue channel (-255 ~ +255).

**Returns:** `Color` — The offsetted colour.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_OffsetColour_1.htm)

#### `public static void RenderBalloonTag(Graphics g, string text, Font font, Color backColor, Color foreColor, PointF tip, bool upright)`

Draw a balloon tag in a container.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `text` (System.String) — Text of tag.
- `font` (System.Drawing.Font) — Font of tag.
- `backColor` (System.Drawing.Color) — Background colour of balloon.
- `foreColor` (System.Drawing.Color) — Foreground colour of balloon edge and text.
- `tip` (System.Drawing.PointF) — Tip of balloon.
- `upright` (System.Boolean) — If true, the balloon will be above the tip point.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderBalloonTag.htm)

#### `public static void RenderBalloonTag(Graphics g, string text, Font font, Color backColor, Color foreColor, PointF tip, RectangleF container)`

Draw a balloon tag in a container.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `text` (System.String) — Text of tag.
- `font` (System.Drawing.Font) — Font of tag.
- `backColor` (System.Drawing.Color) — Background colour of balloon.
- `foreColor` (System.Drawing.Color) — Foreground colout of balloon edge and text.
- `tip` (System.Drawing.PointF) — Tip of balloon (should be inside container).
- `container` (System.Drawing.RectangleF) — Container for balloon.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderBalloonTag_1.htm)

#### `public static void RenderBalloonTag(Graphics g, string text, Font font, PointF tip, RectangleF container)`

Draw a default balloon tag in a container with a Font override.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `text` (System.String) — Text of tag.
- `font` (System.Drawing.Font) — Font of tag.
- `tip` (System.Drawing.PointF) — Tip of balloon.
- `container` (System.Drawing.RectangleF) — Container for balloon.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderBalloonTag_2.htm)

#### `public static void RenderBalloonTag(Graphics g, string text, PointF tip, RectangleF container)`

Draw a default balloon tag in a container.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `text` (System.String) — Text of tag.
- `tip` (System.Drawing.PointF) — Tip of balloon.
- `container` (System.Drawing.RectangleF) — Container for balloon.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderBalloonTag_3.htm)

#### `[ObsoleteAttribute("This method is obsolete, use a RenderIcon() overload instead.")] public static void RenderCenteredIcon(Graphics g, RectangleF frame, Image icon)`

Draw an icon centered in a frame. This code is not zoom-aware, but it will perform consistent rounding (to avoid pixel-jitter when moving the frame across the screen). Icon will not be clipped to the frame.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to render with.
- `frame` (System.Drawing.RectangleF) — Frame to center icon in.
- `icon` (System.Drawing.Image) — Icon to render

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderCenteredIcon.htm)

#### `public static void RenderCenteredIcon(Graphics g, RectangleF frame, Image icon, double alpha)`

Draw an icon centered in a frame. This code is not zoom-aware, but it will perform consistent rounding (to avoid pixel-jitter when moving the frame across the screen). Icon will not be clipped to the frame.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to render with.
- `frame` (System.Drawing.RectangleF) — Frame to center icon in.
- `icon` (System.Drawing.Image) — Icon to render
- `alpha` (System.Double) — Transparency factor.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderCenteredIcon_1.htm)

#### `[ObsoleteAttribute("This method is obsolete, use a RenderIcon() overload instead.")] public static void RenderCenteredIcon(Graphics g, RectangleF frame, Image icon, float scale)`

Draw an icon centered in a frame. This code is not zoom-aware, but it will perform consistent rounding (to avoid pixel-jitter when moving the frame across the screen). Icon will not be clipped to the frame.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to render with.
- `frame` (System.Drawing.RectangleF) — Frame to center icon in.
- `icon` (System.Drawing.Image) — Icon to render
- `scale` (System.Single)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderCenteredIcon_2.htm)

#### `public static void RenderCenteredText(Graphics g, string text, Font font, Color color, PointF center)`

Render a bit of text centered on a point.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `text` (System.String) — Text to draw.
- `font` (System.Drawing.Font) — Font to use.
- `color` (System.Drawing.Color) — Text color.
- `center` (System.Drawing.PointF) — Center point.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderCenteredText.htm)

#### `[ObsoleteAttribute("Use RenderColourIcon(Graphics, Rectangle, Color, Single) instead.")] public static void RenderColourIcon(Graphics g, Rectangle rec, Color colour)`

Render a colour icon into a graphics context. A colour icon contains a black outer and white inner edge.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `rec` (System.Drawing.Rectangle) — Rectangle describing icon dimensions.
- `colour` (System.Drawing.Color) — Colour for icon.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderColourIcon.htm)

#### `public static void RenderColourIcon(Graphics g, Rectangle rec, Color colour, float scale)`

Render a colour icon into a graphics context. A colour icon contains a black outer and white inner edge.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `rec` (System.Drawing.Rectangle) — Rectangle describing icon dimensions.
- `colour` (System.Drawing.Color) — Colour for icon.
- `scale` (System.Single) — Scale adjustment.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderColourIcon_1.htm)

#### `public static void RenderFadedImage(Graphics graphics, Image image, Rectangle destination, double fade)`

Render an icon into a rectangle with a specific fading factor.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `image` (System.Drawing.Image) — Image to draw.
- `destination` (System.Drawing.Rectangle) — Destination rectangle.
- `fade` (System.Double) — Fading amount. 0.0=completely transparent, 1.0=original transparency.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderFadedImage.htm)

#### `public static void RenderFadedImage(Graphics graphics, Image image, Rectangle destination, int alpha)`

Render an icon into a rectangle with a specific fading factor.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `image` (System.Drawing.Image) — Image to draw.
- `destination` (System.Drawing.Rectangle) — Destination rectangle.
- `alpha` (System.Int32) — Alpha value. 0=completely transparent, 255=original transparency.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderFadedImage_1.htm)

#### `public static void RenderHighlightBox(Graphics g, Rectangle box, int cornerRadius)`

Render a typical blueish highlight rectangle

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to render with.
- `box` (System.Drawing.Rectangle) — Dimensions of selection box.
- `cornerRadius` (System.Int32) — Radius (in pixels) of box corners, use zero or less to render crisp rectangles.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderHighlightBox.htm)

#### `public static void RenderHighlightBox(Graphics g, Rectangle box, int cornerRadius, bool drawFill, bool drawEdge)`

Render a typical blueish highlight rectangle

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to render with.
- `box` (System.Drawing.Rectangle) — Dimensions of selection box.
- `cornerRadius` (System.Int32) — Radius (in pixels) of box corners, use zero or less to render crisp rectangles.
- `drawFill` (System.Boolean) — If true, the box fill have a blue background.
- `drawEdge` (System.Boolean) — If true, the box will have a blue edge.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderHighlightBox_1.htm)

#### `public static void RenderHighlightBox(Graphics g, Rectangle box, int cornerRadius, Color fill, Color edge)`

Render a typical blueish highlight rectangle

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to render with.
- `box` (System.Drawing.Rectangle) — Dimensions of selection box.
- `cornerRadius` (System.Int32) — Radius (in pixels) of box corners, use zero or less to render crisp rectangles.
- `fill` (System.Drawing.Color) — Fill colour
- `edge` (System.Drawing.Color) — Edge colour

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderHighlightBox_2.htm)

#### `public static void RenderIcon(Graphics graphics, RectangleF frame, Image icon)`

Render an icon into a frame. The icon is positioned in the center and will be drawn at its own size multiplied by the system UiScale, provided it doesn't exceed the frame.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `frame` (System.Drawing.RectangleF) — Limiting frame.
- `icon` (System.Drawing.Image) — Icon to draw.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderIcon.htm)

#### `public static void RenderObjectOverlay(Graphics graphics, IGH_DocumentObject obj, RectangleF target)`

Render all overlay icons for a specific object. Overlays aren't drawn when the zoom is less than 80%

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `obj` (Grasshopper.Kernel.IGH_DocumentObject) — Object to draw overlay for.
- `target` (System.Drawing.RectangleF) — Destination area for overlay icons.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderObjectOverlay.htm)

#### `public static void RenderObjectOverlay(Graphics graphics, IGH_ObjectProxy obj, RectangleF target)`

Render all overlay icons for a specific object. Overlays aren't drawn when the zoom is less than 80%

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `obj` (Grasshopper.Kernel.IGH_ObjectProxy) — Proxy to draw overlay for.
- `target` (System.Drawing.RectangleF) — Destination area for overlay icons.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderObjectOverlay_1.htm)

#### `public static void RenderRoundBar(Graphics graphics, RectangleF rec, Color colour)`

Render a cylinder shaped object with round caps.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `rec` (System.Drawing.RectangleF) — Rectangle containing shape.
- `colour` (System.Drawing.Color) — Fill colour of shape.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderRoundBar.htm)

#### `public static void RenderTag(Graphics g, string tag, Font font, Color backColor, Color foreColor, RectangleF frame, StringAlignment alignH, StringAlignment alignV)`

Render a text tag into a Graphics surface.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `tag` (System.String) — Text to draw.
- `font` (System.Drawing.Font) — Font to use.
- `backColor` (System.Drawing.Color) — Tag background colour.
- `foreColor` (System.Drawing.Color) — Tag foreground colour.
- `frame` (System.Drawing.RectangleF) — The frame into which the tags should be aligned.
- `alignH` (System.Drawing.StringAlignment) — Horizontal alignment.
- `alignV` (System.Drawing.StringAlignment) — Vertical alignment.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderTag.htm)

#### `[ObsoleteAttribute("This method is obsolete, use a RenderIcon() overload instead.")] public static void RenderUnscaledIcon(Graphics g, Image icon, int cx, int cy)`

Draws an image file at 1:1 scale centered on the given coordinates.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `icon` (System.Drawing.Image) — Image to draw.
- `cx` (System.Int32) — X center of image in Graphics coordinate.
- `cy` (System.Int32) — Y center of image in Graphics coordinate.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderUnscaledIcon.htm)

#### `[ObsoleteAttribute("This method is obsolete, use a RenderIcon() overload instead.")] public static void RenderUnscaledIcon(Graphics g, Image icon, int cx, int cy, double fade)`

Draws an image file at 1:1 scale centered on the given coordinates.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `icon` (System.Drawing.Image) — Image to draw.
- `cx` (System.Int32) — X center of image in Graphics coordinate.
- `cy` (System.Int32) — Y center of image in Graphics coordinate.
- `fade` (System.Double) — Alpha fading factor.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderUnscaledIcon_1.htm)

#### `public static void RenderVerticalString(Graphics g, string text, Font font, Color colour, RectangleF rec, StringFormat format)`

Render vertical text (rotated 90 degrees counter-clockwise) within a rectangle.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `text` (System.String) — Text to render.
- `font` (System.Drawing.Font) — Font to render with.
- `colour` (System.Drawing.Color) — Colour of text content.
- `rec` (System.Drawing.RectangleF)
- `format` (System.Drawing.StringFormat) — Formatting flags.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderVerticalString.htm)

#### `public static void RenderWarningIcon(Graphics graphics, PointF center, float radius, int alpha)`

Render a typical warning icon into a graphics context. A warning icon is an upright yellow triangle.

**Parameters:**
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `center` (System.Drawing.PointF) — Center of icon.
- `radius` (System.Single) — Radius of icon.
- `alpha` (System.Int32) — Transparency of icon.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_RenderWarningIcon.htm)

#### `public static Color ScaleColour(Color col, double factor)`

Multiply the channels of a colour construct with a fixed factor.

**Remarks:** The alpha channel of the colour is not mutated.

**Parameters:**
- `col` (System.Drawing.Color) — The colour to mutate
- `factor` (System.Double) — An adjustment factor.

**Returns:** `Color` — The scaled colour.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_ScaleColour.htm)

#### `public static void ShadowHorizontal(Graphics g, int x0, int x1, int y, int size, bool below, Color color)`

Draws a horizontal shadow edge.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `x0` (System.Int32) — Start of shadow edge.
- `x1` (System.Int32) — End of shadow edge.
- `y` (System.Int32) — Elevation of shadow edge.
- `size` (System.Int32) — Length of cast shadow.
- `below` (System.Boolean) — If true, shadow is drawn below elevation line.
- `color` (System.Drawing.Color) — Colour at shadow edge.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_ShadowHorizontal.htm)

#### `public static void ShadowHorizontal(Graphics g, int x0, int x1, int y, int size = 10, bool below = true, int darkness = 50)`

Draws a horizontal shadow edge.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `x0` (System.Int32) — Start of shadow edge.
- `x1` (System.Int32) — End of shadow edge.
- `y` (System.Int32) — Elevation of shadow edge.
- `size` (System.Int32) — Length of cast shadow.
- `below` (System.Boolean) — If true, shadow is drawn below elevation line.
- `darkness` (System.Int32) — Alpha component of darkness directly underneath the edge.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_ShadowHorizontal_1.htm)

#### `public static void ShadowHorizontal(Graphics g, float x0, float x1, float y, float size = 10f, bool below = true, int darkness = 50)`

Draws a horizontal shadow edge.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `x0` (System.Single) — Start of shadow edge.
- `x1` (System.Single) — End of shadow edge.
- `y` (System.Single) — Elevation of shadow edge.
- `size` (System.Single) — Length of cast shadow.
- `below` (System.Boolean) — If true, shadow is drawn below elevation line.
- `darkness` (System.Int32) — Alpha component of darkness directly underneath the edge.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_ShadowHorizontal_2.htm)

#### `public static void ShadowRectangle(Graphics g, Rectangle rec, int size, Color color)`

Draws shadow edges on the interior of a rectangle.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `rec` (System.Drawing.Rectangle) — Bounding rectangle.
- `size` (System.Int32) — Length of cast shadow.
- `color` (System.Drawing.Color) — Color at edge of shadow.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_ShadowRectangle.htm)

#### `public static void ShadowRectangle(Graphics g, Rectangle rec, int size = 10, int darkness = 50)`

Draws shadow edges on the interior of a rectangle.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `rec` (System.Drawing.Rectangle) — Bounding rectangle.
- `size` (System.Int32) — Length of cast shadow.
- `darkness` (System.Int32) — Alpha component of darkness directly underneath the edge.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_ShadowRectangle_1.htm)

#### `public static void ShadowVertical(Graphics g, int x, int y0, int y1, int size, bool right, Color color)`

Draws a vertical shadow edge.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `x` (System.Int32) — Offset of shadow edge.
- `y0` (System.Int32) — Start of shadow edge.
- `y1` (System.Int32) — End of shadow edge.
- `size` (System.Int32) — Length of cast shadow.
- `right` (System.Boolean) — If true, shadow is drawn to the right of the elevated line.
- `color` (System.Drawing.Color) — Color at shadow edge.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_ShadowVertical.htm)

#### `public static void ShadowVertical(Graphics g, int x, int y0, int y1, int size = 10, bool right = true, int darkness = 50)`

Draws a vertical shadow edge.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `x` (System.Int32) — Offset of shadow edge.
- `y0` (System.Int32) — Start of shadow edge.
- `y1` (System.Int32) — End of shadow edge.
- `size` (System.Int32) — Length of cast shadow.
- `right` (System.Boolean) — If true, shadow is drawn to the right of the elevated line.
- `darkness` (System.Int32) — Alpha component of darkness directly underneath the edge.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_ShadowVertical_1.htm)

#### `public static void ShadowVertical(Graphics g, float x, float y0, float y1, float size = 10f, bool right = true, int darkness = 50)`

Draws a vertical shadow edge.

**Parameters:**
- `g` (System.Drawing.Graphics) — Graphics object to draw with.
- `x` (System.Single) — Offset of shadow edge.
- `y0` (System.Single) — Start of shadow edge.
- `y1` (System.Single) — End of shadow edge.
- `size` (System.Single) — Length of cast shadow.
- `right` (System.Boolean) — If true, shadow is drawn to the right of the elevated line.
- `darkness` (System.Int32) — Alpha component of darkness directly underneath the edge.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_ShadowVertical_2.htm)

#### `public static bool SolveArc(double p0X, double p0Y, double p1X, double p1Y, double dx, double dy, ref RectangleF box, ref float angle, ref float sweep)`

Create a GDI circular arc definition from start-point, end-point and tangent direction.

**Parameters:**
- `p0X` (System.Double)
- `p0Y` (System.Double)
- `p1X` (System.Double)
- `p1Y` (System.Double)
- `dx` (System.Double)
- `dy` (System.Double)
- `box` (System.Drawing.RectangleF)
- `angle` (System.Single)
- `sweep` (System.Single)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_SolveArc_1.htm)

#### `public static bool SolveArc(Point3d p0, Point3d p1, Vector3d dir, ref RectangleF box, ref float angle, ref float sweep)`

Create a GDI circular arc definition from start-point, end-point and tangent direction.

**Parameters:**
- `p0` (Point3d) — Start point of arc.
- `p1` (Point3d) — End point of arc.
- `dir` (Vector3d) — Tangent direction at start of arc.
- `box` (System.Drawing.RectangleF) — Bounding box of arc (out)
- `angle` (System.Single) — Start angle of arc (out)
- `sweep` (System.Single) — Sweep angle of arc (out)

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_SolveArc.htm)

#### `public static bool SolveArc(PointF p0, PointF p1, SizeF dir, ref RectangleF box, ref float angle, ref float sweep)`

Create a GDI circular arc definition from start-point, end-point and tangent direction.

**Parameters:**
- `p0` (System.Drawing.PointF) — Start point of arc.
- `p1` (System.Drawing.PointF) — End point of arc.
- `dir` (System.Drawing.SizeF) — Tangent direction at start of arc.
- `box` (System.Drawing.RectangleF) — Bounding box of arc (out)
- `angle` (System.Single) — Start angle of arc (out)
- `sweep` (System.Single) — Sweep angle of arc (out)

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_SolveArc_2.htm)

#### `public static ColorMatrix TransparencyMatrix(double fade)`

Create an image transparency matrix for a specific blending factor.

**Parameters:**
- `fade` (System.Double) — Alpha fade amount. 0.0=complete transparency, 1.0=original transparency.

**Returns:** `ColorMatrix` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_TransparencyMatrix.htm)

#### `public static ColorMatrix TransparencyMatrix(int alpha)`

Create an image transparency matrix for a specific blending factor.

**Parameters:**
- `alpha` (System.Int32) — Alpha level of color matrix.

**Returns:** `ColorMatrix` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_GraphicsUtil_TransparencyMatrix_1.htm)

### Properties
- `UiScale` (Single, get) — Gets the screen-scaling factor for the entire system. Use the UiAdjust methods in the Grasshopper top level namespace (Global_Proc module) to modify integer and single values.

## GH_HorizontalSeparator (class)

GUI elements for creating visual separations.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_HorizontalSeparator.htm)

### Constructors
- `public GH_HorizontalSeparator()` — Initializes a new instance of the GH_HorizontalSeparator class

### Methods
#### `protected override void OnPaint(PaintEventArgs e)`

(Overrides Control.OnPaint(PaintEventArgs).)

**Parameters:**
- `e` (System.Windows.Forms.PaintEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_HorizontalSeparator_OnPaint.htm)

#### `protected override void OnPaintBackground(PaintEventArgs e)`

(Overrides ScrollableControl.OnPaintBackground(PaintEventArgs).)

**Parameters:**
- `e` (System.Windows.Forms.PaintEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_HorizontalSeparator_OnPaintBackground.htm)

### Properties
- `DisplayMode` (GH_SeparatorDisplay, get/set) — Gets or sets the separator display mode.

## GH_IconLabel (class)

A simple Icon with an icon overlay plus highlight rectangle.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_IconLabel.htm)

### Constructors
- `public GH_IconLabel()` — Initializes a new instance of the GH_IconLabel class

### Properties
- `LabelIcon` (Bitmap, get/set) — 

## GH_IconPicker (class)

Provides a standard icon selection interface.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_IconPicker.htm)

### Constructors
- `public GH_IconPicker()` — Initializes a new instance of the GH_IconPicker class

### Methods
#### `protected override void Dispose(bool disposing)`

Releases the unmanaged resources used by the GH_IconPicker and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_IconPicker_Dispose.htm)

### Properties
- `Icon` (Bitmap, get/set) — Gets or sets the icon to be displayed in this picker.
- `IconChanged` (Boolean, get) — Gets a value indicating whether or not the icon has been changed.

## GH_Interpolation (enum)

Enumerates the possible animation progression types.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_Interpolation.htm)

### Values
- `None` = `0` — No animation occurs.
- `NearestNeighbour` = `1` — Animation interpolation is either equal to A or B, whichever is closest.
- `Linear` = `2` — Linear interpolation between A and B.
- `EaseIn` = `3` — Interpolation that has a derivative of zero at the start.
- `EaseOut` = `4` — Inteprolation that has a derivative of zero at the end.
- `EaseInAndOut` = `5` — Interpolation that has a derivative of zero at both ends.

## GH_LoadProtectDialog (class)

(No description provided in vendor docs for Grasshopper.GUI.GH_LoadProtectDialog.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_LoadProtectDialog.htm)

### Constructors
- `public GH_LoadProtectDialog()` — Initializes a new instance of the GH_LoadProtectDialog class

### Methods
#### `protected override void Dispose(bool disposing)`

Releases the unmanaged resources used by the GH_LoadProtectDialog and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_LoadProtectDialog_Dispose.htm)

#### `public void SetAssemblyName(string name)`



**Parameters:**
- `name` (System.String)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_LoadProtectDialog_SetAssemblyName.htm)

## GH_MaterialPreviewControl (class)

Offers a frontend for Grasshopper Material previews.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_MaterialPreviewControl.htm)

### Constructors
- `public GH_MaterialPreviewControl()` — Initializes a new instance of the GH_MaterialPreviewControl class

### Methods
#### `protected override void Dispose(bool disposing)`

Releases the unmanaged resources used by the GH_MaterialPreviewControl and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_MaterialPreviewControl_Dispose.htm)

### Properties
- `Colour` (Color, get/set) — Gets or sets the colour of this preview.
- `Selected` (Boolean, get/set) — Gets or sets the Selected state of this preview. Selected previews have thick borders drawn around them.
- `SelectOnClick` (Boolean, get/set) — Gets or sets whether this control can be selected.

### Events
#### `ColourChanged` (`Grasshopper.GUI.GH_MaterialPreviewControl.ColourChangedEventHandler`)

**Signature:** `public event GH_MaterialPreviewControl.ColourChangedEventHandler ColourChanged`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_GH_MaterialPreviewControl_ColourChanged.htm)

#### `MaterialImagePostPaint` (`Grasshopper.GUI.GH_MaterialPreviewControl.MaterialImagePostPaintEventHandler`)

**Signature:** `public event GH_MaterialPreviewControl.MaterialImagePostPaintEventHandler MaterialImagePostPaint`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_GH_MaterialPreviewControl_MaterialImagePostPaint.htm)

#### `SelectedChanged` (`Grasshopper.GUI.GH_MaterialPreviewControl.SelectedChangedEventHandler`)

**Signature:** `public event GH_MaterialPreviewControl.SelectedChangedEventHandler SelectedChanged`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_GH_MaterialPreviewControl_SelectedChanged.htm)

## GH_MaterialPreviewControl.ColourChangedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.GH_MaterialPreviewControl.ColourChangedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_MaterialPreviewControl_ColourChangedEventHandler.htm)

## GH_MaterialPreviewControl.MaterialImagePostPaintEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.GH_MaterialPreviewControl.MaterialImagePostPaintEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_MaterialPreviewControl_MaterialImagePostPaintEventHandler.htm)

## GH_MaterialPreviewControl.SelectedChangedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.GH_MaterialPreviewControl.SelectedChangedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_MaterialPreviewControl_SelectedChangedEventHandler.htm)

## GH_MenuTextBox (class)

Utility control for putting textboxes into menus.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_MenuTextBox.htm)

### Constructors
- `public GH_MenuTextBox(ToolStripDropDown dropDown, string text, bool lockFocus)` — Constructor for TextBox collection.

### Methods
#### `public void CloseEntireMenuStructure()`

Call this function to close the ENTIRE menu structure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_MenuTextBox_CloseEntireMenuStructure.htm)

### Properties
- `OriginalText` (String, get/set) — Gets or sets the original text that was visible when the control was first displayed.
- `Text` (String, get/set) — Gets or sets the current text.
- `TextBoxItem` (ToolStripTextBox, get) — Gets the text box control embedded in the menu.
- `Width` (Int32, get/set) — Gets or sets the width of the control.

### Events
#### `KeyDown` (`Grasshopper.GUI.GH_MenuTextBox.KeyDownEventHandler`)

**Signature:** `public event GH_MenuTextBox.KeyDownEventHandler KeyDown`

Raised whenever a key is pressed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_GH_MenuTextBox_KeyDown.htm)

#### `TextChanged` (`Grasshopper.GUI.GH_MenuTextBox.TextChangedEventHandler`)

**Signature:** `public event GH_MenuTextBox.TextChangedEventHandler TextChanged`

Raised whenever the text in the textbox changes.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_GH_MenuTextBox_TextChanged.htm)

## GH_MenuTextBox.KeyDownEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.GH_MenuTextBox.KeyDownEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_MenuTextBox_KeyDownEventHandler.htm)

## GH_MenuTextBox.TextChangedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.GH_MenuTextBox.TextChangedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_MenuTextBox_TextChangedEventHandler.htm)

## GH_MouseTracker (class)

Record mouse-movements over time.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_MouseTracker.htm)

### Constructors
- `public GH_MouseTracker()` — Create a new mouse tracker.
- `public GH_MouseTracker(Point initialPosition)` — Create a new mouse tracker.
- `public GH_MouseTracker(Point initialPosition, Object tag)` — Create a new mouse tracker.

### Methods
#### `public double Deviation()`

Gets the maximum deviation of the recorded mouse path. The maximum deviation is the furthest distance the mouse has been from the first point in the history record.

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_MouseTracker_Deviation.htm)

#### `public double Deviation(DateTime t0, DateTime t1)`

Gets the maximum deviation of the recorded mouse path. The maximum deviation is the furthest distance the mouse has been from the first point in the history span. It does not measure the largest deviation between any two points in the time span, only the first and any subsequent point.

**Parameters:**
- `t0` (System.DateTime) — Represents the earliest time to include.
- `t1` (System.DateTime) — Represents the latest time to include.

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_MouseTracker_Deviation_1.htm)

#### `public void DropFrames(DateTime timeLimit)`

Drop all frames older than a given limit.

**Parameters:**
- `timeLimit` (System.DateTime) — All frames older than the limit will be dropped.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_MouseTracker_DropFrames.htm)

#### `public void DropFrames(double deviation)`

Find the youngest frame that deviates too much from the current locus and drop it and everything preceding it.

**Parameters:**
- `deviation` (System.Double) — Local deviation.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_MouseTracker_DropFrames_1.htm)

#### `public void DropFrames(double deviation, Point anchor)`

Find the youngest frame that deviates too much from the given locus and drop it and everything preceding it.

**Parameters:**
- `deviation` (System.Double) — Local deviation.
- `anchor` (System.Drawing.Point)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_MouseTracker_DropFrames_2.htm)

#### `public int LastFrameWithinDeviation(double deviation)`

Starting from the youngest frame, finds the oldest frame that is still within a maximum deviation of the youngest frame.

**Parameters:**
- `deviation` (System.Double) — Maximum allowed deviation.

**Returns:** `Int32` — The index of the oldest frame with a distance less than the deviation from the youngest frame, or -1 if no such frame could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_MouseTracker_LastFrameWithinDeviation.htm)

#### `public void Record(Point position)`

Record a new mouse event frame. If this frame is closer than [Accuracy]ms to the previously recorded frame, it is not included.

**Parameters:**
- `position` (System.Drawing.Point) — Location of new frame.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_MouseTracker_Record.htm)

#### `public void Record(Point position, Object tag)`

Record a new mouse event frame. If this frame is closer than [Accuracy]ms to the previously recorded frame, it is not included.

**Parameters:**
- `position` (System.Drawing.Point) — Location of new frame.
- `tag` (System.Object)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_MouseTracker_Record_1.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_MouseTracker_ToString.htm)

### Properties
- `Duration` (TimeSpan, get) — Gets the timespan that has passed between the oldest and youngest recorded frames. This is not the same as the time that has passed since recording began.
- `Frame` (GH_TrackerFrame, get) — Gets the position (in control coordinates) of the frame at the given index.
- `FrameCount` (Int32, get) — Gets the number of recorded frames.
- `MaximumDuration` (TimeSpan, get/set) — Gets or sets the maximum allowed duration of the entire record. When new frames are recorded, old frames that are more distant than MaximumDuration will be dropped. The default duration is 10 seconds.
- `MaximumFrameCount` (Int32, get/set) — Gets or sets the maximum number of frames to keep in history. If more frames are added, the oldest ones will be dropped. The default is 16,384 frames.
- `NewestFrame` (GH_TrackerFrame, get) — Gets the most recent frame in recorded history.
- `OldestFrame` (GH_TrackerFrame, get) — Gets the oldest frame in recorded history.
- `TemporalAccuracy` (TimeSpan, get/set) — Gets or sets the maximum temporal accuracy. Frames that are closer together than the accuracy will not be recorded. The default accuracy equals 5ms.

## GH_NumberSliderSnappingEditor (class)

(No description provided in vendor docs for Grasshopper.GUI.GH_NumberSliderSnappingEditor.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_NumberSliderSnappingEditor.htm)

### Constructors
- `public GH_NumberSliderSnappingEditor()` — Initializes a new instance of the GH_NumberSliderSnappingEditor class

### Methods
#### `protected override void Dispose(bool disposing)`

Releases the unmanaged resources used by the GH_NumberSliderSnappingEditor and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_NumberSliderSnappingEditor_Dispose.htm)

### Properties
- `SnapLines` (List<String>, get) — 

## GH_PanelEditorControl (class)

(No description provided in vendor docs for Grasshopper.GUI.GH_PanelEditorControl.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_PanelEditorControl.htm)

### Constructors
- `public GH_PanelEditorControl()` — Initializes a new instance of the GH_PanelEditorControl class

### Methods
#### `protected override void Dispose(bool disposing)`

Releases the unmanaged resources used by the GH_PanelEditorControl and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_PanelEditorControl_Dispose.htm)

#### `protected override void OnPaint(PaintEventArgs e)`

(Overrides Control.OnPaint(PaintEventArgs).)

**Parameters:**
- `e` (System.Windows.Forms.PaintEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_PanelEditorControl_OnPaint.htm)

#### `protected override void OnPaintBackground(PaintEventArgs e)`

(Overrides ScrollableControl.OnPaintBackground(PaintEventArgs).)

**Parameters:**
- `e` (System.Windows.Forms.PaintEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_PanelEditorControl_OnPaintBackground.htm)

#### `public void PickNewFont()`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_PanelEditorControl_PickNewFont.htm)

### Properties
- `PanelContent` (String, get/set) — Gets or sets the panel text on display.
- `PanelProperties` (GH_PanelProperties, get/set) — Gets or sets the panel properties on display.

## GH_QuickImageEditor (class)

(No description provided in vendor docs for Grasshopper.GUI.GH_QuickImageEditor.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_QuickImageEditor.htm)

### Constructors
- `public GH_QuickImageEditor()` — Initializes a new instance of the GH_QuickImageEditor class

### Methods
#### `protected override void Dispose(bool disposing)`

Releases the unmanaged resources used by the GH_QuickImageEditor and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_QuickImageEditor_Dispose.htm)

### Properties
- `CanvasImage` (Bitmap, get/set) — Gets or sets the canvas image.
- `FrameCrop` (Rectangle, get/set) — Gets or sets the cropping rectangle.
- `FrameFocus` (Rectangle3d, get/set) — Gets or sets the rectangle of the focus frame.
- `FrameViewport` (Rectangle3d, get/set) — Gets or sets the rectangle of the viewport frame.
- `ViewportImage` (Bitmap, get/set) — Gets or sets the viewport image.
- `ViewportName` (String, get/set) — Gets or sets the viewport name to include. If the name string is null or empty, the active viewport is used.

## GH_RotationalFrame (struct)

Represents a single frame in a rotational tuning history.

**Remarks:** See AlsoReferenceGrasshopper.GUI Namespace

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_RotationalFrame.htm)

### Properties
- `Angle` (Double, get/set) — Gets or sets the local angle of the rotation event.
- `Pivot` (PointF, get/set) — Gets or sets the pivot of the event.
- `Point` (PointF, get/set) — Gets or sets the location of the rotation event.
- `Time` (DateTime, get/set) — Gets or sets the time when the event was recorded.

## GH_RotationalTuningData<T> (class)

Utility class for keeping track of rotational mouse events.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_RotationalTuningData_1.htm)

### Constructors
- `public GH_RotationalTuningData(T initialValue, PointF pivot)` — Create a new instance of the rotational tuning data.
- `public GH_RotationalTuningData(T initialValue, PointF pivot, PointF initialPoint)` — Create a new instance of the rotational tuning data.

### Methods
#### `public void AddFrame(PointF point)`

Grow the tuning track by one point and compute the angle.

**Parameters:**
- `point` (System.Drawing.PointF) — Point (in canvas coordinates) to record.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_RotationalTuningData_1_AddFrame.htm)

#### `public double TotalAngle()`

Compute the total angle from the first to the last frame.

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_RotationalTuningData_1_TotalAngle.htm)

#### `public double TotalTurns()`

Compute the total number of full turns from the first to the last frame.

**Returns:** `Double` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_RotationalTuningData_1_TotalTurns.htm)

### Properties
- `Frame` (GH_RotationalFrame, get) — Gets the frame at the given index.
- `FrameCount` (Int32, get) — Gets the total number of recorded frames.
- `InitialValue` (T, get) — Gets the initial value.
- `Pivot` (PointF, get/set) — Gets or sets the rotational pivot. You only need to set this if the pivot changes.

## GH_SamplingSphere (class)

Represents a coloured sphere that can be sampled at various rotations.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_SamplingSphere.htm)

### Constructors
- `public GH_SamplingSphere(int size)` — Initializes a new instance of the GH_SamplingSphere class

### Methods
#### `public Point2d[, ] SampleSphere(Sphere sphere)`

Sample the UV locations of a sphere. The sphere should be centered on the world origin.

**Parameters:**
- `sphere` (Sphere) — Sphere to sample.

**Returns:** `Point2d[,]` — A two-dimensional array of Sphere UV locations, one for each sample point.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_SamplingSphere_SampleSphere.htm)

#### `public Bitmap SampleSphere(Sphere sphere, GH_MemoryBitmap texture)`

Sample a sphere with a UV texture and return a bitmap of sampled values.

**Parameters:**
- `sphere` (Sphere) — Sphere to sample, it should be centered on the world origin.
- `texture` (Grasshopper.Kernel.GH_MemoryBitmap) — Texture to sample, the texture will not be modified.

**Returns:** `Bitmap` — A bitmap of Size x Size pixels.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_SamplingSphere_SampleSphere_1.htm)

### Properties
- `SampleCount` (Int32, get) — Gets the total number of samples contained in this sampling grid.
- `Size` (Int32, get) — Gets the size of this sample grid.

## GH_SeparatorDisplay (enum)

Enumerates all possible display modes for horizontal and vertical separators.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_SeparatorDisplay.htm)

### Values
- `None` = `0` — No separator is drawn.
- `Etched` = `1` — A double line etched separator is drawn.
- `EtchedFading` = `2` — A double line etched separator with fading extremes is drawn.
- `Dashed` = `3` — A single line dashed separator is drawn.

## GH_SettingsCategory (class)

Base implementation of IGH_SettingsCategory. Derive from this class and provide an empty constructor to play ball.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_SettingsCategory.htm)

### Constructors
- `protected GH_SettingsCategory(string name, Bitmap icon)` — Create a new top-level category.
- `protected GH_SettingsCategory(string name, string parent, Bitmap icon)` — Create a new nested category.

### Properties
- `Description` (String, get) — 
- `Icon` (Bitmap, get) — 
- `Name` (String, get) — 
- `Parent` (String, get) — 

## GH_Slider (class)

Provides a standard Grasshopper slider as a winforms control.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_Slider.htm)

### Constructors
- `public GH_Slider()` — Initializes a new instance of the GH_Slider class

### Methods
#### `protected override void Input_Assign(string text)`

(Overrides GH_TextInputBaseControl.Input_Assign(String).)

**Parameters:**
- `text` (System.String)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Slider_Input_Assign.htm)

#### `protected override bool Input_Parse(string text)`

(Overrides GH_TextInputBaseControl.Input_Parse(String).)

**Parameters:**
- `text` (System.String)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Slider_Input_Parse.htm)

#### `protected override string Input_Supply()`

(Overrides GH_TextInputBaseControl.Input_Supply().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Slider_Input_Supply.htm)

#### `protected override void OnPaint(PaintEventArgs e)`

(Overrides Control.OnPaint(PaintEventArgs).)

**Parameters:**
- `e` (System.Windows.Forms.PaintEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Slider_OnPaint.htm)

#### `protected override void OnPaintBackground(PaintEventArgs e)`

(Overrides ScrollableControl.OnPaintBackground(PaintEventArgs).)

**Parameters:**
- `e` (System.Windows.Forms.PaintEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Slider_OnPaintBackground.htm)

### Properties
- `ControlEdgeColour` (Color, get/set) — Gets or sets the EdgeColor for this slider control.
- `ControlShadowColour` (Color, get/set) — Gets or sets the Shadow colour for this control.
- `DecimalPlaces` (Int32, get/set) — Gets or sets the number of decimal places allowed for Floating Point Sliders.
- `DrawControlBorder` (Boolean, get/set) — Gets or sets whether the border of the slider should be drawn.
- `DrawControlShadows` (Boolean, get/set) — Gets or sets whether the drop shadows of the slider should be drawn.
- `FormatMask` (String, get/set) — Gets or sets the string format mask to use for the display. Should contain at least one "{0}" element.
- `GripBottomColour` (Color, get/set) — Gets or sets the fill colour for the bottom edge of the slider grip.
- `GripDisplay` (GH_SliderGripDisplay, get/set) — Gets or sets how the slider grip (or thumb) is drawn.
- `GripEdgeColour` (Color, get/set) — Gets or sets the edge colour for the slider grip.
- `GripTopColour` (Color, get/set) — Gets or sets the fill colour for the top edge of the slider grip.
- `Maximum` (Decimal, get/set) — Gets or sets the upper numeric limit for the slider range.
- `Minimum` (Decimal, get/set) — Gets or sets the lower numeric limit for the slider range.
- `RailBrightColour` (Color, get/set) — Gets or sets the edge colour for the bright parts of the rail.
- `RailDarkColour` (Color, get/set) — Gets or sets the edge colour for the dark parts of the rail.
- `RailDisplay` (GH_SliderRailDisplay, get/set) — Gets or sets how the slider rail (or track) is drawn.
- `RailEmptyColour` (Color, get/set) — Gets or sets the fill colour for the empty portion of the rail.
- `RailFullColour` (Color, get/set) — Gets or sets the fill colour for the full portion of the rail.
- `ShadowSize` (Padding, get/set) — Gets or sets the size of the border shadow along all edges of the box.
- `TextColour` (Color, get/set) — Gets or sets the fill colour for the slider text.
- `TickCount` (Int32, get/set) — Gets or sets the number of ticks.
- `TickDisplay` (GH_SliderTickDisplay, get/set) — Gets or sets how the slider ticks are drawn.
- `TickFrequency` (Int32, get/set) — Gets or sets the frequency of large ticks vs small ticks.
- `Type` (GH_SliderAccuracy, get/set) — Gets or sets the numeric type of this slider.
- `Value` (Decimal, get/set) — Gets or sets the numeric value for the slider range.

### Events
#### `ValueChanged` (`Grasshopper.GUI.GH_Slider.ValueChangedEventHandler`)

**Signature:** `public event GH_Slider.ValueChangedEventHandler ValueChanged`

Raised whenever the value of the slider is changed via GUI interaction.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_GH_Slider_ValueChanged.htm)

## GH_Slider.ValueChangedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.GH_Slider.ValueChangedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_Slider_ValueChangedEventHandler.htm)

## GH_SliderInputMode (enum)

Enumerates all possible slider text input modes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_SliderInputMode.htm)

### Values
- `None` = `0` — Text input mode is not available.
- `OnKeyDown` = `1` — Text input mode will be available on keydown.
- `OnDoubleClick` = `2` — Text input mode will be available upon double click.
- `All` = `3` — All text input modes will be available.

## GH_Slider_Obsolete (class)

General purpose numeric slider.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_Slider_Obsolete.htm)

### Constructors
- `public GH_Slider_Obsolete()` — Initializes a new instance of the GH_Slider_Obsolete class

### Methods
#### `protected override void Dispose(bool disposing)`

Releases the unmanaged resources used by the GH_Slider_Obsolete and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Slider_Obsolete_Dispose.htm)

#### `protected override void OnPaint(PaintEventArgs e)`

(Overrides Control.OnPaint(PaintEventArgs).)

**Parameters:**
- `e` (System.Windows.Forms.PaintEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Slider_Obsolete_OnPaint.htm)

#### `protected override void OnPaintBackground(PaintEventArgs e)`

(Overrides ScrollableControl.OnPaintBackground(PaintEventArgs).)

**Parameters:**
- `e` (System.Windows.Forms.PaintEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Slider_Obsolete_OnPaintBackground.htm)

#### `public void OnValueChanged()`

Raise the ValueChanged event

**Remarks:** See AlsoReferenceGH_Slider_Obsolete ClassGrasshopper.GUI Namespace

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Slider_Obsolete_OnValueChanged.htm)

#### `public static IntPtr SendMessage(IntPtr hWnd, int msg, int wParam, IntPtr lParam)`



**Parameters:**
- `hWnd` (System.IntPtr)
- `msg` (System.Int32)
- `wParam` (System.Int32)
- `lParam` (System.IntPtr)

**Returns:** `IntPtr` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Slider_Obsolete_SendMessage.htm)

### Properties
- `DisplayFormat` (String, get/set) — Gets or sets the display format fo the slider. This is the formatting that is applied prior to drawing. The default is {0} which results in a 1:1 display. Use this to append units or symbols.
- `DrawRail` (Boolean, get/set) — Gets or sets a flag that control whether or not grip rails are drawn.
- `GripBackColor` (Color, get/set) — Background colour of slider grip.
- `GripForeColor` (Color, get/set) — Foreground colour of slider grip.
- `IsIntermediate` (Boolean, get) — Gets the intermediate flag. If True, the user is currently dragging the slider.
- `Max` (Double, get/set) — Gets or sets the maximum allowed value.
- `Min` (Double, get/set) — Gets or sets the minimum allowed value.
- `NumericFormat` (String, get/set) — Gets or sets the numeric format for the slider. This is the formatting that is applied to the number. The default is #0.00 which results in a two digit rounding scheme.
- `TextInputMode` (GH_SliderInputMode, get/set) — Gets or sets the Text input mode for this slider.
- `TickFrequency` (Int32, get/set) — Gets or sets the tick frequency of the slider.
- `Value` (Double, get/set) — Gets or sets the current value.
- `ValueF` (Single, get/set) — Gets or sets the current value in single-floating-point precision.

### Events
#### `ValueChanged` (`Grasshopper.GUI.GH_Slider_Obsolete.ValueChangedEventHandler`)

**Signature:** `public event GH_Slider_Obsolete.ValueChangedEventHandler ValueChanged`

Raised whenever the value of the slider is changed via GUI interaction.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_GH_Slider_Obsolete_ValueChanged.htm)

## GH_Slider_Obsolete.ValueChangedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.GH_Slider_Obsolete.ValueChangedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_Slider_Obsolete_ValueChangedEventHandler.htm)

## GH_StandardIcons (class)

Provides access to a set of standard icons.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_StandardIcons.htm)

### Properties
- `BlankObjectIcon_24x24` (Bitmap, get) — Gets the base icon for objects; an empty white square.
- `BlankParameterIcon_24x24` (Bitmap, get) — Gets the base icon for parameters; an empty black hexagon.
- `ClusterIcon_24x24` (Bitmap, get) — Gets the icon used for clusters.
- `FileIconDll_32x32` (Bitmap, get) — Gets the file icon used for Dll files.
- `FileIconExe_32x32` (Bitmap, get) — Gets the file icon used for Exe files.
- `FileIconGH_32x36` (Bitmap, get) — Gets the file icon used for Grasshopper Binary files.
- `FileIconGha_32x32` (Bitmap, get) — Gets the file icon used for Gha files.
- `FileIconGHMissing_32x36` (Bitmap, get) — Gets the file icon used for missing Grasshopper Binary files.
- `FileIconGHX_32x36` (Bitmap, get) — Gets the file icon used for Grasshopper Xml files.
- `FileIconGHXMissing_32x36` (Bitmap, get) — Gets the file icon used for missing Grasshopper Xml files.
- `FileIconRhp_32x32` (Bitmap, get) — Gets the file icon used for Rhp files.
- `FlattenIcon_24x24` (Bitmap, get) — Gets the trunk icon used to indicate flattening.
- `GraftIcon_24x24` (Bitmap, get) — Gets the twig icon used to indicate grafting.
- `GroupIcon_24x24` (Bitmap, get) — Gets the icon used for canvas groups.
- `UnknownObjectIcon_24x24` (Bitmap, get) — Gets the icon used for objects that do not have their own icon and unknown objects.
- `UserObjectIcon_24x24` (Bitmap, get) — Gets the icon used for user objects.

## GH_Tooltip (class)

Implements a custom Tooltip class with advanced functionality. All methods and fields are Shared. There can never be more than one GH_Tooltip.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_Tooltip.htm)

### Methods
#### `public static void Adjust()`

Repositions the tooltip to match with new cursor positions.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Tooltip_Adjust.htm)

#### `public static void AssignTooltipFields(string title = null, string text = null, string description = null, Image icon = null, Image diagram = null)`

Set all fields of the Tooltip. You'll still need to call Show() to display the tooltip.

**Parameters:**
- `title` (System.String) — The Title text on the tooltip
- `text` (System.String) — The text content of the tooltip
- `description` (System.String) — The block text to be displayed in the sunken area of the tooltip.
- `icon` (System.Drawing.Image) — The Icon to be displayed next to the Title.
- `diagram` (System.Drawing.Image) — The image to be displayed in the sunken area.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Tooltip_AssignTooltipFields.htm)

#### `public static void Clear()`

Hides the tooltip and clears all references and caches.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Tooltip_Clear.htm)

#### `public static bool IsOwner(Control test)`

Returns true if the test control matches the internal Owner control. Use this method to make sure you actually own this tooltip prior to clearing it.

**Remarks:** See AlsoReferenceGH_Tooltip ClassGrasshopper.GUI Namespace

**Parameters:**
- `test` (System.Windows.Forms.Control)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Tooltip_IsOwner.htm)

#### `public static bool IsTag(Object test)`

Returns True if the test object matches the internal Tag object

**Remarks:** See AlsoReferenceGH_Tooltip ClassGrasshopper.GUI Namespace

**Parameters:**
- `test` (System.Object)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Tooltip_IsTag.htm)

#### `public static void Layout()`

Layout the tooltip fields. Layout happens automatically if you use the AssigTooltipField method, if however you poke values directly in the tooltip form you must place a call to Layout() before showing the tooltip.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Tooltip_Layout.htm)

#### `public static void Show(Control owner)`

Display the tooltip on screen.

**Parameters:**
- `owner` (System.Windows.Forms.Control)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Tooltip_Show.htm)

#### `public static string TooltipDetailedInformation()`



**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_Tooltip_TooltipDetailedInformation.htm)

### Properties
- `Tag` (Object, get/set) — Gets or sets the tag object for the tooltip
- `TooltipForm` (GH_ToolTipForm, get) — Gets the internal instance of GH_TooltipForm.

## GH_TooltipComponent (class)

This component provides Grasshopper tooltip functionality through a winforms Component.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_TooltipComponent.htm)

### Constructors
- `public GH_TooltipComponent()` — Initializes a new instance of the GH_TooltipComponent class

### Methods
#### `public void Hide()`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_TooltipComponent_Hide.htm)

#### `public bool IsTag(Object object)`

Test whether the current Tooltip tag matches the object.

**Parameters:**
- `object` (System.Object) — Object to test against tag.

**Returns:** `Boolean` — True if the Tag and object are the same.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_TooltipComponent_IsTag.htm)

### Properties
- `Delay` (Int32, get/set) — Gets or sets the delay (in milliseconds) for this tooltip to appear.
- `Enabled` (Boolean, get/set) — Gets or sets the Enabled flag.
- `Tag` (Object, get/set) — Provides an easy way to store information on this Tooltip.
- `Target` (Control, get/set) — Gets or sets the control this tooltip is tied to.

### Events
#### `PopulateTooltip` (`Grasshopper.GUI.GH_TooltipComponent.PopulateTooltipEventHandler`)

**Signature:** `public event GH_TooltipComponent.PopulateTooltipEventHandler PopulateTooltip`

This event is raised just prior to the tooltip display.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_GH_TooltipComponent_PopulateTooltip.htm)

## GH_TooltipComponent.PopulateTooltipEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.GH_TooltipComponent.PopulateTooltipEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_TooltipComponent_PopulateTooltipEventHandler.htm)

## GH_TooltipDisplayEventArgs (class)

Event arguments used in the Tooltip Component.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_TooltipDisplayEventArgs.htm)

### Properties
- `Control` (Control, get) — Gets the control for whom the tooltip is about to be displayed.
- `Description` (String, get/set) — Gets or sets the description of the tooltip. This field is optional.
- `Diagram` (Bitmap, get/set) — Gets or sets the image that is displayed in the details section of the tooltip. This field is optional, but when it is defined, the Description property is ignored.
- `Icon` (Bitmap, get/set) — Gets or sets the icon that is displayed in the upper left hand corner of the tooltip. This field is optional.
- `Point` (Point, get) — Gets the point in control coordinates for the tooltip locus.
- `Region` (Rectangle, get/set) — Gets or sets the active region (in control coordinates) for this tooltip. Once a tooltip is shown inside the region, the tooltip will 'stick' to the mouse for as long as the mouse remains within this region. By default the region is set to Rectangle.Empty, which indicates that region tracking is not used for this tooltip.
- `Text` (String, get/set) — Gets or sets the text of the tooltip. If you do not set the Text property, you must set the Title property or the tooltip will not be shown.
- `Title` (String, get/set) — Gets or sets the title of the tooltip. If you do not set the Title property, you must set the Text property or the tooltip will not be shown.
- `Valid` (Boolean, get) — Gets whether there is currently enough data to display a tooltip.

## GH_TrackerFrame (struct)

Represents a single frame in a mouse-tracker history. A frame represents all relevant mouse and keyboard data at a given time.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_TrackerFrame.htm)

### Constructors
- `public GH_TrackerFrame(Point position)` — Initializes a new instance of the GH_TrackerFrame class

### Properties
- `Buttons` (MouseButtons, get) — The state of the mouse buttons of the frame.
- `Keys` (Keys, get) — The state of the modifier keys (Contro, Shift and Alt) of the frame.
- `PositionControl` (Point, get) — The position (in control coordinates) of the frame.
- `PositionScreen` (Point, get) — The position (in screen coordinates) of the frame.
- `Tag` (Object, get) — Optional Tag object (to be supplied by user).
- `Time` (DateTime, get) — The time at which this frame was recorded.

## GH_UnrecognizedObjectsForm (class)

(No description provided in vendor docs for Grasshopper.GUI.GH_UnrecognizedObjectsForm.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_UnrecognizedObjectsForm.htm)

### Constructors
- `public GH_UnrecognizedObjectsForm()` — Initializes a new instance of the GH_UnrecognizedObjectsForm class

### Methods
#### `protected override void Dispose(bool disposing)`

Releases the unmanaged resources used by the GH_UnrecognizedObjectsForm and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_UnrecognizedObjectsForm_Dispose.htm)

## GH_VerticalScrollBar (class)

Scroll bar control with floating point accuracy.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_VerticalScrollBar.htm)

### Constructors
- `public GH_VerticalScrollBar()` — Initializes a new instance of the GH_VerticalScrollBar class

### Methods
#### `protected override void OnPaint(PaintEventArgs e)`

(Overrides Control.OnPaint(PaintEventArgs).)

**Parameters:**
- `e` (System.Windows.Forms.PaintEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_VerticalScrollBar_OnPaint.htm)

#### `protected override void OnPaintBackground(PaintEventArgs e)`

(Overrides ScrollableControl.OnPaintBackground(PaintEventArgs).)

**Parameters:**
- `e` (System.Windows.Forms.PaintEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_VerticalScrollBar_OnPaintBackground.htm)

### Properties
- `Grip` (Rectangle, get) — Gets the actual dimensions of the scroll bar grip.
- `GripRegion` (Rectangle, get) — Gets the grip region for this scroll bar. The Grip Region is the area in which the grip can be found. It is essentially the Client area minus the padding.
- `ImpliedVerticalOffset` (Int32, get/set) — Gets or sets the offset of the target region as implied by the scroll ratio. If you set this value, the ratio is reverse engineered.
- `ScreenHeight` (Int32, get/set) — Gets or sets the height of the screen area (the visible area)
- `TargetHeight` (Int32, get/set) — Gets or sets the height of the target area (the area being scrolled)
- `TargetRatio` (Double, get/set) — Gets or sets the scroll ration. If the scroll ratio is changed, the OnScrollRatioChanged event will be raised.

### Events
#### `ScrollRatioChanged` (`Grasshopper.GUI.GH_VerticalScrollBar.ScrollRatioChangedEventHandler`)

**Signature:** `public event GH_VerticalScrollBar.ScrollRatioChangedEventHandler ScrollRatioChanged`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_GH_VerticalScrollBar_ScrollRatioChanged.htm)

## GH_VerticalScrollBar.ScrollRatioChangedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.GH_VerticalScrollBar.ScrollRatioChangedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_VerticalScrollBar_ScrollRatioChangedEventHandler.htm)

## GH_VerticalSeparator (class)

GUI elements for creating visual separations.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_VerticalSeparator.htm)

### Constructors
- `public GH_VerticalSeparator()` — Initializes a new instance of the GH_VerticalSeparator class

### Methods
#### `protected override void OnPaint(PaintEventArgs e)`

(Overrides Control.OnPaint(PaintEventArgs).)

**Parameters:**
- `e` (System.Windows.Forms.PaintEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_VerticalSeparator_OnPaint.htm)

#### `protected override void OnPaintBackground(PaintEventArgs e)`

(Overrides ScrollableControl.OnPaintBackground(PaintEventArgs).)

**Parameters:**
- `e` (System.Windows.Forms.PaintEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_VerticalSeparator_OnPaintBackground.htm)

## GH_VoronoiWarningForm (class)

(No description provided in vendor docs for Grasshopper.GUI.GH_VoronoiWarningForm.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_VoronoiWarningForm.htm)

### Constructors
- `public GH_VoronoiWarningForm()` — Initializes a new instance of the GH_VoronoiWarningForm class

### Methods
#### `protected override void Dispose(bool disposing)`

Releases the unmanaged resources used by the GH_VoronoiWarningForm and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_VoronoiWarningForm_Dispose.htm)

## GH_WindowsControlUtil (class)

Exposes some static (Shared) utility functions for setting text-rendering properties en masse.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_WindowsControlUtil.htm)

### Methods
#### `public static void FixTextRenderingDefault(Control control)`

Sets the UseCompatibleTextRendering value to False on the control.

**Parameters:**
- `control` (System.Windows.Forms.Control)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_WindowsControlUtil_FixTextRenderingDefault_1.htm)

#### `public static void FixTextRenderingDefault(Control.ControlCollection iControls)`

Recursively sets the UseCompatibleTextRendering value to False on each control.

**Parameters:**
- `iControls` (System.Windows.Forms.Control.ControlCollection)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_WindowsControlUtil_FixTextRenderingDefault_2.htm)

#### `public static void FixTextRenderingDefault(IEnumerable<Control> C)`



**Parameters:**
- `C` (System.Collections.Generic.IEnumerable<Control>)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_WindowsControlUtil_FixTextRenderingDefault.htm)

## GH_WindowsFormUtil (class)

Exposes some static (Shared) utility functions for positioning custom windows.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_WindowsFormUtil.htm)

### Methods
#### `public static void CenterFormOnCursor(Form F, bool limitToScreen)`

Positions a Form so that it is centered around the cursor

**Parameters:**
- `F` (System.Windows.Forms.Form) — The Form to adjust
- `limitToScreen` (System.Boolean) — If True, the Form won't be allowed to extend beyond the limits of the screen containing the cursor.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_WindowsFormUtil_CenterFormOnCursor_1.htm)

#### `public static void CenterFormOnCursor(Window F, bool limitToScreen)`

Positions a Form so that it is centered around the cursor

**Parameters:**
- `F` (Window) — The Form to adjust
- `limitToScreen` (System.Boolean) — If True, the Form won't be allowed to extend beyond the limits of the screen containing the cursor.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_WindowsFormUtil_CenterFormOnCursor.htm)

#### `public static void CenterFormOnEditor(Form F, bool limitToScreen)`

Positions a Form so that it is centered on the active editor. If the editor doesn't exist or is hidden, the form will be centered on the screen instead.

**Parameters:**
- `F` (System.Windows.Forms.Form) — The Form to adjust
- `limitToScreen` (System.Boolean) — If True, the Form won't be allowed to extend beyond the limits of the screen containing the cursor.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_WindowsFormUtil_CenterFormOnEditor_1.htm)

#### `public static void CenterFormOnEditor(Window F, bool limitToScreen)`

Positions a Form so that it is centered on the active editor. If the editor doesn't exist or is hidden, the form will be centered on the screen instead.

**Parameters:**
- `F` (Window) — The Form to adjust
- `limitToScreen` (System.Boolean) — If True, the Form won't be allowed to extend beyond the limits of the screen containing the cursor.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_WindowsFormUtil_CenterFormOnEditor.htm)

#### `public static void CenterFormOnScreen(Form F, bool limitToScreen)`

Positions a Form so that it is centered on the current screen. The current screen being defined as the screen containing the cursor.

**Parameters:**
- `F` (System.Windows.Forms.Form) — The Form to adjust
- `limitToScreen` (System.Boolean) — If True, the Form won't be allowed to extend beyond the limits of the active screen.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_WindowsFormUtil_CenterFormOnScreen_1.htm)

#### `public static void CenterFormOnScreen(Window F, bool limitToScreen)`

Positions a Form so that it is centered on the current screen. The current screen being defined as the screen containing the cursor.

**Parameters:**
- `F` (Window) — The Form to adjust
- `limitToScreen` (System.Boolean) — If True, the Form won't be allowed to extend beyond the limits of the active screen.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_WindowsFormUtil_CenterFormOnScreen.htm)

#### `public static void CenterFormOnWindow(Form F, Form parentWindow, bool limitToScreen)`

Positions a Form so that it is centered on the supplied window. If the window doesn't exist or is hidden, the form will be centered on the screen instead.

**Parameters:**
- `F` (System.Windows.Forms.Form) — The Form to adjust
- `parentWindow` (System.Windows.Forms.Form)
- `limitToScreen` (System.Boolean) — If True, the Form won't be allowed to extend beyond the limits of the screen containing the cursor.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_WindowsFormUtil_CenterFormOnWindow_1.htm)

#### `public static void CenterFormOnWindow(Window F, Window parentWindow, bool limitToScreen)`

Positions a Form so that it is centered on the supplied window. If the window doesn't exist or is hidden, the form will be centered on the screen instead.

**Parameters:**
- `F` (Window) — The Form to adjust
- `parentWindow` (Window)
- `limitToScreen` (System.Boolean) — If True, the Form won't be allowed to extend beyond the limits of the screen containing the cursor.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_GH_WindowsFormUtil_CenterFormOnWindow.htm)

## GH_YakDownloadFormEto (class)

(No description provided in vendor docs for Grasshopper.GUI.GH_YakDownloadFormEto.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_YakDownloadFormEto.htm)

### Constructors
- `public GH_YakDownloadFormEto(IEnumerable<GH_YakDownloadFormEto.MissingObjectItem> items)` — Initializes a new instance of the GH_YakDownloadFormEto class

## GH_YakDownloadFormEto.MissingObjectItem (class)

(No description provided in vendor docs for Grasshopper.GUI.GH_YakDownloadFormEto.MissingObjectItem.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_GH_YakDownloadFormEto_MissingObjectItem.htm)

### Constructors
- `public MissingObjectItem()` — Initializes a new instance of the GH_YakDownloadFormEto.MissingObjectItem class

### Properties
- `Id` (Guid, get/set) — 
- `Name` (String, get/set) — 
- `Version` (String, get/set) — 

## IGH_Embeddable (interface)

Implement this interface in your custom control if you want to be called before a menu commits or cancels.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_IGH_Embeddable.htm)

### Methods
#### `void PrepareForCancel()`

This will be called prior to a Cancel operation.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_IGH_Embeddable_PrepareForCancel.htm)

#### `void PrepareForCommit()`

This will be called prior to a Commit operation.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_IGH_Embeddable_PrepareForCommit.htm)

## IGH_FileDropTarget (interface)

Implement this interface in your IGH_DocumentObject if you want to participate in Drag+Drop events.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_IGH_FileDropTarget.htm)

### Methods
#### `bool AcceptableFile(string path)`

Gets a value indicating whether or not the path can be handled by the local object.

**Parameters:**
- `path` (System.String) — Path to file in question.

**Returns:** `Boolean` — True if the file can be handled, false if otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_IGH_FileDropTarget_AcceptableFile.htm)

#### `bool HandleDrop(string path, PointF mouse_pt)`

Handle a file drop.

**Parameters:**
- `path` (System.String) — Path of file to handle.
- `mouse_pt` (System.Drawing.PointF) — Point at which the drop occured.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_IGH_FileDropTarget_HandleDrop.htm)

## IGH_SettingCategory (interface)

Represents a single category in the Settings UI. Implement this interface or derive from GH_SettingsCategory to add a new Category to the Grasshopper Settings interface.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_IGH_SettingCategory.htm)

### Properties
- `Description` (String, get) — Category description.
- `Icon` (Bitmap, get) — Category icon. Image should be 48x48 pixels.
- `Name` (String, get) — Category name (not case sensitive). Use dots to separate subcategories.
- `Parent` (String, get) — Gets the name of the parent category. If the category is supposed to be top-level, this property will return null or a String.Empty.

## IGH_SettingFrontend (interface)

Represents a single setting in the Settings UI. Implement this interface to participate in the Grasshopper Settings interface.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_IGH_SettingFrontend.htm)

### Methods
#### `Control SettingsUI()`

Create the winforms UI portion for this setting.

**Returns:** `Control` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_IGH_SettingFrontend_SettingsUI.htm)

### Properties
- `Category` (String, get) — Category of this setting.
- `Keywords` (IEnumerable<String>, get) — Optional keywords for this setting.
- `Name` (String, get) — Name of this setting.

## IGH_Tooltip (interface)

Accessor interface for the Grasshopper Tooltip form.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_IGH_Tooltip.htm)

### Properties
- `Description` (String, get/set) — Gets or sets the tooltip Description. The Description is optional and is displayed in the same area as the TooltipDiagram image.
- `Diagram` (Image, set) — Sets the tooltip diagram image. The diagram image is optional and is displayed in the same area as the ToolTipDescription text.
- `Icon` (Image, set) — Sets the Tooltip Icon image.
- `Palette` (GH_TooltipPalette, get/set) — Gets or sets the tooltip colour palette.
- `Text` (String, get/set) — Gets or sets the tooltip Text.
- `Title` (String, get/set) — Gets or sets the tooltip Title.

