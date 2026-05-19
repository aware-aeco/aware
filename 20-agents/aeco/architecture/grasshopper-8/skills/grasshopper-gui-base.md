---
name: grasshopper-grasshopper-gui-base
description: This skill encodes the grasshopper 8.0 surface of the Grasshopper.GUI.Base namespace — 25 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GH_ColourCube, GH_ColourPickerBase, GH_ColourPickerEventArgs, GH_DigitNumber, GH_DigitScrollerBase, GH_DigitScrollerEventArgs, GH_ScrollBarVerticalBase, GH_SliderBase, and 17 more types.
---

# Grasshopper.GUI.Base

Auto-generated from vendor docs for grasshopper 8.0. 25 types in this namespace.

## GH_ColourCube (class)

Maintains a collection of graphical shapes and coordinates that specify important features of the Colour Space Cube graphics.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_ColourCube.htm)

### Constructors
- `public GH_ColourCube(Rectangle box, GH_ColourSpace space, Point4d color)` — Create a new instance of this class.

### Methods
#### `public Color Average(Color A, Color B)`

Compute the average of two colours.

**Parameters:**
- `A` (System.Drawing.Color) — First colour for average.
- `B` (System.Drawing.Color) — Second colour for average.

**Returns:** `Color` — The average of A and B.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourCube_Average.htm)

#### `public Color Average(Color A, Color B, Color C, Color D)`

Compute the average of four colours.

**Parameters:**
- `A` (System.Drawing.Color) — First colour for average.
- `B` (System.Drawing.Color) — Second colour for average.
- `C` (System.Drawing.Color) — Third colour for average.
- `D` (System.Drawing.Color) — Fourth colour for average.

**Returns:** `Color` — The average of A, B, C and D.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourCube_Average_1.htm)

#### `public Color Blend(Color A, Color B, double t)`

Inteprolate between two colours.

**Parameters:**
- `A` (System.Drawing.Color) — First colour.
- `B` (System.Drawing.Color) — Second colour.
- `t` (System.Double) — Interpolation parameter.

**Returns:** `Color` — Interpolated colour.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourCube_Blend.htm)

#### `public Point4d ColorAtRail(PointF pt)`

Get the color at a position along the depth rail.

**Parameters:**
- `pt` (System.Drawing.PointF) — Point along rail (will be projected into rail limits)

**Returns:** `Point4d` — Point4d structure representing the colour in the current space.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourCube_ColorAtRail.htm)

#### `public Point4d ColorAtSlice(PointF pt)`

Get the color at a position on the slice.

**Parameters:**
- `pt` (System.Drawing.PointF) — Point on slice (will be projected into slice limits)

**Returns:** `Point4d` — Point4d structure representing the colour in the current space.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourCube_ColorAtSlice.htm)

#### `public void RenderAll(Graphics G)`

Call all render methods in the correct order.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourCube_RenderAll.htm)

#### `public void RenderBackEdges(Graphics G)`

Render the edges of the back face creases.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourCube_RenderBackEdges.htm)

#### `public void RenderBackFaces(Graphics G)`

Render the fills of the backfaces.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourCube_RenderBackFaces.htm)

#### `public void RenderDropShadow(Graphics G)`

Render the drop shadow of the cube.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourCube_RenderDropShadow.htm)

#### `public void RenderEdgeShadows(Graphics G)`

Render the edge shadows of the cube due to GI.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourCube_RenderEdgeShadows.htm)

#### `public void RenderForeEdges(Graphics G)`

Render the edges of the fore face creases.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourCube_RenderForeEdges.htm)

#### `public void RenderGrip(Graphics G)`

Render the slice grip.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourCube_RenderGrip.htm)

#### `public void RenderPivot(Graphics G)`

Render the colour pivot on the current slice.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourCube_RenderPivot.htm)

#### `public void RenderSilhouetteEdges(Graphics G)`

Render the edges of the cube silhouettes.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourCube_RenderSilhouetteEdges.htm)

#### `public void RenderSlice(Graphics G)`

Render the square slice with colour gradients.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourCube_RenderSlice.htm)

#### `public void RenderSliceDropShadow(Graphics G)`

Render the drop shadow onto the slice.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourCube_RenderSliceDropShadow.htm)

#### `public void RenderSliceEdgeShadows(Graphics G)`

Render the edge shadows of the slice due to GI.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourCube_RenderSliceEdgeShadows.htm)

#### `public void RenderSliceSilhouetteEdges(Graphics G)`

Render the silhouette edges of the slice.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourCube_RenderSliceSilhouetteEdges.htm)

### Properties
- `BackFace` (GraphicsPath, get) — Gets a graphics path describing the back face of the cube. Caller is responsible for disposing of this path instance.
- `BottomFace` (GraphicsPath, get) — Gets a graphics path describing the bottom face of the cube. Caller is responsible for disposing of this path instance.
- `C0` (Point, get) — Gets the first point in the corner point list.
- `C1` (Point, get) — Gets the second point in the corner point list.
- `C2` (Point, get) — Gets the third point in the corner point list.
- `C3` (Point, get) — Gets the fourth point in the corner point list.
- `C4` (Point, get) — Gets the fifth point in the corner point list.
- `C5` (Point, get) — Gets the sixth point in the corner point list.
- `C6` (Point, get) — Gets the seventh point in the corner point list.
- `C7` (Point, get) — Gets the eighth point in the corner point list.
- `Grip` (Rectangle, get) — Gets the rectangle that describes the slice grip.
- `LeftFace` (GraphicsPath, get) — Gets a graphics path describing the left face of the cube. Caller is responsible for disposing of this path instance.
- `Pivot` (Point, get) — Gets the point that describes the center of the pivot on the slice.
- `S0` (Point, get) — Gets the first point in the shadow point list. This point is coincident with C0.
- `S1` (Point, get) — Gets the second point in the shadow point list. This is the corner of the drop-shadow on the bottom face.
- `S2` (Point, get) — Gets the third point in the shadow point list. This point is the kink of the drop-shadow on the edge connecting C2 and C3.
- `S3` (Point, get) — Gets the fourth point in the shadow point list. This point is coincident with C7.
- `S4` (Point, get) — Gets the fifth point in the shadow point list. This point is coincident with C4.
- `Shadow` (GraphicsPath, get) — Gets a graphics path describing the outline of the drop shadow. Caller is responsible for disposing of this path instance.
- `Silhouette` (GraphicsPath, get) — Gets a graphics path describing the silhouette of the cube. Caller is responsible for disposing of this path instance.
- `Slice` (Rectangle, get) — Gets the rectangle that describes the cutting slide through the current colour space.

