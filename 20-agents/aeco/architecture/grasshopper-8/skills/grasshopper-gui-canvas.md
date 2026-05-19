---
name: grasshopper-grasshopper-gui-canvas
description: This skill encodes the grasshopper 8.0 surface of the Grasshopper.GUI.Canvas namespace — 58 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GH_Border, GH_Canvas, GH_CanvasDocumentChangedEventArgs, GH_Canvas.GH_ImageSettings, GH_CanvasDropTargetValidator, GH_CanvasValidator, GH_CanvasViewportChangedEventArgs, GH_CanvasWidgetListEventArgs, and 50 more types.
---

# Grasshopper.GUI.Canvas

Auto-generated from vendor docs for grasshopper 8.0. 58 types in this namespace.

## GH_Border (class)

Represents a resizing border of a Grasshopper canvas object.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Border.htm)

### Constructors
- `public GH_Border(RectangleF rectangle, GH_BorderTopology topology)` — Create a new border.

### Methods
#### `public bool Contains(PointF pt)`

Test a point for border inclusion.

**Parameters:**
- `pt` (System.Drawing.PointF) — Point to test.

**Returns:** `Boolean` — True if pt is inside or on this border.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Border_Contains.htm)

#### `public static List<GH_Border> CreateBorders(RectangleF box, int borderSize)`

Static (Shared) method for creating a set of borders.

**Parameters:**
- `box` (System.Drawing.RectangleF) — Box for border creation.
- `borderSize` (System.Int32) — The width of all borders.

**Returns:** `List<GH_Border>` — A list of borders.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Border_CreateBorders.htm)

#### `public static List<GH_Border> CreateBorders(RectangleF box, int borderWidth, int borderHeight)`

Static (Shared) method for creating a set of borders.

**Parameters:**
- `box` (System.Drawing.RectangleF) — Box for border creation.
- `borderWidth` (System.Int32) — The width of all vertical.
- `borderHeight` (System.Int32) — The height of all horizontal borders.

**Returns:** `List<GH_Border>` — A list of borders.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Border_CreateBorders_1.htm)

#### `public static List<GH_Border> CreateBorders(RectangleF box, int borderLeft, int borderRight, int borderTop, int borderBottom)`

Static (Shared) method for creating a set of borders.

**Parameters:**
- `box` (System.Drawing.RectangleF) — Box for border creation.
- `borderLeft` (System.Int32) — Width of the leftmost border.
- `borderRight` (System.Int32) — Width of the rightmost border.
- `borderTop` (System.Int32) — Width of the topmost border.
- `borderBottom` (System.Int32) — Width of the bottommost border.

**Returns:** `List<GH_Border>` — A list of borders.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Border_CreateBorders_2.htm)

#### `public static List<GH_Border> CreateBorders(RectangleF box, Padding padding)`

Static (Shared) method for creating a set of borders.

**Parameters:**
- `box` (System.Drawing.RectangleF) — Box for border creation.
- `padding` (System.Windows.Forms.Padding) — The size of the borders along all four edges.

**Returns:** `List<GH_Border>` — A list of borders.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Border_CreateBorders_3.htm)

### Properties
- `Region` (RectangleF, get) — Gets the shape of the border.
- `Size_Cursor` (Cursor, get) — Gets the cursor associated with this border.
- `Topology` (GH_BorderTopology, get) — Gets the type of border.

## GH_BorderTopology (enum)

Enumerates all possble border types.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_BorderTopology.htm)

### Values
- `None` = `-2` — Unset value.
- `All` = `-1` — All possible types.
- `TopRight` = `0` — Corner border in the top right area of the box.
- `TopLeft` = `1` — Corner border in the top left area of the box.
- `BottomLeft` = `2` — Corner border in the bottom left area of the box.
- `BottomRight` = `3` — Corner border in the bottom right area of the box.
- `Top` = `4` — Edge border along the top area of the box.
- `Left` = `5` — Edge border along the left area of the box.
- `Bottom` = `6` — Edge border along the bottom area of the box.
- `Right` = `7` — Edge border along the right area of the box.

## GH_Canvas (class)

The GH_Canvas is the control that handles all mouse and paint events for a single loaded document.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas.htm)

### Constructors
- `public GH_Canvas()` — Initializes a new instance of the GH_Canvas class

### Methods
#### `public void AddTagArtist(IGH_TagArtist artist)`

Add a Tag Artist instance to this canvas.

**Parameters:**
- `artist` (Grasshopper.GUI.Canvas.TagArtists.IGH_TagArtist) — Tag artist to add

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_AddTagArtist.htm)

#### `public void AddValidator(IGH_CanvasValidator validator)`

Add a new drop validator to the canvas.

**Parameters:**
- `validator` (Grasshopper.GUI.Canvas.IGH_CanvasValidator) — Validator to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_AddValidator.htm)

#### `public void AutoSaveDocument(GH_AutoSaveTrigger trigger)`



**Parameters:**
- `trigger` (Grasshopper.Kernel.GH_AutoSaveTrigger)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_AutoSaveDocument.htm)

#### `public ToolStripDropDownMenu CanvasOldSchoolMenu()`



**Returns:** `ToolStripDropDownMenu` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_CanvasOldSchoolMenu.htm)

#### `public void CreateMRUPanels()`

Create new MRU panels. This function only does something if there is no document loaded in the canvas.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_CreateMRUPanels.htm)

#### `public Bitmap CreatePreview(Size size)`

Render this canvas to a thumbnail preview.

**Parameters:**
- `size` (System.Drawing.Size)

**Returns:** `Bitmap` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_CreatePreview.htm)

#### `public void DestroyMRUPanels()`

Destroy any MRU panels that might be in existence.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_DestroyMRUPanels.htm)

#### `protected override void Dispose(bool disposing)`

Releases the unmanaged resources used by the GH_Canvas and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_Dispose.htm)

#### `public List<string> GenerateHiResImage(Rectangle rec, GH_Canvas.GH_ImageSettings settings, out Size totalSize)`

Generate a collection of hi-res images of the document.

**Parameters:**
- `rec` (System.Drawing.Rectangle) — BoundingBox rectangle of export.
- `settings` (Grasshopper.GUI.Canvas.GH_Canvas.GH_ImageSettings) — Hi-res settings.
- `totalSize` (System.Drawing.Size)

**Returns:** `List<String>` — List of file paths that indicate the individual images.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_GenerateHiResImage.htm)

#### `public Bitmap GenerateHiResImageTile(GH_Viewport vp, Color bg)`

Generate a single tile in a Hi-Res image export.

**Parameters:**
- `vp` (Grasshopper.GUI.Canvas.GH_Viewport) — Viewport override
- `bg` (System.Drawing.Color) — Background color override.

**Returns:** `Bitmap` — A list of image tiles.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_GenerateHiResImageTile.htm)

#### `public Bitmap GetCanvasScreenBuffer(GH_CanvasMode modeOverride)`

Get a bitmap that resembles the current state of the canvas.

**Parameters:**
- `modeOverride` (Grasshopper.GUI.Canvas.GH_CanvasMode) — The mode to apply during the off-screen rendering.

**Returns:** `Bitmap` — A Format24bppRgb bitmap similar to the current canvas display.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_GetCanvasScreenBuffer.htm)

#### `public Graphics GetGraphicsObject(bool setupDisplayTransform)`

Gets a graphics object for this control. You are not allowed to draw with this object, use it only for visibility testing and such. If you're inside a canvas update, use the Graphics() property instead. You must dispose of the Graphics object returned by this method or resources will be leaked.

**Parameters:**
- `setupDisplayTransform` (System.Boolean) — If true, the display transform will be added to the graphics object.

**Returns:** `Graphics` — A non-drawing Graphics object on success or null on failure..

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_GetGraphicsObject.htm)

#### `public void HideMRUPanels()`

Hide all existing MRU panels by sliding them out of view.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_HideMRUPanels.htm)

#### `public bool InstantiateNewObject(Guid id, PointF at, bool update)`



**Parameters:**
- `id` (System.Guid)
- `at` (System.Drawing.PointF)
- `update` (System.Boolean)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_InstantiateNewObject.htm)

#### `public bool InstantiateNewObject(Guid id, string init_code, PointF at, bool update)`



**Parameters:**
- `id` (System.Guid)
- `init_code` (System.String)
- `at` (System.Drawing.PointF)
- `update` (System.Boolean)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_InstantiateNewObject_1.htm)

#### `public void Navigate(GH_CanvasNavigation navigation)`

Perform a single navigation step.

**Parameters:**
- `navigation` (Grasshopper.GUI.Canvas.GH_CanvasNavigation) — Type of navigation step.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_Navigate.htm)

#### `protected override void OnPaint(PaintEventArgs e)`

(Overrides Control.OnPaint(PaintEventArgs).)

**Parameters:**
- `e` (System.Windows.Forms.PaintEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_OnPaint.htm)

#### `protected override void OnPaintBackground(PaintEventArgs e)`

(Overrides Control.OnPaintBackground(PaintEventArgs).)

**Parameters:**
- `e` (System.Windows.Forms.PaintEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_OnPaintBackground.htm)

#### `public void OnViewportChanged()`

Ensure the canvas and document viewport data are synchronised and raises the ViewportChanged event.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_OnViewportChanged.htm)

#### `public void ReevaluateMarkovSuggestions()`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_ReevaluateMarkovSuggestions.htm)

#### `public void RemoveAllTagArtists()`

Remove all IGH_TagArtists from this canvas. Do not use this method unless you want to screw over everyone else.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_RemoveAllTagArtists.htm)

#### `public int RemoveTagArtist(Guid artistId)`

Remove all IGH_TagArtists from this canvas that match a certain ID.

**Parameters:**
- `artistId` (System.Guid) — ID of IGH_TagArtists to remove.

**Returns:** `Int32` — The number of IGH_TagArtist instances removed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_RemoveTagArtist_1.htm)

#### `public int RemoveTagArtist(IGH_TagArtist artist)`

Remove a specific tag artist from this canvas. If the instance occurs multiple times in the TagArtist list, all instances will be removed.

**Parameters:**
- `artist` (Grasshopper.GUI.Canvas.TagArtists.IGH_TagArtist) — TagArtist to remove.

**Returns:** `Int32` — The number of IGH_TagArtist instances removed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_RemoveTagArtist.htm)

#### `public void RemoveValidator(IGH_CanvasValidator validator)`

Remove a drop validator from the canvas.

**Parameters:**
- `validator` (Grasshopper.GUI.Canvas.IGH_CanvasValidator) — Validator to remove.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_RemoveValidator.htm)

#### `public void ScheduleRegen(int delay)`

Schedule a regen to occur after the specified number of milliseconds have elapsed. If a Regen is called in this time frame, the schedule will be cleared. Only a single schedule can be active at any time, so any call to ScheduleRegen will clear existing schedules.

**Parameters:**
- `delay` (System.Int32) — Number of milliseconds to wait until repainting the canvas. intervals equal to or less than zero will be ignored.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_ScheduleRegen.htm)

#### `public void SetSmartTextRenderingHint()`

When this method is called during a redraw, the TextRenderingHint of the associated graphics object will be set to either GH_CrispText or GH_SmoothText depending on zoom level.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_SetSmartTextRenderingHint.htm)

#### `public bool ShowComponentSearchBox()`

Display the component Search dialog at the current mouse location. The dialog cannot be shown if ModifiersEnabled=False.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_ShowComponentSearchBox.htm)

#### `public bool ShowComponentSearchBox(Point at)`

Display the component Search dialog at the given coordinate. The dialog cannot be shown if ModifiersEnabled=False.

**Parameters:**
- `at` (System.Drawing.Point) — Location (in screen space) for the center of the search box.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_ShowComponentSearchBox_1.htm)

#### `public void ShowMRUPanels()`

Show any hidden MRU panels by sliding them back into view.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_ShowMRUPanels.htm)

#### `public void ShowNavigationPane()`

Displays the quick-navigation popup pane at the cursor.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_ShowNavigationPane.htm)

#### `public void ShowSearchDialog(bool centerOnCursor)`

Display the Find dialog. If a Find dialog is already active for this canvas, nothing will happen.

**Parameters:**
- `centerOnCursor` (System.Boolean) — If true, the dialog will be centered on the cursor, otherwise it will be put in the upper left corner of the canvas.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_ShowSearchDialog.htm)

#### `public void StartAutoPan()`

Start the auto-panning timer.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_StartAutoPan.htm)

#### `public void StopAutoPan()`

Stop the auto-panning timer.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_StopAutoPan.htm)

#### `public void UpdateDocumentPreview()`

Call this method to update the preview thumbnail for the currently loaded document. If no document is currently loaded, nothing will happen.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Canvas_UpdateDocumentPreview.htm)

### Properties
- `ActiveInteraction` (IGH_MouseInteraction, get/set) — Gets or sets the currently active interaction object.
- `ActiveObject` (IGH_DocumentObject, get/set) — Gets or sets the currently active object.
- `ActiveWidget` (IGH_Widget, get/set) — Gets or sets the currently active widget.
- `CursorCanvasPosition` (PointF, get) — Gets the location of the cursor in Canvas coordinates.
- `CursorControlPosition` (Point, get) — Gets the location of the cursor in Control coordinates.
- `DisplayVoronoiWarning` (Boolean, get/set) — Gets or sets whether the Voronoi Over-dose warning message is displayed. This property is set once at Grasshopper startup to True.
- `Document` (GH_Document, get/set) — Gets or sets the document currently loaded in this canvas.
- `DrawingMode` (GH_CanvasMode, get) — Gets the current drawing mode for this canvas.
- `HasControlWithFocus` (Boolean, get) — Gets whether or not this canvas has a child control on it with focus.
- `IsActiveInteraction` (Boolean, get) — Gets a value indicating whether or not there is an interaction object loaded in this canvas.
- `IsActiveObject` (Boolean, get) — Gets a value indicating whether or not an object is currently activated.
- `IsActiveWidget` (Boolean, get) — Gets a value indicating whether or not a widget is currently activated.
- `IsDocument` (Boolean, get) — Gets a value indicating whether or not a document is currently loaded in this canvas.
- `MarkovSuggestions` (List<IGH_ObjectProxy>, get) — 
- `ModifiersEnabled` (Boolean, get/set) — Gets or sets the modifiers enabled flag. When modifiers are disabled, only zooming and panning is still allowed.
- `NavigationPanDown` (Keys, get/set) — Gets or sets the special key for panning down.
- `NavigationPanLeft` (Keys, get/set) — Gets or sets the special key for panning left.
- `NavigationPanPixels` (Int32, get/set) — Gets or sets the number of pixels for each pan operation.
- `NavigationPanRight` (Keys, get/set) — Gets or sets the special key for panning right.
- `NavigationPanUp` (Keys, get/set) — Gets or sets the special key for panning up.
- `NavigationZoomFactor` (Double, get/set) — Gets or sets the zoom factor for navigation zoom operations.
- `NavigationZoomIn` (Keys, get/set) — Gets or sets the special key for zooming in.
- `NavigationZoomOut` (Keys, get/set) — Gets or sets the special key for zooming in.
- `Painter` (GH_Painter, get) — Gets the painter object that handles most of the drawing logic for this canvas.
- `Painting` (Boolean, get) — Gets whether this canvas is currently busy painting itself.
- `RecordPreviewBoundary` (Boolean, get/set) — 
- `TagArtistIDs` (List<Guid>, get) — Gets all the TagArtist IDs in this Canvas.
- `ThumbnailSize` (Size, get) — Gets the thumbnail size for documents.
- `TooltipDelay` (Int32, get) — Gets the delay (in milliseconds) required for a tooltip popup. This delay is a user setting stored under the Canvas:TooltipDelay field of the core settings.
- `Validator` (IGH_CanvasValidator, get) — Provides access to all the validators associated with this canvas.
- `Viewport` (GH_Viewport, get) — Gets the viewport that determines the panning and zooming for this canvas.
- `Widgets` (List<IGH_Widget>, get) — Gets a list of all the widgets on this canvas.
- `ZoomFadeHigh` (Int32, get) — Gets the ZUI fade alpha value for the high zoom level threshold. This static field gets set on every Canvas paint start. The high threshold is typically used for ZUI elements that only appear when zoomed in.
- `ZoomFadeLow` (Int32, get) — Gets the ZUI fade alpha value for the low zoom level threshold. This static field gets set on every Canvas paint start. The low threshold is typically used for fading of icons and object names.
- `ZoomFadeMedium` (Int32, get) — Gets the ZUI fade alpha value for the medium zoom level threshold. This static field gets set on every Canvas paint start. The medium threshold is typically used for non-informative UI elements such as highlights.
- `ScheduleInactive` (Int32, get) — Defines the delay used to indicate the absence of a schedule.
- `ThumbnailHeight` (Int32, get) — 
- `ThumbnailWidth` (Int32, get) — 

