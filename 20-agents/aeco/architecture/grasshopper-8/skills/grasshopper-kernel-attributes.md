---
name: grasshopper-grasshopper-kernel-attributes
description: This skill encodes the grasshopper 8.0 surface of the Grasshopper.Kernel.Attributes namespace — 4 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GH_ComponentAttributes, GH_FloatingParamAttributes, GH_LinkedParamAttributes, GH_ResizableAttributes<T>.
---

# Grasshopper.Kernel.Attributes

Auto-generated from vendor docs for grasshopper 8.0. 4 types in this namespace.

## GH_ComponentAttributes (class)

These Attributes are the default for GH_Components.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Attributes_GH_ComponentAttributes.htm)

### Constructors
- `public GH_ComponentAttributes(IGH_Component component)` — Initializes a new instance of the GH_ComponentAttributes class

### Methods
#### `public override void AppendToAttributeTree(List<IGH_Attributes> attributes)`

(Overrides GH_Attributes<T>.AppendToAttributeTree(List<IGH_Attributes>).)

**Parameters:**
- `attributes` (System.Collections.Generic.List<IGH_Attributes>)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_ComponentAttributes_AppendToAttributeTree.htm)

#### `public override void ExpireLayout()`

(Overrides GH_Attributes<T>.ExpireLayout().)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_ComponentAttributes_ExpireLayout.htm)

#### `public override bool InvalidateCanvas(GH_Canvas canvas, GH_CanvasMouseEvent e)`

The canvas must be invalidated when the mouse is near enough and this component has variable parameters.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_ComponentAttributes_InvalidateCanvas.htm)

#### `public virtual bool IsMenuRegion(PointF point)`

Determines whether a point is available for context menu popups. By default, IsMenuRegion calls IsPickRegion(PointF).

**Parameters:**
- `point` (System.Drawing.PointF) — Point location to test.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_IsMenuRegion.htm)

#### `public override bool IsPickRegion(PointF point)`

(Overrides GH_Attributes<T>.IsPickRegion(PointF).)

**Parameters:**
- `point` (System.Drawing.PointF)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_ComponentAttributes_IsPickRegion.htm)

#### `public virtual bool IsPickRegion(RectangleF box, GH_PickBox method)`

Determines whether a rectangle is a valid pick region for this object. By default, the picking rectangle is intersected with the Bounds rectangle.

**Parameters:**
- `box` (System.Drawing.RectangleF) — Selection box.
- `method` (Grasshopper.Kernel.GH_PickBox) — Selection method.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_IsPickRegion_1.htm)

#### `public virtual bool IsTooltipRegion(PointF point)`

Determines whether a point is available for tooltip popups. By default, IsMenuRegion calls IsTooltipRegion(PointF).

**Parameters:**
- `point` (System.Drawing.PointF) — Point location to test.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_IsTooltipRegion.htm)

#### `protected override void Layout()`

(Overrides GH_Attributes<T>.Layout().)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_ComponentAttributes_Layout.htm)

#### `public static RectangleF LayoutBounds(IGH_Component owner, RectangleF bounds)`



**Parameters:**
- `owner` (Grasshopper.Kernel.IGH_Component)
- `bounds` (System.Drawing.RectangleF)

**Returns:** `RectangleF` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_ComponentAttributes_LayoutBounds.htm)

#### `public static RectangleF LayoutComponentBox(IGH_Component owner)`

Utility layout method for GH_Component style UI.

**Parameters:**
- `owner` (Grasshopper.Kernel.IGH_Component) — Component to layout.

**Returns:** `RectangleF` — The rectangle describing the box of the component name/icon.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_ComponentAttributes_LayoutComponentBox.htm)

#### `public static void LayoutInputParams(IGH_Component owner, RectangleF componentBox)`



**Parameters:**
- `owner` (Grasshopper.Kernel.IGH_Component)
- `componentBox` (System.Drawing.RectangleF)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_ComponentAttributes_LayoutInputParams.htm)

#### `public static void LayoutOutputParams(IGH_Component owner, RectangleF componentBox)`



**Parameters:**
- `owner` (Grasshopper.Kernel.IGH_Component)
- `componentBox` (System.Drawing.RectangleF)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_ComponentAttributes_LayoutOutputParams.htm)

#### `public void NewInstanceGuid()`

Generate a new instance GUID for the owner object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_NewInstanceGuid.htm)

#### `public void NewInstanceGuid(Guid newId)`

Generate a new instance GUID for the owner object. Do not use this overload unless you're > 1.95m and called David.

**Parameters:**
- `newId` (System.Guid)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_NewInstanceGuid_1.htm)

#### `public void PerformLayout()`

Recompute the layout for these attributes. This function is automatically called during Drawing operations, so you typically don't have to.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_PerformLayout.htm)

#### `protected virtual void PrepareForRender(GH_Canvas canvas)`

This method will always be called exactly once prior to Render(). This would be a good place to make sure all the necessary GUI data is up and running.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas that is about to render these attributes.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_PrepareForRender.htm)

#### `public virtual bool Read(GH_IReader reader)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_Read.htm)

#### `protected override void Render(GH_Canvas canvas, Graphics graphics, GH_CanvasChannel channel)`