## GH_ColourPickerBase (class)

Provides Colour picker GUI not tied to a control-handle.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_ColourPickerBase.htm)

### Constructors
- `public GH_ColourPickerBase()` — Create a new instance of the Colour Picker base control. This constructor assigns the default Colour space model and involves a call to the Grasshopper core settings server. You must call SetupColourPicker() prior to rendering this control on screen.

### Methods
#### `public void Invalidate()`

Raise the Invalidated event.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourPickerBase_Invalidate.htm)

#### `public bool MouseClick(MouseEventArgs args, PointF pt)`

Respond to mouse-click events.

**Parameters:**
- `args` (System.Windows.Forms.MouseEventArgs) — Mouse Event arguments.
- `pt` (System.Drawing.PointF) — Mouse cursor position in floating point accuracy.

**Returns:** `Boolean` — True if a Drag operation was terminated, false if the event went unnoticed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourPickerBase_MouseClick.htm)

#### `public bool MouseDoubleClick(MouseEventArgs args, PointF pt, Control owner, Matrix transform)`

Respond to mouse-doubleclick events.

**Parameters:**
- `args` (System.Windows.Forms.MouseEventArgs) — Mouse Event arguments.
- `pt` (System.Drawing.PointF) — Mouse cursor position in floating point accuracy.
- `owner` (System.Windows.Forms.Control) — Owner control for this colour-picker.
- `transform` (System.Drawing.Drawing2D.Matrix) — Display Transformation matrix of colour-picker.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourPickerBase_MouseDoubleClick.htm)

#### `public bool MouseDown(MouseEventArgs args, PointF pt)`

Respond to mouse-down events.

**Parameters:**
- `args` (System.Windows.Forms.MouseEventArgs) — Mouse Event arguments.
- `pt` (System.Drawing.PointF) — Mouse cursor position in floating point accuracy.

**Returns:** `Boolean` — True if a Drag operation was started, false if the event went unnoticed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourPickerBase_MouseDown.htm)

#### `public bool MouseMove(MouseEventArgs args, PointF pt)`

Respond to mouse-move events.

**Parameters:**
- `args` (System.Windows.Forms.MouseEventArgs) — Mouse Event arguments.
- `pt` (System.Drawing.PointF) — Mouse cursor position in floating point accuracy.

**Returns:** `Boolean` — True if a Drag operation was performed, false if the event went unnoticed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourPickerBase_MouseMove.htm)

#### `public bool MouseUp(MouseEventArgs args, PointF pt)`

Respond to mouse-up events.

**Parameters:**
- `args` (System.Windows.Forms.MouseEventArgs) — Mouse Event arguments.
- `pt` (System.Drawing.PointF) — Mouse cursor position in floating point accuracy.

**Returns:** `Boolean` — True if a Drag operation was terminated, false if the event went unnoticed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourPickerBase_MouseUp.htm)

#### `public void OnColorChanged(bool intermediate)`

Raise the ColorChanged event.

**Parameters:**
- `intermediate` (System.Boolean) — Set the intermediate state of the event arguments.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourPickerBase_OnColorChanged.htm)

#### `public void Render(Graphics G)`

Render this slider into a Graphics context.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics to render with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourPickerBase_Render.htm)

#### `public GH_ToolstripItemKeyHandlerResult RespondToEnter()`



**Returns:** `GH_ToolstripItemKeyHandlerResult` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourPickerBase_RespondToEnter.htm)

#### `public GH_ToolstripItemKeyHandlerResult RespondToEscape()`



**Returns:** `GH_ToolstripItemKeyHandlerResult` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourPickerBase_RespondToEscape.htm)

#### `public void SetUiScaling(float factor)`



**Parameters:**
- `factor` (System.Single)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourPickerBase_SetUiScaling.htm)

#### `public void SetupColourPicker(Color col0, Color col1, GH_ColourSpace space)`

Setup all UI elements.

**Parameters:**
- `col0` (System.Drawing.Color) — Initial colour for this picker.
- `col1` (System.Drawing.Color) — Active colour for this picker.
- `space` (Grasshopper.GUI.Base.GH_ColourSpace) — Colour space mode for this picker.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourPickerBase_SetupColourPicker_1.htm)

#### `public void SetupColourPicker(Color col0, Point4d col1, GH_ColourSpace space)`

Setup all UI elements.

**Parameters:**
- `col0` (System.Drawing.Color) — Initial colour for this picker.
- `col1` (Point4d) — Active colour for this picker.
- `space` (Grasshopper.GUI.Base.GH_ColourSpace) — Colour space mode for this picker.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ColourPickerBase_SetupColourPicker.htm)

### Properties
- `AutoSize` (Boolean, get/set) — Gets or sets a value indicating whether the width and height of the Bounds are adjusted to fit the UI.
- `BackColour` (Color, get/set) — Gets or sets the background colour of the picker.
- `BaseColour` (Color, get) — Gets the original base colour for this picker. Use SetupColourPicker to assign this colour.
- `Bounds` (Rectangle, get/set) — Gets or sets the Bounds for this control.
- `ColourSpace` (GH_ColourSpace, get) — Gets the Colour space mode used in this picker.
- `DesiredHeight` (Int32, get) — Gets the ideal height for this colour picker given it's width and UI settings.
- `DrawAlphaSlider` (Boolean, get/set) — Gets or sets whether or not the alpha slider is included in the UI.
- `DrawBackground` (Boolean, get/set) — Gets or sets whether the background of the picker is drawn.
- `DrawChannelSliders` (Boolean, get/set) — Gets or sets whether or not the three basic channel sliders are included in the UI.
- `DropperPreviewBox` (Rectangle, get) — Gets the rectangle in which the eye-dropped preview will be drawn.
- `Font` (Font, get/set) — Gets or sets the Font used in this Colour Picker. Do not Dispose the Font returned by this property.
- `IsTextInput` (Boolean, get) — Gets whether any of the slider is currently displaying a text input box.
- `Padding` (Padding, get/set) — Gets or sets the Padding for this control.
- `PickColour` (Color, get) — Gets the picked colour.
- `SRCSpaceBox` (Rectangle, get) — Gets the rectangle containing the eye-dropper function.

