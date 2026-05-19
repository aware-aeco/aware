---
name: rhino-rhino-ui-controls-thumbnailui
description: This skill encodes the rhino 7.0 surface of the Rhino.UI.Controls.ThumbnailUI namespace — 9 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ThumbData, Thumbnail, ThumbnailViewModelFactory, IRhRdkContentThumbnail, IRhRdkThumbnail, IRhRdkThumbnailList, IRhRdkContentThumbnailList_Sizes, IRhRdkThumbnailList_Modes, and 1 more types.
---

# Rhino.UI.Controls.ThumbnailUI

Auto-generated from vendor docs for rhino 7.0. 9 types in this namespace.

## IRhRdkContentThumbnail (interface)

[Missing <summary> documentation for "T:Rhino.UI.Controls.ThumbnailUI.IRhRdkContentThumbnail"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_UI_Controls_ThumbnailUI_IRhRdkContentThumbnail.htm)

### Methods
#### `RenderContent ChildContent()`



**Returns:** `RenderContent` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkContentThumbnail.ChildContent"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkContentThumbnail_ChildContent.htm)

#### `void Dib(ref Bitmap dibOut)`

(Inherited from IRhRdkThumbnail.)

**Parameters:**
- `dibOut` (System.Drawing.Bitmap) — [Missing <param name="dibOut"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnail.Dib(System.Drawing.Bitmap@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnail_Dib.htm)

#### `Bitmap GetDib()`

(Inherited from IRhRdkThumbnail.)

**Returns:** `Bitmap` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnail.GetDib"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnail_GetDib.htm)

#### `void GetDisplayRect(ref RectangleF rectOut)`

(Inherited from IRhRdkThumbnail.)

**Parameters:**
- `rectOut` (System.Drawing.RectangleF) — [Missing <param name="rectOut"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnail.GetDisplayRect(System.Drawing.RectangleF@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnail_GetDisplayRect.htm)

#### `Guid GroupId()`



**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkContentThumbnail.GroupId"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkContentThumbnail_GroupId.htm)

#### `Guid Id()`

(Inherited from IRhRdkThumbnail.)

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnail.Id"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnail_Id.htm)

#### `bool IsHot()`

(Inherited from IRhRdkThumbnail.)

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnail.IsHot"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnail_IsHot.htm)

#### `bool IsSelected()`

(Inherited from IRhRdkThumbnail.)

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnail.IsSelected"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnail_IsSelected.htm)

#### `string Label()`

(Inherited from IRhRdkThumbnail.)

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnail.Label"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnail_Label.htm)

#### `RenderContent TopLevelContent()`



**Returns:** `RenderContent` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkContentThumbnail.TopLevelContent"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkContentThumbnail_TopLevelContent.htm)

## IRhRdkContentThumbnailList_Sizes (enum)

[Missing <summary> documentation for "T:Rhino.UI.Controls.ThumbnailUI.IRhRdkContentThumbnailList_Sizes"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_UI_Controls_ThumbnailUI_IRhRdkContentThumbnailList_Sizes.htm)

### Values
- `Tiny` = `0` — Tiny thumbnails.
- `Small` = `1` — Small thumbnails.
- `Medium` = `2` — Medium thumbnails.
- `Large` = `3` — Large thumbnails.
- `Custom` = `4` — Custom-sized thumbnails.

## IRhRdkThumbnail (interface)

[Missing <summary> documentation for "T:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnail"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnail.htm)

### Methods
#### `void Dib(ref Bitmap dibOut)`



**Parameters:**
- `dibOut` (System.Drawing.Bitmap) — [Missing <param name="dibOut"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnail.Dib(System.Drawing.Bitmap@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnail_Dib.htm)

#### `Bitmap GetDib()`



**Returns:** `Bitmap` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnail.GetDib"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnail_GetDib.htm)

#### `void GetDisplayRect(ref RectangleF rectOut)`



**Parameters:**
- `rectOut` (System.Drawing.RectangleF) — [Missing <param name="rectOut"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnail.GetDisplayRect(System.Drawing.RectangleF@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnail_GetDisplayRect.htm)

#### `Guid Id()`



**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnail.Id"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnail_Id.htm)

#### `bool IsHot()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnail.IsHot"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnail_IsHot.htm)

#### `bool IsSelected()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnail.IsSelected"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnail_IsSelected.htm)

#### `string Label()`



**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnail.Label"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnail_Label.htm)

## IRhRdkThumbnailList (interface)

[Missing <summary> documentation for "T:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnailList.htm)

### Methods
#### `void Add(Thumbnail t)`



**Parameters:**
- `t` (Rhino.UI.Controls.ThumbnailUI.Thumbnail) — [Missing <param name="t"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList.Add(Rhino.UI.Controls.ThumbnailUI.Thumbnail)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnailList_Add.htm)

#### `void Clear()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnailList_Clear.htm)

#### `IRhRdkThumbnail Get(ref Guid u)`



**Parameters:**
- `u` (System.Guid) — [Missing <param name="u"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList.Get(System.Guid@)"]

**Returns:** `IRhRdkThumbnail` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList.Get(System.Guid@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnailList_Get.htm)

#### `void GetGridMetrics(ref int w, ref int h, ref int ox, ref int oy)`



**Parameters:**
- `w` (System.Int32) — [Missing <param name="w"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList.GetGridMetrics(System.Int32@,System.Int32@,System.Int32@,System.Int32@)"]
- `h` (System.Int32) — [Missing <param name="h"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList.GetGridMetrics(System.Int32@,System.Int32@,System.Int32@,System.Int32@)"]
- `ox` (System.Int32) — [Missing <param name="ox"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList.GetGridMetrics(System.Int32@,System.Int32@,System.Int32@,System.Int32@)"]
- `oy` (System.Int32) — [Missing <param name="oy"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList.GetGridMetrics(System.Int32@,System.Int32@,System.Int32@,System.Int32@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnailList_GetGridMetrics.htm)

#### `IRhRdkContentThumbnailList_Sizes GetSize()`



**Returns:** `IRhRdkContentThumbnailList_Sizes` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList.GetSize"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnailList_GetSize.htm)

#### `void GetStatisticsHeaderHeight()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnailList_GetStatisticsHeaderHeight.htm)

#### `IRhRdkThumbnailList_Modes Mode()`



**Returns:** `IRhRdkThumbnailList_Modes` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList.Mode"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnailList_Mode.htm)

#### `void SetClientText(string w)`



**Parameters:**
- `w` (System.String) — [Missing <param name="w"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList.SetClientText(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnailList_SetClientText.htm)

#### `void SetCustomBitmapSize(int w, int h)`



**Parameters:**
- `w` (System.Int32) — [Missing <param name="w"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList.SetCustomBitmapSize(System.Int32,System.Int32)"]
- `h` (System.Int32) — [Missing <param name="h"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList.SetCustomBitmapSize(System.Int32,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnailList_SetCustomBitmapSize.htm)

#### `void SetMode(IRhRdkThumbnailList_Modes m, bool b)`



**Parameters:**
- `m` (Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList_Modes) — [Missing <param name="m"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList.SetMode(Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList_Modes,System.Boolean)"]
- `b` (System.Boolean) — [Missing <param name="b"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList.SetMode(Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList_Modes,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnailList_SetMode.htm)

#### `void SetShowLabels(bool b)`



**Parameters:**
- `b` (System.Boolean) — [Missing <param name="b"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList.SetShowLabels(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnailList_SetShowLabels.htm)

#### `IRhRdkThumbnailList_Shapes Shape()`



**Returns:** `IRhRdkThumbnailList_Shapes` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList.Shape"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnailList_Shape.htm)

#### `bool ShowLabels()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList.ShowLabels"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnailList_ShowLabels.htm)

#### `Guid UUID()`



**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList.UUID"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnailList_UUID.htm)

## IRhRdkThumbnailList_Modes (enum)

[Missing <summary> documentation for "T:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList_Modes"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnailList_Modes.htm)

### Values
- `Grid` = `0` — Big thumbnails like Explorer icon mode (default).
- `List` = `1` — Small thumbnails and info on right like Explorer report mode.
- `Tree` = `2` — Tree view mode.

## IRhRdkThumbnailList_Shapes (enum)

[Missing <summary> documentation for "T:Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList_Shapes"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_UI_Controls_ThumbnailUI_IRhRdkThumbnailList_Shapes.htm)

### Values
- `Square` = `0` — Square thumbnails.
- `Wide` = `1` — Wide thumbnails.

## ThumbData (class)

[Missing <summary> documentation for "T:Rhino.UI.Controls.ThumbnailUI.ThumbData"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_UI_Controls_ThumbnailUI_ThumbData.htm)

### Constructors
- `public ThumbData()` — Initializes a new instance of the ThumbData class

### Methods
#### `public static int GetPreviewHeigth(Sizes thumb_size, Shapes shape)`



**Parameters:**
- `thumb_size` (Rhino.Render.DataSources.Sizes) — [Missing <param name="thumb_size"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.ThumbData.GetPreviewHeigth(Rhino.Render.DataSources.Sizes,Rhino.Render.DataSources.Shapes)"]
- `shape` (Rhino.Render.DataSources.Shapes) — [Missing <param name="shape"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.ThumbData.GetPreviewHeigth(Rhino.Render.DataSources.Sizes,Rhino.Render.DataSources.Shapes)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.ThumbData.GetPreviewHeigth(Rhino.Render.DataSources.Sizes,Rhino.Render.DataSources.Shapes)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_ThumbData_GetPreviewHeigth.htm)

#### `public static int GetPreviewWidth(Sizes thumb_size, Shapes shape)`



**Parameters:**
- `thumb_size` (Rhino.Render.DataSources.Sizes) — [Missing <param name="thumb_size"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.ThumbData.GetPreviewWidth(Rhino.Render.DataSources.Sizes,Rhino.Render.DataSources.Shapes)"]
- `shape` (Rhino.Render.DataSources.Shapes) — [Missing <param name="shape"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.ThumbData.GetPreviewWidth(Rhino.Render.DataSources.Sizes,Rhino.Render.DataSources.Shapes)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.ThumbData.GetPreviewWidth(Rhino.Render.DataSources.Sizes,Rhino.Render.DataSources.Shapes)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_ThumbData_GetPreviewWidth.htm)

### Properties
- `Children` (List<ThumbData>, get/set) — 
- `Content` (RenderContent, get/set) — 
- `Id` (Guid, get/set) — 
- `Image` (Bitmap, get/set) — 
- `Intensity` (String, get/set) — 
- `InUse` (Boolean, get/set) — 
- `InUseColor` (List<Color4f>, get/set) — 
- `Name` (String, get/set) — 
- `Parent` (ThumbData, get/set) — 
- `PreviewAppearance` (PreviewAppearance, get/set) — 
- `Selected` (Boolean, get/set) — 
- `Tags` (List<String>, get/set) — 
- `TopLevel` (Boolean, get) — 
- `Type` (String, get/set) — 

### Events
#### `PropertyChanged` (`System.ComponentModel.PropertyChangedEventHandler`)

**Signature:** `public event PropertyChangedEventHandler PropertyChanged`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_UI_Controls_ThumbnailUI_ThumbData_PropertyChanged.htm)

## Thumbnail (class)

[Missing <summary> documentation for "T:Rhino.UI.Controls.ThumbnailUI.Thumbnail"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_UI_Controls_ThumbnailUI_Thumbnail.htm)

### Constructors
- `public Thumbnail(IntPtr pRdkThumbnail)` — Constructor for Thumbnail

### Methods
#### `public void Dib(ref Bitmap dibOut)`



**Parameters:**
- `dibOut` (System.Drawing.Bitmap) — [Missing <param name="dibOut"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.Thumbnail.Dib(System.Drawing.Bitmap@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_Thumbnail_Dib.htm)

#### `public void Dispose()`

Dispose for Thumbnail

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_Thumbnail_Dispose.htm)

#### `protected override void Finalize()`

Destructor for Thumbnail

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_Thumbnail_Finalize.htm)

#### `public Bitmap GetDib()`



**Returns:** `Bitmap` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.Thumbnail.GetDib"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_Thumbnail_GetDib.htm)

#### `public void GetDisplayRect(ref RectangleF rectOut)`



**Parameters:**
- `rectOut` (System.Drawing.RectangleF) — [Missing <param name="rectOut"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.Thumbnail.GetDisplayRect(System.Drawing.RectangleF@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_Thumbnail_GetDisplayRect.htm)

#### `public Guid Id()`



**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.Thumbnail.Id"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_Thumbnail_Id.htm)

#### `public bool IsHot()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.Thumbnail.IsHot"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_Thumbnail_IsHot.htm)

#### `public bool IsSelected()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.Thumbnail.IsSelected"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_Thumbnail_IsSelected.htm)

#### `public string Label()`



**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.Thumbnail.Label"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_Thumbnail_Label.htm)

### Properties
- `CppPointer` (IntPtr, get) — Thumbnail c++ pointer

## ThumbnailViewModelFactory (class)

[Missing <summary> documentation for "T:Rhino.UI.Controls.ThumbnailUI.ThumbnailViewModelFactory"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_UI_Controls_ThumbnailUI_ThumbnailViewModelFactory.htm)

### Constructors
- `public ThumbnailViewModelFactory()` — Initializes a new instance of the ThumbnailViewModelFactory class

### Methods
#### `protected override void Finalize()`

Finalizer

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_FactoryBase_Finalize.htm)

#### `public override IntPtr Get(Guid id)`

(Overrides FactoryBase.Get(Guid).)

**Parameters:**
- `id` (System.Guid) — [Missing <param name="id"/> documentation for "M:Rhino.UI.Controls.ThumbnailUI.ThumbnailViewModelFactory.Get(System.Guid)"]

**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.UI.Controls.ThumbnailUI.ThumbnailViewModelFactory.Get(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_UI_Controls_ThumbnailUI_ThumbnailViewModelFactory_Get.htm)