(Overrides GH_Attributes<T>.Render(GH_Canvas, Graphics, GH_CanvasChannel).)

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas)
- `graphics` (System.Drawing.Graphics)
- `channel` (Grasshopper.GUI.Canvas.GH_CanvasChannel)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_ComponentAttributes_Render.htm)

#### `protected void RenderComponentCapsule(GH_Canvas canvas, Graphics graphics)`

Utility function, draws the complete component capsule including all parameter data.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas to draw into.
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_ComponentAttributes_RenderComponentCapsule.htm)

#### `protected void RenderComponentCapsule(GH_Canvas canvas, Graphics graphics, bool drawComponentBaseBox, bool drawComponentNameBox, bool drawJaggedEdges, bool drawParameterGrips, bool drawParameterNames, bool drawZuiElements)`

Utility function, draws the component capsule including all specified elements.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas to draw into.
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `drawComponentBaseBox` (System.Boolean) — If true, will draw the component background box.
- `drawComponentNameBox` (System.Boolean) — If true, will draw component nickname/icon element.
- `drawJaggedEdges` (System.Boolean) — If true, parameterless sides will have jagged edges drawn on them.
- `drawParameterGrips` (System.Boolean) — If true, will draw parameter grips.
- `drawParameterNames` (System.Boolean) — If true, will draw parameter nicknames.
- `drawZuiElements` (System.Boolean) — If true, will draw ZUI variable parameter interface.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_ComponentAttributes_RenderComponentCapsule_1.htm)

#### `public static void RenderComponentParameters(GH_Canvas canvas, Graphics graphics, IGH_Component owner, GH_PaletteStyle style)`

Utility method for rendering all component parameters.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas to draw to.
- `graphics` (System.Drawing.Graphics) — Graphics to draw with.
- `owner` (Grasshopper.Kernel.IGH_Component) — Component to draw parameters for.
- `style` (Grasshopper.GUI.Canvas.GH_PaletteStyle) — Component capsule palette style.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_ComponentAttributes_RenderComponentParameters.htm)

#### `protected void RenderIncomingWires(GH_Painter painter, IEnumerable<IGH_Param> sources, GH_ParamWireDisplay style)`

Utility function for derived classes. This method draws all the wires going into the left side of the attributes.

**Parameters:**
- `painter` (Grasshopper.GUI.Canvas.GH_Painter) — The graphics object to paint with
- `sources` (System.Collections.Generic.IEnumerable<IGH_Param>) — A list of sources that will be connected with the input grip of this attribute
- `style` (Grasshopper.Kernel.GH_ParamWireDisplay) — Style for wire display.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RenderIncomingWires.htm)

#### `protected void RenderIncomingWires(GH_Painter painter, IEnumerable<IGH_Param> sources, IEnumerable<Pen> styles)`

Utility function for derived classes. This method draws all the wires going into the left side of the attributes.

**Parameters:**
- `painter` (Grasshopper.GUI.Canvas.GH_Painter) — The graphics object to paint with
- `sources` (System.Collections.Generic.IEnumerable<IGH_Param>) — A list of sources that will be connected with the input grip of this attribute
- `styles` (System.Collections.Generic.IEnumerable<Pen>) — A collection of pen objects to use for each source. If a pen is null, the default wire style will be used instead.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RenderIncomingWires_1.htm)

#### `public void RenderToCanvas(GH_Canvas canvas, GH_CanvasChannel channel)`

Render these attributes into a Canvas control. This function places calls to PrepareForRender() and Render(), you should override those.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas to draw on.
- `channel` (Grasshopper.GUI.Canvas.GH_CanvasChannel) — Current drawing channel.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RenderToCanvas.htm)

#### `protected void RenderVariableParameterUI(GH_Canvas canvas, Graphics graphics)`



**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas)
- `graphics` (System.Drawing.Graphics)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_ComponentAttributes_RenderVariableParameterUI.htm)

#### `public virtual GH_ObjectResponse RespondToKeyDown(GH_Canvas sender, KeyEventArgs e)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (System.Windows.Forms.KeyEventArgs)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RespondToKeyDown.htm)

#### `public virtual GH_ObjectResponse RespondToKeyUp(GH_Canvas sender, KeyEventArgs e)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (System.Windows.Forms.KeyEventArgs)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RespondToKeyUp.htm)

#### `public virtual GH_ObjectResponse RespondToMouseDoubleClick(GH_Canvas sender, GH_CanvasMouseEvent e)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RespondToMouseDoubleClick.htm)

#### `public override GH_ObjectResponse RespondToMouseDown(GH_Canvas sender, GH_CanvasMouseEvent e)`

(Overrides GH_Attributes<T>.RespondToMouseDown(GH_Canvas, GH_CanvasMouseEvent).)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_ComponentAttributes_RespondToMouseDown.htm)

#### `public virtual GH_ObjectResponse RespondToMouseMove(GH_Canvas sender, GH_CanvasMouseEvent e)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RespondToMouseMove.htm)

#### `public virtual GH_ObjectResponse RespondToMouseUp(GH_Canvas sender, GH_CanvasMouseEvent e)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RespondToMouseUp.htm)

#### `public override void SetupTooltip(PointF canvasPoint, GH_TooltipDisplayEventArgs e)`

