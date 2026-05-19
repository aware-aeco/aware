---
name: grasshopper-grasshopper-gui-gradient
description: This skill encodes the grasshopper 8.0 surface of the Grasshopper.GUI.Gradient namespace — 7 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GH_Gradient, GH_GradientChangedEventArgs, GH_Grip, GH_GripType, GH_GripSide, GH_Gradient.SelectionChangedEventHandler, GH_Gradient.GradientChangedEventHandler.
---

# Grasshopper.GUI.Gradient

Auto-generated from vendor docs for grasshopper 8.0. 7 types in this namespace.

## GH_Gradient (class)

Represents a colour gradient defined by a succession of grips.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Gradient_GH_Gradient.htm)

### Constructors
- `public GH_Gradient()` — Initializes a new instance of the GH_Gradient class
- `public GH_Gradient(GH_Gradient other)` — Create a duplicate of another gradient.
- `public GH_Gradient(IEnumerable<double> parameters, IEnumerable<Color> colours)` — Create a new gradient from grips and colours.

### Methods
#### `public GH_Grip AddGrip(double t)`

Add a new grip to the gradient. The colour of the grip will be picked so that there is no difference to the gradient, though this is not actually possible when the interpolation mode is not linear.

**Parameters:**
- `t` (System.Double) — Grip parameter.

**Returns:** `GH_Grip` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_AddGrip_1.htm)

#### `public GH_Grip AddGrip(double t, Color c)`

Add a new single-colour grip.

**Parameters:**
- `t` (System.Double) — Parameter for grip.
- `c` (System.Drawing.Color) — Colour of grip.

**Returns:** `GH_Grip` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_AddGrip_2.htm)

#### `public GH_Grip AddGrip(double t, Color c0, Color c1)`

Add a new bi-colour grip.

**Parameters:**
- `t` (System.Double) — Parameter for grip.
- `c0` (System.Drawing.Color) — Left colour of grip.
- `c1` (System.Drawing.Color) — Right colour of grip.

**Returns:** `GH_Grip` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_AddGrip_3.htm)

#### `public void AddGrip(GH_Grip grip)`

Add a new grip to the gradient. This method does not raise the GradientChanged event, so you need to do that yourself by calling OnGradientChanged().

**Parameters:**
- `grip` (Grasshopper.GUI.Gradient.GH_Grip) — Grip to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_AddGrip.htm)

#### `public Color ColourAt(double t)`

Evaluate the colour at a specific parameter.

**Parameters:**
- `t` (System.Double) — Parameter to evaluate.

**Returns:** `Color` — The colour at the given parameter.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_ColourAt.htm)

#### `public static RectangleF DeleteGripRegion(RectangleF destination)`

Gets the Delete Grip region rectangle based on a gradient destination rectangle.

**Parameters:**
- `destination` (System.Drawing.RectangleF) — Gradient destination rectangle.

**Returns:** `RectangleF` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_DeleteGripRegion.htm)

#### `public void DisplayGradientEditor()`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_DisplayGradientEditor.htm)

#### `public void DisplayGripColourPicker(GH_Grip grip)`



**Parameters:**
- `grip` (Grasshopper.GUI.Gradient.GH_Grip)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_DisplayGripColourPicker.htm)

#### `public static GH_Gradient EarthlyBrown()`



**Returns:** `GH_Gradient` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_EarthlyBrown.htm)

#### `public static GH_Gradient Forest()`



**Returns:** `GH_Gradient` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_Forest.htm)

#### `public static GH_Gradient GreyScale()`



**Returns:** `GH_Gradient` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_GreyScale.htm)

#### `public static GH_Gradient Heat()`



**Returns:** `GH_Gradient` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_Heat.htm)

#### `public bool MouseDown(RectangleF dest, PointF pt)`

Begin grip drag operation.

**Parameters:**
- `dest` (System.Drawing.RectangleF) — Destination rectangle of the gradient.
- `pt` (System.Drawing.PointF) — Mouse position in destination coordinates.

**Returns:** `Boolean` — True if a grip drag was started, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_MouseDown.htm)

#### `public bool MouseDragAbort()`

Abort grip drag.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_MouseDragAbort.htm)

#### `public bool MouseMove(RectangleF dest, PointF pt)`

Continue grip dragging operation.

