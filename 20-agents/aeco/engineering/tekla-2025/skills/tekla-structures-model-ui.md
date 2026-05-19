---
name: tekla-tekla-structures-model-ui
description: This skill encodes the tekla 2025.0 surface of the Tekla.Structures.Model.UI namespace — 23 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ClipPlane, ClipPlaneCollection, Color, GraphicPolyLine, GraphicsDrawer, Mesh, ModelObjectSelector, ModelObjectVisualization, and 15 more types.
---

# Tekla.Structures.Model.UI

Auto-generated from vendor docs for tekla 2025.0. 23 types in this namespace.

## ClipPlane (class)

The ClipPlane class defines a clip plane which can be used together with a visible view.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/40d3dd42-53da-9673-f4e1-e4179f76dab0)

### Constructors
- `public ClipPlane()` — Initializes a new instance of the ClipPlane class.

### Methods
#### `public bool Delete()`

Deletes the clip plane from the application view. The application view must be visible.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/6eebebcb-48d6-608d-0dff-8b12c0ccf3ff)

#### `public bool Insert()`

Creates a new clip plane to the application view. The application view must be visible.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/645247ba-2e22-de99-6df9-2f3ff7569531)

#### `public bool Modify()`

Modifies the clip plane position in the application view. The application view must be visible.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/b9b9e337-16d4-22c1-5d05-a8ed788cfcc7)

### Properties
- `ClipPlaneID` (Int32, get) — Gets the clip plane ID.
- `IsBorderVisible` (Boolean, get/set) — Gets or sets a value indicating whether the clip plane border is visible.
- `IsScissorVisible` (Boolean, get/set) — Gets or sets a value indicating whether the clip plane scissor icon is visible.
- `Location` (Point, get/set) — The clip plane location in global coordinates (XYZ).
- `UpVector` (Vector, get/set) — The clip plane up vector in global coordinates (XYZ).
- `View` (View, get/set) — The view the clip plane belongs to.

## ClipPlaneCollection (class)

The ClipPlaneCollection class handles the collection of the clip planes.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/61f3cae0-4db3-2065-f576-1c27d4d48f26)

### Methods
#### `public void CopyTo(Array array, int index)`

Copies the elements of the ICollection to an Array, starting at a particular Array index.

**Parameters:**
- `array` (System.Array) — The one-dimensional Array that is the destination of the elements copied from the ICollection. The Array must have zero-based indexing.
- `index` (System.Int32) — The zero-based index in the array at which copying begins.

[Docs](https://developer.tekla.com/topic/en/18/43/bd115bad-d53a-1fd5-ba78-1caab791cded)

#### `public IEnumerator GetEnumerator()`

Returns an enumerator that iterates through a collection.

**Returns:** `IEnumerator` — An IEnumerator object that can be used to iterate through the collection.

[Docs](https://developer.tekla.com/topic/en/18/43/2f45c0e9-3626-4010-87c5-1d4e27fe8506)

### Properties
- `Count` (Int32, get) — Gets the number of elements contained in the ICollection.
- `IsSynchronized` (Boolean, get) — Gets a value indicating whether access to the ICollection is synchronized (thread safe).
- `SyncRoot` (Object, get) — Gets an object that can be used to synchronize access to the ICollection.

## Color (class)

The Color class represents an RGB color with transparency. The color values must be between 0.0 and 1.0.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/c726c4b0-94af-e2ca-cf89-c7795b8d4d7c)

### Constructors
- `public Color()` — Creates a new color object. The default object is black.
- `public Color(double Red, double Green, double Blue)` — Creates a new color object with the given values.
- `public Color(double Red, double Green, double Blue, double Transparency)` — Creates a new color object with the given values.

### Properties
- `Blue` (Double, get/set) — The blue value of the color, between 0.0 and 1.0.
- `Green` (Double, get/set) — The green value of the color, between 0.0 and 1.0.
- `Red` (Double, get/set) — The red value of the color, between 0.0 and 1.0.
- `Transparency` (Double, get/set) — The transparency value of the color, between 0.0 (completely see-through) and 1.0 (completely solid).

## GraphicPolyLine (class)

(No description provided in vendor docs for Tekla.Structures.Model.UI.GraphicPolyLine.)

[Vendor docs](https://developer.tekla.com/topic/en/18/43/6bdfc7b9-9538-09a9-5536-1c7f879a71ec)

### Constructors
- `public GraphicPolyLine()` — Create a black line with width 1 and solid line type.
- `public GraphicPolyLine(Color color, int width, GraphicPolyLine.LineType type)` — Create a graphic polyline object with 0 points.
- `public GraphicPolyLine(PolyLine polyLine, Color color, int width, GraphicPolyLine.LineType type)` — Create a graphic polyline object.

### Properties
- `Color` (Color, get/set) — The line color.
- `PolyLine` (PolyLine, get/set) — The path of the line.
- `Type` (GraphicPolyLine.LineType, get/set) — The appearance of the line. For instance solid, dashed or dotted.
- `Width` (Int32, get/set) — The line width in pixels. NOTE: Currently the valid width values are 1, 2 and 4

## GraphicPolyLine.LineType (enum)

The appearance of the line. For instance solid, dashed or dotted.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/8ec1e804-dee5-66f5-5005-d52a8e7534b2)

### Values
- `Solid` = `1` — The solid line type.
- `Dashed1` = `2` — The first dashed line type.
- `Dashed2` = `3` — The second dashed line type.
- `DashedAndDotted` = `4` — The dashed and dotted line type.
- `Dotted` = `5` — The dotted line type.

## GraphicsDrawer (class)

The GraphicsDrawer class draws temporary graphics in the currently active rendered view in Tekla Structures.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/8092772c-d824-bb09-119a-66be50652cdd)