### Events
#### `ColorChanged` (`Grasshopper.GUI.Base.GH_ColourPickerBase.ColorChangedEventHandler`)

**Signature:** `public event GH_ColourPickerBase.ColorChangedEventHandler ColorChanged`

Raised whenever the color of this picker is changed due to User-Interface methods.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Base_GH_ColourPickerBase_ColorChanged.htm)

#### `Invalidated` (`Grasshopper.GUI.Base.GH_ColourPickerBase.InvalidatedEventHandler`)

**Signature:** `public event GH_ColourPickerBase.InvalidatedEventHandler Invalidated`

Raised whenever a redraw is required.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Base_GH_ColourPickerBase_Invalidated.htm)

## GH_ColourPickerBase.ColorChangedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Base.GH_ColourPickerBase.ColorChangedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_ColourPickerBase_ColorChangedEventHandler.htm)

## GH_ColourPickerBase.InvalidatedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Base.GH_ColourPickerBase.InvalidatedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_ColourPickerBase_InvalidatedEventHandler.htm)

## GH_ColourPickerEventArgs (class)

Arguments passed via GH_SliderBase.ValueChanged events.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_ColourPickerEventArgs.htm)

### Properties
- `Colour` (Color, get) — Gets the new value of the colour picker. This value is cached, so even if the picker subsequently updates, this property remains constant.
- `ColourPicker` (GH_ColourPickerBase, get) — Gets the instance of the picker that raised the event.
- `Intermediate` (Boolean, get) — Gets a value indicating whether the change was an intermediate one.
- `Original` (Color, get) — Gets the original value of the colour picker. This value is cached, so even if the picker subsequently updates, this property remains constant.

## GH_ColourSpace (enum)

Enumerates the different colour spaces supported by the GH_ColourPickerBase class.

**Remarks:** See AlsoReferenceGrasshopper.GUI.Base Namespace

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_ColourSpace.htm)

### Values
- `None` = `0` — Unset value. Do not use. Seriously. Stay away.
- `RGB` = `1` — Colour picker operates in RGB mode.
- `HSV` = `2` — Colour picker operates in HSV mode.

## GH_DigitAlign (enum)

Enumerates the possible alignments for digits inside the GH_DigitScroller control.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_DigitAlign.htm)

### Values
- `Justify` = `0` — Digits expand to fill out the entire free space.
- `Left` = `1` — Digits are stacked against the left side of the control.
- `Center` = `2` — Digits are stacked together in the center of the control.
- `Right` = `3` — Digits are stacked against the right side of the control.

## GH_DigitNumber (class)

Maintains and provides functionality for evaluating and modifying numbers as used in the GH_DigitScrollerBase control.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_DigitNumber.htm)

### Constructors
- `public GH_DigitNumber(GH_DigitNumber other)` — Create an exact duplicate of another GH_DigitNumber instance.
- `public GH_DigitNumber(int decimalPlaces)` — Create a new GH_DigitNumber instance with the specified number of decimal places.
- `public GH_DigitNumber(int decimalPlaces, int radixPosition)` — Create a new GH_DigitNumber instance with the specified number of decimal places and a given radix position.

### Methods
#### `public bool AssignOffset(int index, decimal offset)`

Assign an offset to a given digit and recursively adjust all leftwards offsets if needed.

**Parameters:**
- `index` (System.Int32) — Index of digit to offset.
- `offset` (System.Decimal) — New offset for digit.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_DigitNumber_AssignOffset.htm)

#### `public void LimitValue()`

Limits the value to the minimum and maximum domain.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_DigitNumber_LimitValue.htm)

#### `public void Reset()`

Reset all parts to zero.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_DigitNumber_Reset.htm)

#### `public void Round()`

Round the number by cancelling the offset in the proper direction.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_DigitNumber_Round.htm)

#### `public override string ToString()`

(Overrides Object.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_DigitNumber_ToString.htm)

### Properties
- `DigitCount` (Int32, get) — Gets or sets the amount of digits in this number.
- `Epsilon` (Decimal, get) — Gets the smallest possible change.
- `IsPrimaryDigitSignificant` (Boolean, get) — Gets whether the primary digit at the given index is significant.
- `IsSecondaryDigitSignificant` (Boolean, get) — Gets whether the secondary digit at the given index is significant.
- `Maximum` (Decimal, get/set) — Gets the maximum value allowed in this number.
- `Minimum` (Decimal, get/set) — Gets the minimum value allowed in this number.
- `Offset` (Decimal, get) — Gets the offset between the primary and secondary digits.
- `PrimaryDigits` (IList<Int32>, get) — Gets the internal list of primary digits. Do not modify this list.
- `PrimaryPositive` (Boolean, get/set) — Gets or sets a value indicating whether the number is positive.
- `Radix` (Int32, get/set) — Gets or sets the radix point index. A negative radix index disables the radix point, zero is not a valid index.
- `RadixIndex` (Int32, get) — Gets the mapped radix.
- `SecondaryDigits` (IList<Int32>, get) — Gets the internal list of secondary digits. Do not modify this list.
- `SecondaryPositive` (Boolean, get) — Gets a value indicating whether the secondary number is positive.
- `Value` (Decimal, get/set) — Gets or sets the value of this number.

## GH_DigitScrollerBase (class)

Provides Numeric digit scrolling GUI not tied to a control-handle.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_DigitScrollerBase.htm)

### Constructors
- `public GH_DigitScrollerBase()` — Initializes a new instance of the GH_DigitScrollerBase class

### Methods
#### `protected override void HandleTextInputAccepted(string text)`

(Overrides GH_TextBoxInputBase.HandleTextInputAccepted(String).)

**Parameters:**
- `text` (System.String)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_DigitScrollerBase_HandleTextInputAccepted.htm)

#### `public void HideTextInputBox()`

Hides the text-input override form (if it is displayed).

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_TextBoxInputBase_HideTextInputBox.htm)

#### `public GH_DigitScrollerBase.GH_MouseAction MouseDown(MouseEventArgs args, PointF pt)`

Respond to mouse-down events.

**Parameters:**
- `args` (System.Windows.Forms.MouseEventArgs) — Mouse Event arguments.
- `pt` (System.Drawing.PointF) — Mouse cursor position in floating point accuracy.