**Parameters:**
- `dest` (System.Drawing.RectangleF) — Destination rectangle of gradient.
- `pt` (System.Drawing.PointF) — Mouse position in destination coordinates.

**Returns:** `Boolean` — True if a grip position was changed, false otherwise. Redraws are only required on TRUE.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_MouseMove.htm)

#### `public bool MouseUp(RectangleF dest, PointF pt, bool deselect)`

Terminate grip drag.

**Parameters:**
- `dest` (System.Drawing.RectangleF) — Destination rectangle of gradient
- `pt` (System.Drawing.PointF) — Mouse location in destination coordinates
- `deselect` (System.Boolean)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_MouseUp.htm)

#### `public int NearestGrip(double t)`

Find the nearest grip to a parameter.

**Parameters:**
- `t` (System.Double) — Parameter to search from.

**Returns:** `Int32` — The grip nearest to the parameter.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_NearestGrip.htm)

#### `public int NearestGrip(double t, GH_GripSide side)`

Find the nearest grip to a parameter.

**Parameters:**
- `t` (System.Double) — Parameter to search from.
- `side` (Grasshopper.GUI.Gradient.GH_GripSide) — Side to search.

**Returns:** `Int32` — The grip nearest to the parameter.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_NearestGrip_1.htm)

#### `public int NearestGrip(RectangleF dest, PointF pt, double maxRadius)`

Find the nearest grip in screen coordinates.

**Parameters:**
- `dest` (System.Drawing.RectangleF) — Retangle that contains the entire gradient.
- `pt` (System.Drawing.PointF) — Point to search from.
- `maxRadius` (System.Double) — Maximum distance between point and grip.

**Returns:** `Int32` — The grip closest to the point.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_NearestGrip_2.htm)

#### `public static RectangleF NewGripRegion(RectangleF destination)`

Gets the new Grip region rectangle based on a gradient destination rectangle.

**Parameters:**
- `destination` (System.Drawing.RectangleF) — Gradient destination rectangle.

**Returns:** `RectangleF` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_NewGripRegion.htm)

#### `public void NormalizeGrips()`

Normalize all grips. This will scale all grip parameters so that the full gradient extend equals 0~1.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_NormalizeGrips.htm)

#### `public void OnGradientChanged()`

Raise the GradientChanged event.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_OnGradientChanged.htm)

#### `public void OnGradientChangedIntermediate()`

Raise the GradientChanged event.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_OnGradientChangedIntermediate.htm)

#### `public void OnSelectionChanged()`

Raise the SelectionChanged event.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_OnSelectionChanged.htm)

#### `public bool Read(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_Read.htm)

#### `public void RemoveGrip(GH_Grip grip)`

Remove the specified grip.

**Parameters:**
- `grip` (Grasshopper.GUI.Gradient.GH_Grip) — Grip to remove.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_RemoveGrip.htm)

#### `public void RemoveGrip(int index)`

Remove the grip at the specified index.

**Parameters:**
- `index` (System.Int32) — Index of grip to remove.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_RemoveGrip_1.htm)

#### `public void Render_Background(Graphics g, RectangleF dest)`



**Parameters:**
- `g` (System.Drawing.Graphics)
- `dest` (System.Drawing.RectangleF)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_Render_Background.htm)

#### `public void Render_Gradient(Graphics g, RectangleF dest)`



**Parameters:**
- `g` (System.Drawing.Graphics)
- `dest` (System.Drawing.RectangleF)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_Render_Gradient.htm)

#### `public void Render_Grips(Graphics g, RectangleF dest)`



**Parameters:**
- `g` (System.Drawing.Graphics)
- `dest` (System.Drawing.RectangleF)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_Render_Grips.htm)

#### `public static GH_Gradient SoGay()`



**Returns:** `GH_Gradient` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_SoGay.htm)

#### `public static GH_Gradient Spectrum()`



**Returns:** `GH_Gradient` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_Spectrum.htm)

#### `public static GH_Gradient Traffic()`



**Returns:** `GH_Gradient` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_Traffic.htm)

#### `public bool Write(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_Write.htm)

#### `public static GH_Gradient Zebra()`



**Returns:** `GH_Gradient` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Gradient_Zebra.htm)