### Constructors
- `public GraphicsDrawer()` — Creates a new graphics drawer instance.

### Methods
#### `public bool DrawLineSegment(LineSegment LineSegment, Color Color)`

Draws a temporary line segment in the currently active rendered view.

**Parameters:**
- `LineSegment` (Tekla.Structures.Geometry3d.LineSegment) — The line to draw.
- `Color` (Tekla.Structures.Model.UI.Color) — The color of the line. The transparency value of the color is ignored.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/47539009-0382-e285-cb13-897763966564)

#### `public bool DrawLineSegment(Point Point1, Point Point2, Color Color)`

Draws a temporary line segment in the currently active rendered view.

**Parameters:**
- `Point1` (Tekla.Structures.Geometry3d.Point) — The first point of the line.
- `Point2` (Tekla.Structures.Geometry3d.Point) — The second point of the line.
- `Color` (Tekla.Structures.Model.UI.Color) — The color of the line. The transparency value of the color is ignored.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/1dbfa523-0c96-1182-67f9-6b97cb659d4c)

#### `public bool DrawMeshLines(Mesh Mesh, Color Color)`

Draws the mesh lines as temporary graphics in the currently active Tekla Structures rendered view.

**Parameters:**
- `Mesh` (Tekla.Structures.Model.UI.Mesh) — The mesh to draw.
- `Color` (Tekla.Structures.Model.UI.Color) — The color of the mesh lines. The transparency value of the color is ignored.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/51ab9888-d8b4-5567-e892-800ccc2421cb)

#### `public bool DrawMeshSurface(Mesh Mesh, Color Color)`

Draws the mesh surface as temporary graphics in the currently active Tekla Structures rendered view. The counterclockwise sides of the mesh triangles are drawn.

**Parameters:**
- `Mesh` (Tekla.Structures.Model.UI.Mesh) — The mesh to draw.
- `Color` (Tekla.Structures.Model.UI.Color) — The color of the mesh.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/e84b3995-55be-c1ab-5d03-6fe1cfa88910)

#### `public int DrawPolyLine(GraphicPolyLine GraphicPolyLine)`

Draw a temporary polyline into all open Tekla Structures views.

**Parameters:**
- `GraphicPolyLine` (Tekla.Structures.Model.UI.GraphicPolyLine) — The polyline to draw.

**Returns:** `Int32` — The identifier of the temporary graphic. You can delete the graphic with RemoveTemporaryGraphicsObject.

[Docs](https://developer.tekla.com/topic/en/18/43/97b4045b-be65-c41a-32bc-740e4d96d702)

#### `public bool DrawText(Point Location, string Text, Color Color)`

Draws a temporary text in the currently active rendered view.

**Parameters:**
- `Location` (Tekla.Structures.Geometry3d.Point) — The location of the text's top-left corner.
- `Text` (System.String) — The text to be drawn. The maximum length is 511 characters.
- `Color` (Tekla.Structures.Model.UI.Color) — The color of the text. The transparency value of the color is ignored.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/b52f67f7-5eab-6845-165e-28d1fd7f05df)

#### `public int DrawTextToView(Point Location, string Text, Color Color, View View)`

Draws a temporary text to the specified view. You can set the view Identifier.ID = 0 for current active view.

**Parameters:**
- `Location` (Tekla.Structures.Geometry3d.Point) — The location of the text's top-left corner.
- `Text` (System.String) — The text to be drawn. The maximum length is 511 characters.
- `Color` (Tekla.Structures.Model.UI.Color) — The color of the text. The transparency value of the color is ignored.
- `View` (Tekla.Structures.Model.UI.View) — The model view (ID = 0 for current active view)

**Returns:** `Int32` — The id of the inserted text.