(Overrides GH_Attributes<T>.SetupTooltip(PointF, GH_TooltipDisplayEventArgs).)

**Parameters:**
- `canvasPoint` (System.Drawing.PointF)
- `e` (Grasshopper.GUI.GH_TooltipDisplayEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_ComponentAttributes_SetupTooltip.htm)

#### `public virtual bool Write(GH_IWriter writer)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_Write.htm)

### Properties
- `AllowMessageBalloon` (Boolean, get) — Gets a value indicating whether these attributes allow warning and error balloons to be drawn on top of them.
- `Bounds` (RectangleF, get/set) — Gets the rectangle that contains the active content of the attributes. Typically the Contents determine the active area for menus, tooltips etc. Attributes are not supposed to draw objects beyond the Bounds.
- `ContentBox` (RectangleF, get) — Gets the bounds of the component area of the attributes.
- `DocObject` (IGH_DocumentObject, get) — Gets the owner object of these attributes.
- `GetTopLevel` (IGH_Attributes, get) — Gets the top-level attributes of the attribute stack these attributes belong to.
- `HasInputGrip` (Boolean, get) — (Overrides GH_Attributes<T>.HasInputGrip.)
- `HasOutputGrip` (Boolean, get) — (Overrides GH_Attributes<T>.HasOutputGrip.)
- `InputGrip` (PointF, get) — Gets the input grip location for these attributes. If HasInputGrip equals False, this point is meaningless.
- `InstanceGuid` (Guid, get) — Gets the instance ID of the document object that owns these attributes.
- `IsTopLevel` (Boolean, get) — Gets whether these attributes are top_level (i.e. no Parent attributes)
- `OutputGrip` (PointF, get) — Gets the output grip location for these attributes. If HasOutputGrip equals False, this point is meaningless.
- `Owner` (T, get) — Gets the type-safe owner object of these attributes. This property is identical to the DocObject property.
- `Parent` (IGH_Attributes, get/set) — Gets or sets the parent attributes. Top level attributes do not have parents.
- `PathName` (String, get) — (Overrides GH_Attributes<T>.PathName.)
- `Pivot` (PointF, get/set) — Gets or sets the pivot for these attributes. The pivot controls the general placement of the attributes. If you want to move the attributes, change the pivot location.
- `Selected` (Boolean, get/set) — Gets or sets the selected state of the top-level attributes.
- `TooltipEnabled` (Boolean, get) — Gets the tooltip enabled value. If False, no further tooltip functions will be called.
- `m_innerBounds` (RectangleF, get) — Represents the Rectangle that contains the component black box.

## GH_FloatingParamAttributes (class)

These Attributes are the default for stand-alone Parameters.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Attributes_GH_FloatingParamAttributes.htm)

### Constructors
- `public GH_FloatingParamAttributes(IGH_Param param)` — Initializes a new instance of the GH_FloatingParamAttributes class

### Methods
#### `public virtual void AppendToAttributeTree(List<IGH_Attributes> attributes)`

Recursively append these attributes and all child attributes to the attribute list.

**Parameters:**
- `attributes` (System.Collections.Generic.List<IGH_Attributes>) — List to append to.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_AppendToAttributeTree.htm)

#### `public virtual void ExpireLayout()`

Expires the entire layout of the attributes. When overridden, implementer must place a call to the base class ExpireLayout().

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_ExpireLayout.htm)

#### `public virtual bool InvalidateCanvas(GH_Canvas canvas, GH_CanvasMouseEvent e)`

If the mouse location should cause a canvas invalidation then return true. You only need to override this function if you draw objects that are dependant on cursor positions outside the bounds of the attributes.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas to potentially invalidate.
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent) — Event arguments for canvas mousemove.

**Returns:** `Boolean` — True if the canvas ought to be invalidated.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_InvalidateCanvas.htm)

#### `public virtual bool IsMenuRegion(PointF point)`

Determines whether a point is available for context menu popups. By default, IsMenuRegion calls IsPickRegion(PointF).

**Parameters:**
- `point` (System.Drawing.PointF) — Point location to test.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_IsMenuRegion.htm)

#### `public virtual bool IsPickRegion(PointF point)`

Determines whether a point is within the pickable region for this object. By default, any point inside the Bounds is treated as pickable.

**Parameters:**
- `point` (System.Drawing.PointF) — Point location to test.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_IsPickRegion.htm)

#### `public virtual bool IsPickRegion(RectangleF box, GH_PickBox method)`

Determines whether a rectangle is a valid pick region for this object. By default, the picking rectangle is intersected with the Bounds rectangle.

**Parameters:**
- `box` (System.Drawing.RectangleF) — Selection box.
- `method` (Grasshopper.Kernel.GH_PickBox) — Selection method.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_IsPickRegion_1.htm)

#### `public virtual bool IsTooltipRegion(PointF point)`

Determines whether a point is available for tooltip popups. By default, IsMenuRegion calls IsTooltipRegion(PointF).

**Parameters:**
- `point` (System.Drawing.PointF) — Point location to test.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_IsTooltipRegion.htm)

#### `protected override void Layout()`

(Overrides GH_Attributes<T>.Layout().)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_FloatingParamAttributes_Layout.htm)

#### `public void NewInstanceGuid()`