### Properties
- `Grip` (GH_Grip, get/set) — Gets or sets the grip at the given index.
- `GripCount` (Int32, get) — Gets the number of grips in this gradient.
- `Linear` (Boolean, get/set) — Gets or sets whether the colours are interpolated linearly between grips.
- `Locked` (Boolean, get/set) — Gets or sets whether this gradient is locked. A locked gradient cannot be modified by the user.
- `SelectedGrip` (GH_Grip, get/set) — Gets or sets the selected grip.

### Events
#### `GradientChanged` (`Grasshopper.GUI.Gradient.GH_Gradient.GradientChangedEventHandler`)

**Signature:** `public event GH_Gradient.GradientChangedEventHandler GradientChanged`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Gradient_GH_Gradient_GradientChanged.htm)

#### `SelectionChanged` (`Grasshopper.GUI.Gradient.GH_Gradient.SelectionChangedEventHandler`)

**Signature:** `public event GH_Gradient.SelectionChangedEventHandler SelectionChanged`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Gradient_GH_Gradient_SelectionChanged.htm)

## GH_Gradient.GradientChangedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Gradient.GH_Gradient.GradientChangedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Gradient_GH_Gradient_GradientChangedEventHandler.htm)

## GH_Gradient.SelectionChangedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Gradient.GH_Gradient.SelectionChangedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Gradient_GH_Gradient_SelectionChangedEventHandler.htm)

## GH_GradientChangedEventArgs (class)

Arguments for te GradientChanged event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Gradient_GH_GradientChangedEventArgs.htm)

### Constructors
- `public GH_GradientChangedEventArgs(GH_Gradient gradient, bool intermediate)` — Initializes a new instance of the GH_GradientChangedEventArgs class

### Properties
- `Gradient` (GH_Gradient, get) — Gets the gradient that raised this event.
- `Intermediate` (Boolean, get) — Gets whether this event is intermediate. I.e. whether it will be followed by another, similar event.

## GH_Grip (class)

Represents a grip in a gradient. A grip defines both where and how the colour of a gradient changes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Gradient_GH_Grip.htm)

### Constructors
- `public GH_Grip()` — Initializes a new instance of the GH_Grip class
- `public GH_Grip(GH_Grip other)` — Initializes a new instance of the GH_Grip class
- `public GH_Grip(double parameter, Color colour)` — Initializes a new instance of the GH_Grip class
- `public GH_Grip(double parameter, Color colourLeft, Color colourRight)` — Initializes a new instance of the GH_Grip class

### Methods
#### `public static Color Blend(Color A, Color B, double t)`



**Parameters:**
- `A` (System.Drawing.Color)
- `B` (System.Drawing.Color)
- `t` (System.Double)

**Returns:** `Color` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Grip_Blend.htm)

#### `public int CompareTo(GH_Grip other)`



**Parameters:**
- `other` (Grasshopper.GUI.Gradient.GH_Grip)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Grip_CompareTo.htm)

#### `public void MutateId()`

Create a new grip ID.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Grip_MutateId.htm)

#### `public bool Read(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Grip_Read.htm)

#### `public bool Write(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Gradient_GH_Grip_Write.htm)

### Properties
- `ColourLeft` (Color, get/set) — Gets the colour to the left of this grip.
- `ColourRight` (Color, get/set) — Gets the colour to the right of this grip.
- `GripId` (Guid, get) — Gets the Grip ID.
- `IsValid` (Boolean, get) — Gets whether this grip is valid. A valid grip must have a non-empty Id and a non-NaN parameter.
- `Parameter` (Double, get/set) — Gets or sets the numeric parameter of the grip.
- `Selected` (Boolean, get/set) — Gets or sets whether this grip is selected. Selection state is not (de)serialized.
- `Type` (GH_GripType, get) — Gets whether the colour across this grip is smooth.

## GH_GripSide (enum)

Enumerates the possible types of colour-in/colour-out behaviours of Gradient Grips.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Gradient_GH_GripSide.htm)

### Values
- `Both` = `0`
- `Left` = `1`
- `Right` = `2`

## GH_GripType (enum)

Enumerates the possible grip types.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Gradient_GH_GripType.htm)

### Values
- `Continuous` = `0` — Grip has the same colour coming in and going out.
- `Discontinuous` = `1` — Grip has a different colour coming in and going out.