### Events
#### `CanvasPaintBackground` (`Grasshopper.GUI.Canvas.GH_Canvas.CanvasPaintBackgroundEventHandler`)

**Signature:** `public event GH_Canvas.CanvasPaintBackgroundEventHandler CanvasPaintBackground`

Raised after the background has been drawn.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPaintBackground.htm)

#### `CanvasPaintBegin` (`Grasshopper.GUI.Canvas.GH_Canvas.CanvasPaintBeginEventHandler`)

**Signature:** `public event GH_Canvas.CanvasPaintBeginEventHandler CanvasPaintBegin`

Raised before a new paint operation starts. This event is always raised, even if the Canvas isn't visible.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPaintBegin.htm)

#### `CanvasPaintEnd` (`Grasshopper.GUI.Canvas.GH_Canvas.CanvasPaintEndEventHandler`)

**Signature:** `public event GH_Canvas.CanvasPaintEndEventHandler CanvasPaintEnd`

Raised after a paint operation completes. This event is always raised, even if the Canvas isn't visible.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPaintEnd.htm)

#### `CanvasPostPaintGroups` (`Grasshopper.GUI.Canvas.GH_Canvas.CanvasPostPaintGroupsEventHandler`)

**Signature:** `public event GH_Canvas.CanvasPostPaintGroupsEventHandler CanvasPostPaintGroups`

Raised after group drawing completes.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPostPaintGroups.htm)

#### `CanvasPostPaintObjects` (`Grasshopper.GUI.Canvas.GH_Canvas.CanvasPostPaintObjectsEventHandler`)

**Signature:** `public event GH_Canvas.CanvasPostPaintObjectsEventHandler CanvasPostPaintObjects`

Raised after object drawing completes.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPostPaintObjects.htm)

#### `CanvasPostPaintOverlay` (`Grasshopper.GUI.Canvas.GH_Canvas.CanvasPostPaintOverlayEventHandler`)

**Signature:** `public event GH_Canvas.CanvasPostPaintOverlayEventHandler CanvasPostPaintOverlay`

Raised after object overlay drawing completes.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPostPaintOverlay.htm)

#### `CanvasPostPaintWidgets` (`Grasshopper.GUI.Canvas.GH_Canvas.CanvasPostPaintWidgetsEventHandler`)

**Signature:** `public event GH_Canvas.CanvasPostPaintWidgetsEventHandler CanvasPostPaintWidgets`

Raised after widgets are drawn. This is the final event in the Drawing pipeline.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPostPaintWidgets.htm)

#### `CanvasPostPaintWires` (`Grasshopper.GUI.Canvas.GH_Canvas.CanvasPostPaintWiresEventHandler`)

**Signature:** `public event GH_Canvas.CanvasPostPaintWiresEventHandler CanvasPostPaintWires`

Raised after wire drawing completes.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPostPaintWires.htm)

#### `CanvasPrePaintGroups` (`Grasshopper.GUI.Canvas.GH_Canvas.CanvasPrePaintGroupsEventHandler`)

**Signature:** `public event GH_Canvas.CanvasPrePaintGroupsEventHandler CanvasPrePaintGroups`

Raised before group drawing starts.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPrePaintGroups.htm)

#### `CanvasPrePaintObjects` (`Grasshopper.GUI.Canvas.GH_Canvas.CanvasPrePaintObjectsEventHandler`)

**Signature:** `public event GH_Canvas.CanvasPrePaintObjectsEventHandler CanvasPrePaintObjects`

Raised before object drawing starts.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPrePaintObjects.htm)

#### `CanvasPrePaintOverlay` (`Grasshopper.GUI.Canvas.GH_Canvas.CanvasPrePaintOverlayEventHandler`)

**Signature:** `public event GH_Canvas.CanvasPrePaintOverlayEventHandler CanvasPrePaintOverlay`

Raised before object overlay drawing starts.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPrePaintOverlay.htm)

#### `CanvasPrePaintWidgets` (`Grasshopper.GUI.Canvas.GH_Canvas.CanvasPrePaintWidgetsEventHandler`)

**Signature:** `public event GH_Canvas.CanvasPrePaintWidgetsEventHandler CanvasPrePaintWidgets`

Raised before Widgets are drawn.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPrePaintWidgets.htm)

#### `CanvasPrePaintWires` (`Grasshopper.GUI.Canvas.GH_Canvas.CanvasPrePaintWiresEventHandler`)

**Signature:** `public event GH_Canvas.CanvasPrePaintWiresEventHandler CanvasPrePaintWires`

Raised before wire drawing starts.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPrePaintWires.htm)

#### `Document_ModifiedChanged` (`Grasshopper.GUI.Canvas.GH_Canvas.Document_ModifiedChangedEventHandler`)

**Signature:** `public event GH_Canvas.Document_ModifiedChangedEventHandler Document_ModifiedChanged`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_Document_ModifiedChanged.htm)

#### `Document_ObjectsAdded` (`Grasshopper.GUI.Canvas.GH_Canvas.Document_ObjectsAddedEventHandler`)

**Signature:** `public event GH_Canvas.Document_ObjectsAddedEventHandler Document_ObjectsAdded`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_Document_ObjectsAdded.htm)

#### `Document_ObjectsDeleted` (`Grasshopper.GUI.Canvas.GH_Canvas.Document_ObjectsDeletedEventHandler`)

**Signature:** `public event GH_Canvas.Document_ObjectsDeletedEventHandler Document_ObjectsDeleted`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_Document_ObjectsDeleted.htm)

#### `Document_SettingsChanged` (`Grasshopper.GUI.Canvas.GH_Canvas.Document_SettingsChangedEventHandler`)

**Signature:** `public event GH_Canvas.Document_SettingsChangedEventHandler Document_SettingsChanged`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_Document_SettingsChanged.htm)

#### `DocumentChanged` (`Grasshopper.GUI.Canvas.GH_Canvas.DocumentChangedEventHandler`)

**Signature:** `public event GH_Canvas.DocumentChangedEventHandler DocumentChanged`

This event is raised whenever a different document is loaded into this canvas.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_DocumentChanged.htm)

#### `DocumentObjectMouseDown` (`Grasshopper.GUI.Canvas.GH_Canvas.DocumentObjectMouseDownEventHandler`)

**Signature:** `public event GH_Canvas.DocumentObjectMouseDownEventHandler DocumentObjectMouseDown`

This event is raised whenever the left mouse button is pressed while over a Document object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_DocumentObjectMouseDown.htm)

#### `ModifiersChanged` (`Grasshopper.GUI.Canvas.GH_Canvas.ModifiersChangedEventHandler`)

**Signature:** `public event GH_Canvas.ModifiersChangedEventHandler ModifiersChanged`

This event is raised whenever the ModifiersEnabled property changes.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_ModifiersChanged.htm)

#### `NavigationSettingsChanged` (`Grasshopper.GUI.Canvas.GH_Canvas.NavigationSettingsChangedEventHandler`)

**Signature:** `public static event GH_Canvas.NavigationSettingsChangedEventHandler NavigationSettingsChanged`

Raised whenever any of the shared navigation properties changed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_NavigationSettingsChanged.htm)

#### `ViewportChanged` (`Grasshopper.GUI.Canvas.GH_Canvas.ViewportChangedEventHandler`)

**Signature:** `public event GH_Canvas.ViewportChangedEventHandler ViewportChanged`

This event is raised whenever the viewport properties are modified, for example when the pan or zoom values are affected.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_ViewportChanged.htm)

#### `WidgetListCreated` (`Grasshopper.GUI.Canvas.GH_Canvas.WidgetListCreatedEventHandler`)

**Signature:** `public static event GH_Canvas.WidgetListCreatedEventHandler WidgetListCreated`

This event is raised once for every GH_Canvas object that is created anew. The event is Shared (static) because it is raised inside the constructor of a GH_Canvas object and therefor cannot be registered on an instance.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_Canvas_WidgetListCreated.htm)

## GH_Canvas.CanvasPaintBackgroundEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.CanvasPaintBackgroundEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPaintBackgroundEventHandler.htm)

## GH_Canvas.CanvasPaintBeginEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.CanvasPaintBeginEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPaintBeginEventHandler.htm)

## GH_Canvas.CanvasPaintEndEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.CanvasPaintEndEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPaintEndEventHandler.htm)

## GH_Canvas.CanvasPostPaintGroupsEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.CanvasPostPaintGroupsEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPostPaintGroupsEventHandler.htm)

## GH_Canvas.CanvasPostPaintObjectsEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.CanvasPostPaintObjectsEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPostPaintObjectsEventHandler.htm)

## GH_Canvas.CanvasPostPaintOverlayEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.CanvasPostPaintOverlayEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPostPaintOverlayEventHandler.htm)

## GH_Canvas.CanvasPostPaintWidgetsEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.CanvasPostPaintWidgetsEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPostPaintWidgetsEventHandler.htm)

## GH_Canvas.CanvasPostPaintWiresEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.CanvasPostPaintWiresEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPostPaintWiresEventHandler.htm)

## GH_Canvas.CanvasPrePaintGroupsEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.CanvasPrePaintGroupsEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPrePaintGroupsEventHandler.htm)

## GH_Canvas.CanvasPrePaintObjectsEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.CanvasPrePaintObjectsEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPrePaintObjectsEventHandler.htm)

## GH_Canvas.CanvasPrePaintOverlayEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.CanvasPrePaintOverlayEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPrePaintOverlayEventHandler.htm)

## GH_Canvas.CanvasPrePaintWidgetsEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.CanvasPrePaintWidgetsEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPrePaintWidgetsEventHandler.htm)

## GH_Canvas.CanvasPrePaintWiresEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.CanvasPrePaintWiresEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_CanvasPrePaintWiresEventHandler.htm)

## GH_Canvas.DocumentChangedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.DocumentChangedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_DocumentChangedEventHandler.htm)

## GH_Canvas.DocumentObjectMouseDownEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.DocumentObjectMouseDownEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_DocumentObjectMouseDownEventHandler.htm)

## GH_Canvas.DocumentObjectsDeletedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.DocumentObjectsDeletedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_DocumentObjectsDeletedEventHandler.htm)

## GH_Canvas.Document_ModifiedChangedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.Document_ModifiedChangedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_Document_ModifiedChangedEventHandler.htm)

## GH_Canvas.Document_ObjectsAddedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.Document_ObjectsAddedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_Document_ObjectsAddedEventHandler.htm)

## GH_Canvas.Document_ObjectsDeletedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.Document_ObjectsDeletedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_Document_ObjectsDeletedEventHandler.htm)

## GH_Canvas.Document_SettingsChangedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.Document_SettingsChangedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_Document_SettingsChangedEventHandler.htm)

## GH_Canvas.GH_ImageSettings (class)

Settings class for high-resolution image output.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_GH_ImageSettings.htm)

### Constructors
- `public GH_ImageSettings()` — Create a new instance of the GH_ImageSettings class with default values.
- `public GH_ImageSettings(string filePath)` — Create a new instance of the GH_ImageSettings class.
- `public GH_ImageSettings(string folder, string filename, string extension)` — Create a new instance of the GH_ImageSettings class.

### Properties
- `BackColour` (Color, get/set) — Gets or sets the background colour to use for the high-resolution rendering.
- `Extension` (String, get/set) — Gets or sets the image extension of the target image.
- `FileName` (String, get/set) — Gets or sets the filename of the target image.
- `Folder` (String, get/set) — Gets or sets the folder in which the target image will be saved.
- `TileSize` (Size, get) — Gets the default tile size for high-resolution export. This is a constant value.
- `Zoom` (Single, get/set) — Gets or sets the zoom at which to export the high-resolution image.

## GH_Canvas.ModifiersChangedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.ModifiersChangedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_ModifiersChangedEventHandler.htm)

## GH_Canvas.NavigationSettingsChangedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.NavigationSettingsChangedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_NavigationSettingsChangedEventHandler.htm)

## GH_Canvas.ViewportChangedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.ViewportChangedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_ViewportChangedEventHandler.htm)

## GH_Canvas.WidgetListCreatedEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_Canvas.WidgetListCreatedEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Canvas_WidgetListCreatedEventHandler.htm)

## GH_CanvasChannel (enum)

Enumerates all the drawing channels that are handled inside the Grasshopper canvas.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_CanvasChannel.htm)

### Values
- `First` = `0` — First channel. At present, the first channel equals the Groups channel.
- `Groups` = `0` — Channel for Group outlines and name tags.
- `Wires` = `10` — Channel for connecting wires.
- `Objects` = `20` — Channels for Document Objects (components, parameters, special objects)
- `Overlay` = `30` — Channel for drawing overlay geometry.
- `Last` = `30` — Last channel. At present, the last channel equals the Overlay channel.

## GH_CanvasDocumentChangedEventArgs (class)

These arguments are used in the DocumentChanged event on GH_Canvas.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_CanvasDocumentChangedEventArgs.htm)

### Constructors
- `public GH_CanvasDocumentChangedEventArgs(GH_Document newDoc, GH_Document oldDoc)` — Initializes a new instance of the GH_CanvasDocumentChangedEventArgs class

### Properties
- `NewDocument` (GH_Document, get) — Gets the document which has just been loaded into the canvas.
- `OldDocument` (GH_Document, get) — Gets the document which was previously loaded into the canvas.

