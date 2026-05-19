---
name: rhino-rhino-render-ui
description: This skill encodes the rhino 8.0 surface of the Rhino.Render.UI namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: UserInterfaceSection, IUserInterfaceSection, WorldMapDayNight.
---

# Rhino.Render.UI

Auto-generated from vendor docs for rhino 8.0. 3 types in this namespace.

## IUserInterfaceSection (interface)

Implement this interface in your user control to get UserInterfaceSection event notification.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_UI_IUserInterfaceSection.htm)

### Methods
#### `void OnUserInterfaceSectionExpanding(UserInterfaceSection userInterfaceSection, bool expanding)`

The UserInterfaceSection object that called this interface method.

**Parameters:**
- `userInterfaceSection` (Rhino.Render.UI.UserInterfaceSection) — The UserInterfaceSection object that called this interface method.
- `expanding` (System.Boolean) — Will be true if the control has been createExpanded or false if it was collapsed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_UI_IUserInterfaceSection_OnUserInterfaceSectionExpanding.htm)

#### `void UserInterfaceDisplayData(UserInterfaceSection userInterfaceSection, RenderContent[] renderContentList)`

Called by UserInterfaceSection when the selected content changes or a content field property value changes.

**Parameters:**
- `userInterfaceSection` (Rhino.Render.UI.UserInterfaceSection) — The UserInterfaceSection object that called this interface method.
- `renderContentList` (Rhino.Render.RenderContent[]) — The currently selected list of content items to edit.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_UI_IUserInterfaceSection_UserInterfaceDisplayData.htm)

### Properties
- `Hidden` (Boolean, get) — Return true if the section should be hidden, else return false.

## UserInterfaceSection (class)

Custom user interface section manager

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_UI_UserInterfaceSection.htm)

### Methods
#### `public void Expand(bool expand)`

Expand or collapse this content section.

**Parameters:**
- `expand` (System.Boolean) — If true then expand the content section otherwise collapse it.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_UI_UserInterfaceSection_Expand.htm)

#### `public static UserInterfaceSection FromWindow(Object window)`

Find the UserInterfaceSection that created the specified instance of a window.

**Parameters:**
- `window` (System.Object) — If window is not null then look for the UserInterfaceSection that created the window.

**Returns:** `UserInterfaceSection` — If a UserInterfaceSection object is found containing a reference to the requested window then return the object otherwise return null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_UI_UserInterfaceSection_FromWindow.htm)

#### `public RenderContent[] GetContentList()`

Returns a list of currently selected content items to be edited.

**Returns:** `RenderContent[]` — Returns a list of currently selected content items to be edited.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_UI_UserInterfaceSection_GetContentList.htm)

#### `public void Show(bool visible)`

Show or hide this content section.

**Parameters:**
- `visible` (System.Boolean) — If true then show the content section otherwise hide it.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_UI_UserInterfaceSection_Show.htm)

### Properties
- `RenderContent` (RenderContent, get) — The RenderContent object that created this user interface object.
- `Window` (Object, get) — The user control associated with this user interface object.

## WorldMapDayNight (class)

[Missing <summary> documentation for "T:Rhino.Render.UI.WorldMapDayNight"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Render_UI_WorldMapDayNight.htm)

### Constructors
- `public WorldMapDayNight()` — Initializes a new instance of the WorldMapDayNight class

### Methods
#### `public void Dispose()`

Releases all resources used by the WorldMapDayNight

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_UI_WorldMapDayNight_Dispose.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_UI_WorldMapDayNight_Finalize.htm)

#### `public bool HasMapForCurrentSettings()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Render.UI.WorldMapDayNight.HasMapForCurrentSettings"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_UI_WorldMapDayNight_HasMapForCurrentSettings.htm)

#### `public Point LocationToMap(Point2d latlong)`



**Parameters:**
- `latlong` (Rhino.Geometry.Point2d) — [Missing <param name="latlong"/> documentation for "M:Rhino.Render.UI.WorldMapDayNight.LocationToMap(Rhino.Geometry.Point2d)"]

**Returns:** `Point` — [Missing <returns> documentation for "M:Rhino.Render.UI.WorldMapDayNight.LocationToMap(Rhino.Geometry.Point2d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_UI_WorldMapDayNight_LocationToMap.htm)

#### `public void MakeMapBitmap()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_UI_WorldMapDayNight_MakeMapBitmap.htm)

#### `public Image Map()`



**Returns:** `Image` — [Missing <returns> documentation for "M:Rhino.Render.UI.WorldMapDayNight.Map"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_UI_WorldMapDayNight_Map.htm)

#### `public Point2d MapToLocation(Point mapPoint)`



**Parameters:**
- `mapPoint` (System.Drawing.Point) — [Missing <param name="mapPoint"/> documentation for "M:Rhino.Render.UI.WorldMapDayNight.MapToLocation(System.Drawing.Point)"]

**Returns:** `Point2d` — [Missing <returns> documentation for "M:Rhino.Render.UI.WorldMapDayNight.MapToLocation(System.Drawing.Point)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_UI_WorldMapDayNight_MapToLocation.htm)

#### `public void SetDayNightDisplay(bool bOn)`



**Parameters:**
- `bOn` (System.Boolean) — [Missing <param name="bOn"/> documentation for "M:Rhino.Render.UI.WorldMapDayNight.SetDayNightDisplay(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_UI_WorldMapDayNight_SetDayNightDisplay.htm)

#### `public void SetEnabled(bool bEnabled)`



**Parameters:**
- `bEnabled` (System.Boolean) — [Missing <param name="bEnabled"/> documentation for "M:Rhino.Render.UI.WorldMapDayNight.SetEnabled(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_UI_WorldMapDayNight_SetEnabled.htm)

#### `public void SetTimeInfo(DateTime dt, double timezone, int daylightSavingMinutes, bool bDaylightSavingsOn)`



**Parameters:**
- `dt` (System.DateTime) — [Missing <param name="dt"/> documentation for "M:Rhino.Render.UI.WorldMapDayNight.SetTimeInfo(System.DateTime,System.Double,System.Int32,System.Boolean)"]
- `timezone` (System.Double) — [Missing <param name="timezone"/> documentation for "M:Rhino.Render.UI.WorldMapDayNight.SetTimeInfo(System.DateTime,System.Double,System.Int32,System.Boolean)"]
- `daylightSavingMinutes` (System.Int32) — [Missing <param name="daylightSavingMinutes"/> documentation for "M:Rhino.Render.UI.WorldMapDayNight.SetTimeInfo(System.DateTime,System.Double,System.Int32,System.Boolean)"]
- `bDaylightSavingsOn` (System.Boolean) — [Missing <param name="bDaylightSavingsOn"/> documentation for "M:Rhino.Render.UI.WorldMapDayNight.SetTimeInfo(System.DateTime,System.Double,System.Int32,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Render_UI_WorldMapDayNight_SetTimeInfo.htm)