**Returns:** `GH_DigitScrollerBase.GH_MouseAction` — True if a Drag operation was started, false if the event went unnoticed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_DigitScrollerBase_MouseDown.htm)

#### `public bool MouseMove(MouseEventArgs args, PointF pt)`

Respond to mouse-move events.

**Parameters:**
- `args` (System.Windows.Forms.MouseEventArgs) — Mouse Event arguments.
- `pt` (System.Drawing.PointF) — Mouse cursor position in floating point accuracy.

**Returns:** `Boolean` — True if a Drag operation was performed, false if the event went unnoticed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_DigitScrollerBase_MouseMove.htm)

#### `public bool MouseUp(MouseEventArgs args, PointF pt)`

Respond to mouse-up events.

**Parameters:**
- `args` (System.Windows.Forms.MouseEventArgs) — Mouse Event arguments.
- `pt` (System.Drawing.PointF) — Mouse cursor position in floating point accuracy.

**Returns:** `Boolean` — True if a Drag operation was terminated, false if the event went unnoticed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_DigitScrollerBase_MouseUp.htm)

#### `public void OnInvalidated()`

Raise the Invalidated event.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_DigitScrollerBase_OnInvalidated.htm)

#### `public void OnValueChanged(bool intermediate)`

Raise the ValueChanged event.

**Parameters:**
- `intermediate` (System.Boolean) — Set the intermediate state of the event arguments.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_DigitScrollerBase_OnValueChanged.htm)

#### `public void Render(Graphics G)`

Render all UI elements.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_DigitScrollerBase_Render.htm)

#### `public GH_ToolstripItemKeyHandlerResult RespondToEnter()`

(Inherited from GH_TextBoxInputBase.)

**Returns:** `GH_ToolstripItemKeyHandlerResult` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_TextBoxInputBase_RespondToEnter.htm)

#### `public GH_ToolstripItemKeyHandlerResult RespondToEscape()`

(Inherited from GH_TextBoxInputBase.)

**Returns:** `GH_ToolstripItemKeyHandlerResult` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_TextBoxInputBase_RespondToEscape.htm)

#### `public void SetupScroller(decimal minimum, decimal maximum, decimal value)`

Set the minimum, maximum and value fields simultaneously.

**Parameters:**
- `minimum` (System.Decimal)
- `maximum` (System.Decimal)
- `value` (System.Decimal)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_DigitScrollerBase_SetupScroller.htm)

#### `public void ShowTextInputBox(Control owner)`



**Parameters:**
- `owner` (System.Windows.Forms.Control)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_DigitScrollerBase_ShowTextInputBox.htm)

#### `public void ShowTextInputBox(Control owner, bool limitToBoundary)`



**Parameters:**
- `owner` (System.Windows.Forms.Control)
- `limitToBoundary` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_DigitScrollerBase_ShowTextInputBox_1.htm)

#### `public void ShowTextInputBox(Control owner, bool limitToBoundary, Matrix transform)`



**Parameters:**
- `owner` (System.Windows.Forms.Control)
- `limitToBoundary` (System.Boolean)
- `transform` (System.Drawing.Drawing2D.Matrix)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_DigitScrollerBase_ShowTextInputBox_2.htm)

#### `public void ShowTextInputBox(Control owner, string content, bool selectContent)`

Display a floating text-box on top of the owner control.

**Parameters:**
- `owner` (System.Windows.Forms.Control) — Control that owns this slider. The coordinates of the Bounds rectangle must be specified in owner coordinates.
- `content` (System.String)
- `selectContent` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_TextBoxInputBase_ShowTextInputBox.htm)

#### `public void ShowTextInputBox(Control owner, string content, bool selectContent, bool limitToBoundary)`

Display a floating text-box on top of the owner control.

**Parameters:**
- `owner` (System.Windows.Forms.Control) — Control that owns this slider. The coordinates of the Bounds rectangle must be specified in owner coordinates.
- `content` (System.String)
- `selectContent` (System.Boolean)
- `limitToBoundary` (System.Boolean) — If True, the pop up input box will be 'clipped' to the owner boundary. At least some part of the slider must be visible for this to work.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_TextBoxInputBase_ShowTextInputBox_1.htm)

#### `public void ShowTextInputBox(Control owner, string content, bool selectContent, bool limitToBoundary, Matrix transform)`

Display a floating text-box on top of the owner control.

**Parameters:**
- `owner` (System.Windows.Forms.Control) — Control that owns this slider. The coordinates of the Bounds rectangle must be specified in owner coordinates.
- `content` (System.String)
- `selectContent` (System.Boolean)
- `limitToBoundary` (System.Boolean) — If True, the pop up input box will be 'clipped' to the owner boundary. At least some part of the slider must be visible for this to work.
- `transform` (System.Drawing.Drawing2D.Matrix) — Transformation matrix to apply to pop up UI.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_TextBoxInputBase_ShowTextInputBox_2.htm)