## GH_CanvasDropTargetValidator (class)

Utility validator for restricting component creation.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_CanvasDropTargetValidator.htm)

### Constructors
- `public GH_CanvasDropTargetValidator()` — Initializes a new instance of the GH_CanvasDropTargetValidator class

### Methods
#### `public override void AddedToCanvas(GH_Canvas canvas)`

(Overrides GH_CanvasValidator.AddedToCanvas(GH_Canvas).)

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasDropTargetValidator_AddedToCanvas.htm)

#### `public override bool AppliesToDocument(Guid id)`

(Overrides GH_CanvasValidator.AppliesToDocument(Guid).)

**Parameters:**
- `id` (System.Guid)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasDropTargetValidator_AppliesToDocument.htm)

#### `public override bool CanAcceptObject(Guid id)`

(Overrides GH_CanvasValidator.CanAcceptObject(Guid).)

**Parameters:**
- `id` (System.Guid)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasDropTargetValidator_CanAcceptObject.htm)

#### `public override bool CanCreateObject(Guid id, PointF pt)`

(Overrides GH_CanvasValidator.CanCreateObject(Guid, PointF).)

**Parameters:**
- `id` (System.Guid)
- `pt` (System.Drawing.PointF)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasDropTargetValidator_CanCreateObject.htm)

#### `public virtual bool CanCreateWire(IGH_Param source, IGH_Param target)`

(Inherited from GH_CanvasValidator.)

**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param)
- `target` (Grasshopper.Kernel.IGH_Param)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasValidator_CanCreateWire.htm)

#### `public override bool CanDeleteObject(IGH_DocumentObject object)`

(Overrides GH_CanvasValidator.CanDeleteObject(IGH_DocumentObject).)

**Parameters:**
- `object` (Grasshopper.Kernel.IGH_DocumentObject)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasDropTargetValidator_CanDeleteObject.htm)

#### `public virtual bool CanDeleteWire(IGH_Param source, IGH_Param target)`

(Inherited from GH_CanvasValidator.)

**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param)
- `target` (Grasshopper.Kernel.IGH_Param)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasValidator_CanDeleteWire.htm)

#### `public override bool CanDragObject(IGH_DocumentObject object, PointF dragFromPoint)`

(Overrides GH_CanvasValidator.CanDragObject(IGH_DocumentObject, PointF).)

**Parameters:**
- `object` (Grasshopper.Kernel.IGH_DocumentObject)
- `dragFromPoint` (System.Drawing.PointF)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasDropTargetValidator_CanDragObject.htm)

#### `public virtual bool CanNavigateCanvas()`

(Inherited from GH_CanvasValidator.)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasValidator_CanNavigateCanvas.htm)

#### `public override bool CanShowCanvasMenu(PointF pt)`

(Overrides GH_CanvasValidator.CanShowCanvasMenu(PointF).)

**Parameters:**
- `pt` (System.Drawing.PointF)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasDropTargetValidator_CanShowCanvasMenu.htm)

#### `public override bool CanShowComponentSearchBox(PointF pt)`

(Overrides GH_CanvasValidator.CanShowComponentSearchBox(PointF).)

**Parameters:**
- `pt` (System.Drawing.PointF)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasDropTargetValidator_CanShowComponentSearchBox.htm)

#### `public virtual bool CanShowObjectMenu(IGH_DocumentObject object)`

(Inherited from GH_CanvasValidator.)

**Parameters:**
- `object` (Grasshopper.Kernel.IGH_DocumentObject)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasValidator_CanShowObjectMenu.htm)

#### `public override void RemovedFromCanvas(GH_Canvas canvas)`

(Overrides GH_CanvasValidator.RemovedFromCanvas(GH_Canvas).)

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasDropTargetValidator_RemovedFromCanvas.htm)

### Properties
- `Canvas` (GH_Canvas, get) — Gets the canvas this validator is associated with.
- `Document` (GH_Document, get) — Gets the document (if it exists) that is currently loaded in the canvas.
- `DropDelegate` (Action, get/set) — Gets or sets the delegate to be invoked when a component is dropped inside the dropregion.
- `DropID` (Guid, get/set) — Gets or sets the ID for the allowed component.
- `DropRegion` (RectangleF, get/set) — Gets or sets the dropregion for this validator. Regions are cloned so you can dispose of the original immedately after assignment.
- `DropText` (String, get/set) — Gets or sets the text to display.

## GH_CanvasMode (enum)

Enumerates all pre-defined Canvas modes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_CanvasMode.htm)

### Values
- `Control` = `0` — Indicates the canvas is currently drawing into the winforms control. This is the most common mode.
- `Thumbnail` = `1` — Indicates the canvas is currently drawing itself to a thumbnail image.
- `Export` = `2` — Indicates the canvas is currently drawing itself to an off-screen image. This typically happens during Hi-Res export operations.

## GH_CanvasNavigation (enum)

Enumerates all possible canvas navigations.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_CanvasNavigation.htm)

### Values
- `None` = `0`
- `PanLeft` = `1`
- `PanRight` = `2`
- `PanUp` = `3`
- `PanDown` = `4`
- `ZoomIn` = `5`
- `ZoomOut` = `6`

## GH_CanvasValidator (class)

Abstract implementation of IGH_CanvasValidator. Inherit from this class rather than implementing IGH_CanvasValidator from scratch.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_CanvasValidator.htm)

### Constructors
- `protected GH_CanvasValidator()` — Initializes a new instance of the GH_CanvasValidator class

### Methods
#### `public virtual void AddedToCanvas(GH_Canvas canvas)`

Override this method if you want to be informed when this validator gets assigned to a canvas. But *always* call the base class method.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasValidator_AddedToCanvas.htm)

#### `public virtual bool AppliesToDocument(Guid id)`



**Parameters:**
- `id` (System.Guid)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasValidator_AppliesToDocument.htm)

#### `public virtual bool CanAcceptObject(Guid id)`



**Parameters:**
- `id` (System.Guid)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasValidator_CanAcceptObject.htm)

#### `public virtual bool CanCreateObject(Guid id, PointF pt)`



**Parameters:**
- `id` (System.Guid)
- `pt` (System.Drawing.PointF)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasValidator_CanCreateObject.htm)

#### `public virtual bool CanCreateWire(IGH_Param source, IGH_Param target)`



**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param)
- `target` (Grasshopper.Kernel.IGH_Param)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasValidator_CanCreateWire.htm)

#### `public virtual bool CanDeleteObject(IGH_DocumentObject object)`



**Parameters:**
- `object` (Grasshopper.Kernel.IGH_DocumentObject)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasValidator_CanDeleteObject.htm)

#### `public virtual bool CanDeleteWire(IGH_Param source, IGH_Param target)`



**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param)
- `target` (Grasshopper.Kernel.IGH_Param)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasValidator_CanDeleteWire.htm)

#### `public virtual bool CanDragObject(IGH_DocumentObject object, PointF dragFromPoint)`



**Parameters:**
- `object` (Grasshopper.Kernel.IGH_DocumentObject)
- `dragFromPoint` (System.Drawing.PointF)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasValidator_CanDragObject.htm)

#### `public virtual bool CanNavigateCanvas()`



**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasValidator_CanNavigateCanvas.htm)

#### `public virtual bool CanShowCanvasMenu(PointF pt)`



**Parameters:**
- `pt` (System.Drawing.PointF)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasValidator_CanShowCanvasMenu.htm)

#### `public virtual bool CanShowComponentSearchBox(PointF pt)`



**Parameters:**
- `pt` (System.Drawing.PointF)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasValidator_CanShowComponentSearchBox.htm)

#### `public virtual bool CanShowObjectMenu(IGH_DocumentObject object)`



**Parameters:**
- `object` (Grasshopper.Kernel.IGH_DocumentObject)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasValidator_CanShowObjectMenu.htm)

#### `public virtual void RemovedFromCanvas(GH_Canvas canvas)`

Override this method if you want to be informed when this validator gets removed from a canvas. But *always* call the base class method.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasValidator_RemovedFromCanvas.htm)

### Properties
- `Canvas` (GH_Canvas, get) — Gets the canvas this validator is associated with.
- `Document` (GH_Document, get) — Gets the document (if it exists) that is currently loaded in the canvas.

## GH_CanvasViewportChangedEventArgs (class)

These arguments are used in the ViewportChanged event on GH_Canvas.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_CanvasViewportChangedEventArgs.htm)

### Properties
- `Canvas` (GH_Canvas, get) — Gets the Canvas whose viewport was changed.
- `Viewport` (GH_Viewport, get) — Gets the new Viewport.

## GH_CanvasWidgetListEventArgs (class)

The event arguments are used in the WidgetListCreated event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_CanvasWidgetListEventArgs.htm)

### Methods
#### `public void AddWidget(IGH_Widget widget)`

Append a widget to the canvas.

**Parameters:**
- `widget` (IGH_Widget) — Widget to append.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasWidgetListEventArgs_AddWidget.htm)

## GH_CanvasWireValidator (class)

Utility validator for restricting wire creation.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_CanvasWireValidator.htm)

### Constructors
- `public GH_CanvasWireValidator()` — Initializes a new instance of the GH_CanvasWireValidator class

### Methods
#### `public override void AddedToCanvas(GH_Canvas canvas)`

(Overrides GH_CanvasValidator.AddedToCanvas(GH_Canvas).)

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasWireValidator_AddedToCanvas.htm)

#### `public override bool AppliesToDocument(Guid id)`

(Overrides GH_CanvasValidator.AppliesToDocument(Guid).)

**Parameters:**
- `id` (System.Guid)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasWireValidator_AppliesToDocument.htm)

#### `public override bool CanAcceptObject(Guid id)`

(Overrides GH_CanvasValidator.CanAcceptObject(Guid).)

**Parameters:**
- `id` (System.Guid)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasWireValidator_CanAcceptObject.htm)

#### `public override bool CanCreateObject(Guid id, PointF pt)`

(Overrides GH_CanvasValidator.CanCreateObject(Guid, PointF).)

**Parameters:**
- `id` (System.Guid)
- `pt` (System.Drawing.PointF)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasWireValidator_CanCreateObject.htm)

#### `public override bool CanCreateWire(IGH_Param source, IGH_Param target)`

(Overrides GH_CanvasValidator.CanCreateWire(IGH_Param, IGH_Param).)

**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param)
- `target` (Grasshopper.Kernel.IGH_Param)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasWireValidator_CanCreateWire.htm)

#### `public override bool CanDeleteObject(IGH_DocumentObject object)`

(Overrides GH_CanvasValidator.CanDeleteObject(IGH_DocumentObject).)

**Parameters:**
- `object` (Grasshopper.Kernel.IGH_DocumentObject)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasWireValidator_CanDeleteObject.htm)

#### `public override bool CanDeleteWire(IGH_Param source, IGH_Param target)`

(Overrides GH_CanvasValidator.CanDeleteWire(IGH_Param, IGH_Param).)

**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param)
- `target` (Grasshopper.Kernel.IGH_Param)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasWireValidator_CanDeleteWire.htm)

#### `public override bool CanDragObject(IGH_DocumentObject object, PointF dragFromPoint)`

(Overrides GH_CanvasValidator.CanDragObject(IGH_DocumentObject, PointF).)

**Parameters:**
- `object` (Grasshopper.Kernel.IGH_DocumentObject)
- `dragFromPoint` (System.Drawing.PointF)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasWireValidator_CanDragObject.htm)

#### `public virtual bool CanNavigateCanvas()`

(Inherited from GH_CanvasValidator.)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasValidator_CanNavigateCanvas.htm)

#### `public override bool CanShowCanvasMenu(PointF pt)`

(Overrides GH_CanvasValidator.CanShowCanvasMenu(PointF).)

**Parameters:**
- `pt` (System.Drawing.PointF)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasWireValidator_CanShowCanvasMenu.htm)

#### `public override bool CanShowComponentSearchBox(PointF pt)`

(Overrides GH_CanvasValidator.CanShowComponentSearchBox(PointF).)

**Parameters:**
- `pt` (System.Drawing.PointF)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasWireValidator_CanShowComponentSearchBox.htm)

#### `public virtual bool CanShowObjectMenu(IGH_DocumentObject object)`

(Inherited from GH_CanvasValidator.)

**Parameters:**
- `object` (Grasshopper.Kernel.IGH_DocumentObject)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasValidator_CanShowObjectMenu.htm)

#### `public override void RemovedFromCanvas(GH_Canvas canvas)`

(Overrides GH_CanvasValidator.RemovedFromCanvas(GH_Canvas).)

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CanvasWireValidator_RemovedFromCanvas.htm)

### Properties
- `Canvas` (GH_Canvas, get) — Gets the canvas this validator is associated with.
- `Document` (GH_Document, get) — Gets the document (if it exists) that is currently loaded in the canvas.
- `WireDelegate` (Action, get/set) — Gets or sets the delegate to be invoked when a wire is created.
- `WireMotionText` (String, get/set) — Gets or sets the text to draw at the wire interior.
- `WireSource` (Guid, get/set) — Gets or sets the wire source filter.
- `WireSourceText` (String, get/set) — Gets or sets the text to draw at the wire source location.
- `WireTarget` (Guid, get/set) — Gets or sets the wire target filter.
- `WireTargetText` (String, get/set) — Gets or sets the text to draw at the wire target location.

## GH_Capsule (class)

Class used to draw standard Grasshopper interface boxes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Capsule.htm)

### Methods
#### `public void AddInputGrip(Point P)`

Add an input grip to this UI Box.

**Parameters:**
- `P` (System.Drawing.Point) — Coordinate of grip.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_AddInputGrip.htm)

#### `public void AddInputGrip(PointF P)`

Add an input grip to this UI Box.

**Parameters:**
- `P` (System.Drawing.PointF) — Coordinate of grip.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_AddInputGrip_1.htm)

#### `public void AddInputGrip(float Y)`

Add an input grip to this UI Box. This is the recommended overload.

**Parameters:**
- `Y` (System.Single) — Y coordinate of grip.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_AddInputGrip_2.htm)

#### `public void AddInputGrip(float X, float Y)`

Add an input grip to this UI Box.

**Parameters:**
- `X` (System.Single) — X coordinate of grip.
- `Y` (System.Single) — Y coordinate of grip.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_AddInputGrip_3.htm)

#### `public void AddOutputGrip(Point P)`

Add an output grip to this UI Box.

**Parameters:**
- `P` (System.Drawing.Point) — Coordinate of grip.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_AddOutputGrip.htm)

#### `public void AddOutputGrip(PointF P)`

Add an output grip to this UI Box.

**Parameters:**
- `P` (System.Drawing.PointF) — Coordinate of grip.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_AddOutputGrip_1.htm)

#### `public void AddOutputGrip(float Y)`

Add an output grip to this UI Box. This is the recommended overload.

**Parameters:**
- `Y` (System.Single) — Y coordinate of grip.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_AddOutputGrip_2.htm)

#### `public void AddOutputGrip(float X, float Y)`

Add an output grip to this UI Box.