Generate a new instance GUID for the owner object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_NewInstanceGuid.htm)

#### `public void NewInstanceGuid(Guid newId)`

Generate a new instance GUID for the owner object. Do not use this overload unless you're > 1.95m and called David.

**Parameters:**
- `newId` (System.Guid)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_NewInstanceGuid_1.htm)

#### `public void PerformLayout()`

Recompute the layout for these attributes. This function is automatically called during Drawing operations, so you typically don't have to.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_PerformLayout.htm)

#### `protected virtual void PrepareForRender(GH_Canvas canvas)`

This method will always be called exactly once prior to Render(). This would be a good place to make sure all the necessary GUI data is up and running.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas that is about to render these attributes.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_PrepareForRender.htm)

#### `public virtual bool Read(GH_IReader reader)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_Read.htm)

#### `protected override void Render(GH_Canvas canvas, Graphics graphics, GH_CanvasChannel channel)`

(Overrides GH_Attributes<T>.Render(GH_Canvas, Graphics, GH_CanvasChannel).)

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas)
- `graphics` (System.Drawing.Graphics)
- `channel` (Grasshopper.GUI.Canvas.GH_CanvasChannel)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_FloatingParamAttributes_Render.htm)

#### `protected void RenderIncomingWires(GH_Painter painter, IEnumerable<IGH_Param> sources, GH_ParamWireDisplay style)`

Utility function for derived classes. This method draws all the wires going into the left side of the attributes.

**Parameters:**
- `painter` (Grasshopper.GUI.Canvas.GH_Painter) — The graphics object to paint with
- `sources` (System.Collections.Generic.IEnumerable<IGH_Param>) — A list of sources that will be connected with the input grip of this attribute
- `style` (Grasshopper.Kernel.GH_ParamWireDisplay) — Style for wire display.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RenderIncomingWires.htm)

#### `protected void RenderIncomingWires(GH_Painter painter, IEnumerable<IGH_Param> sources, IEnumerable<Pen> styles)`

Utility function for derived classes. This method draws all the wires going into the left side of the attributes.

**Parameters:**
- `painter` (Grasshopper.GUI.Canvas.GH_Painter) — The graphics object to paint with
- `sources` (System.Collections.Generic.IEnumerable<IGH_Param>) — A list of sources that will be connected with the input grip of this attribute
- `styles` (System.Collections.Generic.IEnumerable<Pen>) — A collection of pen objects to use for each source. If a pen is null, the default wire style will be used instead.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RenderIncomingWires_1.htm)

#### `public void RenderToCanvas(GH_Canvas canvas, GH_CanvasChannel channel)`

Render these attributes into a Canvas control. This function places calls to PrepareForRender() and Render(), you should override those.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas to draw on.
- `channel` (Grasshopper.GUI.Canvas.GH_CanvasChannel) — Current drawing channel.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RenderToCanvas.htm)

#### `public virtual GH_ObjectResponse RespondToKeyDown(GH_Canvas sender, KeyEventArgs e)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (System.Windows.Forms.KeyEventArgs)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RespondToKeyDown.htm)

#### `public virtual GH_ObjectResponse RespondToKeyUp(GH_Canvas sender, KeyEventArgs e)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (System.Windows.Forms.KeyEventArgs)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RespondToKeyUp.htm)

#### `public override GH_ObjectResponse RespondToMouseDoubleClick(GH_Canvas sender, GH_CanvasMouseEvent e)`

(Overrides GH_Attributes<T>.RespondToMouseDoubleClick(GH_Canvas, GH_CanvasMouseEvent).)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_FloatingParamAttributes_RespondToMouseDoubleClick.htm)

#### `public virtual GH_ObjectResponse RespondToMouseDown(GH_Canvas sender, GH_CanvasMouseEvent e)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RespondToMouseDown.htm)

#### `public virtual GH_ObjectResponse RespondToMouseMove(GH_Canvas sender, GH_CanvasMouseEvent e)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RespondToMouseMove.htm)

#### `public virtual GH_ObjectResponse RespondToMouseUp(GH_Canvas sender, GH_CanvasMouseEvent e)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RespondToMouseUp.htm)

#### `public override void SetupTooltip(PointF point, GH_TooltipDisplayEventArgs e)`

(Overrides GH_Attributes<T>.SetupTooltip(PointF, GH_TooltipDisplayEventArgs).)

**Parameters:**
- `point` (System.Drawing.PointF)
- `e` (Grasshopper.GUI.GH_TooltipDisplayEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_FloatingParamAttributes_SetupTooltip.htm)

#### `public virtual bool Write(GH_IWriter writer)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_Write.htm)