### Properties
- `AllowRadixDrag` (Boolean, get/set) — Gets or sets a value indicating whether the radix is allowed to be dragged around.
- `AllowTextInput` (Boolean, get/set) — Gets or sets whether text input is allowed by double click.
- `AmplifyMotion` (Boolean, get/set) — Gets or sets a value indicating whether vertical mouse drags should be amplified as the mouse moves further and further.
- `BackgroundGradient` (GH_Gradient, get/set) — Gets or sets the background Gradient. If the gradient is not null, it overrides the background colours.
- `BottomColour` (Color, get/set) — Gets or sets the colour used along the bottom edge of the control.
- `Bounds` (Rectangle, get/set) — Gets or sets the Bounds for this scroller.
- `DecimalPlaces` (Int32, get/set) — Gets or sets the number of decimal places available in this scroller. Unlike Radix, this property maintains the decimal position of the value represented by this scroller.
- `DigitAlign` (GH_DigitAlign, get/set) — Gets or sets the digit align mode.
- `Digits` (Int32, get/set) — Gets or sets the number of digits available in this scroller.
- `DrawBackground` (Boolean, get/set) — Gets or sets whether the background is drawn in this control.
- `DrawBorder` (Boolean, get/set) — Gets or sets whether the border is drawn in this control.
- `DrawShadows` (Boolean, get/set) — Gets or sets whether shadows are drawn in this control.
- `EdgeColour` (Color, get/set) — Gets or sets the edge colour of the control.
- `Epsilon` (Decimal, get) — Gets the smallest possible change in the value.
- `Font` (Font, get/set) — Gets or sets the Font for this scroller.
- `IsTextInput` (Boolean, get) — Gets whether the text input box is currently on screen.
- `MaximumValue` (Decimal, get/set) — Gets or sets the upper bound of the scroll value.
- `MinimumValue` (Decimal, get/set) — Gets or sets the lower bound of the scroll value.
- `Prefix` (String, get/set) — Gets or sets the prefix text to be displayed on the scroller.
- `PrefixBox` (Rectangle, get) — Gets the box containing the prefix string. If there is no prefix the PrefixBox will be Rectangle.Empty
- `Radix` (Int32, get/set) — Gets or sets the radix index of this scroller.
- `RadixBox` (Rectangle, get) — Gets the Radix box for this control, if there is no radix box, Rectangle.Empty will be returned.
- `RailColour` (Color, get/set) — Gets or sets the colour used for the rail line.
- `RaiseEvents` (Boolean, get/set) — Gets or sets whether this scroller raises the ValueChanged event.
- `ScrollBoxes` (List<Rectangle>, get) — Gets all scroll boxes (from left to right) for this control, not including the radix box.
- `ShadowColour` (Color, get/set) — Gets or sets the colour used for the shadows.
- `ShadowSize` (Padding, get/set) — Gets or sets the size of the drop shadow along all four edges.
- `SignBox` (Rectangle, get) — Gets the Sign box for this control.
- `Suffix` (String, get/set) — Gets or sets the suffix text to be displayed on the scroller.
- `SuffixBox` (Rectangle, get) — Gets the box containing the suffix string. If there is no suffix the PrefixBox will have zero width
- `TextColour` (Color, get/set) — Gets or sets the colour used for the text inside the scrolls.
- `TopColour` (Color, get/set) — Gets or sets the colour used along the top edge of the control.
- `Value` (Decimal, get/set) — Gets or sets the value displayed in this scroller.

### Events
#### `Invalidated` (`Grasshopper.GUI.Base.GH_DigitScrollerBase.InvalidatedEventHandler`)

**Signature:** `public event GH_DigitScrollerBase.InvalidatedEventHandler Invalidated`

Raised whenever the display of this scroller is changed, but not the value.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Base_GH_DigitScrollerBase_Invalidated.htm)

#### `ValueChanged` (`Grasshopper.GUI.Base.GH_DigitScrollerBase.ValueChangedEventHandler`)

**Signature:** `public event GH_DigitScrollerBase.ValueChangedEventHandler ValueChanged`

Raised whenever the value of this scroller is changed due to User-Interface methods.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Base_GH_DigitScrollerBase_ValueChanged.htm)

## GH_DigitScrollerBase.GH_MouseAction (enum)

Enumerates the possible actions due to mouse-down mouse-move events.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_DigitScrollerBase_GH_MouseAction.htm)

### Values
- `None` = `0` — No specific action.
- `SignDown` = `1` — Mouse down on sign symbol.
- `DigitDown` = `2` — Mouse down on scrolling digit.
- `DigitDrag` = `3` — Mouse move on scrolling digit.
- `RadixDown` = `4` — Mouse down on radix point.
- `RadixDrag` = `5` — Mouse move on radix point.

## GH_DigitScrollerBase.InvalidatedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Base.GH_DigitScrollerBase.InvalidatedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_DigitScrollerBase_InvalidatedEventHandler.htm)

## GH_DigitScrollerBase.ValueChangedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Base.GH_DigitScrollerBase.ValueChangedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_DigitScrollerBase_ValueChangedEventHandler.htm)

## GH_DigitScrollerEventArgs (class)

Arguments passed via GH_digitScrollerBase.ValueChanged events.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_DigitScrollerEventArgs.htm)

### Properties
- `Intermediate` (Boolean, get) — Gets a value indicating whether the change was an intermediate one.
- `Scroller` (GH_DigitScrollerBase, get) — Gets the instance of the scroller that raised the event.
- `Value` (Decimal, get) — Gets the new value of the slider. This value is cached, so even if the slider subsequently updates, this property remains constant.

## GH_ScrollBarVerticalBase (class)

Provides basic vertical scroll bar logic.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_ScrollBarVerticalBase.htm)

### Constructors
- `public GH_ScrollBarVerticalBase()` — Initializes a new instance of the GH_ScrollBarVerticalBase class

### Methods
#### `public void BeginDrag(RectangleF region, double start)`

Start a new drag operation.

**Parameters:**
- `region` (System.Drawing.RectangleF) — Scroll bar region (the rectangle in which the scroll bar is allowed to move).
- `start` (System.Double) — Vertical offset of scroll drag.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ScrollBarVerticalBase_BeginDrag.htm)

#### `public bool ContinueDrag(double position)`

Continue a drag operation.

**Parameters:**
- `position` (System.Double) — New offset hint.

**Returns:** `Boolean` — True if the drag resulted in an offset change, false if not.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ScrollBarVerticalBase_ContinueDrag.htm)

#### `public Rectangle ScrollBar(Rectangle rail)`

Compute the size and position of the scroll bar in the scroll rail.

**Parameters:**
- `rail` (System.Drawing.Rectangle) — Rail for scroll bar to move around in.

**Returns:** `Rectangle` — A rectangle describing the location and extends of the scroll bar.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ScrollBarVerticalBase_ScrollBar.htm)

#### `public RectangleF ScrollBar(RectangleF rail)`

Compute the size and position of the scroll bar in the scroll rail.

**Parameters:**
- `rail` (System.Drawing.RectangleF) — Rail for scroll bar to move around in.

**Returns:** `RectangleF` — A rectangle describing the location and extends of the scroll bar.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_ScrollBarVerticalBase_ScrollBar_1.htm)

### Properties
- `Content` (Int32, get/set) — Gets or sets the height of the content
- `ContentOffset` (Double, get) — Gets the vertical offset (in pixels) of the scrolled content.
- `Display` (Int32, get/set) — Gets or sets the height of the display area.
- `Minimum` (Int32, get/set) — Gets or sets the minimum height of the scrollbar.
- `Offset` (Int32, get) — Gets the offset of the scroller.
- `OffsetNormalised` (Double, get/set) — Gets or sets the normalised offset of the scroller.