**Parameters:**
- `X` (System.Single) — X coordinate of grip.
- `Y` (System.Single) — Y coordinate of grip.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_AddOutputGrip_3.htm)

#### `public bool Contains(PointF pt)`

Test a point for capsule containment.

**Parameters:**
- `pt` (System.Drawing.PointF) — Point to test.

**Returns:** `Boolean` — True if the point is on the interior of the capsule.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_Contains.htm)

#### `public static GH_Capsule CreateCapsule(Rectangle box, GH_Palette palette)`

Create a new blank capsule with default radius and highlight dimensions.

**Parameters:**
- `box` (System.Drawing.Rectangle) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateCapsule.htm)

#### `public static GH_Capsule CreateCapsule(Rectangle box, GH_Palette palette, int radius, int highlight)`

Create a new blank capsule.

**Parameters:**
- `box` (System.Drawing.Rectangle) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.
- `radius` (System.Int32) — Radius of rounded corners. A radius of zero or lower disables corner fillets.
- `highlight` (System.Int32) — Height of the highlight bar. A value of zero or lower disables the highlight.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateCapsule_1.htm)

#### `public static GH_Capsule CreateCapsule(Rectangle box, GH_Palette palette, int[] radii, int highlight)`

Create a new blank capsule.

**Parameters:**
- `box` (System.Drawing.Rectangle) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.
- `radii` (System.Int32[]) — Radii of rounded corners. This array must contain at least 4 integers. Radii are defined clockwise, starting at the upper left corner.
- `highlight` (System.Int32) — Height of the highlight bar. A value of zero or lower disables the highlight.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateCapsule_2.htm)

#### `public static GH_Capsule CreateCapsule(RectangleF box, GH_Palette palette)`

Create a new blank capsule with default radius and highlight dimensions.

**Parameters:**
- `box` (System.Drawing.RectangleF) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateCapsule_3.htm)

#### `public static GH_Capsule CreateCapsule(RectangleF box, GH_Palette palette, int radius, int highlight)`

Create a new blank capsule.

**Parameters:**
- `box` (System.Drawing.RectangleF) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.
- `radius` (System.Int32) — Radius of rounded corners. A radius of zero or lower disables corner fillets.
- `highlight` (System.Int32) — Height of the highlight bar. A value of zero or lower disables the highlight.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateCapsule_4.htm)

#### `public static GH_Capsule CreateCapsule(RectangleF box, GH_Palette palette, int[] radii, int highlight)`

Create a new blank capsule.

**Parameters:**
- `box` (System.Drawing.RectangleF) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.
- `radii` (System.Int32[]) — Radii of rounded corners. This array must contain at least 4 integers. Radii are defined clockwise, starting at the upper left corner.
- `highlight` (System.Int32) — Height of the highlight bar. A value of zero or lower disables the highlight.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateCapsule_5.htm)

#### `public static GH_Capsule CreateTextCapsule(Rectangle box, Rectangle textbox, GH_Palette palette, string text)`

Create a new capsule with content text.

**Parameters:**
- `box` (System.Drawing.Rectangle) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `textbox` (System.Drawing.Rectangle) — Box into which the text will be drawn.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.
- `text` (System.String) — Text to display inside the capsule.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateTextCapsule.htm)

#### `public static GH_Capsule CreateTextCapsule(Rectangle box, Rectangle textbox, GH_Palette palette, string text, Font font)`

Create a new capsule with content text.

**Parameters:**
- `box` (System.Drawing.Rectangle) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `textbox` (System.Drawing.Rectangle) — Box into which the text will be drawn.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.
- `text` (System.String) — Text to display inside the capsule.
- `font` (System.Drawing.Font) — Font to draw text with.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateTextCapsule_1.htm)

#### `public static GH_Capsule CreateTextCapsule(Rectangle box, Rectangle textbox, GH_Palette palette, string text, Font font, GH_Orientation orientation)`

Create a new capsule with content text.

**Parameters:**
- `box` (System.Drawing.Rectangle) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `textbox` (System.Drawing.Rectangle) — Box into which the text will be drawn.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.
- `text` (System.String) — Text to display inside the capsule.
- `font` (System.Drawing.Font) — Font to draw text with.
- `orientation` (Grasshopper.GUI.Canvas.GH_Orientation) — Orientation of text within box.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateTextCapsule_2.htm)

#### `public static GH_Capsule CreateTextCapsule(Rectangle box, Rectangle textbox, GH_Palette palette, string text, Font font, GH_Orientation orientation, int radius, int highlight)`

Create a new capsule with content text.

**Parameters:**
- `box` (System.Drawing.Rectangle) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `textbox` (System.Drawing.Rectangle) — Box into which the text will be drawn.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.
- `text` (System.String) — Text to display inside the capsule.
- `font` (System.Drawing.Font) — Font to draw text with.
- `orientation` (Grasshopper.GUI.Canvas.GH_Orientation) — Orientation of text within box.
- `radius` (System.Int32) — Radius of rounded corners. A radius of zero or lower disables corner fillets.
- `highlight` (System.Int32) — Height of the highlight bar. A value of zero or lower disables the highlight.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateTextCapsule_3.htm)

#### `public static GH_Capsule CreateTextCapsule(Rectangle box, Rectangle textbox, GH_Palette palette, string text, Font font, GH_Orientation orientation, int[] radii, int highlight)`

Create a new capsule with content text.

**Parameters:**
- `box` (System.Drawing.Rectangle) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `textbox` (System.Drawing.Rectangle) — Box into which the text will be drawn.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.
- `text` (System.String) — Text to display inside the capsule.
- `font` (System.Drawing.Font) — Font to draw text with.
- `orientation` (Grasshopper.GUI.Canvas.GH_Orientation) — Orientation of text within box.
- `radii` (System.Int32[]) — Radii of rounded corners. This array must contain at least 4 integers. Radii are defined clockwise, starting at the upper left corner.
- `highlight` (System.Int32) — Height of the highlight bar. A value of zero or lower disables the highlight.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateTextCapsule_4.htm)

#### `public static GH_Capsule CreateTextCapsule(Rectangle box, Rectangle textbox, GH_Palette palette, string text, Font font, int radius, int highlight)`

Create a new capsule with content text.

**Parameters:**
- `box` (System.Drawing.Rectangle) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `textbox` (System.Drawing.Rectangle) — Box into which the text will be drawn.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.
- `text` (System.String) — Text to display inside the capsule.
- `font` (System.Drawing.Font) — Font to draw text with.
- `radius` (System.Int32) — Radius of rounded corners. A radius of zero or lower disables corner fillets.
- `highlight` (System.Int32) — Height of the highlight bar. A value of zero or lower disables the highlight.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateTextCapsule_5.htm)

#### `public static GH_Capsule CreateTextCapsule(Rectangle box, Rectangle textbox, GH_Palette palette, string text, Font font, int[] radii, int highlight)`

Create a new capsule with content text.

**Parameters:**
- `box` (System.Drawing.Rectangle) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `textbox` (System.Drawing.Rectangle) — Box into which the text will be drawn.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.
- `text` (System.String) — Text to display inside the capsule.
- `font` (System.Drawing.Font) — Font to draw text with.
- `radii` (System.Int32[]) — Radii of rounded corners. This array must contain at least 4 integers. Radii are defined clockwise, starting at the upper left corner.
- `highlight` (System.Int32) — Height of the highlight bar. A value of zero or lower disables the highlight.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateTextCapsule_6.htm)

#### `public static GH_Capsule CreateTextCapsule(Rectangle box, Rectangle textbox, GH_Palette palette, string text, int radius, int highlight)`

Create a new capsule with content text.

**Parameters:**
- `box` (System.Drawing.Rectangle) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `textbox` (System.Drawing.Rectangle) — Box into which the text will be drawn.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.
- `text` (System.String) — Text to display inside the capsule.
- `radius` (System.Int32) — Radius of rounded corners. A radius of zero or lower disables corner fillets.
- `highlight` (System.Int32) — Height of the highlight bar. A value of zero or lower disables the highlight.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateTextCapsule_7.htm)

#### `public static GH_Capsule CreateTextCapsule(Rectangle box, Rectangle textbox, GH_Palette palette, string text, int[] radii, int highlight)`

Create a new capsule with content text.

**Parameters:**
- `box` (System.Drawing.Rectangle) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `textbox` (System.Drawing.Rectangle) — Box into which the text will be drawn.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.
- `text` (System.String) — Text to display inside the capsule.
- `radii` (System.Int32[]) — Radii of rounded corners. This array must contain at least 4 integers. Radii are defined clockwise, starting at the upper left corner.
- `highlight` (System.Int32) — Height of the highlight bar. A value of zero or lower disables the highlight.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateTextCapsule_8.htm)

#### `public static GH_Capsule CreateTextCapsule(RectangleF box, RectangleF textbox, GH_Palette palette, string text)`

Create a new capsule with content text.

**Parameters:**
- `box` (System.Drawing.RectangleF) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `textbox` (System.Drawing.RectangleF) — Box into which the text will be drawn.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.
- `text` (System.String) — Text to display inside the capsule.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateTextCapsule_9.htm)

#### `public static GH_Capsule CreateTextCapsule(RectangleF box, RectangleF textbox, GH_Palette palette, string text, Font font)`

Create a new capsule with content text.

**Parameters:**
- `box` (System.Drawing.RectangleF) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `textbox` (System.Drawing.RectangleF) — Box into which the text will be drawn.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.
- `text` (System.String) — Text to display inside the capsule.
- `font` (System.Drawing.Font) — Font to draw text with.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateTextCapsule_10.htm)

#### `public static GH_Capsule CreateTextCapsule(RectangleF box, RectangleF textbox, GH_Palette palette, string text, Font font, GH_Orientation orientation)`

Create a new capsule with content text.

**Parameters:**
- `box` (System.Drawing.RectangleF) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `textbox` (System.Drawing.RectangleF) — Box into which the text will be drawn.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.
- `text` (System.String) — Text to display inside the capsule.
- `font` (System.Drawing.Font) — Font to draw text with.
- `orientation` (Grasshopper.GUI.Canvas.GH_Orientation) — Orientation of text within box.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateTextCapsule_11.htm)

#### `public static GH_Capsule CreateTextCapsule(RectangleF box, RectangleF textbox, GH_Palette palette, string text, Font font, GH_Orientation orientation, int radius, int highlight)`

Create a new capsule with content text.

**Parameters:**
- `box` (System.Drawing.RectangleF) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `textbox` (System.Drawing.RectangleF) — Box into which the text will be drawn.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.
- `text` (System.String) — Text to display inside the capsule.
- `font` (System.Drawing.Font) — Font to draw text with.
- `orientation` (Grasshopper.GUI.Canvas.GH_Orientation) — Orientation of text within box.
- `radius` (System.Int32) — Radius of rounded corners. A radius of zero or lower disables corner fillets.
- `highlight` (System.Int32) — Height of the highlight bar. A value of zero or lower disables the highlight.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateTextCapsule_12.htm)

#### `public static GH_Capsule CreateTextCapsule(RectangleF box, RectangleF textbox, GH_Palette palette, string text, Font font, GH_Orientation orientation, int[] radii, int highlight)`

Create a new capsule with content text.

**Parameters:**
- `box` (System.Drawing.RectangleF) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `textbox` (System.Drawing.RectangleF) — Box into which the text will be drawn.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.
- `text` (System.String) — Text to display inside the capsule.
- `font` (System.Drawing.Font) — Font to draw text with.
- `orientation` (Grasshopper.GUI.Canvas.GH_Orientation) — Orientation of text within box.
- `radii` (System.Int32[]) — Radii of rounded corners. This array must contain at least 4 integers. Radii are defined clockwise, starting at the upper left corner.
- `highlight` (System.Int32) — Height of the highlight bar. A value of zero or lower disables the highlight.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateTextCapsule_13.htm)

#### `public static GH_Capsule CreateTextCapsule(RectangleF box, RectangleF textbox, GH_Palette palette, string text, Font font, int radius, int highlight)`

Create a new capsule with content text.

**Parameters:**
- `box` (System.Drawing.RectangleF) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `textbox` (System.Drawing.RectangleF) — Box into which the text will be drawn.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.
- `text` (System.String) — Text to display inside the capsule.
- `font` (System.Drawing.Font) — Font to draw text with.
- `radius` (System.Int32) — Radius of rounded corners. A radius of zero or lower disables corner fillets.
- `highlight` (System.Int32) — Height of the highlight bar. A value of zero or lower disables the highlight.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateTextCapsule_14.htm)

#### `public static GH_Capsule CreateTextCapsule(RectangleF box, RectangleF textbox, GH_Palette palette, string text, Font font, int[] radii, int highlight)`

Create a new capsule with content text.

**Parameters:**
- `box` (System.Drawing.RectangleF) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `textbox` (System.Drawing.RectangleF) — Box into which the text will be drawn.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.
- `text` (System.String) — Text to display inside the capsule.
- `font` (System.Drawing.Font) — Font to draw text with.
- `radii` (System.Int32[]) — Radii of rounded corners. This array must contain at least 4 integers. Radii are defined clockwise, starting at the upper left corner.
- `highlight` (System.Int32) — Height of the highlight bar. A value of zero or lower disables the highlight.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateTextCapsule_15.htm)

#### `public static GH_Capsule CreateTextCapsule(RectangleF box, RectangleF textbox, GH_Palette palette, string text, int radius, int highlight)`

Create a new capsule with content text.

**Parameters:**
- `box` (System.Drawing.RectangleF) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `textbox` (System.Drawing.RectangleF) — Box into which the text will be drawn.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.
- `text` (System.String) — Text to display inside the capsule.
- `radius` (System.Int32) — Radius of rounded corners. A radius of zero or lower disables corner fillets.
- `highlight` (System.Int32) — Height of the highlight bar. A value of zero or lower disables the highlight.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateTextCapsule_16.htm)

#### `public static GH_Capsule CreateTextCapsule(RectangleF box, RectangleF textbox, GH_Palette palette, string text, int[] radii, int highlight)`

Create a new capsule with content text.

**Parameters:**
- `box` (System.Drawing.RectangleF) — Box for capsule dimensions. The box will be grown until it is at least 3 pixels wide and high.
- `textbox` (System.Drawing.RectangleF) — Box into which the text will be drawn.
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Colour palette to apply.
- `text` (System.String) — Text to display inside the capsule.
- `radii` (System.Int32[]) — Radii of rounded corners. This array must contain at least 4 integers. Radii are defined clockwise, starting at the upper left corner.
- `highlight` (System.Int32) — Height of the highlight bar. A value of zero or lower disables the highlight.

**Returns:** `GH_Capsule` — A new instance of GH_Capsule. Caller is responsible for disposing of the object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_CreateTextCapsule_17.htm)

#### `public void Dispose()`

Releases all resources used by the GH_Capsule

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the GH_Capsule and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_Dispose_1.htm)

#### `public void Render(Graphics G, bool selected, bool locked, bool hidden)`