[Docs](https://developer.tekla.com/topic/en/18/43/df22dab0-8c2f-24dd-8c0c-2296f00bed91)

#### `public void RemoveTemporaryGraphicsObject(int GraphicObjectID)`

Remove a temporary graphic object from all views.

**Parameters:**
- `GraphicObjectID` (System.Int32) — The identifier that was returned when the object was created.

[Docs](https://developer.tekla.com/topic/en/18/43/ed6abf2c-2620-83d6-8236-184adcb99bc6)

#### `public void RemoveTemporaryGraphicsObjects(IEnumerable GraphicObjectIDs)`

Remove a list of temporary graphic objects from all views.

**Parameters:**
- `GraphicObjectIDs` (System.Collections.IEnumerable) — The identifiers that was returned when the object was created.

[Docs](https://developer.tekla.com/topic/en/18/43/a7735db7-bffa-c22a-cb4b-2af599a0c709)

## Mesh (class)

The Mesh class represents a mesh for drawing three-dimensional data.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/9eed1ebf-ebb2-6585-332d-92b29e940c47)

### Constructors
- `public Mesh()` — Creates a new empty mesh instance.
- `public Mesh(ArrayList Points, ArrayList Triangles, ArrayList Lines)` — Creates a new mesh instance with the given points, triangles and lines. Does not check the indices in the given triangles and lines array lists for correctness.

### Methods
#### `public void AddLine(int Index1, int Index2)`

Adds a new line in the mesh.

**Parameters:**
- `Index1` (System.Int32) — The index of the line's start point in the points array list.
- `Index2` (System.Int32) — The index of the line's end point in the points array list.

[Docs](https://developer.tekla.com/topic/en/18/43/de40b8cb-02a7-71d7-1799-a6c0523599a2)

#### `public int AddPoint(Point Point)`

Adds a new point in the mesh.

**Parameters:**
- `Point` (Tekla.Structures.Geometry3d.Point) — The point to be added, must not be null.

**Returns:** `Int32` — The index of the added point.

[Docs](https://developer.tekla.com/topic/en/18/43/0c6131d0-41bc-1a0a-e823-7275f1be56ea)

#### `public void AddTriangle(int Index1, int Index2, int Index3)`

Adds a new triangle in the mesh.

**Parameters:**
- `Index1` (System.Int32) — The index of the triangle's first point in the points array list.
- `Index2` (System.Int32) — The index of the triangle's second point in the points array list.
- `Index3` (System.Int32) — The index of the triangle's third point in the points array list.

[Docs](https://developer.tekla.com/topic/en/18/43/4f77e473-ca53-1251-8c4e-e0d4c52b9b37)

### Properties
- `Lines` (ArrayList, get) — An array list of indices (as integers) pointing to the points array list. Two consecutive indices always represent a single line.
- `Points` (ArrayList, get) — An array list of point objects representing the mesh points.
- `Triangles` (ArrayList, get) — An array list of indices (as integers) pointing to the points array list. Three consecutive indices always represent a single triangle.

## ModelObjectSelector (class)

The ModelObjectSelector class can be used to select objects from the Tekla Structures user interface. Currently, these selections both select the objects from the database and highlight them visually.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/d1c10655-d2b0-2c0f-5bf4-16bdc2b18ca7)

### Constructors
- `public ModelObjectSelector()` — Creates a new model object selector instance.

### Methods
#### `public ModelObjectEnumerator GetObjectsByBoundingBox(Point MinPoint, Point MaxPoint, View View)`

Returns an enumerator of the given view's visible model objects that collide with the given geometrical bounding box. Note that this method uses approximate bounding boxes and thus is NOT EXACT, and may return objects not necessarily colliding with the given box but only being somewhere near to it.

**Parameters:**
- `MinPoint` (Tekla.Structures.Geometry3d.Point) — The minimum point of the bounding box.
- `MaxPoint` (Tekla.Structures.Geometry3d.Point) — The maximum point of the bounding box.
- `View` (Tekla.Structures.Model.UI.View) — The view to get the objects from.

**Returns:** `ModelObjectEnumerator` — A model object enumerator of the visible model objects colliding with the given bounding box.

[Docs](https://developer.tekla.com/topic/en/18/43/0f1d00ed-8269-6cd5-0403-81aa5206b9cd)

#### `public ModelObjectEnumerator GetSelectedObjects()`

Returns an enumerator of all the selected model objects in the model view.

**Returns:** `ModelObjectEnumerator` — A model object enumerator of all the selected model objects.

[Docs](https://developer.tekla.com/topic/en/18/43/a03063f4-8140-c465-1e84-95fbc0291f52)

#### `public bool Select(ArrayList ModelObjects)`

Selects a list of objects from the user interface.

**Parameters:**
- `ModelObjects` (System.Collections.ArrayList) — The list of model objects to select.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/739e39e9-0c4f-e37d-88a5-9fb6fdff508d)

#### `public bool Select(ArrayList ModelObjects, bool ShowDimensions)`

Selects a list of objects from the user interface.

**Parameters:**
- `ModelObjects` (System.Collections.ArrayList) — The list of model objects to select.
- `ShowDimensions` (System.Boolean) — Defines whether to show dimensions of the parts in the selection.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/faf6cfd2-be15-3cec-6dd1-efd77cf5938f)

## ModelObjectVisualization (class)

The class to set and clear temporary visualization (color and transparency) for model objects in view. Permanent representation will be restored when view is redrawn or temporary visualization is cleared. Can be used also to fetch current permanent representation of model object.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/b2abbdce-e25f-de50-7597-bb5d32024ca2)

### Methods
#### `public static bool ClearAllTemporaryStates()`

Clears temporary visualization from all model objects and restores permanent representation.

**Returns:** `Boolean` — True if the permanent representation could be restored.

[Docs](https://developer.tekla.com/topic/en/18/43/46e12a40-2eeb-7017-5030-d4f181c03700)

#### `public static bool ClearTemporaryStates(List<ModelObject> modelObjects)`

Clears temporary visualization from model objects and restores permanent representation.

**Parameters:**
- `modelObjects` (System.Collections.Generic.List<ModelObject>)

**Returns:** `Boolean` — True if the permanent representation could be restored.

[Docs](https://developer.tekla.com/topic/en/18/43/33556963-de00-6812-42b1-f9915c3f0671)

#### `public static IEnumerable<KeyValuePair<Identifier, Color>> GetAllTemporaryColored()`

Gets all temporary colored objects and their color

**Returns:** `IEnumerable<KeyValuePair<Identifier,Color>>` — Enumerable of Identifier-Color pairs

[Docs](https://developer.tekla.com/topic/en/18/43/3eb07452-fe80-8792-4784-d1c60233971a)

#### `public static bool GetRepresentation(ModelObject modelObject, ref Color color)`

Gets current permanent representation (red, blue, green and transparency) of given model object

**Parameters:**
- `modelObject` (Tekla.Structures.Model.ModelObject) — The model object to check.
- `color` (Tekla.Structures.Model.UI.Color.) — The color of given model object.

**Returns:** `Boolean` — True if the permanent representation could be fetched.

[Docs](https://developer.tekla.com/topic/en/18/43/e50d139f-8488-c3ac-59c6-9e75b44ac75d)

#### `public static Color GetTempRepresentation(ModelObject modelObject)`

Gets the temporary representation (red, blue, green and transparency) of given model object

**Parameters:**
- `modelObject` (Tekla.Structures.Model.ModelObject) — The model object to check.

**Returns:** `Color` — The temporary representation or null if not set.

[Docs](https://developer.tekla.com/topic/en/18/43/dd1423f6-244d-f32c-ea02-638b7138b190)

#### `public static bool SetTemporaryState(List<Identifier> identifiers, Color color)`

Sets visualization temporary state (red, blue, green and transparency) for given model object identifiers

**Parameters:**
- `identifiers` (System.Collections.Generic.List<Identifier>) — The list of model object identifiers.
- `color` (Tekla.Structures.Model.UI.Color) — The temporary color for given model objects.

**Returns:** `Boolean` — True if the temporary state could be set.

[Docs](https://developer.tekla.com/topic/en/18/43/283dae38-1f8e-91a7-4bb6-6766c0cb48b4)

#### `public static bool SetTemporaryState(List<ModelObject> modelObjects, Color color)`

Sets visualization temporary state (red, blue, green and transparency) for given model objects

**Parameters:**
- `modelObjects` (System.Collections.Generic.List<ModelObject>) — The list of model objects.
- `color` (Tekla.Structures.Model.UI.Color) — The temporary state for given model objects.

**Returns:** `Boolean` — True if the temporary state could be set.

[Docs](https://developer.tekla.com/topic/en/18/43/4dff8ec4-a069-0729-ad48-b368608af476)

#### `public static bool SetTemporaryStateForAll(Color color)`

Sets temporary visualization state (red, blue, green and transparency) for all model objects

**Parameters:**
- `color` (Tekla.Structures.Model.UI.Color) — The temporary state for all model objects.

**Returns:** `Boolean` — True if the temporary state could be set.

[Docs](https://developer.tekla.com/topic/en/18/43/0eb20b32-3c7f-3c69-db53-87bb751d23ac)

#### `public static bool SetTransparency(List<Identifier> identifiers, TemporaryTransparency transparency)`

Restores permanent representation settings and sets temporary transparency for given model object identifiers

**Parameters:**
- `identifiers` (System.Collections.Generic.List<Identifier>) — The list of model object identifiers.
- `transparency` (Tekla.Structures.Model.UI.TemporaryTransparency) — The temporary transparency for given objects.

**Returns:** `Boolean` — True if the temporary transparency could be set.

[Docs](https://developer.tekla.com/topic/en/18/43/205146e9-e809-6b1d-2557-85889ce28a67)

#### `public static bool SetTransparency(List<ModelObject> modelObjects, TemporaryTransparency transparency)`

Restores permanent representation settings and sets temporary transparency for given model objects

**Parameters:**
- `modelObjects` (System.Collections.Generic.List<ModelObject>) — The list of model objects.
- `transparency` (Tekla.Structures.Model.UI.TemporaryTransparency) — The temporary transparency for given objects.

**Returns:** `Boolean` — True if the temporary transparency could be set.

[Docs](https://developer.tekla.com/topic/en/18/43/fa5656a1-4dc3-bd98-7d72-cfc405834e42)

#### `public static bool SetTransparencyForAll(TemporaryTransparency transparency)`

Restores permanent representation settings and sets temporary transparency for all model objects

**Parameters:**
- `transparency` (Tekla.Structures.Model.UI.TemporaryTransparency) — The temporary transparency for all model objects.

**Returns:** `Boolean` — True if the temporary transparency could be set.

[Docs](https://developer.tekla.com/topic/en/18/43/7b7f6b08-77c6-3c4a-88c0-d0dfd4ceaa47)

#### `public static bool SetVisibility(List<ModelObject> modelObjects, bool? Visible)`

Sets visibility for given model objects.

**Parameters:**
- `modelObjects` (System.Collections.Generic.List<ModelObject>) — The list of model objects, null for all objects
- `Visible` (System.Nullable<Boolean>) — Nullable boolean if to set to visible or reset state

**Returns:** `Boolean` — 

[Docs](https://developer.tekla.com/topic/en/18/43/24fd3a11-5699-9408-bccd-0f50155a6d82)

## ModelViewEnumerator (class)

The ModelViewEnumerator class is an enumerator class for model views. The enumerator enables model view items to be looped.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/04866e15-0045-c9a0-640a-6e74e32b4816)

### Methods
#### `public bool MoveNext()`

Moves to the next view in the enumerator.

**Returns:** `Boolean` — False on failure.

[Docs](https://developer.tekla.com/topic/en/18/43/d8d9f6e5-b4f1-6625-26e7-c1cc98edac57)

#### `public void Reset()`

Resets the enumerator.

[Docs](https://developer.tekla.com/topic/en/18/43/f0907ea0-c4f0-ac68-6a87-d3c37260e066)

### Properties
- `Count` (Int32, get) — Returns the number of views in the enumerator.
- `Current` (View, get) — Returns the current view.
- `CurrentViewId` (Int32, get) — Gets the current view identifier.

## PickInput (class)

The PickInput class handles the input of picked objects and positions.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/2e208f87-8875-9304-4428-7e885b0a6d11)

### Methods
#### `public void CopyTo(Array array, int index)`

Copies the elements of the ICollection to an Array, starting at a particular Array index.

**Parameters:**
- `array` (System.Array) — The one-dimensional Array that is the destination of the elements copied from the ICollection. The Array must have zero-based indexing.
- `index` (System.Int32) — The zero-based index in the array at which copying begins.

[Docs](https://developer.tekla.com/topic/en/18/43/7e74c797-6176-4be3-498a-b159effccf56)

#### `public IEnumerator GetEnumerator()`

Returns an enumerator that iterates through a collection.

**Returns:** `IEnumerator` — An IEnumerator object that can be used to iterate through the collection.

[Docs](https://developer.tekla.com/topic/en/18/43/b36e7354-ce73-6445-48e1-1b28b6a1adfa)

### Properties
- `Count` (Int32, get) — Gets the number of elements contained in the ICollection.
- `IsSynchronized` (Boolean, get) — Gets a value indicating whether access to the ICollection is synchronized (thread safe).
- `SyncRoot` (Object, get) — Gets an object that can be used to synchronize access to the ICollection.

## Picker (class)

The Picker class can be used to query the user to do manual picks of objects and points from the Tekla Structures model. The methods throw an exception if the user interrupts (cancels) the pick command.

**Remarks:** Prompts Tekla Structures prefixes the given prompt with "prompt_" and looks for a translation in the prompts.ail file. If the translation (e.g. "prompt_Pick_first_position") is not found in the prompts.ail file, the prompt string is displayed as such. This feature can be used to give already translated strings to the picker.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/cb1e9d5a-d5bc-21c7-44c2-8d2ed7be833b)

### Constructors
- `public Picker()` — Creates a new picker instance.

### Methods
#### `public PickInput PickFace()`

Queries the user to pick a face from the model.

**Returns:** `PickInput` — A list of the vertices of the face and the object the user picked as one PickInput instance.

[Docs](https://developer.tekla.com/topic/en/18/43/2ab33130-77eb-ba4d-ac7c-75bcfd28b879)

#### `public PickInput PickFace(string Prompt)`

Queries the user to pick a face from the model with the given prompt.

**Parameters:**
- `Prompt` (System.String) — The string to display as user guidance.

**Returns:** `PickInput` — A list of the vertices of the face and the object the user picked as one PickInput instance.

[Docs](https://developer.tekla.com/topic/en/18/43/cff69c8b-cd58-440a-12ec-13070f79791b)

#### `public ArrayList PickLine()`

Queries the user to pick a line from the model.

**Returns:** `ArrayList` — A list of points the user picked.

[Docs](https://developer.tekla.com/topic/en/18/43/b62ed31c-938a-9d36-8940-ca61bafaa0d0)

#### `public ArrayList PickLine(string Prompt)`

Queries the user to pick a line from the model with the given prompt.

**Parameters:**
- `Prompt` (System.String) — The string to display as user guidance.

**Returns:** `ArrayList` — A list of points the user picked.

[Docs](https://developer.tekla.com/topic/en/18/43/b657d3cb-2227-9b0c-774c-869a0ad84dda)

#### `public ModelObject PickObject(Picker.PickObjectEnum Enum)`

Queries the user to pick one model object from the model.

**Parameters:**
- `Enum` (Tekla.Structures.Model.UI.Picker.PickObjectEnum) — Indicates the type of objects to pick.

**Returns:** `ModelObject` — One model object instance.

[Docs](https://developer.tekla.com/topic/en/18/43/b7912521-6faa-9028-79b6-6fb12374a80f)

#### `public ModelObject PickObject(Picker.PickObjectEnum Enum, string Prompt)`

Queries the user to pick one model object from the model with the given prompt.

**Parameters:**
- `Enum` (Tekla.Structures.Model.UI.Picker.PickObjectEnum) — Indicates the type of objects to pick.
- `Prompt` (System.String) — The string to display as user guidance.

**Returns:** `ModelObject` — One model object instance.

[Docs](https://developer.tekla.com/topic/en/18/43/3648eb9a-9367-cda9-8c88-87b772e032b3)

#### `public ModelObjectEnumerator PickObjects(Picker.PickObjectsEnum Enum)`

Queries the user to pick model objects from the model.

**Parameters:**
- `Enum` (Tekla.Structures.Model.UI.Picker.PickObjectsEnum) — Indicates the type and amount of objects to pick.

**Returns:** `ModelObjectEnumerator` — An enumerator of model object instances.

[Docs](https://developer.tekla.com/topic/en/18/43/9af65bd8-dff0-142c-0f7a-0a1eb3341c29)

#### `public ModelObjectEnumerator PickObjects(Picker.PickObjectsEnum Enum, string Prompt)`

Queries the user to pick model objects from the model with the given prompt.

**Parameters:**
- `Enum` (Tekla.Structures.Model.UI.Picker.PickObjectsEnum) — Indicates the type and amount of objects to pick.
- `Prompt` (System.String) — The string to display as user guidance.

**Returns:** `ModelObjectEnumerator` — An enumerator of model object instances.

[Docs](https://developer.tekla.com/topic/en/18/43/18d61ce6-d498-ef54-9367-032b7ec75162)

#### `public Point PickPoint()`

Queries the user to pick a point from the model.

**Returns:** `Point` — The point the user picked.

[Docs](https://developer.tekla.com/topic/en/18/43/7512c677-bc34-36b9-6b2c-f21055dd3a90)

#### `public Point PickPoint(Point ReferencePoint)`

Queries the user to pick a point from the model with reference to the given point.

**Parameters:**
- `ReferencePoint` (Tekla.Structures.Geometry3d.Point) — The point to be used as reference.

**Returns:** `Point` — The point the user picked.

[Docs](https://developer.tekla.com/topic/en/18/43/be6c66b5-0281-3cc3-2409-4b69845fb7a9)

#### `public Point PickPoint(string Prompt)`

Queries the user to pick a point from the model with the given prompt.

**Parameters:**
- `Prompt` (System.String) — The string to display as user guidance.

**Returns:** `Point` — The point the user picked.

[Docs](https://developer.tekla.com/topic/en/18/43/c77fb6d5-b730-2beb-6f94-dd3e5efe1eb9)

#### `public Point PickPoint(string Prompt, Point ReferencePoint)`

Queries the user to pick point with given prompt and reference to another point.

**Parameters:**
- `Prompt` (System.String) — The string to display as user guidance.
- `ReferencePoint` (Tekla.Structures.Geometry3d.Point) — The point to be used as reference.

**Returns:** `Point` — The point the user picked.

[Docs](https://developer.tekla.com/topic/en/18/43/6ef22e29-4b51-ed37-924a-7685879cdf76)

#### `public ArrayList PickPoints(Picker.PickPointEnum Enum)`

Queries the user to pick points from the model.

**Parameters:**
- `Enum` (Tekla.Structures.Model.UI.Picker.PickPointEnum) — Indicates how many points the user must pick.

**Returns:** `ArrayList` — A list of points the user picked.

[Docs](https://developer.tekla.com/topic/en/18/43/f944c4c1-63d3-71cd-97cd-bd4409db7942)

#### `public ArrayList PickPoints(Picker.PickPointEnum Enum, string Prompt)`

Queries the user to pick points from the model with the given prompt and reference to another point.

**Parameters:**
- `Enum` (Tekla.Structures.Model.UI.Picker.PickPointEnum) — Indicates how many points the user must pick.
- `Prompt` (System.String) — The string to display as user guidance.

**Returns:** `ArrayList` — A list of points the user picked.

[Docs](https://developer.tekla.com/topic/en/18/43/206fbceb-330c-c253-0df1-d8bc1b20d1ea)

## Picker.PickObjectEnum (enum)

The possible model object pick types for a single object.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/a753a5ec-dca7-2bf0-480f-061d24ca0cc3)

### Values
- `PICK_ONE_OBJECT` = `0` — Pick one model object of any type.
- `PICK_ONE_PART` = `1` — Pick one part.
- `PICK_ONE_WELD` = `2` — Pick one welding.
- `PICK_ONE_BOLTGROUP` = `3` — Pick one bolt group.
- `PICK_ONE_REINFORCEMENT` = `4` — Pick one reinforcement.

## Picker.PickObjectsEnum (enum)

The possible model object pick types for many objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/1c0dbcdb-bae2-7720-b597-51504c46dddf)

### Values
- `PICK_N_OBJECTS` = `0` — Pick multiple model objects of any type.
- `PICK_N_PARTS` = `1` — Pick multiple parts.
- `PICK_N_WELDS` = `2` — Pick multiple welds.
- `PICK_N_BOLTGROUPS` = `3` — Pick multiple bolt groups.
- `PICK_N_REINFORCEMENTS` = `4` — Pick multiple reinforcements.

## Picker.PickPointEnum (enum)

The possible point pick types.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/6ffaab17-269a-80be-16f4-efb03354d6f4)

### Values
- `PICK_ONE_POINT` = `0` — Pick one point (x,y,z).
- `PICK_TWO_POINTS` = `1` — Pick two points.
- `PICK_POLYGON` = `2` — Pick multiple points.
- `PICK_LINE` = `3` — Pick one line.
- `PICK_FACE` = `4` — Pick one face.

## TemporaryTransparency (enum)

The possible temporary transparencies. Used to temporarily change transparency of a model object to visualize some state in the model.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/9504d3d1-7079-60a2-a399-d920c6492cb7)

### Values
- `HIDDEN` = `0` — Object is hidden.
- `TRANSPARENT` = `1` — Object is transparent.
- `SEMITRANSPARENT` = `3` — Object is semi transparent.
- `SEMIVISIBLE` = `5` — Object is almost fully visible.
- `VISIBLE` = `10` — Object is fully visible.

## View (class)

The View class contains methods related to views.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/073c170c-326e-c26b-ff9c-4bfaa0b04055)

### Constructors
- `public View()` — Initializes a new instance of the View class

### Methods
#### `public bool CreateSnapshot(string filePath, SnapshotSettings snapshotSettings, bool overwrite = true)`



**Parameters:**
- `filePath` (System.String)
- `snapshotSettings` (SnapshotSettings)
- `overwrite` (System.Boolean)

**Returns:** `Boolean` — 

[Docs](https://developer.tekla.com/topic/en/18/43/daf1eedd-c9c4-77fd-53e7-ab93c84fd26f)

#### `public bool Delete()`

Deletes the view.

**Returns:** `Boolean` — True if the operation was successful.

[Docs](https://developer.tekla.com/topic/en/18/43/373b64d1-a896-1729-5baa-c9873e621574)

#### `public ClipPlaneCollection GetClipPlanes()`

Returns all the clip planes of the view.

**Returns:** `ClipPlaneCollection` — The clip planes of the view.

[Docs](https://developer.tekla.com/topic/en/18/43/56c48d87-ad03-15b1-8aa8-56c29b9352ae)

#### `public bool Insert()`

Inserts a new view to the model.

**Returns:** `Boolean` — True if the operation was successful.

[Docs](https://developer.tekla.com/topic/en/18/43/02a81550-35a1-d76e-dfe0-5a682bf66ae6)

#### `public bool IsPerspectiveViewProjection()`

Tells whether the view projection is a perspective projection.

**Returns:** `Boolean` — True if the view projection type is perspective projection.

[Docs](https://developer.tekla.com/topic/en/18/43/9d0d48ec-75ea-3798-8912-55db56d4d897)

#### `public bool IsVisible()`

Tells whether the view is currently visible.

**Returns:** `Boolean` — True if the view is visible.

[Docs](https://developer.tekla.com/topic/en/18/43/017d834b-5c34-1442-bb49-ab67fcc48c80)

#### `public bool Modify()`

Modifies the view parameters.

**Returns:** `Boolean` — True if the operation was successful.

[Docs](https://developer.tekla.com/topic/en/18/43/999bd78f-0586-ace8-02d8-bc4ffeab1ee0)

#### `public bool Select()`

Selects the view.

**Returns:** `Boolean` — True if the operation was successful.

[Docs](https://developer.tekla.com/topic/en/18/43/a3880c31-bba7-5304-200c-642c88898e21)

### Properties
- `CurrentRepresentation` (String, get/set) — Gets or sets the current representation.
- `DisplayCoordinateSystem` (CoordinateSystem, get/set) — The local coordinate system for the display.
- `DisplayType` (View.DisplayOrientationType, get/set) — The display plane type.
- `Identifier` (Identifier, get/set) — The view identifier.
- `Name` (String, get/set) — The name of the view. 84 characters at most.
- `SharedView` (Boolean, get/set) — The property which tells if the view is shared. The value is meaningful only when the model is shared.
- `ViewCoordinateSystem` (CoordinateSystem, get/set) — The local coordinate system for the view. Can be set when creating a view. The origin is always set to zero.
- `ViewDepthDown` (Double, get/set) — The view depth down.
- `ViewDepthUp` (Double, get/set) — The view depth up.
- `ViewFilter` (String, get/set) — The name of the view filter. 256 characters at most.
- `ViewProjection` (View.ViewProjectionType, get/set) — The view projection type.
- `ViewRendering` (View.ViewRenderingType, get) — The view rendering type.
- `VisibilitySettings` (ViewVisibilitySettings, get/set) — The property which tells the view visibility settings for objects.
- `WorkArea` (AABB, get/set) — The view working area.

## View.DisplayOrientationType (enum)

The view plane types. The plane type can be changed.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/c84772c0-1496-84bb-8b31-da24ed2ebab4)

### Values
- `DISPLAY_VIEW_PLANE` = `0` — The display orientation is in the view plane defined by the view coordinate system.
- `DISPLAY_3D` = `1` — The display orientation is in 3D defined by the display coordinate system.

## View.ViewProjectionType (enum)

The view projection types. The projection type can be read but not changed.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/9ce44417-bc61-a3fb-a171-a31c0521a6e6)

### Values
- `ORTHOGONAL_PROJECTION` = `0` — The orthogonal view projection type.
- `PERSPECTIVE_PROJECTION` = `1` — The perspective view projection type.

## View.ViewRenderingType (enum)

The view rendering types. Only rendered views can be modified.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/d557ee40-145a-970a-7ed2-b5921a08d959)

### Values
- `WIREFRAME_VIEW` = `0` — The wireframe view type.
- `RENDERED_VIEW` = `1` — The rendered view type.

## ViewCamera (class)

The ViewCamera class defines a camera which can be used together with a visible view. Always supply a properly orthogonalized camera up vector when rotating the camera.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/fb51cafc-b3f3-36b6-d022-453d1f1d6da3)

### Constructors
- `public ViewCamera()` — Instantiates a new view camera instance with zero length vectors.

### Methods
#### `public bool Modify()`

Updates the camera to the application view.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/25b308fd-2fdb-630f-ef81-dbe854987574)

#### `public bool Select()`

Updates the camera parameters from the application view.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/32e5ccf7-e831-0b1f-6bdc-f27f9d9c5a3b)

### Properties
- `DirectionVector` (Vector, get/set) — The camera direction (controls the camera pan and tilt).
- `FieldOfView` (Double, get/set) — The camera field of view as an angle (degrees) in the perspective view.
- `Location` (Point, get/set) — The camera location in global coordinates (XYZ).
- `UpVector` (Vector, get/set) — The camera up vector (controls the camera roll).
- `View` (View, get/set) — The view where the camera belongs to.
- `ZoomFactor` (Double, get/set) — The camera zoom factor (meter/pixel) in the orthogonal view.

## ViewHandler (class)

The ViewHandler class contains methods for handling views.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/d65d9ef6-7f02-5524-6054-15a0126614e7)

### Constructors
- `public ViewHandler()` — Initializes a new instance of the ViewHandler class

### Methods
#### `public static View GetActiveView()`

Get the current active MDI child window view

**Returns:** `View` — currently active view

[Docs](https://developer.tekla.com/topic/en/18/43/0a00b23a-98fd-37b3-9bc3-9616d63a2c47)

#### `public static ModelViewEnumerator GetAllViews()`

Fetches all the views from the model.

**Returns:** `ModelViewEnumerator` — A model view enumerator with all the views.

[Docs](https://developer.tekla.com/topic/en/18/43/0caa8a9f-c41c-0d38-fdb4-70d46f656bd9)

#### `public static ModelViewEnumerator GetPermanentViews()`

Returns the permanent views.

**Returns:** `ModelViewEnumerator` — A model view enumerator with the permanent views.

[Docs](https://developer.tekla.com/topic/en/18/43/843d23db-2a34-56d7-abfb-792b5fe57558)

#### `public static ModelViewEnumerator GetSelectedViews()`

Returns the views the user has selected.

**Returns:** `ModelViewEnumerator` — A model view enumerator with the selected views.

[Docs](https://developer.tekla.com/topic/en/18/43/99f504e2-d095-f9c4-4852-870dd721f35e)

#### `public static ModelViewEnumerator GetTemporaryViews()`

Returns the temporary views.

**Returns:** `ModelViewEnumerator` — A model view enumerator with the temporary views.

[Docs](https://developer.tekla.com/topic/en/18/43/c33af31d-4e77-1ed5-baac-bb270bd4af2b)

#### `public static ModelViewEnumerator GetVisibleViews()`

Returns the visible views.

**Returns:** `ModelViewEnumerator` — A model view enumerator with the visible views.

[Docs](https://developer.tekla.com/topic/en/18/43/8ea2983b-ed89-6d3f-5fef-f08de2397e6f)

#### `public static bool HideView(View view)`

Closes a model view.

**Parameters:**
- `view` (Tekla.Structures.Model.UI.View) — The view that is to be closed.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/0f04ff24-f332-4456-c8eb-41f24566efec)

#### `public static bool RedrawView(View view)`

Redraws a model view.

**Parameters:**
- `view` (Tekla.Structures.Model.UI.View) — The view that is to be redrawn.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/2bf7baba-1ff8-00c3-012d-cb8023362c85)

#### `public static bool RedrawWorkplane()`

Sets the workplane to current transformation plane and redraws it.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/13a65fe3-f0fc-1cef-233f-48c5c471de84)

#### `public static bool SetRepresentation(string Representation)`

Sets the object representation.

**Parameters:**
- `Representation` (System.String) — The representation that is to be used. 257 characters at most.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/54ce1cb1-f8e3-4544-7558-fbf7f200fd49)

#### `public static bool ShowView(View view)`

Opens a model view.

**Parameters:**
- `view` (Tekla.Structures.Model.UI.View) — The view that is to be shown.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/ad1a67d1-4535-d9fd-c4ef-6f8855c4e934)

#### `public static bool ZoomToBoundingBox(AABB box)`

Zooms the current model view.

**Parameters:**
- `box` (Tekla.Structures.Geometry3d.AABB) — The bounding box the current view is zoomed to.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/cabd4471-1f0a-285c-b4ed-688a935b7c17)

#### `public static bool ZoomToBoundingBox(View view, AABB B)`

Zooms a model view.

**Parameters:**
- `view` (Tekla.Structures.Model.UI.View) — The view that is to be zoomed.
- `B` (Tekla.Structures.Geometry3d.AABB) — The bounding box the view is zoomed to.

**Returns:** `Boolean` — True on success.

[Docs](https://developer.tekla.com/topic/en/18/43/5554e2b7-1f8f-5b6c-37e3-b7217202a73c)

## ViewVisibilitySettings (class)

The View visibility settings class contains object visibility information related to view.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/9edf7de3-5e73-1e02-d13c-0d50f9714cbc)

### Constructors
- `public ViewVisibilitySettings()` — Initializes a new instance of the ViewVisibilitySettings class

### Properties
- `BoltHolesVisible` (Boolean, get/set) — Gets or sets the bolt hole visibility.
- `BoltHolesVisibleInComponents` (Boolean, get/set) — Gets or sets the bolt hole visibility inside components.
- `BoltsVisible` (Boolean, get/set) — Gets or sets the bolt visibility.
- `BoltsVisibleInComponents` (Boolean, get/set) — Gets or sets the bolt visibility inside components.
- `BuildingVisible` (Boolean, get/set) — Gets or sets the building heirarchy visibility.
- `ComponentsVisible` (Boolean, get/set) — Gets or sets the component symbol visibility.
- `ComponentsVisibleInComponents` (Boolean, get/set) — Gets or sets the component symbol visibility inside components.
- `ConstructionLinesVisible` (Boolean, get/set) — Gets or sets the construction line visibility.
- `ConstructionPlanesVisible` (Boolean, get/set) — Gets or sets the construction plane visibility.
- `ConstructionPlanesVisibleInComponents` (Boolean, get/set) — Gets or sets the construction plane visibility inside components.
- `CutsVisible` (Boolean, get/set) — Gets or sets the cut or union visibility.
- `CutsVisibleInComponents` (Boolean, get/set) — Gets or sets the cut visibility inside components.
- `FittingsVisible` (Boolean, get/set) — Gets or sets the fitting visibility.
- `FittingsVisibleInComponents` (Boolean, get/set) — Gets or sets the fitting visibility inside components.
- `GridsVisible` (Boolean, get/set) — Gets or sets the grid visibility.
- `LoadsVisible` (Boolean, get/set) — Gets or sets the load visibility.
- `PartsVisible` (Boolean, get/set) — Gets or sets the part visibility.
- `PartsVisibleInComponents` (Boolean, get/set) — Gets or sets the part visibility inside components.
- `PointsVisible` (Boolean, get/set) — Gets or sets the point visibility.
- `PointsVisibleInComponents` (Boolean, get/set) — Gets or sets the point visibility inside components.
- `PourBreaksVisible` (Boolean, get/set) — Gets or sets the pour break visibility. NOTE! this setting requires that XS_ENABLE_POUR_MANAGEMENT is set to true.
- `PoursVisible` (Boolean, get/set) — Gets or sets the pour visibility. NOTE! this setting requires that XS_ENABLE_POUR_MANAGEMENT is set to true.
- `RebarsVisible` (Boolean, get/set) — Gets or sets the reinforcement visibility.
- `RebarsVisibleInComponents` (Boolean, get/set) — Gets or sets the reinforcement visibility inside components.
- `ReferenceObjectsVisible` (Boolean, get/set) — Gets or sets the reference object visibility.
- `SurfaceTreatmentsVisible` (Boolean, get/set) — Gets or sets the surface treatment visibility.
- `WeldsVisible` (Boolean, get/set) — Gets or sets the weld visibility.
- `WeldsVisibleInComponents` (Boolean, get/set) — Gets or sets the weld visibility inside components.