## GH_SliderAccuracy (enum)

Enumerates the possible numeric slider accuracies.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_SliderAccuracy.htm)

### Values
- `Float` = `0` — Slider supports floating point accuracy.
- `Integer` = `1` — Slider supports integer accuracy.
- `Even` = `2` — Slider only supports even integers.
- `Odd` = `3` — Slider only supports odd integers.

## GH_SliderBase (class)

Provides Numeric slider GUI not tied to a control-handle.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_SliderBase.htm)

### Constructors
- `public GH_SliderBase()` — Initializes a new instance of the GH_SliderBase class

### Methods
#### `public void FixDomain()`

Fix the limits of the slider. Limits will be rounded according to slider type and accuracy. Also, a minimum range for all types is ensured.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_SliderBase_FixDomain.htm)

#### `public void FixValue()`

Fix the value of the slider. The value will be rounded according to slider type and accuracy. Also, the value will be clipped to the limits. Before you call this function, ensure the limits are set up correctly.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_SliderBase_FixValue.htm)

#### `protected override void HandleTextInputAccepted(string text)`

(Overrides GH_TextBoxInputBase.HandleTextInputAccepted(String).)

**Parameters:**
- `text` (System.String)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_SliderBase_HandleTextInputAccepted.htm)

#### `public void HideTextInputBox()`

Hides the text-input override form (if it is displayed).

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_TextBoxInputBase_HideTextInputBox.htm)

#### `public bool KeyDown(KeyEventArgs args)`

Respond to a KeyDown event.

**Parameters:**
- `args` (System.Windows.Forms.KeyEventArgs) — KeyDown event arguments.

**Returns:** `Boolean` — True if something changed, false if not.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_SliderBase_KeyDown.htm)

#### `public bool MouseDown(MouseEventArgs args, PointF pt)`

Respond to mouse-down events.

**Parameters:**
- `args` (System.Windows.Forms.MouseEventArgs) — Mouse Event arguments.
- `pt` (System.Drawing.PointF) — Mouse cursor position in floating point accuracy.

**Returns:** `Boolean` — True if a Drag operation was started, false if the event went unnoticed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_SliderBase_MouseDown.htm)

#### `public bool MouseMove(MouseEventArgs args, PointF pt)`

Respond to mouse-move events.

**Parameters:**
- `args` (System.Windows.Forms.MouseEventArgs) — Mouse Event arguments.
- `pt` (System.Drawing.PointF) — Mouse cursor position in floating point accuracy.

**Returns:** `Boolean` — True if a Drag operation was performed, false if the event went unnoticed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_SliderBase_MouseMove.htm)

#### `public bool MouseUp(MouseEventArgs args, PointF pt)`

Respond to mouse-up events.

**Parameters:**
- `args` (System.Windows.Forms.MouseEventArgs) — Mouse Event arguments.
- `pt` (System.Drawing.PointF) — Mouse cursor position in floating point accuracy.

**Returns:** `Boolean` — True if a Drag operation was terminated, false if the event went unnoticed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_SliderBase_MouseUp.htm)

#### `public void OnValueChanged(bool intermediate)`

Raise the ValueChanged event.

**Parameters:**
- `intermediate` (System.Boolean) — Set the intermediate state of the event arguments.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_SliderBase_OnValueChanged.htm)

#### `public static decimal ProcessNumber(decimal val, GH_SliderAccuracy accuracy, int digits)`

Process a number using the given slider characteristics.

**Parameters:**
- `val` (System.Decimal) — Number to process.
- `accuracy` (Grasshopper.GUI.Base.GH_SliderAccuracy) — Slider rounding accuracy.
- `digits` (System.Int32) — Number of digits to use for floating point rounding.

**Returns:** `Decimal` — Processed number.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_SliderBase_ProcessNumber.htm)

#### `public void Render(Graphics G)`

Render this slider into a Graphics context.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics to render with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_SliderBase_Render.htm)

#### `public GH_ToolstripItemKeyHandlerResult RespondToEnter()`

(Inherited from GH_TextBoxInputBase.)

**Returns:** `GH_ToolstripItemKeyHandlerResult` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_TextBoxInputBase_RespondToEnter.htm)

#### `public GH_ToolstripItemKeyHandlerResult RespondToEscape()`

(Inherited from GH_TextBoxInputBase.)

**Returns:** `GH_ToolstripItemKeyHandlerResult` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_TextBoxInputBase_RespondToEscape.htm)

#### `public void SetSnapRanges(IEnumerable<SliderSnapRange> ranges)`

Assign a collection of snap ranges to this slider.

**Parameters:**
- `ranges` (System.Collections.Generic.IEnumerable<SliderSnapRange>) — Ranges to snap to. Will override whatever snap ranges were defined before.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_SliderBase_SetSnapRanges.htm)

#### `public void ShowTextInputBox(Control owner)`



**Parameters:**
- `owner` (System.Windows.Forms.Control)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_SliderBase_ShowTextInputBox.htm)

#### `public void ShowTextInputBox(Control owner, bool limitToBoundary)`



**Parameters:**
- `owner` (System.Windows.Forms.Control)
- `limitToBoundary` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_SliderBase_ShowTextInputBox_1.htm)

#### `public void ShowTextInputBox(Control owner, bool limitToBoundary, Matrix transform)`



**Parameters:**
- `owner` (System.Windows.Forms.Control)
- `limitToBoundary` (System.Boolean)
- `transform` (System.Drawing.Drawing2D.Matrix)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_SliderBase_ShowTextInputBox_2.htm)

#### `public void ShowTextInputBox(Control owner, bool limitToBoundary, Matrix transform, string content)`



**Parameters:**
- `owner` (System.Windows.Forms.Control)
- `limitToBoundary` (System.Boolean)
- `transform` (System.Drawing.Drawing2D.Matrix)
- `content` (System.String)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_SliderBase_ShowTextInputBox_3.htm)

#### `public void ShowTextInputBox(Control owner, string content, bool selectContent)`

Display a floating text-box on top of the owner control.

**Parameters:**
- `owner` (System.Windows.Forms.Control) — Control that owns this slider. The coordinates of the Bounds rectangle must be specified in owner coordinates.
- `content` (System.String)
- `selectContent` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_TextBoxInputBase_ShowTextInputBox.htm)

#### `public void ShowTextInputBox(Control owner, string content, bool selectContent, bool limitToBoundary)`