Render the capsule onto a graphics context.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with. Zoom factor is implied by the G.Transform property.
- `selected` (System.Boolean) — If true, the capsule will be drawn with selected colours.
- `locked` (System.Boolean) — If True, the capcule will be drawn in Locked style.
- `hidden` (System.Boolean) — If True, the capsule will be drawn in Hidden style.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_Render_1.htm)

#### `public void Render(Graphics G, Color colour)`

Have this capsule draw itself onto a Graphics surface with a custom colour override.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with. Zoom factor is implied by the G.Transform property.
- `colour` (System.Drawing.Color) — Base colour for the capsule. Other colours will be derived from the base colour. If you want full control over every colour involved, then use the overload that allows you to supply a custom Style instead.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_Render_2.htm)

#### `public void Render(Graphics G, GH_PaletteStyle style)`

Have this capsule draw itself onto a Graphics surface with a custom Style override.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with. Zoom factor is implied by the G.Transform property.
- `style` (Grasshopper.GUI.Canvas.GH_PaletteStyle) — Style to draw capsule with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_Render.htm)

#### `public void Render(Graphics G, Image icon, bool selected, bool locked, bool hidden)`

Render the capsule onto a graphics context.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with. Zoom factor is implied by the G.Transform property.
- `icon` (System.Drawing.Image) — Icon to render in center of content/bounding box.
- `selected` (System.Boolean) — If true, the capsule will be drawn with selected colours.
- `locked` (System.Boolean) — If True, the capcule will be drawn in Locked style.
- `hidden` (System.Boolean) — If True, the capsule will be drawn in Hidden style.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_Render_4.htm)

#### `public void Render(Graphics G, Image icon, Color colour)`

Have this capsule draw itself onto a Graphics surface with a custom colour override.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with. Zoom factor is implied by the G.Transform property.
- `icon` (System.Drawing.Image) — Icon to render in center of content/bounding box.
- `colour` (System.Drawing.Color) — Base colour for the capsule. Other colours will be derived from the base colour. If you want full control over every colour involved, then use the overload that allows you to supply a custom Style instead.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_Render_5.htm)

#### `public void Render(Graphics G, Image icon, GH_PaletteStyle style)`

Have this capsule draw itself onto a Graphics surface with a custom Style override.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with. Zoom factor is implied by the G.Transform property.
- `icon` (System.Drawing.Image) — Icon to render in center of content/bounding box.
- `style` (Grasshopper.GUI.Canvas.GH_PaletteStyle) — Style to draw capsule with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_Render_3.htm)

#### `public void SetJaggedEdges(bool left, bool right)`

Assign left and right jagged edges.

**Parameters:**
- `left` (System.Boolean)
- `right` (System.Boolean)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Capsule_SetJaggedEdges.htm)

### Properties
- `Box` (Rectangle, get) — Gets the outline box for this Capsule.
- `Box_Content` (Rectangle, get) — Gets the content box for this Capsule. Text and icons are drawn inside this box.
- `Font` (Font, get/set) — Gets or sets the Font to use for text rendering.
- `Highlight` (Int32, get) — Gets the highlight size for this Capsule. Zero or negative means there is no highlight.
- `HighlightShape` (GraphicsPath, get) — Gets the graphics path that describes the highlight bar shape of the capsule.
- `InputGrips` (List<Point>, get) — Gets the list of locally defined input grip locations.
- `JaggedLeft` (Boolean, get) — Gets whether the left edge of this capsule is jagged.
- `JaggedRight` (Boolean, get) — Gets whether the right edge of this capsule is jagged.
- `MaxRadius` (Int32, get) — Gets the largest radius defined for this capsule.
- `OutlineShape` (GraphicsPath, get) — Gets the graphics path that describes the outer edge of the capsule.
- `OutputGrips` (List<Point>, get) — Gets the list of locally defined output grip locations.
- `Palette` (GH_Palette, get/set) — Gets or sets the palette for this Capsule.
- `Radius` (Int32, get) — Gets the radius of the rounded corners of this Capsule.
- `RenderEngine` (GH_CapsuleRenderEngine, get) — Gets the RenderEngine associated with this capsule. You typically don't need to go this far, just use one of the Render() overloads on GH_Capsule directly.
- `Text` (String, get/set) — Gets or sets the text to render in the content box.
- `TextOrientation` (GH_Orientation, get/set) — Gets or sets the orientation of the text within the content box.
- `DefaultHighlight` (Int32, get) — The default size (in pixel units) of the highlight bar.
- `DefaultRadius` (Int32, get) — The default radius (in pixel units) of the rounded corners.

## GH_CapsuleRenderEngine (class)

Provides basic Render methods for capsule display. You typically don't need this class, just use the Render() overloads on GH_Capsule directly.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine.htm)

### Methods
#### `public static GraphicsPath CreateHighlightBar(Rectangle rec, int radius, int size)`

Static (Shared in VB) method for creating Grasshopper Capsule highlight fills.

**Parameters:**
- `rec` (System.Drawing.Rectangle) — Rectangle for entire fill.
- `radius` (System.Int32) — Corner radius.
- `size` (System.Int32) — Highlight fill height.

**Returns:** `GraphicsPath` — Graphics path that describes the highlight fill shape.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_CreateHighlightBar.htm)

#### `public static GraphicsPath CreateHighlightBar(Rectangle rec, int radius, int size, bool jaggedLeft, bool jaggedRight)`

Static (Shared in VB) method for creating Grasshopper Capsule highlight fills.

**Parameters:**
- `rec` (System.Drawing.Rectangle) — Rectangle for entire fill.
- `radius` (System.Int32) — Corner radius.
- `size` (System.Int32) — Highlight fill height.
- `jaggedLeft` (System.Boolean)
- `jaggedRight` (System.Boolean)

**Returns:** `GraphicsPath` — Graphics path that describes the highlight fill shape.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_CreateHighlightBar_1.htm)

#### `public static GraphicsPath CreateHighlightBar(Rectangle rec, int R0, int R1, int size)`

Static (Shared in VB) method for creating Grasshopper Capsule highlight fills.

**Parameters:**
- `rec` (System.Drawing.Rectangle) — Rectangle for entire fill.
- `R0` (System.Int32) — Radius of upper left corner.
- `R1` (System.Int32) — Radius of upper right corner.
- `size` (System.Int32) — Highlight fill height.

**Returns:** `GraphicsPath` — Graphics path that describes the highlight fill shape.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_CreateHighlightBar_2.htm)

#### `public static GraphicsPath CreateHighlightBar(Rectangle rec, int R0, int R1, int size, bool jaggedLeft, bool jaggedRight)`

Static (Shared in VB) method for creating Grasshopper Capsule highlight fills.

**Parameters:**
- `rec` (System.Drawing.Rectangle) — Rectangle for entire fill.
- `R0` (System.Int32) — Radius of upper left corner.
- `R1` (System.Int32) — Radius of upper right corner.
- `size` (System.Int32) — Highlight fill height.
- `jaggedLeft` (System.Boolean)
- `jaggedRight` (System.Boolean)

**Returns:** `GraphicsPath` — Graphics path that describes the highlight fill shape.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_CreateHighlightBar_3.htm)

#### `public static GraphicsPath CreateJaggedRectangle(RectangleF rec, float R0, float R1, float R2, float R3, bool jaggedLeft, bool jaggedRight)`

Static (Shared in VB) method for creating rectangles with rounded corners.

**Parameters:**
- `rec` (System.Drawing.RectangleF) — Rectangle to fillet. Rectangles smaller than 3 pixels wide/high will be grown.
- `R0` (System.Single) — Radius of upper left corner. Zero or negative values omit fillets.
- `R1` (System.Single) — Radius of upper right corner. Zero or negative values omit fillets.
- `R2` (System.Single) — Radius of lower right corner. Zero or negative values omit fillets.
- `R3` (System.Single) — Radius of lower left corner. Zero or negative values omit fillets.
- `jaggedLeft` (System.Boolean) — If true, the left edge of the rectangle will be jagged and R0 and R3 will be ignored.
- `jaggedRight` (System.Boolean) — If true, the right edge of the rectangle will be jagged and R1 and R2 will be ignored.

**Returns:** `GraphicsPath` — Graphics path that describes the rounded rectangle.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_CreateJaggedRectangle.htm)

#### `public static GraphicsPath CreateRoundedRectangle(Rectangle rec, int radius)`

Static (Shared in VB) method for creating rectangles with rounded corners.

**Parameters:**
- `rec` (System.Drawing.Rectangle) — Rectangle to fillet. Rectangles smaller than 3 pixels wide/high will be grown.
- `radius` (System.Int32) — Radius of corner fillets.

**Returns:** `GraphicsPath` — Graphics path that describes the rounded rectangle.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_CreateRoundedRectangle.htm)

#### `public static GraphicsPath CreateRoundedRectangle(Rectangle rec, int R0, int R1, int R2, int R3)`

Static (Shared in VB) method for creating rectangles with rounded corners.

**Parameters:**
- `rec` (System.Drawing.Rectangle) — Rectangle to fillet. Rectangles smaller than 3 pixels wide/high will be grown.
- `R0` (System.Int32) — Radius of upper left corner. Zero or negative values omit fillets.
- `R1` (System.Int32) — Radius of upper right corner. Zero or negative values omit fillets.
- `R2` (System.Int32) — Radius of lower right corner. Zero or negative values omit fillets.
- `R3` (System.Int32) — Radius of lower left corner. Zero or negative values omit fillets.

**Returns:** `GraphicsPath` — Graphics path that describes the rounded rectangle.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_CreateRoundedRectangle_1.htm)

#### `public static GraphicsPath CreateRoundedRectangle(RectangleF rec, float radius)`

Static (Shared in VB) method for creating rectangles with rounded corners.

**Parameters:**
- `rec` (System.Drawing.RectangleF) — Rectangle to fillet. Rectangles smaller than 3 pixels wide/high will be grown.
- `radius` (System.Single) — Radius of corner fillets.

**Returns:** `GraphicsPath` — Graphics path that describes the rounded rectangle.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_CreateRoundedRectangle_2.htm)

#### `public static GraphicsPath CreateRoundedRectangle(RectangleF rec, float R0, float R1, float R2, float R3)`

Static (Shared in VB) method for creating rectangles with rounded corners.

**Parameters:**
- `rec` (System.Drawing.RectangleF) — Rectangle to fillet. Rectangles smaller than 3 pixels wide/high will be grown.
- `R0` (System.Single) — Radius of upper left corner. Zero or negative values omit fillets.
- `R1` (System.Single) — Radius of upper right corner. Zero or negative values omit fillets.
- `R2` (System.Single) — Radius of lower right corner. Zero or negative values omit fillets.
- `R3` (System.Single) — Radius of lower left corner. Zero or negative values omit fillets.

**Returns:** `GraphicsPath` — Graphics path that describes the rounded rectangle.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_CreateRoundedRectangle_3.htm)

#### `public static GH_Palette GetImpliedPalette(IGH_ActiveObject obj)`

Static (Shared in VB) method for solving which palette is the default given a specific object's properties. Default palettes are either GH_Palette.Normal, GH_Palette.Warning or GH_Palette.Error depending on the objects' runtime message level.

**Parameters:**
- `obj` (Grasshopper.Kernel.IGH_ActiveObject) — Object to evaluate.

**Returns:** `GH_Palette` — The default palette.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_GetImpliedPalette.htm)

#### `public static GH_PaletteStyle GetImpliedStyle(GH_Palette palette, bool selected, bool locked, bool hidden)`

Static (Shared in VB) method for solving which palette style is implied by the properties.

**Parameters:**
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Palette hint.
- `selected` (System.Boolean) — Specifies whether the selected flavour of a palette needs to be returned.
- `locked` (System.Boolean) — If True, the returned palette style is always Locked.
- `hidden` (System.Boolean) — If True, the returned palette style is always Hidden.

**Returns:** `GH_PaletteStyle` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_GetImpliedStyle_1.htm)

#### `public static GH_PaletteStyle GetImpliedStyle(GH_Palette palette, IGH_Attributes attributes)`

Static (Shared in VB) method for solving which palette style is implied by the properties.

**Parameters:**
- `palette` (Grasshopper.GUI.Canvas.GH_Palette) — Palette hint.
- `attributes` (Grasshopper.Kernel.IGH_Attributes) — Attributes to parse for style details.

**Returns:** `GH_PaletteStyle` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_GetImpliedStyle.htm)

#### `public void RenderAlphaFill(Graphics G)`

Render a standard transparancy background hatch. This is only needed when the capsule background colour is not fully opaque. This is usually the second stage in a capsule rendering, right after the Grips are drawn.

**Parameters:**
- `G` (System.Drawing.Graphics)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_RenderAlphaFill.htm)

#### `public void RenderBackground(Graphics G, float zoom, GH_PaletteStyle style)`

Render a default capsule background fill. This is usually the third stage in a capsule rendering.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with.
- `zoom` (System.Single) — Zoom of graphics object.
- `style` (Grasshopper.GUI.Canvas.GH_PaletteStyle) — Final style to draw with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_RenderBackground.htm)

#### `public void RenderBackground_Alternative(Graphics G, Color colourOverride, bool drawAlphaGrid)`

Render a custom capsule background fill. This is usually the third stage in a capsule rendering.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with.
- `colourOverride` (System.Drawing.Color) — Custom background colour.
- `drawAlphaGrid` (System.Boolean) — If true, and the background colour is not opaque, an alpha grid will be drawn as well.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_RenderBackground_Alternative.htm)

#### `public void RenderBoundaryDots(Graphics G, int count, GH_PaletteStyle style)`

Render a collection of boundary dots along the top edge of the capsule.

**Parameters:**
- `G` (System.Drawing.Graphics)
- `count` (System.Int32) — Number of dots to render.
- `style` (Grasshopper.GUI.Canvas.GH_PaletteStyle)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_RenderBoundaryDots.htm)

#### `public void RenderGrips(Graphics G)`

Render all the grips associates with the capsule. Grips are not drawn if the zoom level is less than 50%. This is usually the first stage in a Capsule rendering.

**Parameters:**
- `G` (System.Drawing.Graphics)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_RenderGrips.htm)

#### `public void RenderGrips_Alternative(Graphics G)`

Render all the grips associates with the capsule as semi-circles. This is needed if the capsule background is not fully opaque. Grips are not drawn if the zoom level is less than 50%. This is usually the first stage in a Capsule rendering.

**Parameters:**
- `G` (System.Drawing.Graphics)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_RenderGrips_Alternative.htm)

#### `public void RenderHighlight(Graphics G)`

Render a highlight bar. This is highly zoom-dependant. Highlight rendering is usually the fourth stage in a Capsule Rendering, after the background but prior to the edges.

**Parameters:**
- `G` (System.Drawing.Graphics)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_RenderHighlight.htm)

#### `public void RenderIcon(Graphics G, Image icon, int offsetX = 0, int offsetY = 0)`

Render an image icon centered into the box. If the content box has been defined the icon will be centered there, otherwise the outline box will be used.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics surface to draw onto.
- `icon` (System.Drawing.Image) — Icon to draw.
- `offsetX` (System.Int32) — Additional x-offset (in original pixel units).
- `offsetY` (System.Int32) — Additional y-offset (in original pixel units).

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_RenderIcon_1.htm)

