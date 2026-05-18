---
name: tekla-tekla-structures-model-ui
description: This skill encodes the tekla 2026.0 surface of the Tekla.Structures.Model.UI namespace — 24 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ClipBox, ClipPlane, Color, ClipPlaneCollection, GraphicPolyLine, GraphicsDrawer, ModelObjectSelector, Mesh, and 16 more types.
---

# Tekla.Structures.Model.UI

Auto-generated from vendor docs for tekla 2026.0. 24 types in this namespace.

## ClipBox (class)

The ClipBox class groups multiple clip planes under a single identifier.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/ab38dbfe-74bb-ea8f-28a5-d7df4b3a4393)

### Constructors
- `ClipBox(...)` — Initializes a new instance of the ClipBox class with 6 clip planes.

### Methods
#### `Delete(...)`

Deletes the clip box from the application view. The application view must be visible.

[Docs](https://developer.tekla.com/topic/en/18/47/e0b710e1-ea5b-4e8f-1c48-3910136d2e63)

#### `Insert(...)`

Creates a new clip box to the application view. The application view must be visible.

[Docs](https://developer.tekla.com/topic/en/18/47/cbd05c59-7ef5-514e-22f8-49ef5f7ce7db)

#### `Modify(...)`

Modifies the clip box position in the application view. The application view must be visible.

[Docs](https://developer.tekla.com/topic/en/18/47/b12bea25-5e8e-46b8-0fb0-cd5c0cf1a017)

### Properties
- `ClipPlanes` (object, get/set) — Gets or sets the clip planes contained by this clip box.
- `View` (object, get/set) — Gets the view the clip box belongs to.

## ClipPlane (class)

The ClipPlane class defines a clip plane which can be used together with a visible view.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/40d3dd42-53da-9673-f4e1-e4179f76dab0)

### Constructors
- `ClipPlane(...)` — Initializes a new instance of the ClipPlane class.

### Methods
#### `Delete(...)`

Deletes the clip plane from the application view. The application view must be visible.

[Docs](https://developer.tekla.com/topic/en/18/47/6eebebcb-48d6-608d-0dff-8b12c0ccf3ff)

#### `Insert(...)`

Creates a new clip plane to the application view. The application view must be visible.

[Docs](https://developer.tekla.com/topic/en/18/47/645247ba-2e22-de99-6df9-2f3ff7569531)

#### `Modify(...)`

Modifies the clip plane position in the application view. The application view must be visible.

[Docs](https://developer.tekla.com/topic/en/18/47/b9b9e337-16d4-22c1-5d05-a8ed788cfcc7)

### Properties
- `ClipPlaneID` (object, get/set) — Gets the clip plane ID.
- `IsBorderVisible` (object, get/set) — Gets or sets a value indicating whether the clip plane border is visible.
- `IsScissorVisible` (object, get/set) — Gets or sets a value indicating whether the clip plane scissor icon is visible.
- `Location` (object, get/set) — The clip plane location in global coordinates (XYZ).
- `UpVector` (object, get/set) — The clip plane up vector in global coordinates (XYZ).
- `View` (object, get/set) — The view the clip plane belongs to.

## ClipPlaneCollection (class)

The ClipPlaneCollection class handles the collection of the clip planes.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/61f3cae0-4db3-2065-f576-1c27d4d48f26)

### Methods
#### `CopyTo(...)`

Copies the elements of the ICollection to an Array, starting at a particular Array index.

[Docs](https://developer.tekla.com/topic/en/18/47/bd115bad-d53a-1fd5-ba78-1caab791cded)

#### `GetEnumerator(...)`

Returns an enumerator that iterates through a collection.

[Docs](https://developer.tekla.com/topic/en/18/47/2f45c0e9-3626-4010-87c5-1d4e27fe8506)

### Properties
- `Count` (object, get/set) — Gets the number of elements contained in the ICollection.
- `IsSynchronized` (object, get/set) — Gets a value indicating whether access to the ICollection is synchronized (thread safe).
- `SyncRoot` (object, get/set) — Gets an object that can be used to synchronize access to the ICollection.

## Color (class)

The Color class represents an RGB color with transparency. The color values must be between 0.0 and 1.0.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c726c4b0-94af-e2ca-cf89-c7795b8d4d7c)

### Constructors
- `Color(...)` — Creates a new color object. The default object is black.
- `Color(...)` — Creates a new color object with the given values.
- `Color(...)` — Creates a new color object with the given values.

### Properties
- `Blue` (object, get/set) — The blue value of the color, between 0.0 and 1.0.
- `Green` (object, get/set) — The green value of the color, between 0.0 and 1.0.
- `Red` (object, get/set) — The red value of the color, between 0.0 and 1.0.
- `Transparency` (object, get/set) — The transparency value of the color, between 0.0 (completely see-through) and 1.0 (completely solid).

## GraphicPolyLine (class)

(No description provided in vendor docs for Tekla.Structures.Model.UI.GraphicPolyLine.)

[Vendor docs](https://developer.tekla.com/topic/en/18/47/6bdfc7b9-9538-09a9-5536-1c7f879a71ec)

### Constructors
- `GraphicPolyLine(...)` — Create a black line with width 1 and solid line type.
- `GraphicPolyLine(...)` — Create a graphic polyline object with 0 points.
- `GraphicPolyLine(...)` — Create a graphic polyline object.

### Properties
- `Color` (object, get/set) — The line color.
- `PolyLine` (object, get/set) — The path of the line.
- `Type` (object, get/set) — The appearance of the line. For instance solid, dashed or dotted.
- `Width` (object, get/set) — The line width in pixels. NOTE: Currently the valid width values are 1, 2 and 4

## GraphicPolyLine.LineType (enum)

The appearance of the line. For instance solid, dashed or dotted.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/8ec1e804-dee5-66f5-5005-d52a8e7534b2)

### Values
- `Solid` = `1` — The solid line type.
- `Dashed1` = `2` — The first dashed line type.
- `Dashed2` = `3` — The second dashed line type.
- `DashedAndDotted` = `4` — The dashed and dotted line type.
- `Dotted` = `5` — The dotted line type.

## GraphicsDrawer (class)

The GraphicsDrawer class draws temporary graphics in the currently active rendered view in Tekla Structures.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/8092772c-d824-bb09-119a-66be50652cdd)

### Constructors
- `GraphicsDrawer(...)` — Creates a new graphics drawer instance.

### Methods
#### `DrawLineSegment(LineSegment, Color)(...)`

Draws a temporary line segment in the currently active rendered view.

[Docs](https://developer.tekla.com/topic/en/18/47/47539009-0382-e285-cb13-897763966564)

#### `DrawLineSegment(Point, Point, Color)(...)`

Draws a temporary line segment in the currently active rendered view.

[Docs](https://developer.tekla.com/topic/en/18/47/1dbfa523-0c96-1182-67f9-6b97cb659d4c)

#### `DrawMeshLines(...)`

Draws the mesh lines as temporary graphics in the currently active Tekla Structures rendered view.

[Docs](https://developer.tekla.com/topic/en/18/47/51ab9888-d8b4-5567-e892-800ccc2421cb)

#### `DrawMeshSurface(...)`

Draws the mesh surface as temporary graphics in the currently active Tekla Structures rendered view. The counterclockwise sides of the mesh triangles are drawn.

[Docs](https://developer.tekla.com/topic/en/18/47/e84b3995-55be-c1ab-5d03-6fe1cfa88910)

#### `DrawPolyLine(...)`

Draw a temporary polyline into all open Tekla Structures views.

[Docs](https://developer.tekla.com/topic/en/18/47/97b4045b-be65-c41a-32bc-740e4d96d702)

#### `DrawText(...)`

Draws a temporary text in the currently active rendered view.

[Docs](https://developer.tekla.com/topic/en/18/47/b52f67f7-5eab-6845-165e-28d1fd7f05df)

#### `DrawTextToView(...)`

Draws a temporary text to the specified view. You can set the view Identifier.ID = 0 for current active view.

[Docs](https://developer.tekla.com/topic/en/18/47/df22dab0-8c2f-24dd-8c0c-2296f00bed91)

#### `RemoveTemporaryGraphicsObject(...)`

Remove a temporary graphic object from all views.

[Docs](https://developer.tekla.com/topic/en/18/47/ed6abf2c-2620-83d6-8236-184adcb99bc6)

#### `RemoveTemporaryGraphicsObjects(...)`

Remove a list of temporary graphic objects from all views.

[Docs](https://developer.tekla.com/topic/en/18/47/a7735db7-bffa-c22a-cb4b-2af599a0c709)

## Mesh (class)

The Mesh class represents a mesh for drawing three-dimensional data.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9eed1ebf-ebb2-6585-332d-92b29e940c47)

### Constructors
- `Mesh(...)` — Creates a new empty mesh instance.
- `Mesh(...)` — Creates a new mesh instance with the given points, triangles and lines. Does not check the indices in the given triangles and lines array lists for correctness.

### Methods
#### `AddLine(...)`

Adds a new line in the mesh.

[Docs](https://developer.tekla.com/topic/en/18/47/de40b8cb-02a7-71d7-1799-a6c0523599a2)

#### `AddPoint(...)`

Adds a new point in the mesh.

[Docs](https://developer.tekla.com/topic/en/18/47/0c6131d0-41bc-1a0a-e823-7275f1be56ea)

#### `AddTriangle(...)`

Adds a new triangle in the mesh.

[Docs](https://developer.tekla.com/topic/en/18/47/4f77e473-ca53-1251-8c4e-e0d4c52b9b37)

### Properties
- `Lines` (object, get/set) — An array list of indices (as integers) pointing to the points array list. Two consecutive indices always represent a single line.
- `Points` (object, get/set) — An array list of point objects representing the mesh points.
- `Triangles` (object, get/set) — An array list of indices (as integers) pointing to the points array list. Three consecutive indices always represent a single triangle.

## ModelObjectSelector (class)

The ModelObjectSelector class can be used to select objects from the Tekla Structures user interface. Currently, these selections both select the objects from the database and highlight them visually.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d1c10655-d2b0-2c0f-5bf4-16bdc2b18ca7)

### Constructors
- `ModelObjectSelector(...)` — Creates a new model object selector instance.

### Methods
#### `GetObjectsByBoundingBox(...)`

Returns an enumerator of the given view's visible model objects that collide with the given geometrical bounding box. Note that this method uses approximate bounding boxes and thus is NOT EXACT, and may return objects not necessarily colliding with the given box but only being somewhere near to it.

[Docs](https://developer.tekla.com/topic/en/18/47/0f1d00ed-8269-6cd5-0403-81aa5206b9cd)

#### `GetSelectedObjects(...)`

Returns an enumerator of all the selected model objects in the model view.

[Docs](https://developer.tekla.com/topic/en/18/47/a03063f4-8140-c465-1e84-95fbc0291f52)

#### `Select(ArrayList)(...)`

Selects a list of objects from the user interface.

[Docs](https://developer.tekla.com/topic/en/18/47/739e39e9-0c4f-e37d-88a5-9fb6fdff508d)

#### `Select(ArrayList, Boolean)(...)`

Selects a list of objects from the user interface.

[Docs](https://developer.tekla.com/topic/en/18/47/faf6cfd2-be15-3cec-6dd1-efd77cf5938f)

## ModelObjectVisualization (class)

The class to set and clear temporary visualization (color and transparency) for model objects in view. Permanent representation will be restored when view is redrawn or temporary visualization is cleared. Can be used also to fetch current permanent representation of model object.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/b2abbdce-e25f-de50-7597-bb5d32024ca2)

### Methods
#### `ClearAllTemporaryStates(...)`

Clears temporary visualization from all model objects and restores permanent representation.

[Docs](https://developer.tekla.com/topic/en/18/47/46e12a40-2eeb-7017-5030-d4f181c03700)

#### `ClearTemporaryStates(...)`

Clears temporary visualization from model objects and restores permanent representation.

[Docs](https://developer.tekla.com/topic/en/18/47/33556963-de00-6812-42b1-f9915c3f0671)

#### `GetAllTemporaryColored(...)`

Gets all temporary colored objects and their color

[Docs](https://developer.tekla.com/topic/en/18/47/3eb07452-fe80-8792-4784-d1c60233971a)

#### `GetRepresentation(...)`

Gets current permanent representation (red, blue, green and transparency) of given model object

[Docs](https://developer.tekla.com/topic/en/18/47/e50d139f-8488-c3ac-59c6-9e75b44ac75d)

#### `GetTempRepresentation(...)`

Gets the temporary representation (red, blue, green and transparency) of given model object

[Docs](https://developer.tekla.com/topic/en/18/47/dd1423f6-244d-f32c-ea02-638b7138b190)

#### `SetTemporaryState(List.Identifier., Color)(...)`

Sets visualization temporary state (red, blue, green and transparency) for given model object identifiers

[Docs](https://developer.tekla.com/topic/en/18/47/283dae38-1f8e-91a7-4bb6-6766c0cb48b4)

#### `SetTemporaryState(List.ModelObject., Color)(...)`

Sets visualization temporary state (red, blue, green and transparency) for given model objects

[Docs](https://developer.tekla.com/topic/en/18/47/4dff8ec4-a069-0729-ad48-b368608af476)

#### `SetTemporaryStateForAll(...)`

Sets temporary visualization state (red, blue, green and transparency) for all model objects

[Docs](https://developer.tekla.com/topic/en/18/47/0eb20b32-3c7f-3c69-db53-87bb751d23ac)

#### `SetTransparency(List.Identifier., TemporaryTransparency)(...)`

Restores permanent representation settings and sets temporary transparency for given model object identifiers

[Docs](https://developer.tekla.com/topic/en/18/47/205146e9-e809-6b1d-2557-85889ce28a67)

#### `SetTransparency(List.ModelObject., TemporaryTransparency)(...)`

Restores permanent representation settings and sets temporary transparency for given model objects

[Docs](https://developer.tekla.com/topic/en/18/47/fa5656a1-4dc3-bd98-7d72-cfc405834e42)

#### `SetTransparencyForAll(...)`

Restores permanent representation settings and sets temporary transparency for all model objects

[Docs](https://developer.tekla.com/topic/en/18/47/7b7f6b08-77c6-3c4a-88c0-d0dfd4ceaa47)

#### `SetVisibility(...)`

Sets visibility for given model objects.

[Docs](https://developer.tekla.com/topic/en/18/47/24fd3a11-5699-9408-bccd-0f50155a6d82)

#### `UnhideParts(...)`

Unide all parts in the model.

[Docs](https://developer.tekla.com/topic/en/18/47/3ecd7a0e-f4e0-e8ac-36b1-73da625bfcaf)

## ModelViewEnumerator (class)

The ModelViewEnumerator class is an enumerator class for model views. The enumerator enables model view items to be looped.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/04866e15-0045-c9a0-640a-6e74e32b4816)

### Methods
#### `MoveNext(...)`

Moves to the next view in the enumerator.

[Docs](https://developer.tekla.com/topic/en/18/47/d8d9f6e5-b4f1-6625-26e7-c1cc98edac57)

#### `Reset(...)`

Resets the enumerator.

[Docs](https://developer.tekla.com/topic/en/18/47/f0907ea0-c4f0-ac68-6a87-d3c37260e066)

### Properties
- `Count` (object, get/set) — Returns the number of views in the enumerator.
- `Current` (object, get/set) — Returns the current view.
- `CurrentViewId` (object, get/set) — Gets the current view identifier.

## PickInput (class)

The PickInput class handles the input of picked objects and positions.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2e208f87-8875-9304-4428-7e885b0a6d11)

### Methods
#### `CopyTo(...)`

Copies the elements of the ICollection to an Array, starting at a particular Array index.

[Docs](https://developer.tekla.com/topic/en/18/47/7e74c797-6176-4be3-498a-b159effccf56)

#### `GetEnumerator(...)`

Returns an enumerator that iterates through a collection.

[Docs](https://developer.tekla.com/topic/en/18/47/b36e7354-ce73-6445-48e1-1b28b6a1adfa)

### Properties
- `Count` (object, get/set) — Gets the number of elements contained in the ICollection.
- `IsSynchronized` (object, get/set) — Gets a value indicating whether access to the ICollection is synchronized (thread safe).
- `SyncRoot` (object, get/set) — Gets an object that can be used to synchronize access to the ICollection.

## Picker (class)

The Picker class can be used to query the user to do manual picks of objects and points from the Tekla Structures model. The methods throw an exception if the user interrupts (cancels) the pick command.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/cb1e9d5a-d5bc-21c7-44c2-8d2ed7be833b)

### Constructors
- `Picker(...)` — Creates a new picker instance.

### Methods
#### `PickFace(String)(...)`

Queries the user to pick a face from the model with the given prompt.

[Docs](https://developer.tekla.com/topic/en/18/47/cff69c8b-cd58-440a-12ec-13070f79791b)

#### `PickFace.(...)`

Queries the user to pick a face from the model.

[Docs](https://developer.tekla.com/topic/en/18/47/2ab33130-77eb-ba4d-ac7c-75bcfd28b879)

#### `PickLine(String)(...)`

Queries the user to pick a line from the model with the given prompt.

[Docs](https://developer.tekla.com/topic/en/18/47/b657d3cb-2227-9b0c-774c-869a0ad84dda)

#### `PickLine.(...)`

Queries the user to pick a line from the model.

[Docs](https://developer.tekla.com/topic/en/18/47/b62ed31c-938a-9d36-8940-ca61bafaa0d0)

#### `PickObject(Picker.PickObjectEnum)(...)`

Queries the user to pick one model object from the model.

[Docs](https://developer.tekla.com/topic/en/18/47/b7912521-6faa-9028-79b6-6fb12374a80f)

#### `PickObject(Picker.PickObjectEnum, String)(...)`

Queries the user to pick one model object from the model with the given prompt.

[Docs](https://developer.tekla.com/topic/en/18/47/3648eb9a-9367-cda9-8c88-87b772e032b3)

#### `PickObjects(Picker.PickObjectsEnum)(...)`

Queries the user to pick model objects from the model.

[Docs](https://developer.tekla.com/topic/en/18/47/9af65bd8-dff0-142c-0f7a-0a1eb3341c29)

#### `PickObjects(Picker.PickObjectsEnum, String)(...)`

Queries the user to pick model objects from the model with the given prompt.

[Docs](https://developer.tekla.com/topic/en/18/47/18d61ce6-d498-ef54-9367-032b7ec75162)

#### `PickPoint(Point)(...)`

Queries the user to pick a point from the model with reference to the given point.

[Docs](https://developer.tekla.com/topic/en/18/47/be6c66b5-0281-3cc3-2409-4b69845fb7a9)

#### `PickPoint(String)(...)`

Queries the user to pick a point from the model with the given prompt.

[Docs](https://developer.tekla.com/topic/en/18/47/c77fb6d5-b730-2beb-6f94-dd3e5efe1eb9)

#### `PickPoint(String, Point)(...)`

Queries the user to pick point with given prompt and reference to another point.

[Docs](https://developer.tekla.com/topic/en/18/47/6ef22e29-4b51-ed37-924a-7685879cdf76)

#### `PickPoint.(...)`

Queries the user to pick a point from the model.

[Docs](https://developer.tekla.com/topic/en/18/47/7512c677-bc34-36b9-6b2c-f21055dd3a90)

#### `PickPoints(Picker.PickPointEnum)(...)`

Queries the user to pick points from the model.

[Docs](https://developer.tekla.com/topic/en/18/47/f944c4c1-63d3-71cd-97cd-bd4409db7942)

#### `PickPoints(Picker.PickPointEnum, String)(...)`

Queries the user to pick points from the model with the given prompt and reference to another point.

[Docs](https://developer.tekla.com/topic/en/18/47/206fbceb-330c-c253-0df1-d8bc1b20d1ea)

## Picker.PickObjectEnum (enum)

The possible model object pick types for a single object.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a753a5ec-dca7-2bf0-480f-061d24ca0cc3)

### Values
- `PICK_ONE_OBJECT` = `0` — Pick one model object of any type.
- `PICK_ONE_PART` = `1` — Pick one part.
- `PICK_ONE_WELD` = `2` — Pick one welding.
- `PICK_ONE_BOLTGROUP` = `3` — Pick one bolt group.
- `PICK_ONE_REINFORCEMENT` = `4` — Pick one reinforcement.

## Picker.PickObjectsEnum (enum)

The possible model object pick types for many objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1c0dbcdb-bae2-7720-b597-51504c46dddf)

### Values
- `PICK_N_OBJECTS` = `0` — Pick multiple model objects of any type.
- `PICK_N_PARTS` = `1` — Pick multiple parts.
- `PICK_N_WELDS` = `2` — Pick multiple welds.
- `PICK_N_BOLTGROUPS` = `3` — Pick multiple bolt groups.
- `PICK_N_REINFORCEMENTS` = `4` — Pick multiple reinforcements.

## Picker.PickPointEnum (enum)

The possible point pick types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/6ffaab17-269a-80be-16f4-efb03354d6f4)

### Values
- `PICK_ONE_POINT` = `0` — Pick one point (x,y,z).
- `PICK_TWO_POINTS` = `1` — Pick two points.
- `PICK_POLYGON` = `2` — Pick multiple points.
- `PICK_LINE` = `3` — Pick one line.
- `PICK_FACE` = `4` — Pick one face.

## TemporaryTransparency (enum)

The possible temporary transparencies. Used to temporarily change transparency of a model object to visualize some state in the model.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9504d3d1-7079-60a2-a399-d920c6492cb7)

### Values
- `HIDDEN` = `0` — Object is hidden.
- `TRANSPARENT` = `1` — Object is transparent.
- `SEMITRANSPARENT` = `3` — Object is semi transparent.
- `SEMIVISIBLE` = `5` — Object is almost fully visible.
- `VISIBLE` = `10` — Object is fully visible.

## View (class)

The View class contains methods related to views.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/073c170c-326e-c26b-ff9c-4bfaa0b04055)

### Constructors
- `View(...)` — Initializes a new instance of the View class

### Methods
#### `CreateSnapshot(...)`



[Docs](https://developer.tekla.com/topic/en/18/47/daf1eedd-c9c4-77fd-53e7-ab93c84fd26f)

#### `Delete(...)`

Deletes the view.

[Docs](https://developer.tekla.com/topic/en/18/47/373b64d1-a896-1729-5baa-c9873e621574)

#### `GetClipPlanes(...)`

Returns all the clip planes of the view.

[Docs](https://developer.tekla.com/topic/en/18/47/56c48d87-ad03-15b1-8aa8-56c29b9352ae)

#### `Insert(...)`

Inserts a new view to the model.

[Docs](https://developer.tekla.com/topic/en/18/47/02a81550-35a1-d76e-dfe0-5a682bf66ae6)

#### `IsPerspectiveViewProjection(...)`

Tells whether the view projection is a perspective projection.

[Docs](https://developer.tekla.com/topic/en/18/47/9d0d48ec-75ea-3798-8912-55db56d4d897)

#### `IsVisible(...)`

Tells whether the view is currently visible.

[Docs](https://developer.tekla.com/topic/en/18/47/017d834b-5c34-1442-bb49-ab67fcc48c80)

#### `Modify(...)`

Modifies the view parameters.

[Docs](https://developer.tekla.com/topic/en/18/47/999bd78f-0586-ace8-02d8-bc4ffeab1ee0)

#### `Select(...)`

Selects the view.

[Docs](https://developer.tekla.com/topic/en/18/47/a3880c31-bba7-5304-200c-642c88898e21)

### Properties
- `CurrentRepresentation` (object, get/set) — Gets or sets the current representation.
- `DisplayCoordinateSystem` (object, get/set) — The local coordinate system for the display.
- `DisplayType` (object, get/set) — The display plane type.
- `Identifier` (object, get/set) — The view identifier.
- `Name` (object, get/set) — The name of the view. 84 characters at most.
- `SharedView` (object, get/set) — The property which tells if the view is shared. The value is meaningful only when the model is shared.
- `ViewCoordinateSystem` (object, get/set) — The local coordinate system for the view. Can be set when creating a view. The origin is always set to zero.
- `ViewDepthDown` (object, get/set) — The view depth down.
- `ViewDepthUp` (object, get/set) — The view depth up.
- `ViewFilter` (object, get/set) — The name of the view filter. 256 characters at most.
- `ViewProjection` (object, get/set) — The view projection type.
- `ViewRendering` (object, get/set) — The view rendering type.
- `VisibilitySettings` (object, get/set) — The property which tells the view visibility settings for objects.
- `WorkArea` (object, get/set) — The view working area.

## View.DisplayOrientationType (enum)

The view plane types. The plane type can be changed.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c84772c0-1496-84bb-8b31-da24ed2ebab4)

### Values
- `DISPLAY_VIEW_PLANE` = `0` — The display orientation is in the view plane defined by the view coordinate system.
- `DISPLAY_3D` = `1` — The display orientation is in 3D defined by the display coordinate system.

## View.ViewProjectionType (enum)

The view projection types. The projection type can be read but not changed.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9ce44417-bc61-a3fb-a171-a31c0521a6e6)

### Values
- `ORTHOGONAL_PROJECTION` = `0` — The orthogonal view projection type.
- `PERSPECTIVE_PROJECTION` = `1` — The perspective view projection type.

## View.ViewRenderingType (enum)

The view rendering types. Only rendered views can be modified.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d557ee40-145a-970a-7ed2-b5921a08d959)

### Values
- `WIREFRAME_VIEW` = `0` — The wireframe view type.
- `RENDERED_VIEW` = `1` — The rendered view type.

## ViewCamera (class)

The ViewCamera class defines a camera which can be used together with a visible view. Always supply a properly orthogonalized camera up vector when rotating the camera.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/fb51cafc-b3f3-36b6-d022-453d1f1d6da3)

### Constructors
- `ViewCamera(...)` — Instantiates a new view camera instance with zero length vectors.

### Methods
#### `Modify(...)`

Updates the camera to the application view.

[Docs](https://developer.tekla.com/topic/en/18/47/25b308fd-2fdb-630f-ef81-dbe854987574)

#### `Select(...)`

Updates the camera parameters from the application view.

[Docs](https://developer.tekla.com/topic/en/18/47/32e5ccf7-e831-0b1f-6bdc-f27f9d9c5a3b)

### Properties
- `DirectionVector` (object, get/set) — The camera direction (controls the camera pan and tilt).
- `FieldOfView` (object, get/set) — The camera field of view as an angle (degrees) in the perspective view.
- `Location` (object, get/set) — The camera location in global coordinates (XYZ).
- `UpVector` (object, get/set) — The camera up vector (controls the camera roll).
- `View` (object, get/set) — The view where the camera belongs to.
- `ZoomFactor` (object, get/set) — The camera zoom factor (meter/pixel) in the orthogonal view.

## ViewHandler (class)

The ViewHandler class contains methods for handling views.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/d65d9ef6-7f02-5524-6054-15a0126614e7)

### Constructors
- `ViewHandler(...)` — Initializes a new instance of the ViewHandler class

### Methods
#### `GetActiveView(...)`

Get the current active MDI child window view

[Docs](https://developer.tekla.com/topic/en/18/47/0a00b23a-98fd-37b3-9bc3-9616d63a2c47)

#### `GetAllViews(...)`

Fetches all the views from the model.

[Docs](https://developer.tekla.com/topic/en/18/47/0caa8a9f-c41c-0d38-fdb4-70d46f656bd9)

#### `GetPermanentViews(...)`

Returns the permanent views.

[Docs](https://developer.tekla.com/topic/en/18/47/843d23db-2a34-56d7-abfb-792b5fe57558)

#### `GetSelectedViews(...)`

Returns the views the user has selected.

[Docs](https://developer.tekla.com/topic/en/18/47/99f504e2-d095-f9c4-4852-870dd721f35e)

#### `GetTemporaryViews(...)`

Returns the temporary views.

[Docs](https://developer.tekla.com/topic/en/18/47/c33af31d-4e77-1ed5-baac-bb270bd4af2b)

#### `GetVisibleViews(...)`

Returns the visible views.

[Docs](https://developer.tekla.com/topic/en/18/47/8ea2983b-ed89-6d3f-5fef-f08de2397e6f)

#### `HideView(...)`

Closes a model view.

[Docs](https://developer.tekla.com/topic/en/18/47/0f04ff24-f332-4456-c8eb-41f24566efec)

#### `RedrawView(...)`

Redraws a model view.

[Docs](https://developer.tekla.com/topic/en/18/47/2bf7baba-1ff8-00c3-012d-cb8023362c85)

#### `RedrawWorkplane(...)`

Sets the workplane to current transformation plane and redraws it.

[Docs](https://developer.tekla.com/topic/en/18/47/13a65fe3-f0fc-1cef-233f-48c5c471de84)

#### `SetActiveView(...)`

Set the current active MDI child window view.

[Docs](https://developer.tekla.com/topic/en/18/47/16f16693-48c5-ab07-62f1-49c302fafad1)

#### `SetRepresentation(...)`

Sets the object representation.

[Docs](https://developer.tekla.com/topic/en/18/47/54ce1cb1-f8e3-4544-7558-fbf7f200fd49)

#### `ShowView(...)`

Opens a model view.

[Docs](https://developer.tekla.com/topic/en/18/47/ad1a67d1-4535-d9fd-c4ef-6f8855c4e934)

#### `ZoomToBoundingBox(AABB)(...)`

Zooms the current model view.

[Docs](https://developer.tekla.com/topic/en/18/47/cabd4471-1f0a-285c-b4ed-688a935b7c17)

#### `ZoomToBoundingBox(View, AABB)(...)`

Zooms a model view.

[Docs](https://developer.tekla.com/topic/en/18/47/5554e2b7-1f8f-5b6c-37e3-b7217202a73c)

## ViewVisibilitySettings (class)

The View visibility settings class contains object visibility information related to view.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9edf7de3-5e73-1e02-d13c-0d50f9714cbc)

### Constructors
- `ViewVisibilitySettings(...)` — Initializes a new instance of the ViewVisibilitySettings class

### Properties
- `BoltHolesVisible` (object, get/set) — Gets or sets the bolt hole visibility.
- `BoltHolesVisibleInComponents` (object, get/set) — Gets or sets the bolt hole visibility inside components.
- `BoltsVisible` (object, get/set) — Gets or sets the bolt visibility.
- `BoltsVisibleInComponents` (object, get/set) — Gets or sets the bolt visibility inside components.
- `BuildingVisible` (object, get/set) — Gets or sets the building heirarchy visibility.
- `ComponentsVisible` (object, get/set) — Gets or sets the component symbol visibility.
- `ComponentsVisibleInComponents` (object, get/set) — Gets or sets the component symbol visibility inside components.
- `ConstructionLinesVisible` (object, get/set) — Gets or sets the construction line visibility.
- `ConstructionPlanesVisible` (object, get/set) — Gets or sets the construction plane visibility.
- `ConstructionPlanesVisibleInComponents` (object, get/set) — Gets or sets the construction plane visibility inside components.
- `CutsVisible` (object, get/set) — Gets or sets the cut or union visibility.
- `CutsVisibleInComponents` (object, get/set) — Gets or sets the cut visibility inside components.
- `FittingsVisible` (object, get/set) — Gets or sets the fitting visibility.
- `FittingsVisibleInComponents` (object, get/set) — Gets or sets the fitting visibility inside components.
- `GridsVisible` (object, get/set) — Gets or sets the grid visibility.
- `LoadsVisible` (object, get/set) — Gets or sets the load visibility.
- `PartsVisible` (object, get/set) — Gets or sets the part visibility.
- `PartsVisibleInComponents` (object, get/set) — Gets or sets the part visibility inside components.
- `PointsVisible` (object, get/set) — Gets or sets the point visibility.
- `PointsVisibleInComponents` (object, get/set) — Gets or sets the point visibility inside components.
- `PourBreaksVisible` (object, get/set) — Gets or sets the pour break visibility. NOTE! this setting requires that XS_ENABLE_POUR_MANAGEMENT is set to true.
- `PoursVisible` (object, get/set) — Gets or sets the pour visibility. NOTE! this setting requires that XS_ENABLE_POUR_MANAGEMENT is set to true.
- `RebarsVisible` (object, get/set) — Gets or sets the reinforcement visibility.
- `RebarsVisibleInComponents` (object, get/set) — Gets or sets the reinforcement visibility inside components.
- `ReferenceObjectsVisible` (object, get/set) — Gets or sets the reference object visibility.
- `SurfaceTreatmentsVisible` (object, get/set) — Gets or sets the surface treatment visibility.
- `WeldsVisible` (object, get/set) — Gets or sets the weld visibility.
- `WeldsVisibleInComponents` (object, get/set) — Gets or sets the weld visibility inside components.