Display a floating text-box on top of the owner control.

**Parameters:**
- `owner` (System.Windows.Forms.Control) — Control that owns this slider. The coordinates of the Bounds rectangle must be specified in owner coordinates.
- `content` (System.String)
- `selectContent` (System.Boolean)
- `limitToBoundary` (System.Boolean) — If True, the pop up input box will be 'clipped' to the owner boundary. At least some part of the slider must be visible for this to work.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_TextBoxInputBase_ShowTextInputBox_1.htm)

#### `public void ShowTextInputBox(Control owner, string content, bool selectContent, bool limitToBoundary, Matrix transform)`

Display a floating text-box on top of the owner control.

**Parameters:**
- `owner` (System.Windows.Forms.Control) — Control that owns this slider. The coordinates of the Bounds rectangle must be specified in owner coordinates.
- `content` (System.String)
- `selectContent` (System.Boolean)
- `limitToBoundary` (System.Boolean) — If True, the pop up input box will be 'clipped' to the owner boundary. At least some part of the slider must be visible for this to work.
- `transform` (System.Drawing.Drawing2D.Matrix) — Transformation matrix to apply to pop up UI.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_TextBoxInputBase_ShowTextInputBox_2.htm)

### Properties
- `Bounds` (Rectangle, get/set) — Gets or sets the bounds of the slider.
- `ControlBackColour` (Color, get/set) — Gets or sets the BackColour for this slider control.
- `ControlEdgeColour` (Color, get/set) — Gets or sets the EdgeColor for this slider control.
- `ControlShadowColour` (Color, get/set) — Gets or sets the Shadow colour for this control.
- `DecimalPlaces` (Int32, get/set) — Gets or sets the number of decimal places allowed for floating point type sliders. Valid values between zero and 12.
- `DrawControlBackground` (Boolean, get/set) — Gets or sets whether the background of the slider should be drawn.
- `DrawControlBorder` (Boolean, get/set) — Gets or sets whether the border of the slider should be drawn.
- `DrawControlShadows` (Boolean, get/set) — Gets or sets whether the drop shadows of the slider should be drawn.
- `Epsilon` (Decimal, get) — Gets the smallest value change supported by the current slider properties.
- `Font` (Font, get/set) — Gets or sets the font for this slider.
- `FormatMask` (String, get/set) — Gets or sets the string format mask to use for the display. Should contain at least one "{0}" element.
- `Grip` (RectangleF, get) — Gets the box of the current grip. If the slider value/limits are not set up correctly then this rectangle may well be outside of the bounds. If the limits are coincident, this will fail (divide by zero).
- `GripBottomColour` (Color, get/set) — Gets or sets the fill colour for the bottom edge of the slider grip.
- `GripDisplay` (GH_SliderGripDisplay, get/set) — Gets or sets how the slider grip (or thumb) is drawn.
- `GripEdgeColour` (Color, get/set) — Gets or sets the edge colour for the slider grip.
- `GripText` (String, get) — Gets the text that will be drawn on the slider.
- `GripTopColour` (Color, get/set) — Gets or sets the fill colour for the top edge of the slider grip.
- `GripWidth` (Int32, get) — Gets an estimate of the maximum number of pixels taken up by the grip. This property depends on many different factors, though should remain constant when other properties are not changed. If the Font hasn't been specified, the default Windows.Forms.Control font is used.
- `IsTextInput` (Boolean, get) — Gets whether the text input box is currently on screen.
- `Maximum` (Decimal, get/set) — Gets or sets the upper limit of the slider.
- `Minimum` (Decimal, get/set) — Gets or sets the lower limit of the slider.
- `NormalizedValue` (Double, get/set) — Gets or sets the normalized value within the slider domain.
- `Padding` (Padding, get/set) — Gets or sets the padding around the slider border.
- `Rail` (Rectangle, get) — Gets the shape of the rail. The height of the rectangle will be zero, but the y-position will be rounded to integer precision.
- `RailBrightColour` (Color, get/set) — Gets or sets the bright colour used to draw rails and ticks.
- `RailDarkColour` (Color, get/set) — Gets or sets the dark colour used to draw rails and ticks.
- `RailDisplay` (GH_SliderRailDisplay, get/set) — Gets or sets how the slider rail (or track) is drawn.
- `RailEmptyColour` (Color, get/set) — Gets or sets the fill colour for the empty portion of the rail.
- `RailFullColour` (Color, get/set) — Gets or sets the fill colour for the full portion of the rail.
- `RaiseEvents` (Boolean, get/set) — Gets or sets a value indicating whether this slider raises the ValueChanged event.
- `ShadowSize` (Padding, get/set) — Gets or sets the size of the border shadow along all edges of the box.
- `SnapDistance` (Decimal, get/set) — Gets or sets the snap distance in absolute values.
- `TextColour` (Color, get/set) — Gets or sets the fill colour for the slider text.
- `TextInputHandlerDelegate` (GH_SliderBase.TextInputHandler, get/set) — Gets or sets the custom handler for text input. If you do not set a custom handler the default behaviour will be used.
- `TickCount` (Int32, get/set) — Gets or sets the number of ticks displayed.
- `TickDisplay` (GH_SliderTickDisplay, get/set) — Gets or sets how the slider ticks are drawn.
- `TickFrequency` (Int32, get/set) — Gets or sets the frequency of large ticks vs small ticks.
- `Ticks` (List<Int32>, get) — Gets a list of all the tick offset x positions.
- `Type` (GH_SliderAccuracy, get/set) — Gets or sets the accuracy type of the slider.
- `Value` (Decimal, get/set) — Gets or sets the value of the slider.
- `ValueF` (Single, get/set) — Gets or sets the value of the slider in single precision.

### Events
#### `ValueChanged` (`Grasshopper.GUI.Base.GH_SliderBase.ValueChangedEventHandler`)

**Signature:** `public event GH_SliderBase.ValueChangedEventHandler ValueChanged`

Raised whenever the value of this slider is changed due to User-Interface methods.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Base_GH_SliderBase_ValueChanged.htm)

## GH_SliderBase.DrawSliderChannel (delegate)

This delegate is used to intervene in the slider drawing process.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_SliderBase_DrawSliderChannel.htm)

## GH_SliderBase.TextInputHandler (delegate)

Delegate used during Text Input handling.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_SliderBase_TextInputHandler.htm)