#### `public void RenderIcon(Graphics G, Image icon, RectangleF box, int offsetX = 0, int offsetY = 0)`

Render an image icon centered into the box. If the content box has been defined the icon will be centered there, otherwise the outline box will be used.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics surface to draw onto.
- `icon` (System.Drawing.Image) — Icon to draw.
- `box` (System.Drawing.RectangleF) — Box override.
- `offsetX` (System.Int32) — Additional x-offset (in original pixel units).
- `offsetY` (System.Int32) — Additional y-offset (in original pixel units).

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_RenderIcon.htm)

#### `public static void RenderInputGrip(Graphics G, float zoom, PointF loc, bool full)`

Static (Shared in VB) method to render a default input grip.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics objects to draw with.
- `zoom` (System.Single) — Zoom level of graphics surface.
- `loc` (System.Drawing.PointF) — Location of grip.
- `full` (System.Boolean) — If true, a complete circular grip will be draw. If false, a semi-circular grip will be drawn.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_RenderInputGrip.htm)

#### `public Rectangle RenderMessage(Graphics G, string message, GH_PaletteStyle style)`

Render a text message underneath the capsule.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with.
- `message` (System.String) — Message to draw.
- `style` (Grasshopper.GUI.Canvas.GH_PaletteStyle) — Style to use.

**Returns:** `Rectangle` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_RenderMessage.htm)

#### `public void RenderOutlines(Graphics G, float zoom, GH_PaletteStyle style)`

Render default capsule outlines. If the zoom factor is less than 50% the inner outline is not drawn. This is usually the fifth stage in a Capsule rendering.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with.
- `zoom` (System.Single) — Zoom level of graphics object.
- `style` (Grasshopper.GUI.Canvas.GH_PaletteStyle) — Palette style that defines edge properties.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_RenderOutlines.htm)

#### `public static void RenderOutputGrip(Graphics G, float zoom, PointF loc, bool full)`

Static (Shared in VB) method to render a default output grip.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics objects to draw with.
- `zoom` (System.Single) — Zoom level of graphics surface.
- `loc` (System.Drawing.PointF) — Location of grip.
- `full` (System.Boolean) — If true, a complete circular grip will be draw. If false, a semi-circular grip will be drawn.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_RenderOutputGrip.htm)

#### `public void RenderText(Graphics G, Color colour)`

Render the capsule text. This is usually the sixth and final stage in a capsule rendering.

**Parameters:**
- `G` (System.Drawing.Graphics) — Graphics object to draw with.
- `colour` (System.Drawing.Color) — Colour of text.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_CapsuleRenderEngine_RenderText.htm)

## GH_DocDiagramPainter (class)

This class paints diagrammatic images of a collection of objects.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_DocDiagramPainter.htm)

### Constructors
- `public GH_DocDiagramPainter()` — Initializes a new instance of the GH_DocDiagramPainter class

### Methods
#### `public Point MapPoint(Point pt)`



**Parameters:**
- `pt` (System.Drawing.Point)

**Returns:** `Point` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_DocDiagramPainter_MapPoint.htm)

#### `public PointF MapPoint(PointF pt)`



**Parameters:**
- `pt` (System.Drawing.PointF)

**Returns:** `PointF` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_DocDiagramPainter_MapPoint_1.htm)

#### `public Rectangle MapRectangle(Rectangle rec)`



**Parameters:**
- `rec` (System.Drawing.Rectangle)

**Returns:** `Rectangle` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_DocDiagramPainter_MapRectangle.htm)

#### `public int MapX(int x)`



**Parameters:**
- `x` (System.Int32)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_DocDiagramPainter_MapX.htm)

#### `public float MapX(float x)`



**Parameters:**
- `x` (System.Single)

**Returns:** `Single` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_DocDiagramPainter_MapX_1.htm)

#### `public int MapY(int y)`



**Parameters:**
- `y` (System.Int32)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_DocDiagramPainter_MapY.htm)

#### `public float MapY(float y)`



**Parameters:**
- `y` (System.Single)

**Returns:** `Single` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_DocDiagramPainter_MapY_1.htm)

#### `public bool PaintDiagram(IEnumerable<IGH_DocumentObject> objs, int approx_size, int inflate)`

Paint a diagram of a document.

**Parameters:**
- `objs` (System.Collections.Generic.IEnumerable<IGH_DocumentObject>) — Objects to paint.
- `approx_size` (System.Int32) — Approximate size along each side.
- `inflate` (System.Int32) — Padding around the objects.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_DocDiagramPainter_PaintDiagram.htm)

#### `public bool PaintDiagram(IEnumerable<IGH_DocumentObject> objs, int mapWidth, int mapHeight, int inflate)`

Paint a diagram of a document.

**Parameters:**
- `objs` (System.Collections.Generic.IEnumerable<IGH_DocumentObject>) — Objects to paint.
- `mapWidth` (System.Int32) — Width (in pixels) of diagram.
- `mapHeight` (System.Int32) — Height (in pixels) of diagram.
- `inflate` (System.Int32) — Padding around the objects.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_DocDiagramPainter_PaintDiagram_1.htm)

#### `public Point UnmapPoint(Point pt)`



**Parameters:**
- `pt` (System.Drawing.Point)

**Returns:** `Point` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_DocDiagramPainter_UnmapPoint.htm)

#### `public Rectangle UnmapRectangle(Rectangle rec)`



**Parameters:**
- `rec` (System.Drawing.Rectangle)

**Returns:** `Rectangle` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_DocDiagramPainter_UnmapRectangle.htm)

#### `public int UnmapX(int x)`



**Parameters:**
- `x` (System.Int32)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_DocDiagramPainter_UnmapX.htm)

#### `public int UnmapY(int y)`



**Parameters:**
- `y` (System.Int32)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_DocDiagramPainter_UnmapY.htm)

### Properties
- `BoundingBox` (RectangleF, get) — Gets the 2D domain of the diagram in canvas coordinates.
- `DrawingBox` (Rectangle, get) — Gets the 2D domain of the diagram in image coordinates.
- `IgnoreSelectedStates` (Boolean, get/set) — Gets or sets the Ignore Selected State flag. When set to True, selected objects are not drawn in a different colour.
- `Image` (Bitmap, get) — Gets the diagram image (only available after a call to PaintDiagram().
- `Size` (Size, get) — Gets the width and height (in pixels) of this diagram image.
- `Zoom` (Single, get) — Gets the zoom factor for this diagram.

## GH_DragDropFileAction (enum)

Enumerates the possible actions for drag+drop loading ghx files.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_DragDropFileAction.htm)

### Values
- `None` = `0` — Indicates no action is defined.
- `Open` = `1` — Open the file(s) as a new document.
- `Examine` = `2` — Examine the file(s) using FileViewer.exe
- `Insert` = `3` — Insert the file(s) into the current document.
- `Group` = `4` — Insert the file(s) into the current document as separate groups.
- `Cluster` = `5` — Insert the file(s) into the current document as clusters.

## GH_NamedView (class)

Named views are used both to store named views but also to allow smooth programmatic canvas navigation.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_NamedView.htm)

### Constructors
- `public GH_NamedView()` — Create a default named view.
- `public GH_NamedView(Rectangle view_box, RectangleF target_box)` — Create a new view useful for box-fit transitions.
- `public GH_NamedView(GH_Viewport view, Point point, PointF target)` — Create a new view useful for point match transitions.
- `public GH_NamedView(GH_Viewport view, Rectangle region, PointF target)` — Create a new view useful for point inclusion transitions.
- `public GH_NamedView(string view_name, PointF view_point, float view_zoom = 1f, GH_NamedViewType view_type = GH_NamedViewType.target)` — Create a new named view.

### Methods
#### `public void LoadFromViewport(GH_Viewport vp, GH_NamedViewType view_type)`

Create a named view from a viewport.

**Parameters:**
- `vp` (Grasshopper.GUI.Canvas.GH_Viewport) — Viewport to mimic.
- `view_type` (Grasshopper.GUI.Canvas.GH_NamedViewType) — Type of named view to create.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_NamedView_LoadFromViewport.htm)

#### `public bool Read(GH_IReader reader)`



**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_NamedView_Read.htm)

#### `public void SetToViewport(GH_Canvas cv)`

Set the named view to the canvas viewport and redraw the canvas.

**Parameters:**
- `cv` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas to set

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_NamedView_SetToViewport.htm)

#### `public void SetToViewport(GH_Canvas cv, int length)`

Animates a view transition.

**Parameters:**
- `cv` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas to apply the transition to.
- `length` (System.Int32) — Length (in milliseconds) of the transition.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_NamedView_SetToViewport_1.htm)

#### `public void SetToViewport(GH_Viewport vp)`

Set the named view to a viewport.

**Parameters:**
- `vp` (Grasshopper.GUI.Canvas.GH_Viewport) — Viewport to set.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_NamedView_SetToViewport_2.htm)

#### `public bool Write(GH_IWriter writer)`



**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_NamedView_Write.htm)

### Properties
- `Name` (String, get/set) — Gets or sets the name of the view.
- `Point` (PointF, get/set) — Gets or sets the anchor point of the named view.
- `Type` (GH_NamedViewType, get/set) — Gets or sets a value indicating whether the anchor point represents the target or the center.
- `Zoom` (Single, get/set) — Gets or sets the zoom factor of the named view.

### Events
#### `SmoothFrame` (`Grasshopper.GUI.Canvas.GH_NamedView.SmoothFrameEventHandler`)

**Signature:** `public event GH_NamedView.SmoothFrameEventHandler SmoothFrame`

This event is raised during smooth animations.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/E_Grasshopper_GUI_Canvas_GH_NamedView_SmoothFrame.htm)

## GH_NamedView.SmoothFrameEventHandler (delegate)

(No description provided in vendor docs for Grasshopper.GUI.Canvas.GH_NamedView.SmoothFrameEventHandler.)

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_NamedView_SmoothFrameEventHandler.htm)

## GH_NamedViewType (enum)

Types of named view anchor points.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_NamedViewType.htm)

### Values
- `target` = `0` — Anchor represents the location of the target (the canvas origin).
- `center` = `1` — Anchor represents the point that is supposed to end up in the middle of the canvas.

## GH_ObjectResponse (enum)

Lists the possible responses an object can give when confronted with a UI event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_ObjectResponse.htm)

### Values
- `Ignore` = `0` — Event was ignored.
- `Capture` = `1` — Event caused the object to become active.
- `Release` = `2` — Object was active, but event caused object to become inactive.
- `Handled` = `3` — Something happened, but no capture is needed.

## GH_Orientation (enum)

Enumerates possible alignments.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Orientation.htm)

### Values
- `horizontal_center` = `0` — Text is drawn horizontally, centered in the middle of the context.
- `horizontal_near` = `1` — Text is drawn horizontally, aligned near to the context edge.
- `horizontal_far` = `2` — Text is drawn horizontally, aligned far from the context edge.
- `vertical_center` = `10` — Text is drawn vertically, centered in the middle of the context.
- `vertical_near` = `11` — Text is drawn vertically, aligned near to the context edge.
- `vertical_far` = `12` — Text is drawn vertically, aligned far from the context edge.

## GH_Painter (class)

This class performs most of the drawing operations required for Grasshopper Canvas controls.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Painter.htm)

### Constructors
- `public GH_Painter(GH_Canvas owner)` — Constructor.

### Methods
#### `public static GraphicsPath ConnectionPath(PointF pointA, PointF pointB, GH_WireDirection directionA, GH_WireDirection directionB)`



**Parameters:**
- `pointA` (System.Drawing.PointF)
- `pointB` (System.Drawing.PointF)
- `directionA` (Grasshopper.GUI.Canvas.GH_WireDirection)
- `directionB` (Grasshopper.GUI.Canvas.GH_WireDirection)

**Returns:** `GraphicsPath` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Painter_ConnectionPath.htm)

#### `public static BezierF ConnectionPathBezier(PointF source, PointF target)`



**Parameters:**
- `source` (System.Drawing.PointF)
- `target` (System.Drawing.PointF)

**Returns:** `BezierF` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Painter_ConnectionPathBezier.htm)

#### `public bool ConnectionVisible(PointF a, PointF b)`

Test whether a wire ought to be drawn at all.

**Parameters:**
- `a` (System.Drawing.PointF) — Start point of wire.
- `b` (System.Drawing.PointF) — End point of wire.

**Returns:** `Boolean` — True if the wire is visible and ought to be drawn.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Painter_ConnectionVisible.htm)

#### `public static GH_WireType DetermineWireType(IGH_Structure target)`

Utility method for determining wire display types.

**Parameters:**
- `target` (Grasshopper.Kernel.Data.IGH_Structure) — Pointer to transfer data.

**Returns:** `GH_WireType` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Painter_DetermineWireType.htm)

#### `public void DrawBackground(GH_Viewport viewport)`



**Parameters:**
- `viewport` (Grasshopper.GUI.Canvas.GH_Viewport)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Painter_DrawBackground.htm)

#### `public void DrawConnection(PointF pointA, PointF pointB, GH_WireDirection directionA, GH_WireDirection directionB, bool selectedA, bool selectedB, GH_WireType type)`



**Parameters:**
- `pointA` (System.Drawing.PointF)
- `pointB` (System.Drawing.PointF)
- `directionA` (Grasshopper.GUI.Canvas.GH_WireDirection)
- `directionB` (Grasshopper.GUI.Canvas.GH_WireDirection)
- `selectedA` (System.Boolean)
- `selectedB` (System.Boolean)
- `type` (Grasshopper.GUI.Canvas.GH_WireType)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Painter_DrawConnection.htm)

#### `public void DrawMiddleGround(GH_Document doc, GH_CanvasChannel channel)`



**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)
- `channel` (Grasshopper.GUI.Canvas.GH_CanvasChannel)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Painter_DrawMiddleGround.htm)

#### `public void DrawNULLBuffer()`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Painter_DrawNULLBuffer.htm)

#### `public void DrawNoDocumentMessage()`



[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Painter_DrawNoDocumentMessage.htm)

#### `public void DrawPivots(GH_Document doc)`



**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Painter_DrawPivots.htm)

#### `public void DrawRecipientLinks(GH_Document doc)`



**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Painter_DrawRecipientLinks.htm)

#### `protected void DrawRecipientLinks(IGH_Param param)`



**Parameters:**
- `param` (Grasshopper.Kernel.IGH_Param)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Painter_DrawRecipientLinks_1.htm)

#### `public void DrawStateFlags(GH_Document doc)`



**Parameters:**
- `doc` (Grasshopper.Kernel.GH_Document)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Painter_DrawStateFlags.htm)

#### `public Brush GenerateWirePen_Fill(PointF a, PointF b, bool asel, bool bsel, bool empty)`



**Parameters:**
- `a` (System.Drawing.PointF)
- `b` (System.Drawing.PointF)
- `asel` (System.Boolean)
- `bsel` (System.Boolean)
- `empty` (System.Boolean)