### Properties
- `AllowMessageBalloon` (Boolean, get) — Gets a value indicating whether these attributes allow warning and error balloons to be drawn on top of them.
- `Bounds` (RectangleF, get/set) — Gets the rectangle that contains the active content of the attributes. Typically the Contents determine the active area for menus, tooltips etc. Attributes are not supposed to draw objects beyond the Bounds.
- `DocObject` (IGH_DocumentObject, get) — Gets the owner object of these attributes.
- `GetTopLevel` (IGH_Attributes, get) — Gets the top-level attributes of the attribute stack these attributes belong to.
- `HasInputGrip` (Boolean, get) — Gets a value indicating whether or not these attributes have an input grip.
- `HasOutputGrip` (Boolean, get) — Gets a value indicating whether or not these attributes have an output grip.
- `InputGrip` (PointF, get) — Gets the input grip location for these attributes. If HasInputGrip equals False, this point is meaningless.
- `InstanceGuid` (Guid, get) — Gets the instance ID of the document object that owns these attributes.
- `IsTopLevel` (Boolean, get) — Gets whether these attributes are top_level (i.e. no Parent attributes)
- `OutputGrip` (PointF, get) — Gets the output grip location for these attributes. If HasOutputGrip equals False, this point is meaningless.
- `Owner` (T, get) — Gets the type-safe owner object of these attributes. This property is identical to the DocObject property.
- `Parent` (IGH_Attributes, get/set) — Gets or sets the parent attributes. Top level attributes do not have parents.
- `PathName` (String, get) — Get a description of the location of these attributes within the local attribute stack.
- `Pivot` (PointF, get/set) — Gets or sets the pivot for these attributes. The pivot controls the general placement of the attributes. If you want to move the attributes, change the pivot location.
- `Selected` (Boolean, get/set) — Gets or sets the selected state of the top-level attributes.
- `TooltipEnabled` (Boolean, get) — Gets the tooltip enabled value. If False, no further tooltip functions will be called.
- `DefaultWidth` (Int32, get) — Defines the default (and minimum) width of a floating parameter.
- `IconHeight` (Int32, get) — Defines the default height of a floating parameter in Icon display mode.
- `TextHeight` (Int32, get) — Defines the default height of a floating parameter in Text display mode.

## GH_LinkedParamAttributes (class)

These Attributes are assigned to parameters that are part of a GH_Component.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Attributes_GH_LinkedParamAttributes.htm)

### Constructors
- `public GH_LinkedParamAttributes(IGH_Param param, IGH_Attributes parent)` — Initializes a new instance of the GH_LinkedParamAttributes class

### Methods
#### `public virtual void AppendToAttributeTree(List<IGH_Attributes> attributes)`

Recursively append these attributes and all child attributes to the attribute list.

**Parameters:**
- `attributes` (System.Collections.Generic.List<IGH_Attributes>) — List to append to.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_AppendToAttributeTree.htm)

#### `public virtual void ExpireLayout()`

Expires the entire layout of the attributes. When overridden, implementer must place a call to the base class ExpireLayout().

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_ExpireLayout.htm)

#### `public virtual bool InvalidateCanvas(GH_Canvas canvas, GH_CanvasMouseEvent e)`

If the mouse location should cause a canvas invalidation then return true. You only need to override this function if you draw objects that are dependant on cursor positions outside the bounds of the attributes.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas to potentially invalidate.
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent) — Event arguments for canvas mousemove.

**Returns:** `Boolean` — True if the canvas ought to be invalidated.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_InvalidateCanvas.htm)

#### `public virtual bool IsMenuRegion(PointF point)`

Determines whether a point is available for context menu popups. By default, IsMenuRegion calls IsPickRegion(PointF).

**Parameters:**
- `point` (System.Drawing.PointF) — Point location to test.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_IsMenuRegion.htm)

#### `public virtual bool IsPickRegion(PointF point)`

Determines whether a point is within the pickable region for this object. By default, any point inside the Bounds is treated as pickable.

**Parameters:**
- `point` (System.Drawing.PointF) — Point location to test.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_IsPickRegion.htm)

#### `public virtual bool IsPickRegion(RectangleF box, GH_PickBox method)`

Determines whether a rectangle is a valid pick region for this object. By default, the picking rectangle is intersected with the Bounds rectangle.

**Parameters:**
- `box` (System.Drawing.RectangleF) — Selection box.
- `method` (Grasshopper.Kernel.GH_PickBox) — Selection method.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_IsPickRegion_1.htm)

#### `public virtual bool IsTooltipRegion(PointF point)`

Determines whether a point is available for tooltip popups. By default, IsMenuRegion calls IsTooltipRegion(PointF).

**Parameters:**
- `point` (System.Drawing.PointF) — Point location to test.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_IsTooltipRegion.htm)

#### `protected override void Layout()`

(Overrides GH_Attributes<T>.Layout().)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_LinkedParamAttributes_Layout.htm)

#### `public void NewInstanceGuid()`

Generate a new instance GUID for the owner object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_NewInstanceGuid.htm)

#### `public void NewInstanceGuid(Guid newId)`

Generate a new instance GUID for the owner object. Do not use this overload unless you're > 1.95m and called David.

**Parameters:**
- `newId` (System.Guid)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_NewInstanceGuid_1.htm)

#### `public void PerformLayout()`

Recompute the layout for these attributes. This function is automatically called during Drawing operations, so you typically don't have to.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_PerformLayout.htm)

#### `protected virtual void PrepareForRender(GH_Canvas canvas)`

This method will always be called exactly once prior to Render(). This would be a good place to make sure all the necessary GUI data is up and running.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas that is about to render these attributes.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_PrepareForRender.htm)

#### `public virtual bool Read(GH_IReader reader)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_Read.htm)

#### `protected override void Render(GH_Canvas canvas, Graphics graphics, GH_CanvasChannel channel)`