## GH_SliderBase.ValueChangedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Base.GH_SliderBase.ValueChangedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_SliderBase_ValueChangedEventHandler.htm)

## GH_SliderEventArgs (class)

Arguments passed via GH_SliderBase.ValueChanged events.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_SliderEventArgs.htm)

### Properties
- `Intermediate` (Boolean, get) — Gets a value indicating whether the change was an intermediate one.
- `Slider` (GH_SliderBase, get) — Gets the instance of the slider that raised the event.
- `Value` (Decimal, get) — Gets the new value of the slider. This value is cached, so even if the slider subsequently updates, this property remains constant.

## GH_SliderGripDisplay (enum)

Enumerates the possible slider grip displays.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_SliderGripDisplay.htm)

### Values
- `None` = `0` — No grip is drawn.
- `Numeric` = `1` — A single box containing the slider value is drawn.
- `Shape` = `2` — A constant shape grip is drawn (round for floating point, diamond for integers)
- `ShapeAndText` = `3` — Same as Shape, but text with the slider value is drawn besides the grip.

## GH_SliderRailDisplay (enum)

Enumerates the possible slider rail displays.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_SliderRailDisplay.htm)

### Values
- `None` = `0` — No rail is drawn.
- `Simple` = `1` — A rail consisting of a single line is drawn.
- `Etched` = `2` — A rail consisting of an etched line is drawn.
- `Filled` = `3` — A thick rail with a filled out left portion is drawn.

## GH_SliderTickDisplay (enum)

Enumerates the possible slider tick displays.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_SliderTickDisplay.htm)

### Values
- `None` = `0` — No slider ticks are drawn.
- `Simple` = `1` — Slider ticks are drawn as single lines.
- `Etched` = `2` — Slider ticks are drawn as etched lines.

## GH_TextBoxInputBase (class)

(No description provided in vendor docs for Grasshopper.GUI.Base.GH_TextBoxInputBase.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_GH_TextBoxInputBase.htm)

### Constructors
- `protected GH_TextBoxInputBase()` — Initializes a new instance of the GH_TextBoxInputBase class

### Methods
#### `protected abstract void HandleTextInputAccepted(string text)`



**Parameters:**
- `text` (System.String)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_TextBoxInputBase_HandleTextInputAccepted.htm)

#### `public void HideTextInputBox()`

Hides the text-input override form (if it is displayed).

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_TextBoxInputBase_HideTextInputBox.htm)

#### `public GH_ToolstripItemKeyHandlerResult RespondToEnter()`



**Returns:** `GH_ToolstripItemKeyHandlerResult` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_TextBoxInputBase_RespondToEnter.htm)

#### `public GH_ToolstripItemKeyHandlerResult RespondToEscape()`



**Returns:** `GH_ToolstripItemKeyHandlerResult` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_TextBoxInputBase_RespondToEscape.htm)

#### `public void ShowTextInputBox(Control owner, string content, bool selectContent)`

Display a floating text-box on top of the owner control.

**Parameters:**
- `owner` (System.Windows.Forms.Control) — Control that owns this slider. The coordinates of the Bounds rectangle must be specified in owner coordinates.
- `content` (System.String)
- `selectContent` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_TextBoxInputBase_ShowTextInputBox.htm)

#### `public void ShowTextInputBox(Control owner, string content, bool selectContent, bool limitToBoundary)`

Display a floating text-box on top of the owner control.

**Parameters:**
- `owner` (System.Windows.Forms.Control) — Control that owns this slider. The coordinates of the Bounds rectangle must be specified in owner coordinates.
- `content` (System.String)
- `selectContent` (System.Boolean)
- `limitToBoundary` (System.Boolean) — If True, the pop up input box will be 'clipped' to the owner boundary. At least some part of the slider must be visible for this to work.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_TextBoxInputBase_ShowTextInputBox_1.htm)

#### `public void ShowTextInputBox(Control owner, string content, bool selectContent, bool limitToBoundary, Matrix transform)`

Display a floating text-box on top of the owner control.

**Parameters:**
- `owner` (System.Windows.Forms.Control) — Control that owns this slider. The coordinates of the Bounds rectangle must be specified in owner coordinates.
- `content` (System.String)
- `selectContent` (System.Boolean)
- `limitToBoundary` (System.Boolean) — If True, the pop up input box will be 'clipped' to the owner boundary. At least some part of the slider must be visible for this to work.
- `transform` (System.Drawing.Drawing2D.Matrix) — Transformation matrix to apply to pop up UI.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_GH_TextBoxInputBase_ShowTextInputBox_2.htm)

### Properties
- `Font` (Font, get/set) — Gets or sets the Font to use in this control.
- `IsTextInput` (Boolean, get) — Gets whether the text input box is currently on screen.

## SliderSnapRange (struct)

Represents a snap range on a slider.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Base_SliderSnapRange.htm)

### Constructors
- `public SliderSnapRange(decimal value)` — Initializes a new instance of the SliderSnapRange class
- `public SliderSnapRange(decimal value0, decimal value1)` — Initializes a new instance of the SliderSnapRange class

### Methods
#### `public bool CanMerge(SliderSnapRange other)`



**Parameters:**
- `other` (Grasshopper.GUI.Base.SliderSnapRange)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_SliderSnapRange_CanMerge.htm)

#### `public int CompareTo(SliderSnapRange other)`



**Parameters:**
- `other` (Grasshopper.GUI.Base.SliderSnapRange)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_SliderSnapRange_CompareTo.htm)

#### `public decimal DistanceTo(decimal value)`

Compute the distance from a value to this snap range.

**Parameters:**
- `value` (System.Decimal)

**Returns:** `Decimal` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_SliderSnapRange_DistanceTo.htm)

#### `public SliderSnapRange Merge(SliderSnapRange other)`



**Parameters:**
- `other` (Grasshopper.GUI.Base.SliderSnapRange)

**Returns:** `SliderSnapRange` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_SliderSnapRange_Merge.htm)

#### `public decimal SnapValue(decimal value)`

Gets the snapped value of another value.

**Parameters:**
- `value` (System.Decimal)

**Returns:** `Decimal` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Base_SliderSnapRange_SnapValue.htm)

### Properties
- `IsSingleton` (Boolean, get) — 
- `Max` (Decimal, get) — 
- `Min` (Decimal, get) — 