**Returns:** `Brush` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Painter_GenerateWirePen_Fill.htm)

## GH_Palette (enum)

Lists all possible Capsule Palette styles in Grasshopper.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Palette.htm)

### Values
- `Normal` = `0`
- `Hidden` = `1`
- `Locked` = `2`
- `Warning` = `3`
- `Error` = `4`
- `White` = `5`
- `Grey` = `6`
- `Black` = `7`
- `Brown` = `8`
- `Pink` = `9`
- `Blue` = `10`
- `Transparent` = `11`

## GH_PaletteStyle (class)

Contains all the colours for a single capsule palette.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_PaletteStyle.htm)

### Constructors
- `public GH_PaletteStyle(Color fill)` — Construct a new GH_Box_Palette_Colours instance with a single base colour.
- `public GH_PaletteStyle(Color fill, Color edge)` — Construct a new GH_Box_Palette_Colours instance.
- `public GH_PaletteStyle(Color fill, Color edge, Color text)` — Construct a new GH_Box_Palette_Colours instance.

### Methods
#### `public virtual Brush CreateBrush(RectangleF rec, float zoom)`

Create a fill brush. At low zoom levels the brush will be solid colour, at medium to high zoom levels the brush will have a gradient.

**Parameters:**
- `rec` (System.Drawing.RectangleF) — Target rectangle of fill brush.
- `zoom` (System.Single) — Zoom level of target graphics surface.

**Returns:** `Brush` — A brush object that can be used to fill a custom area.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_PaletteStyle_CreateBrush.htm)

### Properties
- `Edge` (Color, get/set) — Gets or sets the edge of this style.
- `Fill` (Color, get/set) — Gets or sets the fill of this style.
- `Text` (Color, get/set) — Gets or sets the text of this style.

## GH_ResizeBorder (class)

Inheritance HierarchySystem.Object Grasshopper.GUI.Canvas.GH_Border Grasshopper.GUI.Canvas.GH_ResizeBorder Namespace: Grasshopper.GUI.Canvas Assembly: Grasshopper (in Grasshopper.dll)SyntaxC#VBCopypublic class GH_ResizeBorder : GH_BorderPublic Class GH_ResizeBorder Inherits GH_BorderThe GH_ResizeBorder type exposes the following members.Constructors NameDescriptionGH_ResizeBorder(GH_Border) Create a new instance of the GH_ResizeBorder class. GH_ResizeBorder(RectangleF, GH_BorderTopology) Create a new instance of the GH_ResizeBorder class. TopProperties NameDescriptionAnchor Gets or sets the anchor point for sizing operations. This point is determined automatically when you call Setup(), however you can alter it afterwards if you want. Region Gets the shape of the border. (Inherited from GH_Border.)Size_Cursor Gets the cursor associated with this border. (Inherited from GH_Border.)Topology Gets the type of border. (Inherited from GH_Border.)TopMethods NameDescriptionContains Test a point for border inclusion. (Inherited from GH_Border.)Setup(IGH_Attributes, PointF) Set up a new sizing operation. Setup(IGH_Attributes, PointF, SizeF) Set up a new sizing operation. Setup(RectangleF, PointF, PointF) Set up a new sizing operation. Setup(IGH_Attributes, PointF, SizeF, SizeF) Set up a new sizing operation. Setup(RectangleF, PointF, PointF, SizeF) Set up a new sizing operation. Setup(RectangleF, PointF, PointF, SizeF, SizeF) Set up a new sizing operation. Solve Solve a resizing step. TopSee AlsoReferenceGrasshopper.GUI.Canvas Namespace

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_ResizeBorder.htm)

### Constructors
- `public GH_ResizeBorder(GH_Border borders)` — Create a new instance of the GH_ResizeBorder class.
- `public GH_ResizeBorder(RectangleF region, GH_BorderTopology topology)` — Create a new instance of the GH_ResizeBorder class.

### Methods
#### `public bool Contains(PointF pt)`

Test a point for border inclusion.

**Parameters:**
- `pt` (System.Drawing.PointF) — Point to test.

**Returns:** `Boolean` — True if pt is inside or on this border.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Border_Contains.htm)

#### `public void Setup(IGH_Attributes att, PointF MouseCursor)`

Set up a new sizing operation.

**Parameters:**
- `att` (Grasshopper.Kernel.IGH_Attributes) — Attributes of shape to resize.
- `MouseCursor` (System.Drawing.PointF) — Location of cursor in shape space coordinates.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_ResizeBorder_Setup.htm)

#### `public void Setup(IGH_Attributes att, PointF MouseCursor, SizeF MinSize)`

Set up a new sizing operation.

**Parameters:**
- `att` (Grasshopper.Kernel.IGH_Attributes) — Attributes of shape to resize.
- `MouseCursor` (System.Drawing.PointF) — Location of cursor in shape space coordinates.
- `MinSize` (System.Drawing.SizeF) — Minimum allowed size for shape.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_ResizeBorder_Setup_1.htm)

#### `public void Setup(IGH_Attributes att, PointF MouseCursor, SizeF MinSize, SizeF MaxSize)`

Set up a new sizing operation.

**Parameters:**
- `att` (Grasshopper.Kernel.IGH_Attributes) — Attributes of shape to resize.
- `MouseCursor` (System.Drawing.PointF) — Location of cursor in shape space coordinates.
- `MinSize` (System.Drawing.SizeF) — Minimum allowed size for shape.
- `MaxSize` (System.Drawing.SizeF) — Maximum allowed size for shape.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_ResizeBorder_Setup_2.htm)

#### `public void Setup(RectangleF ShapeRegion, PointF ShapePivot, PointF MouseCursor)`

Set up a new sizing operation.

**Parameters:**
- `ShapeRegion` (System.Drawing.RectangleF) — Shape of the object to resize.
- `ShapePivot` (System.Drawing.PointF) — Pivot of the object to resize.
- `MouseCursor` (System.Drawing.PointF) — Location of cursor in shape space coordinates.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_ResizeBorder_Setup_3.htm)

#### `public void Setup(RectangleF ShapeRegion, PointF ShapePivot, PointF MouseCursor, SizeF MinSize)`

Set up a new sizing operation.

**Parameters:**
- `ShapeRegion` (System.Drawing.RectangleF) — Shape of the object to resize.
- `ShapePivot` (System.Drawing.PointF) — Pivot of the object to resize.
- `MouseCursor` (System.Drawing.PointF) — Location of cursor in shape space coordinates.
- `MinSize` (System.Drawing.SizeF) — Minimum allowed size for shape.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_ResizeBorder_Setup_4.htm)

#### `public void Setup(RectangleF ShapeRegion, PointF ShapePivot, PointF MouseCursor, SizeF MinSize, SizeF MaxSize)`

Set up a new sizing operation.

**Parameters:**
- `ShapeRegion` (System.Drawing.RectangleF) — Shape of the object to resize.
- `ShapePivot` (System.Drawing.PointF) — Pivot of the object to resize.
- `MouseCursor` (System.Drawing.PointF) — Location of cursor in shape space coordinates.
- `MinSize` (System.Drawing.SizeF) — Minimum allowed size for shape.
- `MaxSize` (System.Drawing.SizeF) — Maximum allowed size for shape.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_ResizeBorder_Setup_5.htm)

#### `public void Solve(PointF nCursor, out RectangleF new_shape, out PointF new_pivot)`

Solve a resizing step.

**Parameters:**
- `nCursor` (System.Drawing.PointF) — The new cursor location.
- `new_shape` (System.Drawing.RectangleF) — The resized shape bounding box.
- `new_pivot` (System.Drawing.PointF) — The adjusted shape pivot.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_ResizeBorder_Solve.htm)

### Properties
- `Anchor` (PointF, get/set) — Gets or sets the anchor point for sizing operations. This point is determined automatically when you call Setup(), however you can alter it afterwards if you want.
- `Region` (RectangleF, get) — Gets the shape of the border.
- `Size_Cursor` (Cursor, get) — Gets the cursor associated with this border.
- `Topology` (GH_BorderTopology, get) — Gets the type of border.

## GH_Skin (class)

Provides static access to typical Palletes and other GUI colours.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Skin.htm)

### Methods
#### `public static void LoadSkin()`

Instantiate all palette and gui defaults. This function reads the colour values out of grasshopper_gui.xml settings database if it exists.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Skin_LoadSkin.htm)

#### `public static void SaveSkin()`

Store all palette and gui defaults. This function writes the colour values out to grasshopper_gui.xml settings database.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Skin_SaveSkin.htm)

### Properties
- `canvas_back` (Color, get) — Specifies canvas background colour.
- `canvas_edge` (Color, get) — Specifies canvas edge colour.
- `canvas_grid` (Color, get) — Specifies canvas grid colour.
- `canvas_grid_col` (Int32, get) — Specifies the interval of the canvas grid columns.
- `canvas_grid_row` (Int32, get) — Specifies the interval of the canvas grid rows.
- `canvas_mono` (Boolean, get) — Canvas monochromatic flag. If True, the canvas background is a solid colour.
- `canvas_mono_color` (Color, get) — If canvas_mono is set to true, this colour specified the solid background fill for the canvas.
- `canvas_shade` (Color, get) — Specifies the colour of the canvas drop shadow.
- `canvas_shade_size` (Int32, get) — Specifies the size of the canvas drop shadow.
- `group_back` (Color, get) — Specifies the default fill colour for Group objects.
- `palette_black_selected` (GH_PaletteStyle, get) — Black palette, selected.
- `palette_black_standard` (GH_PaletteStyle, get) — Black palette, unselected.
- `palette_blue_selected` (GH_PaletteStyle, get) — Blue palette, selected.
- `palette_blue_standard` (GH_PaletteStyle, get) — Blue palette, unselected.
- `palette_brown_selected` (GH_PaletteStyle, get) — Brown palette, selected.
- `palette_brown_standard` (GH_PaletteStyle, get) — Brown palette, unselected.
- `palette_error_selected` (GH_PaletteStyle, get) — Errors, selected palette. Default background for parameters and components that carry errors.
- `palette_error_standard` (GH_PaletteStyle, get) — Errors, unselected palette. Default background for parameters and components that carry errors.
- `palette_grey_selected` (GH_PaletteStyle, get) — Grey palette, selected.
- `palette_grey_standard` (GH_PaletteStyle, get) — Grey palette, unselected.
- `palette_hidden_selected` (GH_PaletteStyle, get) — Hidden, selected palette. Default background for PanelHidden (preview=off) parameters and components.
- `palette_hidden_standard` (GH_PaletteStyle, get) — Hidden, unselected palette. Default background for PanelHidden (preview=off) parameters and components.
- `palette_locked_selected` (GH_PaletteStyle, get) — Locked, selected palette. Default background for locked parameters and components.
- `palette_locked_standard` (GH_PaletteStyle, get) — Locked, unselected palette. Default background for locked parameters and components.
- `palette_normal_selected` (GH_PaletteStyle, get) — Normal, selected palette. Default background for parameters and components.
- `palette_normal_standard` (GH_PaletteStyle, get) — Normal, unselected palette. Default background for parameters and components.
- `palette_pink_selected` (GH_PaletteStyle, get) — Pink palette, selected.
- `palette_pink_standard` (GH_PaletteStyle, get) — Pink palette, unselected.
- `palette_trans_selected` (GH_PaletteStyle, get) — Transparent palette, selected.
- `palette_trans_standard` (GH_PaletteStyle, get) — Transparent palette, unselected.
- `palette_warning_selected` (GH_PaletteStyle, get) — Warnings, selected palette. Default background for parameters and components that carry warnings.
- `palette_warning_standard` (GH_PaletteStyle, get) — Warnings, unselected palette. Default background for parameters and components that carry warnings.
- `palette_white_selected` (GH_PaletteStyle, get) — White palette, selected.
- `palette_white_standard` (GH_PaletteStyle, get) — White palette, unselected.
- `panel_back` (Color, get) — Specifies the default fill colour for Panel objects.
- `wire_default` (Color, get) — Specifies the default wire colour.
- `wire_empty` (Color, get) — Specifies the colour for empty (null) wires.
- `wire_selected_a` (Color, get) — Specifies the colour for wires at the selected end.
- `wire_selected_b` (Color, get) — Specifies the colour for wires at the unselected end.
- `zui_edge` (Color, get) — Specifies the default edge colour for ZUI elements.
- `zui_edge_highlight` (Color, get) — Specifies the default edge colour for highlighted ZUI elements.
- `zui_fill` (Color, get) — Specifies the default fill colour for ZUI elements.
- `zui_fill_highlight` (Color, get) — Specifies the default fill colour for highlighted ZUI elements.

## GH_Viewport (class)

Provides functionality for panning and zooming in a GH_Canvas environment.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Viewport.htm)

### Constructors
- `public GH_Viewport()` — Initializes a new instance of the GH_Viewport class
- `public GH_Viewport(GH_Viewport viewport)` — Initializes a new instance of the GH_Viewport class
- `public GH_Viewport(Point target)` — Initializes a new instance of the GH_Viewport class
- `public GH_Viewport(Point target, float zoom)` — Initializes a new instance of the GH_Viewport class

### Methods
#### `public void ApplyProjection(Graphics G)`

Apply the current display transformation to a Graphics object.

**Parameters:**
- `G` (System.Drawing.Graphics)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_ApplyProjection.htm)

#### `public void ComputeProjection()`

Forces a recomputation of all cached data.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_ComputeProjection.htm)

#### `public bool DollyZoom(GH_CanvasMouseEvent e)`

Advanced interface function for mouse 'dolly' zooming.

**Parameters:**
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent) — Event arguments.

**Returns:** `Boolean` — True if a change to the viewport was made, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_DollyZoom.htm)

#### `public GH_Viewport Duplicate()`



**Returns:** `GH_Viewport` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_Duplicate.htm)

#### `public void Focus(IGH_Attributes attribute)`

Look at a specific object.

**Parameters:**
- `attribute` (Grasshopper.Kernel.IGH_Attributes) — Object attribute to focus on.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_Focus.htm)

#### `public void Focus(List<IGH_Attributes> attributes)`

Look at a set of specific objects.

**Parameters:**
- `attributes` (System.Collections.Generic.List<IGH_Attributes>) — Object attributes to focus on.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_Focus_1.htm)

#### `public void Focus(Point pt)`

Look at a specific point.

**Parameters:**
- `pt` (System.Drawing.Point) — Point to focus on.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_Focus_2.htm)

#### `public void Focus(PointF pt)`

Look at a specific point.

**Parameters:**
- `pt` (System.Drawing.PointF) — Point to focus on.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_Focus_3.htm)

#### `public bool IsVisible(ref PointF pt, float margin = 0f)`

Test visibility of a point.

**Parameters:**
- `pt` (System.Drawing.PointF) — Point in canvas coordinates.
- `margin` (System.Single) — Fuzzy area around point to test.

**Returns:** `Boolean` — True if the point is visible in the control upon projection.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_IsVisible.htm)

#### `public bool IsVisible(ref RectangleF rec, float margin = 0f)`

Test visibility of a rectangle.