(Overrides GH_Attributes<T>.Render(GH_Canvas, Graphics, GH_CanvasChannel).)

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas)
- `graphics` (System.Drawing.Graphics)
- `channel` (Grasshopper.GUI.Canvas.GH_CanvasChannel)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_LinkedParamAttributes_Render.htm)

#### `protected void RenderIncomingWires(GH_Painter painter, IEnumerable<IGH_Param> sources, GH_ParamWireDisplay style)`

Utility function for derived classes. This method draws all the wires going into the left side of the attributes.

**Parameters:**
- `painter` (Grasshopper.GUI.Canvas.GH_Painter) — The graphics object to paint with
- `sources` (System.Collections.Generic.IEnumerable<IGH_Param>) — A list of sources that will be connected with the input grip of this attribute
- `style` (Grasshopper.Kernel.GH_ParamWireDisplay) — Style for wire display.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RenderIncomingWires.htm)

#### `protected void RenderIncomingWires(GH_Painter painter, IEnumerable<IGH_Param> sources, IEnumerable<Pen> styles)`

Utility function for derived classes. This method draws all the wires going into the left side of the attributes.

**Parameters:**
- `painter` (Grasshopper.GUI.Canvas.GH_Painter) — The graphics object to paint with
- `sources` (System.Collections.Generic.IEnumerable<IGH_Param>) — A list of sources that will be connected with the input grip of this attribute
- `styles` (System.Collections.Generic.IEnumerable<Pen>) — A collection of pen objects to use for each source. If a pen is null, the default wire style will be used instead.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RenderIncomingWires_1.htm)

#### `public void RenderToCanvas(GH_Canvas canvas, GH_CanvasChannel channel)`

Render these attributes into a Canvas control. This function places calls to PrepareForRender() and Render(), you should override those.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas to draw on.
- `channel` (Grasshopper.GUI.Canvas.GH_CanvasChannel) — Current drawing channel.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RenderToCanvas.htm)

#### `public virtual GH_ObjectResponse RespondToKeyDown(GH_Canvas sender, KeyEventArgs e)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (System.Windows.Forms.KeyEventArgs)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RespondToKeyDown.htm)

#### `public virtual GH_ObjectResponse RespondToKeyUp(GH_Canvas sender, KeyEventArgs e)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (System.Windows.Forms.KeyEventArgs)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RespondToKeyUp.htm)

#### `public virtual GH_ObjectResponse RespondToMouseDoubleClick(GH_Canvas sender, GH_CanvasMouseEvent e)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RespondToMouseDoubleClick.htm)

#### `public virtual GH_ObjectResponse RespondToMouseDown(GH_Canvas sender, GH_CanvasMouseEvent e)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RespondToMouseDown.htm)

#### `public virtual GH_ObjectResponse RespondToMouseMove(GH_Canvas sender, GH_CanvasMouseEvent e)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RespondToMouseMove.htm)

#### `public virtual GH_ObjectResponse RespondToMouseUp(GH_Canvas sender, GH_CanvasMouseEvent e)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RespondToMouseUp.htm)

#### `public override void SetupTooltip(PointF point, GH_TooltipDisplayEventArgs e)`

(Overrides GH_Attributes<T>.SetupTooltip(PointF, GH_TooltipDisplayEventArgs).)

**Parameters:**
- `point` (System.Drawing.PointF)
- `e` (Grasshopper.GUI.GH_TooltipDisplayEventArgs)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_LinkedParamAttributes_SetupTooltip.htm)

#### `public virtual bool Write(GH_IWriter writer)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_Write.htm)

### Properties
- `AllowMessageBalloon` (Boolean, get) — Gets a value indicating whether these attributes allow warning and error balloons to be drawn on top of them.
- `Bounds` (RectangleF, get/set) — Gets the rectangle that contains the active content of the attributes. Typically the Contents determine the active area for menus, tooltips etc. Attributes are not supposed to draw objects beyond the Bounds.
- `DocObject` (IGH_DocumentObject, get) — Gets the owner object of these attributes.
- `GetTopLevel` (IGH_Attributes, get) — Gets the top-level attributes of the attribute stack these attributes belong to.
- `HasInputGrip` (Boolean, get) — (Overrides GH_Attributes<T>.HasInputGrip.)
- `HasOutputGrip` (Boolean, get) — (Overrides GH_Attributes<T>.HasOutputGrip.)
- `InputGrip` (PointF, get) — Gets the input grip location for these attributes. If HasInputGrip equals False, this point is meaningless.
- `InstanceGuid` (Guid, get) — Gets the instance ID of the document object that owns these attributes.
- `IsTopLevel` (Boolean, get) — Gets whether these attributes are top_level (i.e. no Parent attributes)
- `OutputGrip` (PointF, get) — Gets the output grip location for these attributes. If HasOutputGrip equals False, this point is meaningless.
- `Owner` (T, get) — Gets the type-safe owner object of these attributes. This property is identical to the DocObject property.
- `Parent` (IGH_Attributes, get/set) — Gets or sets the parent attributes. Top level attributes do not have parents.
- `PathName` (String, get) — Get a description of the location of these attributes within the local attribute stack.
- `Pivot` (PointF, get/set) — Gets or sets the pivot for these attributes. The pivot controls the general placement of the attributes. If you want to move the attributes, change the pivot location.
- `Selected` (Boolean, get/set) — Gets or sets the selected state of the top-level attributes.
- `TooltipEnabled` (Boolean, get) — Gets the tooltip enabled value. If False, no further tooltip functions will be called.