**Parameters:**
- `rec` (System.Drawing.RectangleF) — Rectangle in canvas coordinates.
- `margin` (System.Single) — Fuzzy area around rectangle to test.

**Returns:** `Boolean` — True if the rectangle is visible in the control upon projection.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_IsVisible_1.htm)

#### `public float LimitUnit(float guide_value, float no_less_than = -3.402823E+38f, float no_more_than = 3.402823E+38f)`

Utility function for calculating pixel dimensions in a zoom-aware environment. The desired value is put through the zoom projection and if the resulting size (as displayed on the screen) exceeds the visual limits it is clipped. This function can be used for example to make sure that a certain penwidth never exceeds visual limits (i.e. it doesn't get too thin or too thick on the screen).

**Parameters:**
- `guide_value` (System.Single) — Initial value.
- `no_less_than` (System.Single) — Minimum allowed visible size on screen.
- `no_more_than` (System.Single) — Maximum allowed visible size on screen.

**Returns:** `Single` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_LimitUnit.htm)

#### `public void Project(ref PointF pt)`

Transform a point from canvas into control coordinate space.

**Parameters:**
- `pt` (System.Drawing.PointF) — Point to transform.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_Project.htm)

#### `public PointF ProjectPoint(PointF pt)`

Project a point from canvas coordinates into control coordinates.

**Parameters:**
- `pt` (System.Drawing.PointF) — The point in canvas coordinates.

**Returns:** `PointF` — The associated point in control pixel coordinates.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_ProjectPoint.htm)

#### `public RectangleF ProjectRectangle(RectangleF rec)`

Project a rectangle from canvas coordinates into control coordinates.

**Parameters:**
- `rec` (System.Drawing.RectangleF) — The rectangle in canvas coordinates.

**Returns:** `RectangleF` — The associated rectangle in control pixel coordinates.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_ProjectRectangle.htm)

#### `public float ProjectX(float x)`

Project a value along constant X from canvas coordinates into control coordinates.

**Parameters:**
- `x` (System.Single) — The constant X value in canvas coordinates.

**Returns:** `Single` — The associated X value in control pixel coordinates.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_ProjectX.htm)

#### `public float ProjectY(float y)`

Project a value along constant Y from canvas coordinates into control coordinates.

**Parameters:**
- `y` (System.Single) — The constant Y value in canvas coordinates.

**Returns:** `Single` — The associated Y value in control pixel coordinates.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_ProjectY.htm)

#### `public void Set(GH_Viewport other)`



**Parameters:**
- `other` (Grasshopper.GUI.Canvas.GH_Viewport)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_Set.htm)

#### `public float SolveUnit(float desired, float no_less_than, float no_more_than)`

Utility function for calculating graphics dimensions in a zoom-aware environment. This function can be used for example to compute the linewidth of a pen which always needs to appear as 3 pixels thick on the screen regardless of zoom values.

**Parameters:**
- `desired` (System.Single) — Desired value in pixel units.
- `no_less_than` (System.Single) — Minimum allowed size in graphics coordinates.
- `no_more_than` (System.Single) — Maximum allowed size in graphics coordinates.

**Returns:** `Single` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_SolveUnit.htm)

#### `public void Unproject(ref PointF pt)`

Transform a point from control into canvas coordinate space.

**Parameters:**
- `pt` (System.Drawing.PointF) — Point to transform.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_Unproject.htm)

#### `public PointF UnprojectPoint(PointF pt)`

Project a point from control coordinates into canvas coordinates.

**Parameters:**
- `pt` (System.Drawing.PointF) — The point in control coordinates.

**Returns:** `PointF` — The associated point in canvas coordinates.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_UnprojectPoint.htm)

#### `public RectangleF UnprojectRectangle(RectangleF rec)`

Project a rectangle from control coordinates into canvas coordinates.

**Parameters:**
- `rec` (System.Drawing.RectangleF) — The rectangle in control coordinates.

**Returns:** `RectangleF` — The associated rectangle in canvas coordinates.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_UnprojectRectangle.htm)

#### `public float UnprojectX(float x)`

Project a value along constant X from control coordinates into canvas coordinates.

**Parameters:**
- `x` (System.Single) — The constant X value in control coordinates.

**Returns:** `Single` — The associated X value in canvas coordinates.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_UnprojectX.htm)

#### `public float UnprojectY(float y)`

Project a value along constant Y from control coordinates into canvas coordinates.

**Parameters:**
- `y` (System.Single) — The constant Y value in control coordinates.

**Returns:** `Single` — The associated Y value in canvas coordinates.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_UnprojectY.htm)

#### `public Matrix XFormMatrix(GH_Viewport.GH_DisplayMatrix projection)`

Gets the display transformation cached by this viewport.

**Parameters:**
- `projection` (Grasshopper.GUI.Canvas.GH_Viewport.GH_DisplayMatrix)

**Returns:** `Matrix` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_GH_Viewport_XFormMatrix.htm)

### Properties
- `ControlMidPoint` (PointF, get) — Gets the point in the exact center of the viewport in control coordinates.
- `Diagonal` (Single, get) — Gets the length of the diagonal of the viewport in canvas coordinates.
- `Height` (Int32, get/set) — Gets or sets the height of the viewport. Typically this is tied to the dimensions of the canvas. Height is not allowed to go below 5 pixels.
- `MidPoint` (PointF, get/set) — Gets or sets the canvas coordinate that is directly underneath the center of the viewport.
- `ScreenPort` (Rectangle, get) — Gets the dimensions of the viewport in control coordinates.
- `Size` (Size, get/set) — Gets or sets the size of the viewport. The size is typically tied to the dimensions of the Canvas.
- `Target` (Point, get/set) — Gets or sets the location of the target pixel. The target represents where the canvas origin is drawn.
- `TargetRatio` (SizeF, get/set) — Gets or sets the target ratio with respect to the viewport dimensions. This is a useful tool to prevent weird view changes during a canvas resize.
- `Tx` (Int32, get/set) — Gets or sets the x-component of the target pixel. The target represents where the canvas origin is drawn.
- `Ty` (Int32, get/set) — Gets or sets the y-component of the target pixel. The target represents where the canvas origin is drawn.
- `VisibleRegion` (RectangleF, get) — Gets the rectangle in canvas coordinates that is visible in the control.
- `Width` (Int32, get/set) — Gets or sets the width of the viewport. Typically this is tied to the dimensions of the canvas. Width is not allowed to go below 5 pixels.
- `Zoom` (Single, get/set) — Gets or sets the Zoom factor of the viewport. Please assign only sensible values.
- `Zoom[Boolean]` (Single, set) — Sets the zoom of the viewport with a view anchor.
- `ZoomInverse` (Single, get) — Gets the inverse of the zoom.
- `ZoomDefault` (Single, get) — 
- `ZoomDefaultLower` (Single, get) — 
- `ZoomDefaultUpper` (Single, get) — 
- `ZoomMaximum` (Single, get) — 
- `ZoomMinimum` (Single, get) — 

## GH_Viewport.GH_DisplayMatrix (enum)

Enumerates the possible Display Transformation matrices.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_Viewport_GH_DisplayMatrix.htm)

### Values
- `CanvasToControl` = `0`
- `ControlToCanvas` = `1`

## GH_WireDirection (enum)

Identifies a wire direction.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_WireDirection.htm)

### Values
- `left` = `0`
- `right` = `1`

## GH_WireType (enum)

Enumerates the different kinds of connecting wires.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_GH_WireType.htm)

### Values
- `dynamic` = `0` — Dynamic indicates the wire is in the process of being made.
- `generic` = `2` — Generic connector wire. Can't go wrong with generic.
- `faint` = `3` — Similar to generic, but less pronounced.
- `null` = `4` — Wire with null data transfer.
- `item` = `5` — Wire with a single data transfer.
- `list` = `6` — Wire with a single list transfer.
- `tree` = `7` — Wire with multiple list transfer.
- `dynamicAlternative1` = `8` — Wire type used by copyname=true wires.

## IGH_CanvasValidator (interface)

'Interface used for limiting a collection of typical actions on the canvas. Do not implement this interface directly if you can help it, instead inherit from GH_CanvasValidator.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_IGH_CanvasValidator.htm)

### Methods
#### `void AddedToCanvas(GH_Canvas canvas)`

This method will be called by the Canvas when this validator is added to it.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas which just added this validator.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_IGH_CanvasValidator_AddedToCanvas.htm)

#### `bool AppliesToDocument(Guid id)`

Test whether this validator applies to a specific document.

**Parameters:**
- `id` (System.Guid) — Document id or Guid.Empty if no document is available.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_IGH_CanvasValidator_AppliesToDocument.htm)

#### `bool CanAcceptObject(Guid id)`

Validates whether a specific component can be accepted by the canvas at all.

**Parameters:**
- `id` (System.Guid)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_IGH_CanvasValidator_CanAcceptObject.htm)

#### `bool CanCreateObject(Guid id, PointF pt)`

Validates whether a specific component may be inserted at a specific point.

**Parameters:**
- `id` (System.Guid)
- `pt` (System.Drawing.PointF)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_IGH_CanvasValidator_CanCreateObject.htm)

#### `bool CanCreateWire(IGH_Param source, IGH_Param target)`

Validates whether a wire-operation can commence.

**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param)
- `target` (Grasshopper.Kernel.IGH_Param)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_IGH_CanvasValidator_CanCreateWire.htm)

#### `bool CanDeleteObject(IGH_DocumentObject object)`

Validates whether a specific object can be deleted.

**Parameters:**
- `object` (Grasshopper.Kernel.IGH_DocumentObject)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_IGH_CanvasValidator_CanDeleteObject.htm)

#### `bool CanDeleteWire(IGH_Param source, IGH_Param target)`

Validates whether a wire can be deleted.

**Parameters:**
- `source` (Grasshopper.Kernel.IGH_Param)
- `target` (Grasshopper.Kernel.IGH_Param)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_IGH_CanvasValidator_CanDeleteWire.htm)

#### `bool CanDragObject(IGH_DocumentObject object, PointF dragFromPoint)`

Validates whether some components can be dragged.

**Parameters:**
- `object` (Grasshopper.Kernel.IGH_DocumentObject)
- `dragFromPoint` (System.Drawing.PointF)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_IGH_CanvasValidator_CanDragObject.htm)

#### `bool CanNavigateCanvas()`

Validates whether the canvas can be navigated (panning, zooming etc)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_IGH_CanvasValidator_CanNavigateCanvas.htm)

#### `bool CanShowCanvasMenu(PointF pt)`

Validates whether the canvas menus (both the radial and the old-fashioned one) are allowed to pop up at a specific point.

**Parameters:**
- `pt` (System.Drawing.PointF)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_IGH_CanvasValidator_CanShowCanvasMenu.htm)

#### `bool CanShowComponentSearchBox(PointF pt)`

Validates whether the double-click-component-search-window is allowed to pop up at a specific point.

**Parameters:**
- `pt` (System.Drawing.PointF)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_IGH_CanvasValidator_CanShowComponentSearchBox.htm)

#### `bool CanShowObjectMenu(IGH_DocumentObject object)`

Validates whether the object menu can be shown.

**Parameters:**
- `object` (Grasshopper.Kernel.IGH_DocumentObject)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_IGH_CanvasValidator_CanShowObjectMenu.htm)

#### `void RemovedFromCanvas(GH_Canvas canvas)`

This method will be called by the Canvas when this validator is removed from it.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas which just removed this validator.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_IGH_CanvasValidator_RemovedFromCanvas.htm)

## IGH_ResponsiveObject (interface)

If you wish to participate in Canvas UI events, you must implement this interface.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_IGH_ResponsiveObject.htm)

### Methods
#### `GH_ObjectResponse RespondToKeyDown(GH_Canvas sender, KeyEventArgs e)`



**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (System.Windows.Forms.KeyEventArgs)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_IGH_ResponsiveObject_RespondToKeyDown.htm)

#### `GH_ObjectResponse RespondToKeyUp(GH_Canvas sender, KeyEventArgs e)`



**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (System.Windows.Forms.KeyEventArgs)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_IGH_ResponsiveObject_RespondToKeyUp.htm)

#### `GH_ObjectResponse RespondToMouseDoubleClick(GH_Canvas sender, GH_CanvasMouseEvent e)`

This function will be called whenever the left button is double-clicked over the canvas. If you are active, you will be the only object who gets called.

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas) — Base canvas.
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent) — Mouse event arguments.

**Returns:** `GH_ObjectResponse` — All Response flags are valid.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_IGH_ResponsiveObject_RespondToMouseDoubleClick.htm)

#### `GH_ObjectResponse RespondToMouseDown(GH_Canvas sender, GH_CanvasMouseEvent e)`

This function will be called whenever a mouse button is pressed over the canvas. If you are active, you will be the only object who gets called. If you are inactive, you might get called if nobody on top of you decides to become active.

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas) — Base canvas.
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent) — Mouse event arguments.

**Returns:** `GH_ObjectResponse` — All Response flags are valid.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_IGH_ResponsiveObject_RespondToMouseDown.htm)

#### `GH_ObjectResponse RespondToMouseMove(GH_Canvas sender, GH_CanvasMouseEvent e)`

This function will be called when the mouse moves across the canvas. If you are active, you will be the only object who gets called. If you are inactive, you might get called if nobody on top of you decides to become active.

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas) — Base canvas.
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent) — Mouse event arguments.

**Returns:** `GH_ObjectResponse` — All Response flags are valid.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_IGH_ResponsiveObject_RespondToMouseMove.htm)

#### `GH_ObjectResponse RespondToMouseUp(GH_Canvas sender, GH_CanvasMouseEvent e)`

This function will be called whenever a mouse button is released over the canvas. If you are active, you will be the only object who gets called. If you are inactive, you will not be called at all for MouseUp events.

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas) — Base canvas.
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent) — Mouse event arguments.

**Returns:** `GH_ObjectResponse` — All Response flags are valid.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_IGH_ResponsiveObject_RespondToMouseUp.htm)

## IGH_TooltipAwareObject (interface)

Implement this interface if you want your object to participate in Grasshopper Canvas tooltips.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_IGH_TooltipAwareObject.htm)

### Methods
#### `bool IsTooltipRegion(PointF canvasPoint)`

Determine whether the specified pixel should result in a tooltip when the cursor hovers over it.

**Parameters:**
- `canvasPoint` (System.Drawing.PointF) — Point in Canvas coordinates.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_IGH_TooltipAwareObject_IsTooltipRegion.htm)

#### `void SetupTooltip(PointF canvasPoint, GH_TooltipDisplayEventArgs e)`

This function is called when a tooltip it about to be displayed.

**Parameters:**
- `canvasPoint` (System.Drawing.PointF) — The pixel coordinate on the canvas of the cursor.
- `e` (Grasshopper.GUI.GH_TooltipDisplayEventArgs) — Tooltip display arguments.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_IGH_TooltipAwareObject_SetupTooltip.htm)

### Properties
- `TooltipEnabled` (Boolean, get) — Gets the tooltip enabled value. If False, no further tooltip functions will be called.