## GH_ResizableAttributes<T> (class)

These Attributes provide basic resizing logic.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_Kernel_Attributes_GH_ResizableAttributes_1.htm)

### Constructors
- `protected GH_ResizableAttributes(T owner)` — Initializes a new instance of the GH_ResizableAttributes<T> class

### Methods
#### `public virtual void AppendToAttributeTree(List<IGH_Attributes> attributes)`

Recursively append these attributes and all child attributes to the attribute list.

**Parameters:**
- `attributes` (System.Collections.Generic.List<IGH_Attributes>) — List to append to.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_AppendToAttributeTree.htm)

#### `public virtual void ExpireLayout()`

Expires the entire layout of the attributes. When overridden, implementer must place a call to the base class ExpireLayout().

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_ExpireLayout.htm)

#### `public virtual bool InvalidateCanvas(GH_Canvas canvas, GH_CanvasMouseEvent e)`

If the mouse location should cause a canvas invalidation then return true. You only need to override this function if you draw objects that are dependant on cursor positions outside the bounds of the attributes.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas to potentially invalidate.
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent) — Event arguments for canvas mousemove.

**Returns:** `Boolean` — True if the canvas ought to be invalidated.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_InvalidateCanvas.htm)

#### `public virtual bool IsMenuRegion(PointF point)`

Determines whether a point is available for context menu popups. By default, IsMenuRegion calls IsPickRegion(PointF).

**Parameters:**
- `point` (System.Drawing.PointF) — Point location to test.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_IsMenuRegion.htm)

#### `public virtual bool IsPickRegion(PointF point)`

Determines whether a point is within the pickable region for this object. By default, any point inside the Bounds is treated as pickable.

**Parameters:**
- `point` (System.Drawing.PointF) — Point location to test.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_IsPickRegion.htm)

#### `public virtual bool IsPickRegion(RectangleF box, GH_PickBox method)`

Determines whether a rectangle is a valid pick region for this object. By default, the picking rectangle is intersected with the Bounds rectangle.

**Parameters:**
- `box` (System.Drawing.RectangleF) — Selection box.
- `method` (Grasshopper.Kernel.GH_PickBox) — Selection method.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_IsPickRegion_1.htm)

#### `public virtual bool IsTooltipRegion(PointF point)`

Determines whether a point is available for tooltip popups. By default, IsMenuRegion calls IsTooltipRegion(PointF).

**Parameters:**
- `point` (System.Drawing.PointF) — Point location to test.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_IsTooltipRegion.htm)

#### `protected virtual void Layout()`

Perform dedicated layout logic specific to this particular DocumentObject. This method is called from PerformLayout if the existing layout is stale.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_Layout.htm)

#### `public void NewInstanceGuid()`

Generate a new instance GUID for the owner object.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_NewInstanceGuid.htm)

#### `public void NewInstanceGuid(Guid newId)`

Generate a new instance GUID for the owner object. Do not use this overload unless you're > 1.95m and called David.

**Parameters:**
- `newId` (System.Guid)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_NewInstanceGuid_1.htm)

#### `public void PerformLayout()`

Recompute the layout for these attributes. This function is automatically called during Drawing operations, so you typically don't have to.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_PerformLayout.htm)

#### `protected virtual void PrepareForRender(GH_Canvas canvas)`

This method will always be called exactly once prior to Render(). This would be a good place to make sure all the necessary GUI data is up and running.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas that is about to render these attributes.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_PrepareForRender.htm)

#### `public virtual bool Read(GH_IReader reader)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_Read.htm)

#### `protected virtual void Render(GH_Canvas canvas, Graphics graphics, GH_CanvasChannel channel)`

Override this function to supply custom Render logic.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas in which these attributes are to be drawn.
- `graphics` (System.Drawing.Graphics) — Graphics object to draw with.
- `channel` (Grasshopper.GUI.Canvas.GH_CanvasChannel) — Current drawing channel.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_Render.htm)

#### `protected void RenderIncomingWires(GH_Painter painter, IEnumerable<IGH_Param> sources, GH_ParamWireDisplay style)`

Utility function for derived classes. This method draws all the wires going into the left side of the attributes.

**Parameters:**
- `painter` (Grasshopper.GUI.Canvas.GH_Painter) — The graphics object to paint with
- `sources` (System.Collections.Generic.IEnumerable<IGH_Param>) — A list of sources that will be connected with the input grip of this attribute
- `style` (Grasshopper.Kernel.GH_ParamWireDisplay) — Style for wire display.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RenderIncomingWires.htm)

#### `protected void RenderIncomingWires(GH_Painter painter, IEnumerable<IGH_Param> sources, IEnumerable<Pen> styles)`

Utility function for derived classes. This method draws all the wires going into the left side of the attributes.

**Parameters:**
- `painter` (Grasshopper.GUI.Canvas.GH_Painter) — The graphics object to paint with
- `sources` (System.Collections.Generic.IEnumerable<IGH_Param>) — A list of sources that will be connected with the input grip of this attribute
- `styles` (System.Collections.Generic.IEnumerable<Pen>) — A collection of pen objects to use for each source. If a pen is null, the default wire style will be used instead.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RenderIncomingWires_1.htm)

#### `public void RenderToCanvas(GH_Canvas canvas, GH_CanvasChannel channel)`

Render these attributes into a Canvas control. This function places calls to PrepareForRender() and Render(), you should override those.

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas to draw on.
- `channel` (Grasshopper.GUI.Canvas.GH_CanvasChannel) — Current drawing channel.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RenderToCanvas.htm)

#### `public virtual GH_ObjectResponse RespondToKeyDown(GH_Canvas sender, KeyEventArgs e)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (System.Windows.Forms.KeyEventArgs)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RespondToKeyDown.htm)

#### `public virtual GH_ObjectResponse RespondToKeyUp(GH_Canvas sender, KeyEventArgs e)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (System.Windows.Forms.KeyEventArgs)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RespondToKeyUp.htm)

#### `public virtual GH_ObjectResponse RespondToMouseDoubleClick(GH_Canvas sender, GH_CanvasMouseEvent e)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_RespondToMouseDoubleClick.htm)

#### `public override GH_ObjectResponse RespondToMouseDown(GH_Canvas sender, GH_CanvasMouseEvent e)`

(Overrides GH_Attributes<T>.RespondToMouseDown(GH_Canvas, GH_CanvasMouseEvent).)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_ResizableAttributes_1_RespondToMouseDown.htm)

#### `public override GH_ObjectResponse RespondToMouseMove(GH_Canvas sender, GH_CanvasMouseEvent e)`

(Overrides GH_Attributes<T>.RespondToMouseMove(GH_Canvas, GH_CanvasMouseEvent).)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_ResizableAttributes_1_RespondToMouseMove.htm)

#### `public override GH_ObjectResponse RespondToMouseUp(GH_Canvas sender, GH_CanvasMouseEvent e)`

(Overrides GH_Attributes<T>.RespondToMouseUp(GH_Canvas, GH_CanvasMouseEvent).)

**Parameters:**
- `sender` (Grasshopper.GUI.Canvas.GH_Canvas)
- `e` (Grasshopper.GUI.GH_CanvasMouseEvent)

**Returns:** `GH_ObjectResponse` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_Attributes_GH_ResizableAttributes_1_RespondToMouseUp.htm)

#### `public virtual void SetupTooltip(PointF point, GH_TooltipDisplayEventArgs e)`

Populates the Grasshopper tooltip with all relevant data. If this function returns True, it is assumed that all possible fields have been filled out and the tooltip is ready for display.

**Parameters:**
- `point` (System.Drawing.PointF) — The pixel coordinate on the canvas of the cursor.
- `e` (Grasshopper.GUI.GH_TooltipDisplayEventArgs) — Tooltip event arguments.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_SetupTooltip.htm)

#### `public virtual bool Write(GH_IWriter writer)`

(Inherited from GH_Attributes<T>.)

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter)

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_Kernel_GH_Attributes_1_Write.htm)

### Properties
- `AllowMessageBalloon` (Boolean, get) — Gets a value indicating whether these attributes allow warning and error balloons to be drawn on top of them.
- `Bounds` (RectangleF, get/set) — Gets the rectangle that contains the active content of the attributes. Typically the Contents determine the active area for menus, tooltips etc. Attributes are not supposed to draw objects beyond the Bounds.
- `DocObject` (IGH_DocumentObject, get) — Gets the owner object of these attributes.
- `GetTopLevel` (IGH_Attributes, get) — Gets the top-level attributes of the attribute stack these attributes belong to.
- `HasInputGrip` (Boolean, get) — Gets a value indicating whether or not these attributes have an input grip.
- `HasOutputGrip` (Boolean, get) — Gets a value indicating whether or not these attributes have an output grip.
- `InputGrip` (PointF, get) — Gets the input grip location for these attributes. If HasInputGrip equals False, this point is meaningless.
- `InstanceGuid` (Guid, get) — Gets the instance ID of the document object that owns these attributes.
- `IsTopLevel` (Boolean, get) — Gets whether these attributes are top_level (i.e. no Parent attributes)
- `MaximumSize` (Size, get) — 
- `MinimumSize` (Size, get) — 
- `OutputGrip` (PointF, get) — Gets the output grip location for these attributes. If HasOutputGrip equals False, this point is meaningless.
- `Owner` (T, get) — Gets the type-safe owner object of these attributes. This property is identical to the DocObject property.
- `Parent` (IGH_Attributes, get/set) — Gets or sets the parent attributes. Top level attributes do not have parents.
- `PathName` (String, get) — Get a description of the location of these attributes within the local attribute stack.
- `Pivot` (PointF, get/set) — Gets or sets the pivot for these attributes. The pivot controls the general placement of the attributes. If you want to move the attributes, change the pivot location.
- `Selected` (Boolean, get/set) — Gets or sets the selected state of the top-level attributes.
- `SizingBorders` (Padding, get) — 
- `TooltipEnabled` (Boolean, get) — Gets the tooltip enabled value. If False, no further tooltip functions will be called.

