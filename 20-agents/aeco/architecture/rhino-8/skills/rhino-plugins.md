---
name: rhino-rhino-plugins
description: This skill encodes the rhino 8.0 surface of the Rhino.PlugIns namespace — 35 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: DigitizerPlugIn, CustomRenderSaveFileTypes, FileExportPlugIn, FileTypeList, FileImportPlugIn, LicenseChangedEventArgs, LicenseData, LicenseLease, and 27 more types.
---

# Rhino.PlugIns

Auto-generated from vendor docs for rhino 8.0. 35 types in this namespace.

## CustomRenderSaveFileTypes (class)

[Missing <summary> documentation for "T:Rhino.PlugIns.CustomRenderSaveFileTypes"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_CustomRenderSaveFileTypes.htm)

### Methods
#### `public void RegisterFileType(IEnumerable<string> extensions, string description, CustomRenderSaveFileTypes.SaveFileHandler saveFileHandler)`

Call this method to register a custom file save type with the render output window save dialog.

**Parameters:**
- `extensions` (System.Collections.Generic.IEnumerable<String>) — List of one or more file extension associated with this custom type, for example: HDR, HDRI
- `description` (System.String) — File extension description which appears in the file save dialog file type combo box.
- `saveFileHandler` (Rhino.PlugIns.CustomRenderSaveFileTypes.SaveFileHandler) — [Missing <param name="saveFileHandler"/> documentation for "M:Rhino.PlugIns.CustomRenderSaveFileTypes.RegisterFileType(System.Collections.Generic.IEnumerable{System.String},System.String,Rhino.PlugIns.CustomRenderSaveFileTypes.SaveFileHandler)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_CustomRenderSaveFileTypes_RegisterFileType.htm)

## CustomRenderSaveFileTypes.SaveFileHandler (delegate)

Called when a user chooses to save a rendered scene as this custom file type.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_CustomRenderSaveFileTypes_SaveFileHandler.htm)

## DescriptionType (enum)

Rhino plug-in developer information fields.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_DescriptionType.htm)

### Values
- `Organization` = `0`
- `Address` = `1`
- `Country` = `2`
- `Phone` = `3`
- `WebSite` = `4`
- `Email` = `5`
- `UpdateUrl` = `6`
- `Fax` = `7`
- `Icon` = `8`

## DigitizerPlugIn (class)

A Rhino plug-in that interfaces with 3-D digitizing or input devices.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_DigitizerPlugIn.htm)

### Constructors
- `protected DigitizerPlugIn()` — Initializes a new instance of the DigitizerPlugIn class

### Methods
#### `protected bool AskUserForLicense(LicenseBuildType productBuildType, bool standAlone, string textMask, Object parentWindow, ValidateProductKeyDelegate validateProductKeyDelegate, OnLeaseChangedDelegate onLeaseChangedDelegate)`

(Inherited from PlugIn.)

**Parameters:**
- `productBuildType` (Rhino.PlugIns.LicenseBuildType) — [Missing <param name="productBuildType"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `standAlone` (System.Boolean) — [Missing <param name="standAlone"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `textMask` (System.String) — [Missing <param name="textMask"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `parentWindow` (System.Object) — [Missing <param name="parentWindow"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `validateProductKeyDelegate` (Rhino.PlugIns.ValidateProductKeyDelegate) — [Missing <param name="validateProductKeyDelegate"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `onLeaseChangedDelegate` (Rhino.PlugIns.OnLeaseChangedDelegate) — [Missing <param name="onLeaseChangedDelegate"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_AskUserForLicense.htm)

#### `public PersistentSettings CommandSettings(string name)`

(Inherited from PlugIn.)

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.PlugIns.PlugIn.CommandSettings(System.String)"]

**Returns:** `PersistentSettings` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.CommandSettings(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_CommandSettings.htm)

#### `protected virtual void CreateCommands()`

Called right after plug-in is created and is responsible for creating all of the commands in a given plug-in. The base class implementation Constructs an instance of every publicly exported command class in your plug-in's assembly.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_CreateCommands.htm)

#### `public virtual bool DisplayHelp(IntPtr windowHandle)`

Called by Rhino if AddToHelpMenu is true and menu item associated with this plug-in is selected.

**Parameters:**
- `windowHandle` (System.IntPtr) — Native Window handle of the active Rhino interface.

**Returns:** `Boolean` — true = Help displayed successfully, false = Error displaying help

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_DisplayHelp.htm)

#### `protected virtual void DocumentPropertiesDialogPages(RhinoDoc doc, List<OptionsDialogPage> pages)`

Override this function if you want to extend the document properties sections of the options dialog. This function is called whenever the user brings up the Options dialog.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — document that the pages are set up for
- `pages` (System.Collections.Generic.List<OptionsDialogPage>) — list of pages to add your custom options dialog page(s) to.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_DocumentPropertiesDialogPages.htm)

#### `protected abstract bool EnableDigitizer(bool enable)`

Called by Rhino to enable/disable input from the digitizer. If enable is true and EnableDigitizer() returns false, then Rhino will not calibrate the digitizer.

**Parameters:**
- `enable` (System.Boolean) — If true, enable the digitizer. If false, disable the digitizer.

**Returns:** `Boolean` — true if the operation succeeded; otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_DigitizerPlugIn_EnableDigitizer.htm)

#### `public Command[] GetCommands()`

All of the commands associated with this plug-in.

**Returns:** `Command[]` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.GetCommands"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetCommands.htm)

#### `protected bool GetLicense(LicenseBuildType productBuildType, ValidateProductKeyDelegate validateProductKeyDelegate, OnLeaseChangedDelegate leaseChangedDelegate)`

Verifies that there is a valid product license for your plug-in, using the Rhino licensing system. If the plug-in is installed as a standalone node, the locally installed license will be validated. If the plug-in is installed as a network node, a loaner license will be requested by the system's assigned Zoo server. If the Zoo server finds and returns a license, then this license will be validated. If no license is found, then the user will be prompted to provide a license key, which will be validated.

**Parameters:**
- `productBuildType` (Rhino.PlugIns.LicenseBuildType) — The product build contentType required by your plug-in.
- `validateProductKeyDelegate` (Rhino.PlugIns.ValidateProductKeyDelegate) — Since the Rhino licensing system knows nothing about your product license, you will need to validate the product license provided by the Rhino licensing system. This is done by supplying a callback function, or delegate, that can be called to perform the validation.
- `leaseChangedDelegate` (Rhino.PlugIns.OnLeaseChangedDelegate) — Called by the ZooClient when the cloud zoo lease is changed.

**Returns:** `Boolean` — true if a valid license was found. false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetLicense.htm)

#### `protected bool GetLicense(LicenseCapabilities licenseCapabilities, string textMask, ValidateProductKeyDelegate validateProductKeyDelegate, OnLeaseChangedDelegate leaseChangedDelegate)`

Verifies that there is a valid product license for your plug-in, using the Rhino licensing system. If the plug-in is installed as a standalone node, the locally installed license will be validated. If the plug-in is installed as a network node, a loaner license will be requested by the system's assigned Zoo server. If the Zoo server finds and returns a license, then this license will be validated. If no license is found, then the user will be prompted to provide a license key, which will be validated.

**Parameters:**
- `licenseCapabilities` (Rhino.PlugIns.LicenseCapabilities) — In the event that a license was not found, or if the user wants to change the way your plug-in is licenses, then provide what capabilities your license has by using this enumeration flag.
- `textMask` (System.String) — In the event that the user needs to be asked for a license, then you can provide a text mask, which helps the user to distinguish between proper and improper user input of your license code. Note, if you do not want to use a text mask, then pass in a null value for this parameter. For more information on text masks, search MSDN for the System.Windows.Forms.MaskedTextBox class.
- `validateProductKeyDelegate` (Rhino.PlugIns.ValidateProductKeyDelegate) — Since the Rhino licensing system knows nothing about your product license, you will need to validate the product license provided by the Rhino licensing system. This is done by supplying a callback function, or delegate, that can be called to perform the validation.
- `leaseChangedDelegate` (Rhino.PlugIns.OnLeaseChangedDelegate) — Called by the ZooClient when the cloud zoo lease is changed.

**Returns:** `Boolean` — true if a valid license was found. false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetLicense_1.htm)

#### `protected bool GetLicenseOwner(out string registeredOwner, out string registeredOrganization)`

Get the customer name and organization used when entering the product license.

**Parameters:**
- `registeredOwner` (System.String) — [Missing <param name="registeredOwner"/> documentation for "M:Rhino.PlugIns.PlugIn.GetLicenseOwner(System.String@,System.String@)"]
- `registeredOrganization` (System.String) — [Missing <param name="registeredOrganization"/> documentation for "M:Rhino.PlugIns.PlugIn.GetLicenseOwner(System.String@,System.String@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.GetLicenseOwner(System.String@,System.String@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetLicenseOwner.htm)

#### `public virtual Object GetPlugInObject()`

(Inherited from PlugIn.)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.GetPlugInObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetPlugInObject.htm)

#### `public Bitmap Icon(Size size)`

Returns the plug-in's icon in bitmap form.

**Parameters:**
- `size` (System.Drawing.Size) — The desired size in pixels.

**Returns:** `Bitmap` — The icon if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_Icon.htm)

#### `public virtual bool IsTextureSupported(RenderTexture texture)`

Returns true if this renderer can render the texture natively without needing it to be baked into a bitmap, false otherwise. By default, returns false for all textures.

**Parameters:**
- `texture` (Rhino.Render.RenderTexture) — [Missing <param name="texture"/> documentation for "M:Rhino.PlugIns.PlugIn.IsTextureSupported(Rhino.Render.RenderTexture)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.IsTextureSupported(Rhino.Render.RenderTexture)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_IsTextureSupported.htm)

#### `[ObsoleteAttribute("Use ObjectPropertiesPages(ObjectPropertiesPageCollection collection) instead")] protected virtual void ObjectPropertiesPages(List<ObjectPropertiesPage> pages)`

Override this function is you want to extend the object properties dialog

**Parameters:**
- `pages` (System.Collections.Generic.List<ObjectPropertiesPage>) — [Missing <param name="pages"/> documentation for "M:Rhino.PlugIns.PlugIn.ObjectPropertiesPages(System.Collections.Generic.List{Rhino.UI.ObjectPropertiesPage})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ObjectPropertiesPages_1.htm)

#### `protected virtual void ObjectPropertiesPages(ObjectPropertiesPageCollection collection)`

Override this function is you want to extend the object properties dialog. This method will be called each time a new document is created for each instance of a object properties panel. On Windows there will be a single panel per document but on Mac there may be many properties panel per document.

**Parameters:**
- `collection` (Rhino.UI.ObjectPropertiesPageCollection) — Add custom pages by calling collection.Add

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ObjectPropertiesPages.htm)

#### `protected virtual LoadReturnCode OnLoad(ref string errorMessage)`

Is called when the plug-in is being loaded.

**Parameters:**
- `errorMessage` (System.String) — If a load error is returned and this string is set. This string is the error message that will be reported back to the user.

**Returns:** `LoadReturnCode` — An appropriate load return code. The default implementation returns Success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_OnLoad.htm)

#### `protected virtual void OnShutdown()`

(Inherited from PlugIn.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_OnShutdown.htm)

#### `protected virtual void OptionsDialogPages(List<OptionsDialogPage> pages)`

Override this function if you want to extend the options dialog. This function is called whenever the user brings up the Options dialog.

**Parameters:**
- `pages` (System.Collections.Generic.List<OptionsDialogPage>) — list of pages to add your custom options dialog page(s) to.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_OptionsDialogPages.htm)

#### `protected virtual void ReadDocument(RhinoDoc doc, BinaryArchiveReader archive, FileReadOptions options)`

Called whenever a Rhino document is being loaded and plug-in user data was encountered written by a plug-in with this plug-in's GUID.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — A Rhino document that is being loaded.
- `archive` (Rhino.FileIO.BinaryArchiveReader) — OpenNURBS file archive object Rhino is using to read this file. Use BinaryArchiveReader.Read*() functions to read plug-in data. If any BinaryArchive.Read*() functions throws an exception then archive.ReadErrorOccurve will be true and you should immediately return.
- `options` (Rhino.FileIO.FileReadOptions) — Describes what is being written.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ReadDocument.htm)

#### `protected bool RegisterCommand(Command command)`

(Inherited from PlugIn.)

**Parameters:**
- `command` (Rhino.Commands.Command) — [Missing <param name="command"/> documentation for "M:Rhino.PlugIns.PlugIn.RegisterCommand(Rhino.Commands.Command)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.RegisterCommand(Rhino.Commands.Command)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_RegisterCommand.htm)

#### `protected virtual void ResetMessageBoxes()`

(Inherited from PlugIn.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ResetMessageBoxes.htm)

#### `protected bool ReturnLicense()`

Returns, or releases, a product license that was obtained from the Rhino licensing system. Note, most plug-ins do not need to call this as the Rhino licensing system will return all licenses when Rhino shuts down.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.ReturnLicense"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ReturnLicense.htm)

#### `public void SaveSettings()`

Write settings to disk which will raise a SettingsSaved event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_SaveSettings.htm)

#### `public void SendPoint(Point3d point, MouseButton mousebuttons, bool shiftKey, bool controlKey)`

If the digitizer is enabled, call this function to send a point to Rhino. Call this function as much as you like. The digitizers that Rhino currently supports send a point every 15 milliseconds or so. This function should be called when users press or release any digitizer button.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — 3d point in digitizer coordinates.
- `mousebuttons` (Rhino.UI.MouseButton) — corresponding digitizer button is down.
- `shiftKey` (System.Boolean) — true if the Shift keyboard key was pressed. Otherwise, false.
- `controlKey` (System.Boolean) — true if the Control keyboard key was pressed. Otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_DigitizerPlugIn_SendPoint.htm)

#### `public void SendRay(Ray3d ray, MouseButton mousebuttons, bool shiftKey, bool controlKey)`

If the digitizer is enabled, call this function to send a point and direction to Rhino. Call this function as much as you like. The digitizers that Rhino currently supports send a point every 15 milliseconds or so. This function should be called when users press or release any digitizer button.

**Parameters:**
- `ray` (Rhino.Geometry.Ray3d) — 3d ray in digitizer coordinates.
- `mousebuttons` (Rhino.UI.MouseButton) — corresponding digitizer button is down.
- `shiftKey` (System.Boolean) — true if the Shift keyboard key was pressed. Otherwise, false.
- `controlKey` (System.Boolean) — true if the Control keyboard key was pressed. Otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_DigitizerPlugIn_SendRay.htm)

#### `protected void SetLicenseCapabilities(string textMask, LicenseCapabilities capabilities, Guid licenseId)`

(Inherited from PlugIn.)

**Parameters:**
- `textMask` (System.String) — [Missing <param name="textMask"/> documentation for "M:Rhino.PlugIns.PlugIn.SetLicenseCapabilities(System.String,Rhino.PlugIns.LicenseCapabilities,System.Guid)"]
- `capabilities` (Rhino.PlugIns.LicenseCapabilities) — [Missing <param name="capabilities"/> documentation for "M:Rhino.PlugIns.PlugIn.SetLicenseCapabilities(System.String,Rhino.PlugIns.LicenseCapabilities,System.Guid)"]
- `licenseId` (System.Guid) — [Missing <param name="licenseId"/> documentation for "M:Rhino.PlugIns.PlugIn.SetLicenseCapabilities(System.String,Rhino.PlugIns.LicenseCapabilities,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_SetLicenseCapabilities.htm)

#### `protected virtual bool ShouldCallWriteDocument(FileWriteOptions options)`

Called whenever a Rhino is about to save a .3dm file. If you want to save plug-in document data when a model is saved in a version 5 .3dm file, then you must override this function to return true and you must override WriteDocument().

**Parameters:**
- `options` (Rhino.FileIO.FileWriteOptions) — The file write options, such as "include preview image" and "include render meshes".

**Returns:** `Boolean` — true if the plug-in wants to save document user data in the version 5 .3dm file. The default returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ShouldCallWriteDocument.htm)

#### `protected virtual void WriteDocument(RhinoDoc doc, BinaryArchiveWriter archive, FileWriteOptions options)`

Called when Rhino is saving a .3dm file to allow the plug-in to save document user data.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — The Rhino document instance that is being saved.
- `archive` (Rhino.FileIO.BinaryArchiveWriter) — OpenNURBS file archive object Rhino is using to write the file. Use BinaryArchiveWriter.Write*() functions to write plug-in data. OR use the ArchivableDictionary If any BinaryArchiveWriter.Write*() functions throw an exception, then archive.WriteErrorOccured will be true and you should immediately return. Setting archive.WriteErrorOccured to true will cause Rhino to stop saving the file.
- `options` (Rhino.FileIO.FileWriteOptions) — The file write options, such as "include preview image" and "include render meshes".

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_WriteDocument.htm)

### Properties
- `AddToHelpMenu` (Boolean, get) — Called by Rhino to determine if the plug-in name should be added to the Rhino Help/Plug-ins menu.
- `Assembly` (Assembly, get) — Source assembly for this plug-in.
- `Description` (String, get) — Returns the description of the plug-in, as found in the plug-in's assembly attributes.
- `DigitizerUnitSystem` (UnitSystem, get) — Unit system of the points the digitizer passes to SendPoint(). Rhino uses this value when it calibrates a digitizer. This unit system must be not change.
- `Id` (Guid, get) — Returns the Guid, or unique Id, of the plug-in.
- `LicenseId` (Guid, get) — (Inherited from PlugIn.)
- `LoadAtStartup` (Boolean, get) — Obsolete. (Inherited from PlugIn.)
- `LoadTime` (PlugInLoadTime, get) — Plug-ins are typically loaded on demand when they are first needed. You can change this behavior to load the plug-in at during different stages in time by overriding this property.
- `LocalPlugInName` (String, get) — Optionally override this to provide a localized plug-in name
- `Name` (String, get) — Returns the name of the plug-in, as found in the plug-in's assembly attributes.
- `PointTolerance` (Double, get) — The point tolerance is the distance the digitizer must move (in digitizer coordinates) for a new point to be considered real rather than noise. Small desktop digitizer arms have values like 0.001 inches and 0.01 millimeters. This value should never be smaller than the accuracy of the digitizing device.
- `Settings` (PersistentSettings, get) — Persistent plug-in settings.
- `SettingsDirectory` (String, get) — Get the plug-in's settings directory. This is the directory where the plug-in's persistent settings files are saved. This directory will be located in the user's profile folder. Note, this does not verify the directory exists.
- `SettingsDirectoryAllUsers` (String, get) — Get the plug-in's "all users" settings directory. This directory will be located in the system's program data folder. Note, this does not verify the directory exists.
- `Version` (String, get) — Returns the version of the plug-in, as found in the plug-in's assembly attributes.
- `WindowPositionSettings` (PersistentSettings, get) — (Inherited from PlugIn.)

### Events
#### `SettingsSaved` (`System.EventHandler<PersistentSettingsSavedEventArgs>`)

**Signature:** `public event EventHandler<PersistentSettingsSavedEventArgs> SettingsSaved`

This event is raised when an instance of Rhino has modified the external settings file associated with this plug-in's Settings property.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_PlugIns_PlugIn_SettingsSaved.htm)

## FileExportPlugIn (class)

Rhino plug-in that exports data from Rhino to other file formats; can support more than one format.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_FileExportPlugIn.htm)

### Constructors
- `protected FileExportPlugIn()` — Initializes a new instance of the FileExportPlugIn class

### Methods
#### `protected abstract FileTypeList AddFileTypes(FileWriteOptions options)`



**Parameters:**
- `options` (Rhino.FileIO.FileWriteOptions) — [Missing <param name="options"/> documentation for "M:Rhino.PlugIns.FileExportPlugIn.AddFileTypes(Rhino.FileIO.FileWriteOptions)"]

**Returns:** `FileTypeList` — [Missing <returns> documentation for "M:Rhino.PlugIns.FileExportPlugIn.AddFileTypes(Rhino.FileIO.FileWriteOptions)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_FileExportPlugIn_AddFileTypes.htm)

#### `protected bool AskUserForLicense(LicenseBuildType productBuildType, bool standAlone, string textMask, Object parentWindow, ValidateProductKeyDelegate validateProductKeyDelegate, OnLeaseChangedDelegate onLeaseChangedDelegate)`

(Inherited from PlugIn.)

**Parameters:**
- `productBuildType` (Rhino.PlugIns.LicenseBuildType) — [Missing <param name="productBuildType"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `standAlone` (System.Boolean) — [Missing <param name="standAlone"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `textMask` (System.String) — [Missing <param name="textMask"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `parentWindow` (System.Object) — [Missing <param name="parentWindow"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `validateProductKeyDelegate` (Rhino.PlugIns.ValidateProductKeyDelegate) — [Missing <param name="validateProductKeyDelegate"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `onLeaseChangedDelegate` (Rhino.PlugIns.OnLeaseChangedDelegate) — [Missing <param name="onLeaseChangedDelegate"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_AskUserForLicense.htm)

#### `public PersistentSettings CommandSettings(string name)`

(Inherited from PlugIn.)

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.PlugIns.PlugIn.CommandSettings(System.String)"]

**Returns:** `PersistentSettings` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.CommandSettings(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_CommandSettings.htm)

#### `protected virtual void CreateCommands()`

Called right after plug-in is created and is responsible for creating all of the commands in a given plug-in. The base class implementation Constructs an instance of every publicly exported command class in your plug-in's assembly.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_CreateCommands.htm)

#### `public virtual bool DisplayHelp(IntPtr windowHandle)`

Called by Rhino if AddToHelpMenu is true and menu item associated with this plug-in is selected.

**Parameters:**
- `windowHandle` (System.IntPtr) — Native Window handle of the active Rhino interface.

**Returns:** `Boolean` — true = Help displayed successfully, false = Error displaying help

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_DisplayHelp.htm)

#### `protected virtual void DisplayOptionsDialog(IntPtr parent, string description, string extension)`



**Parameters:**
- `parent` (System.IntPtr) — [Missing <param name="parent"/> documentation for "M:Rhino.PlugIns.FileExportPlugIn.DisplayOptionsDialog(System.IntPtr,System.String,System.String)"]
- `description` (System.String) — [Missing <param name="description"/> documentation for "M:Rhino.PlugIns.FileExportPlugIn.DisplayOptionsDialog(System.IntPtr,System.String,System.String)"]
- `extension` (System.String) — [Missing <param name="extension"/> documentation for "M:Rhino.PlugIns.FileExportPlugIn.DisplayOptionsDialog(System.IntPtr,System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_FileExportPlugIn_DisplayOptionsDialog.htm)

#### `protected virtual void DocumentPropertiesDialogPages(RhinoDoc doc, List<OptionsDialogPage> pages)`

Override this function if you want to extend the document properties sections of the options dialog. This function is called whenever the user brings up the Options dialog.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — document that the pages are set up for
- `pages` (System.Collections.Generic.List<OptionsDialogPage>) — list of pages to add your custom options dialog page(s) to.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_DocumentPropertiesDialogPages.htm)

#### `public Command[] GetCommands()`

All of the commands associated with this plug-in.

**Returns:** `Command[]` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.GetCommands"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetCommands.htm)

#### `protected bool GetLicense(LicenseBuildType productBuildType, ValidateProductKeyDelegate validateProductKeyDelegate, OnLeaseChangedDelegate leaseChangedDelegate)`

Verifies that there is a valid product license for your plug-in, using the Rhino licensing system. If the plug-in is installed as a standalone node, the locally installed license will be validated. If the plug-in is installed as a network node, a loaner license will be requested by the system's assigned Zoo server. If the Zoo server finds and returns a license, then this license will be validated. If no license is found, then the user will be prompted to provide a license key, which will be validated.

**Parameters:**
- `productBuildType` (Rhino.PlugIns.LicenseBuildType) — The product build contentType required by your plug-in.
- `validateProductKeyDelegate` (Rhino.PlugIns.ValidateProductKeyDelegate) — Since the Rhino licensing system knows nothing about your product license, you will need to validate the product license provided by the Rhino licensing system. This is done by supplying a callback function, or delegate, that can be called to perform the validation.
- `leaseChangedDelegate` (Rhino.PlugIns.OnLeaseChangedDelegate) — Called by the ZooClient when the cloud zoo lease is changed.

**Returns:** `Boolean` — true if a valid license was found. false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetLicense.htm)

#### `protected bool GetLicense(LicenseCapabilities licenseCapabilities, string textMask, ValidateProductKeyDelegate validateProductKeyDelegate, OnLeaseChangedDelegate leaseChangedDelegate)`

Verifies that there is a valid product license for your plug-in, using the Rhino licensing system. If the plug-in is installed as a standalone node, the locally installed license will be validated. If the plug-in is installed as a network node, a loaner license will be requested by the system's assigned Zoo server. If the Zoo server finds and returns a license, then this license will be validated. If no license is found, then the user will be prompted to provide a license key, which will be validated.

**Parameters:**
- `licenseCapabilities` (Rhino.PlugIns.LicenseCapabilities) — In the event that a license was not found, or if the user wants to change the way your plug-in is licenses, then provide what capabilities your license has by using this enumeration flag.
- `textMask` (System.String) — In the event that the user needs to be asked for a license, then you can provide a text mask, which helps the user to distinguish between proper and improper user input of your license code. Note, if you do not want to use a text mask, then pass in a null value for this parameter. For more information on text masks, search MSDN for the System.Windows.Forms.MaskedTextBox class.
- `validateProductKeyDelegate` (Rhino.PlugIns.ValidateProductKeyDelegate) — Since the Rhino licensing system knows nothing about your product license, you will need to validate the product license provided by the Rhino licensing system. This is done by supplying a callback function, or delegate, that can be called to perform the validation.
- `leaseChangedDelegate` (Rhino.PlugIns.OnLeaseChangedDelegate) — Called by the ZooClient when the cloud zoo lease is changed.

**Returns:** `Boolean` — true if a valid license was found. false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetLicense_1.htm)

#### `protected bool GetLicenseOwner(out string registeredOwner, out string registeredOrganization)`

Get the customer name and organization used when entering the product license.

**Parameters:**
- `registeredOwner` (System.String) — [Missing <param name="registeredOwner"/> documentation for "M:Rhino.PlugIns.PlugIn.GetLicenseOwner(System.String@,System.String@)"]
- `registeredOrganization` (System.String) — [Missing <param name="registeredOrganization"/> documentation for "M:Rhino.PlugIns.PlugIn.GetLicenseOwner(System.String@,System.String@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.GetLicenseOwner(System.String@,System.String@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetLicenseOwner.htm)

#### `public virtual Object GetPlugInObject()`

(Inherited from PlugIn.)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.GetPlugInObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetPlugInObject.htm)

#### `public Bitmap Icon(Size size)`

Returns the plug-in's icon in bitmap form.

**Parameters:**
- `size` (System.Drawing.Size) — The desired size in pixels.

**Returns:** `Bitmap` — The icon if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_Icon.htm)

#### `public virtual bool IsTextureSupported(RenderTexture texture)`

Returns true if this renderer can render the texture natively without needing it to be baked into a bitmap, false otherwise. By default, returns false for all textures.

**Parameters:**
- `texture` (Rhino.Render.RenderTexture) — [Missing <param name="texture"/> documentation for "M:Rhino.PlugIns.PlugIn.IsTextureSupported(Rhino.Render.RenderTexture)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.IsTextureSupported(Rhino.Render.RenderTexture)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_IsTextureSupported.htm)

#### `[ObsoleteAttribute("Use ObjectPropertiesPages(ObjectPropertiesPageCollection collection) instead")] protected virtual void ObjectPropertiesPages(List<ObjectPropertiesPage> pages)`

Override this function is you want to extend the object properties dialog

**Parameters:**
- `pages` (System.Collections.Generic.List<ObjectPropertiesPage>) — [Missing <param name="pages"/> documentation for "M:Rhino.PlugIns.PlugIn.ObjectPropertiesPages(System.Collections.Generic.List{Rhino.UI.ObjectPropertiesPage})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ObjectPropertiesPages_1.htm)

#### `protected virtual void ObjectPropertiesPages(ObjectPropertiesPageCollection collection)`

Override this function is you want to extend the object properties dialog. This method will be called each time a new document is created for each instance of a object properties panel. On Windows there will be a single panel per document but on Mac there may be many properties panel per document.

**Parameters:**
- `collection` (Rhino.UI.ObjectPropertiesPageCollection) — Add custom pages by calling collection.Add

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ObjectPropertiesPages.htm)

#### `protected virtual LoadReturnCode OnLoad(ref string errorMessage)`

Is called when the plug-in is being loaded.

**Parameters:**
- `errorMessage` (System.String) — If a load error is returned and this string is set. This string is the error message that will be reported back to the user.

**Returns:** `LoadReturnCode` — An appropriate load return code. The default implementation returns Success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_OnLoad.htm)

#### `protected virtual void OnShutdown()`

(Inherited from PlugIn.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_OnShutdown.htm)

#### `protected virtual void OptionsDialogPages(List<OptionsDialogPage> pages)`

Override this function if you want to extend the options dialog. This function is called whenever the user brings up the Options dialog.

**Parameters:**
- `pages` (System.Collections.Generic.List<OptionsDialogPage>) — list of pages to add your custom options dialog page(s) to.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_OptionsDialogPages.htm)

#### `protected virtual void ReadDocument(RhinoDoc doc, BinaryArchiveReader archive, FileReadOptions options)`

Called whenever a Rhino document is being loaded and plug-in user data was encountered written by a plug-in with this plug-in's GUID.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — A Rhino document that is being loaded.
- `archive` (Rhino.FileIO.BinaryArchiveReader) — OpenNURBS file archive object Rhino is using to read this file. Use BinaryArchiveReader.Read*() functions to read plug-in data. If any BinaryArchive.Read*() functions throws an exception then archive.ReadErrorOccurve will be true and you should immediately return.
- `options` (Rhino.FileIO.FileReadOptions) — Describes what is being written.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ReadDocument.htm)

#### `protected bool RegisterCommand(Command command)`

(Inherited from PlugIn.)

**Parameters:**
- `command` (Rhino.Commands.Command) — [Missing <param name="command"/> documentation for "M:Rhino.PlugIns.PlugIn.RegisterCommand(Rhino.Commands.Command)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.RegisterCommand(Rhino.Commands.Command)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_RegisterCommand.htm)

#### `protected virtual void ResetMessageBoxes()`

(Inherited from PlugIn.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ResetMessageBoxes.htm)

#### `protected bool ReturnLicense()`

Returns, or releases, a product license that was obtained from the Rhino licensing system. Note, most plug-ins do not need to call this as the Rhino licensing system will return all licenses when Rhino shuts down.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.ReturnLicense"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ReturnLicense.htm)

#### `public void SaveSettings()`

Write settings to disk which will raise a SettingsSaved event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_SaveSettings.htm)

#### `protected void SetLicenseCapabilities(string textMask, LicenseCapabilities capabilities, Guid licenseId)`

(Inherited from PlugIn.)

**Parameters:**
- `textMask` (System.String) — [Missing <param name="textMask"/> documentation for "M:Rhino.PlugIns.PlugIn.SetLicenseCapabilities(System.String,Rhino.PlugIns.LicenseCapabilities,System.Guid)"]
- `capabilities` (Rhino.PlugIns.LicenseCapabilities) — [Missing <param name="capabilities"/> documentation for "M:Rhino.PlugIns.PlugIn.SetLicenseCapabilities(System.String,Rhino.PlugIns.LicenseCapabilities,System.Guid)"]
- `licenseId` (System.Guid) — [Missing <param name="licenseId"/> documentation for "M:Rhino.PlugIns.PlugIn.SetLicenseCapabilities(System.String,Rhino.PlugIns.LicenseCapabilities,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_SetLicenseCapabilities.htm)

#### `protected virtual bool ShouldCallWriteDocument(FileWriteOptions options)`

Called whenever a Rhino is about to save a .3dm file. If you want to save plug-in document data when a model is saved in a version 5 .3dm file, then you must override this function to return true and you must override WriteDocument().

**Parameters:**
- `options` (Rhino.FileIO.FileWriteOptions) — The file write options, such as "include preview image" and "include render meshes".

**Returns:** `Boolean` — true if the plug-in wants to save document user data in the version 5 .3dm file. The default returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ShouldCallWriteDocument.htm)

#### `protected virtual void WriteDocument(RhinoDoc doc, BinaryArchiveWriter archive, FileWriteOptions options)`

Called when Rhino is saving a .3dm file to allow the plug-in to save document user data.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — The Rhino document instance that is being saved.
- `archive` (Rhino.FileIO.BinaryArchiveWriter) — OpenNURBS file archive object Rhino is using to write the file. Use BinaryArchiveWriter.Write*() functions to write plug-in data. OR use the ArchivableDictionary If any BinaryArchiveWriter.Write*() functions throw an exception, then archive.WriteErrorOccured will be true and you should immediately return. Setting archive.WriteErrorOccured to true will cause Rhino to stop saving the file.
- `options` (Rhino.FileIO.FileWriteOptions) — The file write options, such as "include preview image" and "include render meshes".

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_WriteDocument.htm)

#### `protected abstract WriteFileResult WriteFile(string filename, int index, RhinoDoc doc, FileWriteOptions options)`



**Parameters:**
- `filename` (System.String) — [Missing <param name="filename"/> documentation for "M:Rhino.PlugIns.FileExportPlugIn.WriteFile(System.String,System.Int32,Rhino.RhinoDoc,Rhino.FileIO.FileWriteOptions)"]
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.PlugIns.FileExportPlugIn.WriteFile(System.String,System.Int32,Rhino.RhinoDoc,Rhino.FileIO.FileWriteOptions)"]
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.PlugIns.FileExportPlugIn.WriteFile(System.String,System.Int32,Rhino.RhinoDoc,Rhino.FileIO.FileWriteOptions)"]
- `options` (Rhino.FileIO.FileWriteOptions) — [Missing <param name="options"/> documentation for "M:Rhino.PlugIns.FileExportPlugIn.WriteFile(System.String,System.Int32,Rhino.RhinoDoc,Rhino.FileIO.FileWriteOptions)"]

**Returns:** `WriteFileResult` — [Missing <returns> documentation for "M:Rhino.PlugIns.FileExportPlugIn.WriteFile(System.String,System.Int32,Rhino.RhinoDoc,Rhino.FileIO.FileWriteOptions)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_FileExportPlugIn_WriteFile.htm)

### Properties
- `AddToHelpMenu` (Boolean, get) — Called by Rhino to determine if the plug-in name should be added to the Rhino Help/Plug-ins menu.
- `Assembly` (Assembly, get) — Source assembly for this plug-in.
- `Description` (String, get) — Returns the description of the plug-in, as found in the plug-in's assembly attributes.
- `Id` (Guid, get) — Returns the Guid, or unique Id, of the plug-in.
- `LicenseId` (Guid, get) — (Inherited from PlugIn.)
- `LoadAtStartup` (Boolean, get) — Obsolete. (Inherited from PlugIn.)
- `LoadTime` (PlugInLoadTime, get) — Plug-ins are typically loaded on demand when they are first needed. You can change this behavior to load the plug-in at during different stages in time by overriding this property.
- `LocalPlugInName` (String, get) — Optionally override this to provide a localized plug-in name
- `Name` (String, get) — Returns the name of the plug-in, as found in the plug-in's assembly attributes.
- `Settings` (PersistentSettings, get) — Persistent plug-in settings.
- `SettingsDirectory` (String, get) — Get the plug-in's settings directory. This is the directory where the plug-in's persistent settings files are saved. This directory will be located in the user's profile folder. Note, this does not verify the directory exists.
- `SettingsDirectoryAllUsers` (String, get) — Get the plug-in's "all users" settings directory. This directory will be located in the system's program data folder. Note, this does not verify the directory exists.
- `ShouldDisplayOptionsDialog` (Boolean, get) — 
- `Version` (String, get) — Returns the version of the plug-in, as found in the plug-in's assembly attributes.
- `WindowPositionSettings` (PersistentSettings, get) — (Inherited from PlugIn.)

### Events
#### `SettingsSaved` (`System.EventHandler<PersistentSettingsSavedEventArgs>`)

**Signature:** `public event EventHandler<PersistentSettingsSavedEventArgs> SettingsSaved`

This event is raised when an instance of Rhino has modified the external settings file associated with this plug-in's Settings property.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_PlugIns_PlugIn_SettingsSaved.htm)

## FileImportPlugIn (class)

Rhino plug-in that imports data from other file formats into Rhino; can support more that one format.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_FileImportPlugIn.htm)

### Constructors
- `protected FileImportPlugIn()` — Initializes a new instance of the FileImportPlugIn class

### Methods
#### `protected abstract FileTypeList AddFileTypes(FileReadOptions options)`



**Parameters:**
- `options` (Rhino.FileIO.FileReadOptions) — [Missing <param name="options"/> documentation for "M:Rhino.PlugIns.FileImportPlugIn.AddFileTypes(Rhino.FileIO.FileReadOptions)"]

**Returns:** `FileTypeList` — [Missing <returns> documentation for "M:Rhino.PlugIns.FileImportPlugIn.AddFileTypes(Rhino.FileIO.FileReadOptions)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_FileImportPlugIn_AddFileTypes.htm)

#### `protected bool AskUserForLicense(LicenseBuildType productBuildType, bool standAlone, string textMask, Object parentWindow, ValidateProductKeyDelegate validateProductKeyDelegate, OnLeaseChangedDelegate onLeaseChangedDelegate)`

(Inherited from PlugIn.)

**Parameters:**
- `productBuildType` (Rhino.PlugIns.LicenseBuildType) — [Missing <param name="productBuildType"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `standAlone` (System.Boolean) — [Missing <param name="standAlone"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `textMask` (System.String) — [Missing <param name="textMask"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `parentWindow` (System.Object) — [Missing <param name="parentWindow"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `validateProductKeyDelegate` (Rhino.PlugIns.ValidateProductKeyDelegate) — [Missing <param name="validateProductKeyDelegate"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `onLeaseChangedDelegate` (Rhino.PlugIns.OnLeaseChangedDelegate) — [Missing <param name="onLeaseChangedDelegate"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_AskUserForLicense.htm)

#### `public PersistentSettings CommandSettings(string name)`

(Inherited from PlugIn.)

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.PlugIns.PlugIn.CommandSettings(System.String)"]

**Returns:** `PersistentSettings` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.CommandSettings(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_CommandSettings.htm)

#### `protected virtual void CreateCommands()`

Called right after plug-in is created and is responsible for creating all of the commands in a given plug-in. The base class implementation Constructs an instance of every publicly exported command class in your plug-in's assembly.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_CreateCommands.htm)

#### `public virtual bool DisplayHelp(IntPtr windowHandle)`

Called by Rhino if AddToHelpMenu is true and menu item associated with this plug-in is selected.

**Parameters:**
- `windowHandle` (System.IntPtr) — Native Window handle of the active Rhino interface.

**Returns:** `Boolean` — true = Help displayed successfully, false = Error displaying help

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_DisplayHelp.htm)

#### `protected virtual void DisplayOptionsDialog(IntPtr parent, string description, string extension)`



**Parameters:**
- `parent` (System.IntPtr) — [Missing <param name="parent"/> documentation for "M:Rhino.PlugIns.FileImportPlugIn.DisplayOptionsDialog(System.IntPtr,System.String,System.String)"]
- `description` (System.String) — [Missing <param name="description"/> documentation for "M:Rhino.PlugIns.FileImportPlugIn.DisplayOptionsDialog(System.IntPtr,System.String,System.String)"]
- `extension` (System.String) — [Missing <param name="extension"/> documentation for "M:Rhino.PlugIns.FileImportPlugIn.DisplayOptionsDialog(System.IntPtr,System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_FileImportPlugIn_DisplayOptionsDialog.htm)

#### `protected virtual void DocumentPropertiesDialogPages(RhinoDoc doc, List<OptionsDialogPage> pages)`

Override this function if you want to extend the document properties sections of the options dialog. This function is called whenever the user brings up the Options dialog.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — document that the pages are set up for
- `pages` (System.Collections.Generic.List<OptionsDialogPage>) — list of pages to add your custom options dialog page(s) to.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_DocumentPropertiesDialogPages.htm)

#### `public Command[] GetCommands()`

All of the commands associated with this plug-in.

**Returns:** `Command[]` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.GetCommands"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetCommands.htm)

#### `protected bool GetLicense(LicenseBuildType productBuildType, ValidateProductKeyDelegate validateProductKeyDelegate, OnLeaseChangedDelegate leaseChangedDelegate)`

Verifies that there is a valid product license for your plug-in, using the Rhino licensing system. If the plug-in is installed as a standalone node, the locally installed license will be validated. If the plug-in is installed as a network node, a loaner license will be requested by the system's assigned Zoo server. If the Zoo server finds and returns a license, then this license will be validated. If no license is found, then the user will be prompted to provide a license key, which will be validated.

**Parameters:**
- `productBuildType` (Rhino.PlugIns.LicenseBuildType) — The product build contentType required by your plug-in.
- `validateProductKeyDelegate` (Rhino.PlugIns.ValidateProductKeyDelegate) — Since the Rhino licensing system knows nothing about your product license, you will need to validate the product license provided by the Rhino licensing system. This is done by supplying a callback function, or delegate, that can be called to perform the validation.
- `leaseChangedDelegate` (Rhino.PlugIns.OnLeaseChangedDelegate) — Called by the ZooClient when the cloud zoo lease is changed.

**Returns:** `Boolean` — true if a valid license was found. false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetLicense.htm)

#### `protected bool GetLicense(LicenseCapabilities licenseCapabilities, string textMask, ValidateProductKeyDelegate validateProductKeyDelegate, OnLeaseChangedDelegate leaseChangedDelegate)`

Verifies that there is a valid product license for your plug-in, using the Rhino licensing system. If the plug-in is installed as a standalone node, the locally installed license will be validated. If the plug-in is installed as a network node, a loaner license will be requested by the system's assigned Zoo server. If the Zoo server finds and returns a license, then this license will be validated. If no license is found, then the user will be prompted to provide a license key, which will be validated.

**Parameters:**
- `licenseCapabilities` (Rhino.PlugIns.LicenseCapabilities) — In the event that a license was not found, or if the user wants to change the way your plug-in is licenses, then provide what capabilities your license has by using this enumeration flag.
- `textMask` (System.String) — In the event that the user needs to be asked for a license, then you can provide a text mask, which helps the user to distinguish between proper and improper user input of your license code. Note, if you do not want to use a text mask, then pass in a null value for this parameter. For more information on text masks, search MSDN for the System.Windows.Forms.MaskedTextBox class.
- `validateProductKeyDelegate` (Rhino.PlugIns.ValidateProductKeyDelegate) — Since the Rhino licensing system knows nothing about your product license, you will need to validate the product license provided by the Rhino licensing system. This is done by supplying a callback function, or delegate, that can be called to perform the validation.
- `leaseChangedDelegate` (Rhino.PlugIns.OnLeaseChangedDelegate) — Called by the ZooClient when the cloud zoo lease is changed.

**Returns:** `Boolean` — true if a valid license was found. false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetLicense_1.htm)

#### `protected bool GetLicenseOwner(out string registeredOwner, out string registeredOrganization)`

Get the customer name and organization used when entering the product license.

**Parameters:**
- `registeredOwner` (System.String) — [Missing <param name="registeredOwner"/> documentation for "M:Rhino.PlugIns.PlugIn.GetLicenseOwner(System.String@,System.String@)"]
- `registeredOrganization` (System.String) — [Missing <param name="registeredOrganization"/> documentation for "M:Rhino.PlugIns.PlugIn.GetLicenseOwner(System.String@,System.String@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.GetLicenseOwner(System.String@,System.String@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetLicenseOwner.htm)

#### `public virtual Object GetPlugInObject()`

(Inherited from PlugIn.)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.GetPlugInObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetPlugInObject.htm)

#### `public Bitmap Icon(Size size)`

Returns the plug-in's icon in bitmap form.

**Parameters:**
- `size` (System.Drawing.Size) — The desired size in pixels.

**Returns:** `Bitmap` — The icon if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_Icon.htm)

#### `public virtual bool IsTextureSupported(RenderTexture texture)`

Returns true if this renderer can render the texture natively without needing it to be baked into a bitmap, false otherwise. By default, returns false for all textures.

**Parameters:**
- `texture` (Rhino.Render.RenderTexture) — [Missing <param name="texture"/> documentation for "M:Rhino.PlugIns.PlugIn.IsTextureSupported(Rhino.Render.RenderTexture)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.IsTextureSupported(Rhino.Render.RenderTexture)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_IsTextureSupported.htm)

#### `protected string MakeReferenceTableName(string nameToPrefix)`



**Parameters:**
- `nameToPrefix` (System.String) — [Missing <param name="nameToPrefix"/> documentation for "M:Rhino.PlugIns.FileImportPlugIn.MakeReferenceTableName(System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.PlugIns.FileImportPlugIn.MakeReferenceTableName(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_FileImportPlugIn_MakeReferenceTableName.htm)

#### `[ObsoleteAttribute("Use ObjectPropertiesPages(ObjectPropertiesPageCollection collection) instead")] protected virtual void ObjectPropertiesPages(List<ObjectPropertiesPage> pages)`

Override this function is you want to extend the object properties dialog

**Parameters:**
- `pages` (System.Collections.Generic.List<ObjectPropertiesPage>) — [Missing <param name="pages"/> documentation for "M:Rhino.PlugIns.PlugIn.ObjectPropertiesPages(System.Collections.Generic.List{Rhino.UI.ObjectPropertiesPage})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ObjectPropertiesPages_1.htm)

#### `protected virtual void ObjectPropertiesPages(ObjectPropertiesPageCollection collection)`

Override this function is you want to extend the object properties dialog. This method will be called each time a new document is created for each instance of a object properties panel. On Windows there will be a single panel per document but on Mac there may be many properties panel per document.

**Parameters:**
- `collection` (Rhino.UI.ObjectPropertiesPageCollection) — Add custom pages by calling collection.Add

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ObjectPropertiesPages.htm)

#### `protected virtual LoadReturnCode OnLoad(ref string errorMessage)`

Is called when the plug-in is being loaded.

**Parameters:**
- `errorMessage` (System.String) — If a load error is returned and this string is set. This string is the error message that will be reported back to the user.

**Returns:** `LoadReturnCode` — An appropriate load return code. The default implementation returns Success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_OnLoad.htm)

#### `protected virtual void OnShutdown()`

(Inherited from PlugIn.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_OnShutdown.htm)

#### `protected virtual void OptionsDialogPages(List<OptionsDialogPage> pages)`

Override this function if you want to extend the options dialog. This function is called whenever the user brings up the Options dialog.

**Parameters:**
- `pages` (System.Collections.Generic.List<OptionsDialogPage>) — list of pages to add your custom options dialog page(s) to.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_OptionsDialogPages.htm)

#### `protected virtual void ReadDocument(RhinoDoc doc, BinaryArchiveReader archive, FileReadOptions options)`

Called whenever a Rhino document is being loaded and plug-in user data was encountered written by a plug-in with this plug-in's GUID.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — A Rhino document that is being loaded.
- `archive` (Rhino.FileIO.BinaryArchiveReader) — OpenNURBS file archive object Rhino is using to read this file. Use BinaryArchiveReader.Read*() functions to read plug-in data. If any BinaryArchive.Read*() functions throws an exception then archive.ReadErrorOccurve will be true and you should immediately return.
- `options` (Rhino.FileIO.FileReadOptions) — Describes what is being written.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ReadDocument.htm)

#### `protected abstract bool ReadFile(string filename, int index, RhinoDoc doc, FileReadOptions options)`



**Parameters:**
- `filename` (System.String) — [Missing <param name="filename"/> documentation for "M:Rhino.PlugIns.FileImportPlugIn.ReadFile(System.String,System.Int32,Rhino.RhinoDoc,Rhino.FileIO.FileReadOptions)"]
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.PlugIns.FileImportPlugIn.ReadFile(System.String,System.Int32,Rhino.RhinoDoc,Rhino.FileIO.FileReadOptions)"]
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.PlugIns.FileImportPlugIn.ReadFile(System.String,System.Int32,Rhino.RhinoDoc,Rhino.FileIO.FileReadOptions)"]
- `options` (Rhino.FileIO.FileReadOptions) — [Missing <param name="options"/> documentation for "M:Rhino.PlugIns.FileImportPlugIn.ReadFile(System.String,System.Int32,Rhino.RhinoDoc,Rhino.FileIO.FileReadOptions)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.FileImportPlugIn.ReadFile(System.String,System.Int32,Rhino.RhinoDoc,Rhino.FileIO.FileReadOptions)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_FileImportPlugIn_ReadFile.htm)

#### `protected bool RegisterCommand(Command command)`

(Inherited from PlugIn.)

**Parameters:**
- `command` (Rhino.Commands.Command) — [Missing <param name="command"/> documentation for "M:Rhino.PlugIns.PlugIn.RegisterCommand(Rhino.Commands.Command)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.RegisterCommand(Rhino.Commands.Command)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_RegisterCommand.htm)

#### `protected virtual void ResetMessageBoxes()`

(Inherited from PlugIn.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ResetMessageBoxes.htm)

#### `protected bool ReturnLicense()`

Returns, or releases, a product license that was obtained from the Rhino licensing system. Note, most plug-ins do not need to call this as the Rhino licensing system will return all licenses when Rhino shuts down.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.ReturnLicense"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ReturnLicense.htm)

#### `public void SaveSettings()`

Write settings to disk which will raise a SettingsSaved event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_SaveSettings.htm)

#### `protected void SetLicenseCapabilities(string textMask, LicenseCapabilities capabilities, Guid licenseId)`

(Inherited from PlugIn.)

**Parameters:**
- `textMask` (System.String) — [Missing <param name="textMask"/> documentation for "M:Rhino.PlugIns.PlugIn.SetLicenseCapabilities(System.String,Rhino.PlugIns.LicenseCapabilities,System.Guid)"]
- `capabilities` (Rhino.PlugIns.LicenseCapabilities) — [Missing <param name="capabilities"/> documentation for "M:Rhino.PlugIns.PlugIn.SetLicenseCapabilities(System.String,Rhino.PlugIns.LicenseCapabilities,System.Guid)"]
- `licenseId` (System.Guid) — [Missing <param name="licenseId"/> documentation for "M:Rhino.PlugIns.PlugIn.SetLicenseCapabilities(System.String,Rhino.PlugIns.LicenseCapabilities,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_SetLicenseCapabilities.htm)

#### `protected virtual bool ShouldCallWriteDocument(FileWriteOptions options)`

Called whenever a Rhino is about to save a .3dm file. If you want to save plug-in document data when a model is saved in a version 5 .3dm file, then you must override this function to return true and you must override WriteDocument().

**Parameters:**
- `options` (Rhino.FileIO.FileWriteOptions) — The file write options, such as "include preview image" and "include render meshes".

**Returns:** `Boolean` — true if the plug-in wants to save document user data in the version 5 .3dm file. The default returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ShouldCallWriteDocument.htm)

#### `protected virtual void WriteDocument(RhinoDoc doc, BinaryArchiveWriter archive, FileWriteOptions options)`

Called when Rhino is saving a .3dm file to allow the plug-in to save document user data.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — The Rhino document instance that is being saved.
- `archive` (Rhino.FileIO.BinaryArchiveWriter) — OpenNURBS file archive object Rhino is using to write the file. Use BinaryArchiveWriter.Write*() functions to write plug-in data. OR use the ArchivableDictionary If any BinaryArchiveWriter.Write*() functions throw an exception, then archive.WriteErrorOccured will be true and you should immediately return. Setting archive.WriteErrorOccured to true will cause Rhino to stop saving the file.
- `options` (Rhino.FileIO.FileWriteOptions) — The file write options, such as "include preview image" and "include render meshes".

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_WriteDocument.htm)

### Properties
- `AddToHelpMenu` (Boolean, get) — Called by Rhino to determine if the plug-in name should be added to the Rhino Help/Plug-ins menu.
- `Assembly` (Assembly, get) — Source assembly for this plug-in.
- `Description` (String, get) — Returns the description of the plug-in, as found in the plug-in's assembly attributes.
- `Id` (Guid, get) — Returns the Guid, or unique Id, of the plug-in.
- `LicenseId` (Guid, get) — (Inherited from PlugIn.)
- `LoadAtStartup` (Boolean, get) — Obsolete. (Inherited from PlugIn.)
- `LoadTime` (PlugInLoadTime, get) — Plug-ins are typically loaded on demand when they are first needed. You can change this behavior to load the plug-in at during different stages in time by overriding this property.
- `LocalPlugInName` (String, get) — Optionally override this to provide a localized plug-in name
- `Name` (String, get) — Returns the name of the plug-in, as found in the plug-in's assembly attributes.
- `Settings` (PersistentSettings, get) — Persistent plug-in settings.
- `SettingsDirectory` (String, get) — Get the plug-in's settings directory. This is the directory where the plug-in's persistent settings files are saved. This directory will be located in the user's profile folder. Note, this does not verify the directory exists.
- `SettingsDirectoryAllUsers` (String, get) — Get the plug-in's "all users" settings directory. This directory will be located in the system's program data folder. Note, this does not verify the directory exists.
- `Version` (String, get) — Returns the version of the plug-in, as found in the plug-in's assembly attributes.
- `WindowPositionSettings` (PersistentSettings, get) — (Inherited from PlugIn.)

### Events
#### `SettingsSaved` (`System.EventHandler<PersistentSettingsSavedEventArgs>`)

**Signature:** `public event EventHandler<PersistentSettingsSavedEventArgs> SettingsSaved`

This event is raised when an instance of Rhino has modified the external settings file associated with this plug-in's Settings property.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_PlugIns_PlugIn_SettingsSaved.htm)

## FileTypeList (class)

[Missing <summary> documentation for "T:Rhino.PlugIns.FileTypeList"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_FileTypeList.htm)

### Constructors
- `public FileTypeList()` — Initializes a new instance of the FileTypeList class
- `public FileTypeList(string description, string extension)` — Initializes a new instance of the FileTypeList class
- `public FileTypeList(string description, string extension, bool showOptionsButtonInFileDialog)` — Initializes a new instance of the FileTypeList class

### Methods
#### `public int AddFileType(string description, IEnumerable<string> extensions)`



**Parameters:**
- `description` (System.String) — [Missing <param name="description"/> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.Collections.Generic.IEnumerable{System.String})"]
- `extensions` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="extensions"/> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_FileTypeList_AddFileType.htm)

#### `public int AddFileType(string description, IEnumerable<string> extensions, bool showOptionsButtonInFileDialog)`



**Parameters:**
- `description` (System.String) — [Missing <param name="description"/> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.Collections.Generic.IEnumerable{System.String},System.Boolean)"]
- `extensions` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="extensions"/> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.Collections.Generic.IEnumerable{System.String},System.Boolean)"]
- `showOptionsButtonInFileDialog` (System.Boolean) — [Missing <param name="showOptionsButtonInFileDialog"/> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.Collections.Generic.IEnumerable{System.String},System.Boolean)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.Collections.Generic.IEnumerable{System.String},System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_FileTypeList_AddFileType_1.htm)

#### `public int AddFileType(string description, string extension)`



**Parameters:**
- `description` (System.String) — [Missing <param name="description"/> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.String)"]
- `extension` (System.String) — [Missing <param name="extension"/> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.String)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_FileTypeList_AddFileType_2.htm)

#### `public int AddFileType(string description, string extension, bool showOptionsButtonInFileDialog)`



**Parameters:**
- `description` (System.String) — [Missing <param name="description"/> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.String,System.Boolean)"]
- `extension` (System.String) — [Missing <param name="extension"/> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.String,System.Boolean)"]
- `showOptionsButtonInFileDialog` (System.Boolean) — [Missing <param name="showOptionsButtonInFileDialog"/> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.String,System.Boolean)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_FileTypeList_AddFileType_3.htm)

#### `public int AddFileType(string description, string extension1, string extension2)`



**Parameters:**
- `description` (System.String) — [Missing <param name="description"/> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.String,System.String)"]
- `extension1` (System.String) — [Missing <param name="extension1"/> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.String,System.String)"]
- `extension2` (System.String) — [Missing <param name="extension2"/> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.String,System.String)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_FileTypeList_AddFileType_4.htm)

#### `public int AddFileType(string description, string extension1, string extension2, bool showOptionsButtonInFileDialog)`



**Parameters:**
- `description` (System.String) — [Missing <param name="description"/> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.String,System.String,System.Boolean)"]
- `extension1` (System.String) — [Missing <param name="extension1"/> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.String,System.String,System.Boolean)"]
- `extension2` (System.String) — [Missing <param name="extension2"/> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.String,System.String,System.Boolean)"]
- `showOptionsButtonInFileDialog` (System.Boolean) — [Missing <param name="showOptionsButtonInFileDialog"/> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.String,System.String,System.Boolean)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.PlugIns.FileTypeList.AddFileType(System.String,System.String,System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_FileTypeList_AddFileType_5.htm)

## LicenseBuildType (enum)

License build contentType enumerations.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_LicenseBuildType.htm)

### Values
- `Unspecified` = `0` — An unspecified build
- `Release` = `100` — A release build (e.g. commercial, education, NFR, etc.)
- `Evaluation` = `200` — A evaluation build
- `Beta` = `300` — A beta build (e.g. WIP)

## LicenseCapabilities (enum)

Controls the buttons that will appear on the license notification window that is displayed if a license for the requesting product is not found. Note, the "Close" button will always be displayed.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_LicenseCapabilities.htm)

### Values
- `NoCapabilities` = `0` — Only the "Close" button will be displayed
- `CanBePurchased` = `1` — Shows "Buy a license" button
- `CanBeSpecified` = `2` — OBSOLETE: Shows ""Enter a license" and "Use a Zoo" buttons. Use SupportsStandalone | SupportsZoo instead.
- `CanBeEvaluated` = `4` — Shows "Evaluate" button
- `EvaluationIsExpired` = `8` — Shows "Evaluate" button disabled
- `SupportsRhinoAccounts` = `16` — Supports getting a license from a Cloud Zoo / Rhino Account
- `SupportsStandalone` = `32` — Supports single-computer licensing
- `SupportsZooPerUser` = `64` — Supports getting a license from a Zoo server
- `SupportsZooPerCore` = `128` — Supports getting a license from a Zoo server
- `SupportsLicenseDiscovery` = `256` — Supports license discovery API flow

## LicenseChangedEventArgs (class)

[Missing <summary> documentation for "T:Rhino.PlugIns.LicenseChangedEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_LicenseChangedEventArgs.htm)

### Constructors
- `public LicenseChangedEventArgs()` — Initializes a new instance of the LicenseChangedEventArgs class

## LicenseData (class)

Zoo plug-in license data.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_LicenseData.htm)

### Constructors
- `public LicenseData()` — Public constructor.
- `public LicenseData(string productLicense, string serialNumber, string licenseTitle)` — Public constructor
- `public LicenseData(string productLicense, string serialNumber, string licenseTitle, LicenseBuildType buildType)` — Public constructor
- `public LicenseData(string productLicense, string serialNumber, string licenseTitle, LicenseBuildType buildType, int licenseCount)` — Public constructor
- `public LicenseData(string productLicense, string serialNumber, string licenseTitle, LicenseBuildType buildType, int licenseCount, DateTime? expirationDate)` — Public constructor
- `public LicenseData(string productLicense, string serialNumber, string licenseTitle, LicenseBuildType buildType, int licenseCount, DateTime? expirationDate, Icon productIcon)` — Public constructor
- `public LicenseData(string productLicense, string serialNumber, string licenseTitle, LicenseBuildType buildType, int licenseCount, DateTime? expirationDate, Icon productIcon, bool requiresOnlineValidation, bool isUpgradeFromPreviousVersion)` — Public constructor

### Methods
#### `public void Dispose()`

Releases all resources used by the LicenseData

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseData_Dispose.htm)

#### `public static bool IsNotValid(LicenseData data)`

Indicates whether a LicenseData object is either null or invalid.

**Parameters:**
- `data` (Rhino.PlugIns.LicenseData) — [Missing <param name="data"/> documentation for "M:Rhino.PlugIns.LicenseData.IsNotValid(Rhino.PlugIns.LicenseData)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.LicenseData.IsNotValid(Rhino.PlugIns.LicenseData)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseData_IsNotValid.htm)

#### `public bool IsValid()`

Public validator.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.LicenseData.IsValid"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseData_IsValid.htm)

#### `public bool IsValid(bool ignoreExpirationDate)`



**Parameters:**
- `ignoreExpirationDate` (System.Boolean) — [Missing <param name="ignoreExpirationDate"/> documentation for "M:Rhino.PlugIns.LicenseData.IsValid(System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.LicenseData.IsValid(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseData_IsValid_2.htm)

#### `public static bool IsValid(LicenseData data)`

Indicates whether a LicenseData object is not null and valid.

**Parameters:**
- `data` (Rhino.PlugIns.LicenseData) — [Missing <param name="data"/> documentation for "M:Rhino.PlugIns.LicenseData.IsValid(Rhino.PlugIns.LicenseData)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.LicenseData.IsValid(Rhino.PlugIns.LicenseData)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseData_IsValid_1.htm)

### Properties
- `BuildType` (LicenseBuildType, get/set) — The build of the product that this license work with. When your product requests a license from the Zoo, it will specify one of these build types.
- `DateToExpire` (Nullable<DateTime>, get/set) — The date and time the license is set to expire. This is provided by the plug in that validated the license. This time value should be in Coordinated Universal Time (UTC).
- `ErrorMessage` (String, get/set) — Error message set by calls to callback functions
- `IsUpgradeFromPreviousVersion` (Boolean, get/set) — Set to true if this license requires a previous version license to be entered. Caller must also pass VerifyPreviousVersionLicenseDelegate to GetLicense/AskUserForLicense.
- `LicenseCount` (Int32, get/set) — The number of instances supported by this license. This is provided by the plug in that validated the license.
- `LicenseExpires` (Boolean, get) — 
- `LicenseTitle` (String, get/set) — The title of the license. This is provided by the plug-in that validated the license. (e.g. "Rhinoceros 6.0 Commercial")
- `ProductIcon` (Icon, get/set) — The product's icon. This will displayed in the "license" page in the Options dialog. Note, this can be null. Note, LicenseData creates it's own copy of the icon.
- `ProductLicense` (String, get/set) — The actual product license. This is provided by the plug-in that validated the license.
- `RequiresOnlineValidation` (Boolean, get/set) — Set to true if this license requires online validation. Caller must also pass VerifyOnlineValidationCodeDelegate to GetLicense/AskUserForLicense
- `SerialNumber` (String, get/set) — The "for display only" product license. This is provided by the plug-in that validated the license.

## LicenseIdAttribute (class)

[Missing <summary> documentation for "T:Rhino.PlugIns.LicenseIdAttribute"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_LicenseIdAttribute.htm)

### Constructors
- `public LicenseIdAttribute(string value)` — Initializes a new instance of the LicenseIdAttribute class

### Properties
- `Value` (String, get) — 

## LicenseLease (class)

LicenseLease represents a lease returned from the Cloud Zoo

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_LicenseLease.htm)

### Constructors
- `public LicenseLease(IntPtr unmanagedPointer)` — Initializes a new instance of the LicenseLease class
- `public LicenseLease(string productId, string groupName, string groupId, string userName, string userId, string productTitle, string productVersion, string productEdition, string leaseId, DateTime iat, DateTime exp)` — Initializes a new instance of the LicenseLease class
- `public LicenseLease(string productId, string groupName, string groupId, string userName, string userId, string productTitle, string productVersion, string productEdition, string leaseId, DateTime iat, DateTime exp, DateTime renewable_until)` — Initializes a new instance of the LicenseLease class

### Methods
#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseLease_Finalize.htm)

### Properties
- `Expiration` (DateTime, get) — The date when this lease will expire
- `GroupId` (String, get) — ID of Rhino Accounts group that this lease came from
- `GroupName` (String, get) — Name of Rhino Accounts group that this lease came from
- `IssuedAt` (DateTime, get) — The date this lease was issued
- `LeaseId` (String, get) — The ID of this lease.
- `ProductEdition` (String, get) — Edition of product that this lease is issued to. For example, "Commercial" or "Beta"
- `ProductId` (String, get) — The ID of the product that this lease is issued to
- `ProductTitle` (String, get) — Title of product that this lease is issued to. For example, "Rhino 6"
- `ProductVersion` (String, get) — Version of product that this lease is issued to. For example, "6.0"
- `RenewableUntil` (Nullable<DateTime>, get) — The date when the license in the cloud zoo entity will expire, if any
- `UserId` (String, get) — ID of Rhino Accounts user that was logged in when this lease was obtained
- `UserName` (String, get) — Name of Rhino Accounts user that was logged in when this lease was obtained

## LicenseLeaseChangedEventArgs (class)

Arguments for OnLeaseChangedDelegate

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_LicenseLeaseChangedEventArgs.htm)

### Constructors
- `public LicenseLeaseChangedEventArgs(LicenseLease lease)` — Initializes a new instance of the LicenseLeaseChangedEventArgs class

### Methods
#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseLeaseChangedEventArgs_Finalize.htm)

### Properties
- `Lease` (LicenseLease, get) — The lease returned by Rhino Accounts server

## LicenseStatus (class)

LicenseStatus class.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_LicenseStatus.htm)

### Constructors
- `public LicenseStatus()` — Public constructor.

### Properties
- `BuildType` (LicenseBuildType, get/set) — The build contentType of the product, where: 100 = A release build, either commercial, education, NFR, etc. 200 = A evaluation build 300 = A beta build, such as a WIP.
- `CheckOutExpirationDate` (Nullable<DateTime>, get/set) — The date and time the checked out license will expire. Note, this is only set if m_license_type = Standalone or CloudZoo and if "limited license checkout" was enabled on the Zoo server in the case of Standalone. Note, date and time is in local time coordinates.
- `CloudZooLeaseExpiration` (Nullable<DateTime>, get/set) — Returns the expiration date of the lease this instance represents.
- `CloudZooLeaseIsValid` (Boolean, get/set) — Returns true if the Cloud Zoo Lease represented by this instance is actively being managed by the Cloud Zoo Manager; else returns false.
- `ExpirationDate` (Nullable<DateTime>, get/set) — The date and time the license will expire. This value can be null if: 1.) The license contentType is "Standalone" and the license does not expire. 2.) The license contentType is "Network". 3.) The license contentType is "NetworkCheckedOut" and the checkout does not expire Note, date and time is in local time coordinates.
- `LicenseTitle` (String, get/set) — The title of the license. (e.g. "Rhinoceros 6.0 Commercial")
- `LicenseType` (LicenseType, get/set) — The license contentType. (e.g. Standalone, Network, etc.)
- `PluginId` (Guid, get/set) — The ID of the plug-in that owns this license information
- `ProductIcon` (Icon, get/set) — The product's icon. Note, this can be null.
- `ProductId` (Guid, get/set) — The id of the product or plug in.
- `RegisteredOrganization` (String, get/set) — The registered organization of the product (e.g. "Robert McNeel and Associates")
- `RegisteredOwner` (String, get/set) — The registered owner of the product. (e.g. "Dale Fugier")
- `SerialNumber` (String, get/set) — The "for display only" product license or serial number.

## LicenseType (enum)

LicenseType enumeration.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_LicenseType.htm)

### Values
- `Standalone` = `0` — A standalone license
- `Network` = `1` — A network license that has not been fulfilled by a Zoo
- `NetworkLoanedOut` = `2` — A license on temporary loan from a Zoo
- `NetworkCheckedOut` = `3` — A license on permanent check out from a Zoo
- `CloudZoo` = `4` — A lease granted by the Cloud Zoo

## LicenseUtils (class)

License Manager Utilities.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_LicenseUtils.htm)

### Methods
#### `public static bool AskUserForLicense(int productType, bool standAlone, Object parentWindow, string textMask, ValidateProductKeyDelegate validateProductKeyDelegate, OnLeaseChangedDelegate onLeaseChangedDelegate, string product_path, string product_title, Guid pluginId, Guid licenseId, LicenseCapabilities capabilities)`

This version of Rhino.PlugIns.LicenseUtils.AskUserForLicense is used by Rhino C++ plug-ins.

**Parameters:**
- `productType` (System.Int32) — [Missing <param name="productType"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]
- `standAlone` (System.Boolean) — [Missing <param name="standAlone"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]
- `parentWindow` (System.Object) — [Missing <param name="parentWindow"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]
- `textMask` (System.String) — [Missing <param name="textMask"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]
- `validateProductKeyDelegate` (Rhino.PlugIns.ValidateProductKeyDelegate) — [Missing <param name="validateProductKeyDelegate"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]
- `onLeaseChangedDelegate` (Rhino.PlugIns.OnLeaseChangedDelegate) — [Missing <param name="onLeaseChangedDelegate"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]
- `product_path` (System.String) — [Missing <param name="product_path"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]
- `product_title` (System.String) — [Missing <param name="product_title"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]
- `pluginId` (System.Guid) — [Missing <param name="pluginId"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]
- `licenseId` (System.Guid) — [Missing <param name="licenseId"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]
- `capabilities` (Rhino.PlugIns.LicenseCapabilities) — [Missing <param name="capabilities"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseUtils_AskUserForLicense_1.htm)

#### `public static bool AskUserForLicense(int productType, bool standAlone, Object parentWindow, string textMask, ValidateProductKeyDelegate validateProductKeyDelegate, OnLeaseChangedDelegate onLeaseChangedDelegate, VerifyLicenseKeyDelegate verifyLicenseKeyDelegate, VerifyPreviousVersionLicenseDelegate verifyPreviousVersionLicenseKeyDelegate, string product_path, string product_title, Guid pluginId, Guid licenseId, LicenseCapabilities capabilities)`



**Parameters:**
- `productType` (System.Int32) — [Missing <param name="productType"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]
- `standAlone` (System.Boolean) — [Missing <param name="standAlone"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]
- `parentWindow` (System.Object) — [Missing <param name="parentWindow"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]
- `textMask` (System.String) — [Missing <param name="textMask"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]
- `validateProductKeyDelegate` (Rhino.PlugIns.ValidateProductKeyDelegate) — [Missing <param name="validateProductKeyDelegate"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]
- `onLeaseChangedDelegate` (Rhino.PlugIns.OnLeaseChangedDelegate) — [Missing <param name="onLeaseChangedDelegate"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]
- `verifyLicenseKeyDelegate` (Rhino.PlugIns.VerifyLicenseKeyDelegate) — [Missing <param name="verifyLicenseKeyDelegate"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]
- `verifyPreviousVersionLicenseKeyDelegate` (Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate) — [Missing <param name="verifyPreviousVersionLicenseKeyDelegate"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]
- `product_path` (System.String) — [Missing <param name="product_path"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]
- `product_title` (System.String) — [Missing <param name="product_title"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]
- `pluginId` (System.Guid) — [Missing <param name="pluginId"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]
- `licenseId` (System.Guid) — [Missing <param name="licenseId"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]
- `capabilities` (Rhino.PlugIns.LicenseCapabilities) — [Missing <param name="capabilities"/> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.LicenseUtils.AskUserForLicense(System.Int32,System.Boolean,System.Object,System.String,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.String,System.String,System.Guid,System.Guid,Rhino.PlugIns.LicenseCapabilities)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseUtils_AskUserForLicense.htm)

#### `public static bool CheckInLicense(Guid productId)`

Checks in a previously checked out license to the Zoo server from which it was checked out.

**Parameters:**
- `productId` (System.Guid) — The Guid of the product that you want to check in.

**Returns:** `Boolean` — true if the license was checked in successful. false if not successful or on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseUtils_CheckInLicense.htm)

#### `public static bool CheckOutLicense(Guid productId)`

Checks out a license that is on loan from a Zoo server on a permanent basis.

**Parameters:**
- `productId` (System.Guid) — The Guid of the product that you want to check out.

**Returns:** `Boolean` — true if the license was checked out successful. false if not successful or on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseUtils_CheckOutLicense.htm)

#### `public static bool ConvertLicense(Guid productId)`

Converts a product license from a standalone node to a network node.

**Parameters:**
- `productId` (System.Guid) — The Guid of the product that you want to check in.

**Returns:** `Boolean` — true if the license was successfully converted. false if not successful or on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseUtils_ConvertLicense.htm)

#### `public static bool DeleteLicense(Guid productId)`

Deletes a license along with its license file.

**Parameters:**
- `productId` (System.Guid) — [Missing <param name="productId"/> documentation for "M:Rhino.PlugIns.LicenseUtils.DeleteLicense(System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.LicenseUtils.DeleteLicense(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseUtils_DeleteLicense.htm)

#### `public static string Echo(string message)`

Test connectivity with the Zoo.

**Parameters:**
- `message` (System.String) — [Missing <param name="message"/> documentation for "M:Rhino.PlugIns.LicenseUtils.Echo(System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.PlugIns.LicenseUtils.Echo(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseUtils_Echo.htm)

#### `public static bool GetLicense(ValidateProductKeyDelegate validateProductKeyDelegate, OnLeaseChangedDelegate leaseChangedDelegate, int product_type, int capabilities, string textMask, string product_path, string product_title, Guid pluginId, Guid licenseId)`

This version of Rhino.PlugIns.LicenseUtils.GetLicense is used by Rhino C++ plug-ins.

**Parameters:**
- `validateProductKeyDelegate` (Rhino.PlugIns.ValidateProductKeyDelegate) — [Missing <param name="validateProductKeyDelegate"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicense(Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,System.Int32,System.Int32,System.String,System.String,System.String,System.Guid,System.Guid)"]
- `leaseChangedDelegate` (Rhino.PlugIns.OnLeaseChangedDelegate) — [Missing <param name="leaseChangedDelegate"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicense(Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,System.Int32,System.Int32,System.String,System.String,System.String,System.Guid,System.Guid)"]
- `product_type` (System.Int32) — [Missing <param name="product_type"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicense(Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,System.Int32,System.Int32,System.String,System.String,System.String,System.Guid,System.Guid)"]
- `capabilities` (System.Int32) — [Missing <param name="capabilities"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicense(Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,System.Int32,System.Int32,System.String,System.String,System.String,System.Guid,System.Guid)"]
- `textMask` (System.String) — [Missing <param name="textMask"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicense(Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,System.Int32,System.Int32,System.String,System.String,System.String,System.Guid,System.Guid)"]
- `product_path` (System.String) — [Missing <param name="product_path"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicense(Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,System.Int32,System.Int32,System.String,System.String,System.String,System.Guid,System.Guid)"]
- `product_title` (System.String) — [Missing <param name="product_title"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicense(Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,System.Int32,System.Int32,System.String,System.String,System.String,System.Guid,System.Guid)"]
- `pluginId` (System.Guid) — [Missing <param name="pluginId"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicense(Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,System.Int32,System.Int32,System.String,System.String,System.String,System.Guid,System.Guid)"]
- `licenseId` (System.Guid) — [Missing <param name="licenseId"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicense(Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,System.Int32,System.Int32,System.String,System.String,System.String,System.Guid,System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicense(Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,System.Int32,System.Int32,System.String,System.String,System.String,System.Guid,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseUtils_GetLicense_1.htm)

#### `public static bool GetLicense(ValidateProductKeyDelegate validateProductKeyDelegate, OnLeaseChangedDelegate leaseChangedDelegate, VerifyLicenseKeyDelegate verifyLicenseKeyDelegate, VerifyPreviousVersionLicenseDelegate verifyPreviousVersionLicenseKeyDelegate, int product_type, int capabilities, string textMask, string product_path, string product_title, Guid pluginId, Guid licenseId)`



**Parameters:**
- `validateProductKeyDelegate` (Rhino.PlugIns.ValidateProductKeyDelegate) — [Missing <param name="validateProductKeyDelegate"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicense(Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.Int32,System.Int32,System.String,System.String,System.String,System.Guid,System.Guid)"]
- `leaseChangedDelegate` (Rhino.PlugIns.OnLeaseChangedDelegate) — [Missing <param name="leaseChangedDelegate"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicense(Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.Int32,System.Int32,System.String,System.String,System.String,System.Guid,System.Guid)"]
- `verifyLicenseKeyDelegate` (Rhino.PlugIns.VerifyLicenseKeyDelegate) — [Missing <param name="verifyLicenseKeyDelegate"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicense(Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.Int32,System.Int32,System.String,System.String,System.String,System.Guid,System.Guid)"]
- `verifyPreviousVersionLicenseKeyDelegate` (Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate) — [Missing <param name="verifyPreviousVersionLicenseKeyDelegate"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicense(Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.Int32,System.Int32,System.String,System.String,System.String,System.Guid,System.Guid)"]
- `product_type` (System.Int32) — [Missing <param name="product_type"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicense(Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.Int32,System.Int32,System.String,System.String,System.String,System.Guid,System.Guid)"]
- `capabilities` (System.Int32) — [Missing <param name="capabilities"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicense(Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.Int32,System.Int32,System.String,System.String,System.String,System.Guid,System.Guid)"]
- `textMask` (System.String) — [Missing <param name="textMask"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicense(Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.Int32,System.Int32,System.String,System.String,System.String,System.Guid,System.Guid)"]
- `product_path` (System.String) — [Missing <param name="product_path"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicense(Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.Int32,System.Int32,System.String,System.String,System.String,System.Guid,System.Guid)"]
- `product_title` (System.String) — [Missing <param name="product_title"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicense(Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.Int32,System.Int32,System.String,System.String,System.String,System.Guid,System.Guid)"]
- `pluginId` (System.Guid) — [Missing <param name="pluginId"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicense(Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.Int32,System.Int32,System.String,System.String,System.String,System.Guid,System.Guid)"]
- `licenseId` (System.Guid) — [Missing <param name="licenseId"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicense(Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.Int32,System.Int32,System.String,System.String,System.String,System.Guid,System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicense(Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate,Rhino.PlugIns.VerifyLicenseKeyDelegate,Rhino.PlugIns.VerifyPreviousVersionLicenseDelegate,System.Int32,System.Int32,System.String,System.String,System.String,System.Guid,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseUtils_GetLicense.htm)

#### `public static LicenseCapabilities GetLicenseCapabilities(int filter)`

Converts an integer to a LicenseCapabilities flag

**Parameters:**
- `filter` (System.Int32) — [Missing <param name="filter"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicenseCapabilities(System.Int32)"]

**Returns:** `LicenseCapabilities` — [Missing <returns> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicenseCapabilities(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseUtils_GetLicenseCapabilities.htm)

#### `public static LicenseStatus[] GetLicenseStatus()`

Returns the current status of every license for UI purposes.

**Returns:** `LicenseStatus[]` — [Missing <returns> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicenseStatus"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseUtils_GetLicenseStatus.htm)

#### `public static int GetLicenseType(Guid productId)`

Returns the contentType of a specified product license

**Parameters:**
- `productId` (System.Guid) — [Missing <param name="productId"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicenseType(System.Guid)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.PlugIns.LicenseUtils.GetLicenseType(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseUtils_GetLicenseType.htm)

#### `public static LicenseStatus GetOneLicenseStatus(Guid productid)`

Returns the current status of a license for UI purposes.

**Parameters:**
- `productid` (System.Guid) — [Missing <param name="productid"/> documentation for "M:Rhino.PlugIns.LicenseUtils.GetOneLicenseStatus(System.Guid)"]

**Returns:** `LicenseStatus` — [Missing <returns> documentation for "M:Rhino.PlugIns.LicenseUtils.GetOneLicenseStatus(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseUtils_GetOneLicenseStatus.htm)

#### `public static bool Initialize()`

Initializes the license manager.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.LicenseUtils.Initialize"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseUtils_Initialize.htm)

#### `public static bool IsCheckOutEnabled()`

Returns whether or not license checkout is enabled.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.LicenseUtils.IsCheckOutEnabled"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseUtils_IsCheckOutEnabled.htm)

#### `public static bool LicenseOptionsHandler(Guid pluginId, Guid licenseId, string productTitle, bool standAlone)`



**Parameters:**
- `pluginId` (System.Guid) — [Missing <param name="pluginId"/> documentation for "M:Rhino.PlugIns.LicenseUtils.LicenseOptionsHandler(System.Guid,System.Guid,System.String,System.Boolean)"]
- `licenseId` (System.Guid) — [Missing <param name="licenseId"/> documentation for "M:Rhino.PlugIns.LicenseUtils.LicenseOptionsHandler(System.Guid,System.Guid,System.String,System.Boolean)"]
- `productTitle` (System.String) — [Missing <param name="productTitle"/> documentation for "M:Rhino.PlugIns.LicenseUtils.LicenseOptionsHandler(System.Guid,System.Guid,System.String,System.Boolean)"]
- `standAlone` (System.Boolean) — [Missing <param name="standAlone"/> documentation for "M:Rhino.PlugIns.LicenseUtils.LicenseOptionsHandler(System.Guid,System.Guid,System.String,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.LicenseUtils.LicenseOptionsHandler(System.Guid,System.Guid,System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseUtils_LicenseOptionsHandler.htm)

#### `public static bool LoginToCloudZoo()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.LicenseUtils.LoginToCloudZoo"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseUtils_LoginToCloudZoo.htm)

#### `public static bool LogoutOfCloudZoo()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.LicenseUtils.LogoutOfCloudZoo"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseUtils_LogoutOfCloudZoo.htm)

#### `public static bool ReturnLicense(Guid productId)`

OBSOLETE - REMOVE WHEN POSSIBLE.

**Parameters:**
- `productId` (System.Guid) — [Missing <param name="productId"/> documentation for "M:Rhino.PlugIns.LicenseUtils.ReturnLicense(System.Guid)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.LicenseUtils.ReturnLicense(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseUtils_ReturnLicense.htm)

#### `public static void ShowBuyLicenseUi(Guid productId)`



**Parameters:**
- `productId` (System.Guid) — [Missing <param name="productId"/> documentation for "M:Rhino.PlugIns.LicenseUtils.ShowBuyLicenseUi(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseUtils_ShowBuyLicenseUi.htm)

#### `public static bool ShowLicenseValidationUi(string cdkey)`

ShowLicenseValidationUi

**Parameters:**
- `cdkey` (System.String) — [Missing <param name="cdkey"/> documentation for "M:Rhino.PlugIns.LicenseUtils.ShowLicenseValidationUi(System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.LicenseUtils.ShowLicenseValidationUi(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseUtils_ShowLicenseValidationUi.htm)

#### `public static bool ShowRhinoExpiredMessage(Mode mode, ref int result)`

Show Rhino or Beta expired message

**Parameters:**
- `mode` (Rhino.Runtime.Mode) — [Missing <param name="mode"/> documentation for "M:Rhino.PlugIns.LicenseUtils.ShowRhinoExpiredMessage(Rhino.Runtime.Mode,System.Int32@)"]
- `result` (System.Int32) — [Missing <param name="result"/> documentation for "M:Rhino.PlugIns.LicenseUtils.ShowRhinoExpiredMessage(Rhino.Runtime.Mode,System.Int32@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.LicenseUtils.ShowRhinoExpiredMessage(Rhino.Runtime.Mode,System.Int32@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_LicenseUtils_ShowRhinoExpiredMessage.htm)

## LoadPlugInResult (enum)

Result of attempting to load a plug-in

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_LoadPlugInResult.htm)

### Values
- `Success` = `0` — Successfully loaded
- `SuccessAlreadyLoaded` = `1`
- `ErrorUnknown` = `2`

## LoadReturnCode (enum)

Rhino plug-in loading return codes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_LoadReturnCode.htm)

### Values
- `Success` = `1`
- `ErrorShowDialog` = `0`
- `ErrorNoDialog` = `-1`

## OnLeaseChangedDelegate (delegate)

Called by Rhino to signal that a lease from Rhino Accounts has changed. If LicenseLeaseChangedEventArgs.Lease is null, then the server has signaled that this product is no longer licensed. Your plug-in must change behavior to behave appropriately.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_OnLeaseChangedDelegate.htm)

## PlugIn (class)

A general purpose utility plug-in that can contain one or more commands.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_PlugIn.htm)

### Constructors
- `protected PlugIn()` — Initializes a new instance of the PlugIn class

### Methods
#### `protected bool AskUserForLicense(LicenseBuildType productBuildType, bool standAlone, string textMask, Object parentWindow, ValidateProductKeyDelegate validateProductKeyDelegate, OnLeaseChangedDelegate onLeaseChangedDelegate)`



**Parameters:**
- `productBuildType` (Rhino.PlugIns.LicenseBuildType) — [Missing <param name="productBuildType"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `standAlone` (System.Boolean) — [Missing <param name="standAlone"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `textMask` (System.String) — [Missing <param name="textMask"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `parentWindow` (System.Object) — [Missing <param name="parentWindow"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `validateProductKeyDelegate` (Rhino.PlugIns.ValidateProductKeyDelegate) — [Missing <param name="validateProductKeyDelegate"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `onLeaseChangedDelegate` (Rhino.PlugIns.OnLeaseChangedDelegate) — [Missing <param name="onLeaseChangedDelegate"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_AskUserForLicense.htm)

#### `public PersistentSettings CommandSettings(string name)`



**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.PlugIns.PlugIn.CommandSettings(System.String)"]

**Returns:** `PersistentSettings` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.CommandSettings(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_CommandSettings.htm)

#### `protected virtual void CreateCommands()`

Called right after plug-in is created and is responsible for creating all of the commands in a given plug-in. The base class implementation Constructs an instance of every publicly exported command class in your plug-in's assembly.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_CreateCommands.htm)

#### `public virtual bool DisplayHelp(IntPtr windowHandle)`

Called by Rhino if AddToHelpMenu is true and menu item associated with this plug-in is selected.

**Parameters:**
- `windowHandle` (System.IntPtr) — Native Window handle of the active Rhino interface.

**Returns:** `Boolean` — true = Help displayed successfully, false = Error displaying help

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_DisplayHelp.htm)

#### `protected virtual void DocumentPropertiesDialogPages(RhinoDoc doc, List<OptionsDialogPage> pages)`

Override this function if you want to extend the document properties sections of the options dialog. This function is called whenever the user brings up the Options dialog.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — document that the pages are set up for
- `pages` (System.Collections.Generic.List<OptionsDialogPage>) — list of pages to add your custom options dialog page(s) to.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_DocumentPropertiesDialogPages.htm)

#### `public static PlugIn Find(Assembly pluginAssembly)`

Finds the plug-in instance that was loaded from a given assembly.

**Parameters:**
- `pluginAssembly` (System.Reflection.Assembly) — The plug-in assembly. You can get the assembly instance at runtime with the Assembly instance property.

**Returns:** `PlugIn` — The assembly plug-in instance if successful. Otherwise, null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_Find_1.htm)

#### `public static PlugIn Find(Guid plugInId)`

Finds the plug-in instance that was loaded from a given plug-in Id.

**Parameters:**
- `plugInId` (System.Guid) — The plug-in Id.

**Returns:** `PlugIn` — The plug-in instance if successful. Otherwise, null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_Find.htm)

#### `public static void FlushSettingsSavedQueue()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_FlushSettingsSavedQueue.htm)

#### `public Command[] GetCommands()`

All of the commands associated with this plug-in.

**Returns:** `Command[]` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.GetCommands"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetCommands.htm)

#### `public static string[] GetEnglishCommandNames(Guid pluginId)`

Gets names of all "non-test" commands for a given plug-in.

**Parameters:**
- `pluginId` (System.Guid) — The plug-in ID.

**Returns:** `String[]` — An array with all plug-in names. This can be empty, but not null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetEnglishCommandNames.htm)

#### `public static string[] GetInstalledPlugInFolders()`



**Returns:** `String[]` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.GetInstalledPlugInFolders"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetInstalledPlugInFolders.htm)

#### `public static string[] GetInstalledPlugInNames()`

Returns the names of all installed Rhino plug-ins.

**Returns:** `String[]` — The names if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetInstalledPlugInNames.htm)

#### `public static string[] GetInstalledPlugInNames(PlugInType typeFilter, bool loaded, bool unloaded)`

Gets a list of installed plug-in names. The list can be restricted by some filters.

**Parameters:**
- `typeFilter` (Rhino.PlugIns.PlugInType) — The enumeration flags that determine which types of plug-ins are included.
- `loaded` (System.Boolean) — true if loaded plug-ins are returned.
- `unloaded` (System.Boolean) — true if unloaded plug-ins are returned.

**Returns:** `String[]` — An array of installed plug-in names. This can be empty, but not null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetInstalledPlugInNames_1.htm)

#### `public static string[] GetInstalledPlugInNames(PlugInType typeFilter, bool loaded, bool unloaded, bool localizedPlugInName)`

Gets a list of installed plug-in names. The list can be restricted by some filters.

**Parameters:**
- `typeFilter` (Rhino.PlugIns.PlugInType) — The enumeration flags that determine which types of plug-ins are included.
- `loaded` (System.Boolean) — true if loaded plug-ins are returned.
- `unloaded` (System.Boolean) — true if unloaded plug-ins are returned.
- `localizedPlugInName` (System.Boolean) — If true localized plug-in names are returned otherwise; English names are returned.

**Returns:** `String[]` — An array of installed plug-in names. This can be empty, but not null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetInstalledPlugInNames_2.htm)

#### `public static Dictionary<Guid, string> GetInstalledPlugIns()`

Get a list of all registered plug-in's regardless of if they are loaded or not.

**Returns:** `Dictionary<Guid,String>` — Dictionary with plug-in ID as key and localized plug-in name as value

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetInstalledPlugIns.htm)

#### `public static Dictionary<Guid, string> GetInstalledPlugIns(bool localizedPlugInName)`

Get a list of all registered plug-in's regardless of if they are loaded or not.

**Parameters:**
- `localizedPlugInName` (System.Boolean) — If true then the localize plug-in name is returned otherwise; the English name is used.

**Returns:** `Dictionary<Guid,String>` — Dictionary with plug-in ID as key and plug-in name as value

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetInstalledPlugIns_1.htm)

#### `protected bool GetLicense(LicenseBuildType productBuildType, ValidateProductKeyDelegate validateProductKeyDelegate, OnLeaseChangedDelegate leaseChangedDelegate)`

Verifies that there is a valid product license for your plug-in, using the Rhino licensing system. If the plug-in is installed as a standalone node, the locally installed license will be validated. If the plug-in is installed as a network node, a loaner license will be requested by the system's assigned Zoo server. If the Zoo server finds and returns a license, then this license will be validated. If no license is found, then the user will be prompted to provide a license key, which will be validated.

**Parameters:**
- `productBuildType` (Rhino.PlugIns.LicenseBuildType) — The product build contentType required by your plug-in.
- `validateProductKeyDelegate` (Rhino.PlugIns.ValidateProductKeyDelegate) — Since the Rhino licensing system knows nothing about your product license, you will need to validate the product license provided by the Rhino licensing system. This is done by supplying a callback function, or delegate, that can be called to perform the validation.
- `leaseChangedDelegate` (Rhino.PlugIns.OnLeaseChangedDelegate) — Called by the ZooClient when the cloud zoo lease is changed.

**Returns:** `Boolean` — true if a valid license was found. false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetLicense.htm)

#### `protected bool GetLicense(LicenseCapabilities licenseCapabilities, string textMask, ValidateProductKeyDelegate validateProductKeyDelegate, OnLeaseChangedDelegate leaseChangedDelegate)`

Verifies that there is a valid product license for your plug-in, using the Rhino licensing system. If the plug-in is installed as a standalone node, the locally installed license will be validated. If the plug-in is installed as a network node, a loaner license will be requested by the system's assigned Zoo server. If the Zoo server finds and returns a license, then this license will be validated. If no license is found, then the user will be prompted to provide a license key, which will be validated.

**Parameters:**
- `licenseCapabilities` (Rhino.PlugIns.LicenseCapabilities) — In the event that a license was not found, or if the user wants to change the way your plug-in is licenses, then provide what capabilities your license has by using this enumeration flag.
- `textMask` (System.String) — In the event that the user needs to be asked for a license, then you can provide a text mask, which helps the user to distinguish between proper and improper user input of your license code. Note, if you do not want to use a text mask, then pass in a null value for this parameter. For more information on text masks, search MSDN for the System.Windows.Forms.MaskedTextBox class.
- `validateProductKeyDelegate` (Rhino.PlugIns.ValidateProductKeyDelegate) — Since the Rhino licensing system knows nothing about your product license, you will need to validate the product license provided by the Rhino licensing system. This is done by supplying a callback function, or delegate, that can be called to perform the validation.
- `leaseChangedDelegate` (Rhino.PlugIns.OnLeaseChangedDelegate) — Called by the ZooClient when the cloud zoo lease is changed.

**Returns:** `Boolean` — true if a valid license was found. false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetLicense_1.htm)

#### `protected bool GetLicenseOwner(out string registeredOwner, out string registeredOrganization)`

Get the customer name and organization used when entering the product license.

**Parameters:**
- `registeredOwner` (System.String) — [Missing <param name="registeredOwner"/> documentation for "M:Rhino.PlugIns.PlugIn.GetLicenseOwner(System.String@,System.String@)"]
- `registeredOrganization` (System.String) — [Missing <param name="registeredOrganization"/> documentation for "M:Rhino.PlugIns.PlugIn.GetLicenseOwner(System.String@,System.String@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.GetLicenseOwner(System.String@,System.String@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetLicenseOwner.htm)

#### `public static bool GetLoadProtection(Guid pluginId, out bool loadSilently)`

Get load protection state for a plug-in

**Parameters:**
- `pluginId` (System.Guid) — [Missing <param name="pluginId"/> documentation for "M:Rhino.PlugIns.PlugIn.GetLoadProtection(System.Guid,System.Boolean@)"]
- `loadSilently` (System.Boolean) — [Missing <param name="loadSilently"/> documentation for "M:Rhino.PlugIns.PlugIn.GetLoadProtection(System.Guid,System.Boolean@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.GetLoadProtection(System.Guid,System.Boolean@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetLoadProtection.htm)

#### `public static PlugInInfo GetPlugInInfo(Guid pluginId)`

Returns detailed information about an installed Rhino plug-in.

**Parameters:**
- `pluginId` (System.Guid) — The id of the plug-in.

**Returns:** `PlugInInfo` — Detailed information about an installed Rhino plug-in if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetPlugInInfo.htm)

#### `public virtual Object GetPlugInObject()`



**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.GetPlugInObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetPlugInObject.htm)

#### `public static PersistentSettings GetPluginSettings(Guid plugInId, bool load)`



**Parameters:**
- `plugInId` (System.Guid) — [Missing <param name="plugInId"/> documentation for "M:Rhino.PlugIns.PlugIn.GetPluginSettings(System.Guid,System.Boolean)"]
- `load` (System.Boolean) — [Missing <param name="load"/> documentation for "M:Rhino.PlugIns.PlugIn.GetPluginSettings(System.Guid,System.Boolean)"]

**Returns:** `PersistentSettings` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.GetPluginSettings(System.Guid,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetPluginSettings.htm)

#### `public Bitmap Icon(Size size)`

Returns the plug-in's icon in bitmap form.

**Parameters:**
- `size` (System.Drawing.Size) — The desired size in pixels.

**Returns:** `Bitmap` — The icon if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_Icon.htm)

#### `public static Guid IdFromFileName(string filename)`

Attempt to get a plugiin id from just the filename of a plug-in

**Parameters:**
- `filename` (System.String) — plug-in filename

**Returns:** `Guid` — id on success; Guid.Empty if no plug-in could be found

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_IdFromFileName.htm)

#### `public static Guid IdFromName(string pluginName)`

Gets the id of an installed plug-in giving the plug-in's name.

**Parameters:**
- `pluginName` (System.String) — The name of the installed plug-in.

**Returns:** `Guid` — The id if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_IdFromName.htm)

#### `public static Guid IdFromPath(string pluginPath)`

Gets the id of an installed plug-in giving the plug-in's file path.

**Parameters:**
- `pluginPath` (System.String) — The path to the installed plug-in.

**Returns:** `Guid` — The id if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_IdFromPath.htm)

#### `public virtual bool IsTextureSupported(RenderTexture texture)`

Returns true if this renderer can render the texture natively without needing it to be baked into a bitmap, false otherwise. By default, returns false for all textures.

**Parameters:**
- `texture` (Rhino.Render.RenderTexture) — [Missing <param name="texture"/> documentation for "M:Rhino.PlugIns.PlugIn.IsTextureSupported(Rhino.Render.RenderTexture)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.IsTextureSupported(Rhino.Render.RenderTexture)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_IsTextureSupported.htm)

#### `public static void LoadComputeExtensionPlugins()`

Used by compute's startup code to load plug-ins that have registered custom endpoints

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_LoadComputeExtensionPlugins.htm)

#### `public static bool LoadPlugIn(Guid pluginId)`

Loads an installed plug-in.

**Parameters:**
- `pluginId` (System.Guid) — The id of the installed plug-in.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_LoadPlugIn.htm)

#### `public static bool LoadPlugIn(Guid pluginId, bool loadQuietly, bool forceLoad)`

Loads an installed plug-in.

**Parameters:**
- `pluginId` (System.Guid) — The id of the installed plug-in.
- `loadQuietly` (System.Boolean) — Load the plug-in quietly.
- `forceLoad` (System.Boolean) — Load plug-in even if previous attempt to load has failed.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_LoadPlugIn_1.htm)

#### `public static LoadPlugInResult LoadPlugIn(string path, out Guid plugInId)`

Attempt to load a plug-in at a path. Loaded plug-ins are remembered by Rhino between sessions, so this function can also be considered a plug-in installation routine

**Parameters:**
- `path` (System.String) — full path to plug-in to attempt to load
- `plugInId` (System.Guid) — If successful (or the plug-in is already loaded), the unique id for the plug-in is returned here. Guid.Empty on failure

**Returns:** `LoadPlugInResult` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.LoadPlugIn(System.String,System.Guid@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_LoadPlugIn_2.htm)

#### `public static string NameFromPath(string pluginPath)`

Gets a plug-in name for an installed plug-in given the path to that plug-in.

**Parameters:**
- `pluginPath` (System.String) — The path of the plug-in.

**Returns:** `String` — The plug-in name.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_NameFromPath.htm)

#### `[ObsoleteAttribute("Use ObjectPropertiesPages(ObjectPropertiesPageCollection collection) instead")] protected virtual void ObjectPropertiesPages(List<ObjectPropertiesPage> pages)`

Override this function is you want to extend the object properties dialog

**Parameters:**
- `pages` (System.Collections.Generic.List<ObjectPropertiesPage>) — [Missing <param name="pages"/> documentation for "M:Rhino.PlugIns.PlugIn.ObjectPropertiesPages(System.Collections.Generic.List{Rhino.UI.ObjectPropertiesPage})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ObjectPropertiesPages_1.htm)

#### `protected virtual void ObjectPropertiesPages(ObjectPropertiesPageCollection collection)`

Override this function is you want to extend the object properties dialog. This method will be called each time a new document is created for each instance of a object properties panel. On Windows there will be a single panel per document but on Mac there may be many properties panel per document.

**Parameters:**
- `collection` (Rhino.UI.ObjectPropertiesPageCollection) — Add custom pages by calling collection.Add

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ObjectPropertiesPages.htm)

#### `protected virtual LoadReturnCode OnLoad(ref string errorMessage)`

Is called when the plug-in is being loaded.

**Parameters:**
- `errorMessage` (System.String) — If a load error is returned and this string is set. This string is the error message that will be reported back to the user.

**Returns:** `LoadReturnCode` — An appropriate load return code. The default implementation returns Success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_OnLoad.htm)

#### `protected virtual void OnShutdown()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_OnShutdown.htm)

#### `protected virtual void OptionsDialogPages(List<OptionsDialogPage> pages)`

Override this function if you want to extend the options dialog. This function is called whenever the user brings up the Options dialog.

**Parameters:**
- `pages` (System.Collections.Generic.List<OptionsDialogPage>) — list of pages to add your custom options dialog page(s) to.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_OptionsDialogPages.htm)

#### `public static string PathFromId(Guid pluginId)`

Gets the path to an installed plug-in given the id of that plug-in

**Parameters:**
- `pluginId` (System.Guid) — [Missing <param name="pluginId"/> documentation for "M:Rhino.PlugIns.PlugIn.PathFromId(System.Guid)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.PathFromId(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_PathFromId.htm)

#### `public static string PathFromName(string pluginName)`

Gets the path to an installed plug-in given the name of that plug-in

**Parameters:**
- `pluginName` (System.String) — [Missing <param name="pluginName"/> documentation for "M:Rhino.PlugIns.PlugIn.PathFromName(System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.PathFromName(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_PathFromName.htm)

#### `public static bool PlugInExists(Guid id, out bool loaded, out bool loadProtected)`

Verifies that a Rhino plug-in is installed.

**Parameters:**
- `id` (System.Guid) — The id of the plug-in.
- `loaded` (System.Boolean) — The loaded state of the plug-in.
- `loadProtected` (System.Boolean) — The load protected state of the plug-in.

**Returns:** `Boolean` — Returns true if the plug-in exists, or is installed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_PlugInExists.htm)

#### `public static void RaiseOnPlugInSettingsSavedEvent()`

Raise any pending OnPlugInSettingsSaved events, the events are normally queued while a command is running and fired while Rhino is in an idle state. Calling this method will raise any pending changed events regardless of Rhino's current idle state or if a command is running.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_RaiseOnPlugInSettingsSavedEvent.htm)

#### `protected virtual void ReadDocument(RhinoDoc doc, BinaryArchiveReader archive, FileReadOptions options)`

Called whenever a Rhino document is being loaded and plug-in user data was encountered written by a plug-in with this plug-in's GUID.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — A Rhino document that is being loaded.
- `archive` (Rhino.FileIO.BinaryArchiveReader) — OpenNURBS file archive object Rhino is using to read this file. Use BinaryArchiveReader.Read*() functions to read plug-in data. If any BinaryArchive.Read*() functions throws an exception then archive.ReadErrorOccurve will be true and you should immediately return.
- `options` (Rhino.FileIO.FileReadOptions) — Describes what is being written.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ReadDocument.htm)

#### `protected bool RegisterCommand(Command command)`



**Parameters:**
- `command` (Rhino.Commands.Command) — [Missing <param name="command"/> documentation for "M:Rhino.PlugIns.PlugIn.RegisterCommand(Rhino.Commands.Command)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.RegisterCommand(Rhino.Commands.Command)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_RegisterCommand.htm)

#### `protected virtual void ResetMessageBoxes()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ResetMessageBoxes.htm)

#### `protected bool ReturnLicense()`

Returns, or releases, a product license that was obtained from the Rhino licensing system. Note, most plug-ins do not need to call this as the Rhino licensing system will return all licenses when Rhino shuts down.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.ReturnLicense"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ReturnLicense.htm)

#### `public static void SavePluginSettings(Guid plugInId)`



**Parameters:**
- `plugInId` (System.Guid) — [Missing <param name="plugInId"/> documentation for "M:Rhino.PlugIns.PlugIn.SavePluginSettings(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_SavePluginSettings.htm)

#### `public void SaveSettings()`

Write settings to disk which will raise a SettingsSaved event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_SaveSettings.htm)

#### `protected void SetLicenseCapabilities(string textMask, LicenseCapabilities capabilities, Guid licenseId)`



**Parameters:**
- `textMask` (System.String) — [Missing <param name="textMask"/> documentation for "M:Rhino.PlugIns.PlugIn.SetLicenseCapabilities(System.String,Rhino.PlugIns.LicenseCapabilities,System.Guid)"]
- `capabilities` (Rhino.PlugIns.LicenseCapabilities) — [Missing <param name="capabilities"/> documentation for "M:Rhino.PlugIns.PlugIn.SetLicenseCapabilities(System.String,Rhino.PlugIns.LicenseCapabilities,System.Guid)"]
- `licenseId` (System.Guid) — [Missing <param name="licenseId"/> documentation for "M:Rhino.PlugIns.PlugIn.SetLicenseCapabilities(System.String,Rhino.PlugIns.LicenseCapabilities,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_SetLicenseCapabilities.htm)

#### `public static void SetLoadProtection(Guid pluginId, bool loadSilently)`

Set load protection state for a certain plug-in

**Parameters:**
- `pluginId` (System.Guid) — [Missing <param name="pluginId"/> documentation for "M:Rhino.PlugIns.PlugIn.SetLoadProtection(System.Guid,System.Boolean)"]
- `loadSilently` (System.Boolean) — [Missing <param name="loadSilently"/> documentation for "M:Rhino.PlugIns.PlugIn.SetLoadProtection(System.Guid,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_SetLoadProtection.htm)

#### `protected virtual bool ShouldCallWriteDocument(FileWriteOptions options)`

Called whenever a Rhino is about to save a .3dm file. If you want to save plug-in document data when a model is saved in a version 5 .3dm file, then you must override this function to return true and you must override WriteDocument().

**Parameters:**
- `options` (Rhino.FileIO.FileWriteOptions) — The file write options, such as "include preview image" and "include render meshes".

**Returns:** `Boolean` — true if the plug-in wants to save document user data in the version 5 .3dm file. The default returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ShouldCallWriteDocument.htm)

#### `protected virtual void WriteDocument(RhinoDoc doc, BinaryArchiveWriter archive, FileWriteOptions options)`

Called when Rhino is saving a .3dm file to allow the plug-in to save document user data.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — The Rhino document instance that is being saved.
- `archive` (Rhino.FileIO.BinaryArchiveWriter) — OpenNURBS file archive object Rhino is using to write the file. Use BinaryArchiveWriter.Write*() functions to write plug-in data. OR use the ArchivableDictionary If any BinaryArchiveWriter.Write*() functions throw an exception, then archive.WriteErrorOccured will be true and you should immediately return. Setting archive.WriteErrorOccured to true will cause Rhino to stop saving the file.
- `options` (Rhino.FileIO.FileWriteOptions) — The file write options, such as "include preview image" and "include render meshes".

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_WriteDocument.htm)

### Properties
- `AddToHelpMenu` (Boolean, get) — Called by Rhino to determine if the plug-in name should be added to the Rhino Help/Plug-ins menu.
- `AskOnLoadProtection` (Boolean, get/set) — If true, Rhino will display a warning dialog when load-protected plug-ins are attempting to load. If false, load-protected plug-ins will silently not load.
- `Assembly` (Assembly, get) — Source assembly for this plug-in.
- `Description` (String, get) — Returns the description of the plug-in, as found in the plug-in's assembly attributes.
- `Id` (Guid, get) — Returns the Guid, or unique Id, of the plug-in.
- `InstalledPlugInCount` (Int32, get) — Returns the number of installed Rhino plug-ins.
- `LicenseId` (Guid, get) — 
- `LoadAtStartup` (Boolean, get) — Obsolete.
- `LoadTime` (PlugInLoadTime, get) — Plug-ins are typically loaded on demand when they are first needed. You can change this behavior to load the plug-in at during different stages in time by overriding this property.
- `LocalPlugInName` (String, get) — Optionally override this to provide a localized plug-in name
- `Name` (String, get) — Returns the name of the plug-in, as found in the plug-in's assembly attributes.
- `Settings` (PersistentSettings, get) — Persistent plug-in settings.
- `SettingsDirectory` (String, get) — Get the plug-in's settings directory. This is the directory where the plug-in's persistent settings files are saved. This directory will be located in the user's profile folder. Note, this does not verify the directory exists.
- `SettingsDirectoryAllUsers` (String, get) — Get the plug-in's "all users" settings directory. This directory will be located in the system's program data folder. Note, this does not verify the directory exists.
- `Version` (String, get) — Returns the version of the plug-in, as found in the plug-in's assembly attributes.
- `WindowPositionSettings` (PersistentSettings, get) — 

### Events
#### `SettingsSaved` (`System.EventHandler<PersistentSettingsSavedEventArgs>`)

**Signature:** `public event EventHandler<PersistentSettingsSavedEventArgs> SettingsSaved`

This event is raised when an instance of Rhino has modified the external settings file associated with this plug-in's Settings property.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_PlugIns_PlugIn_SettingsSaved.htm)

#### `UnknownUserData` (`System.EventHandler<PlugIn.UnknownUserDataEventArgs>`)

**Signature:** `public static event EventHandler<PlugIn.UnknownUserDataEventArgs> UnknownUserData`

This event is raised when a 3dm file is loaded with unknown user data, and the plug-in to handle it could not be loaded.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_PlugIns_PlugIn_UnknownUserData.htm)

## PlugIn.UnknownUserDataEventArgs (class)

Event argument passed to the UnknownUserData event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_PlugIn_UnknownUserDataEventArgs.htm)

### Properties
- `Document` (RhinoDoc, get) — 
- `PlugInId` (Guid, get) — 

## PlugInDescriptionAttribute (class)

Rhino plug-in developer information attributes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_PlugInDescriptionAttribute.htm)

### Constructors
- `public PlugInDescriptionAttribute(DescriptionType descriptionType, string value)` — Initializes a new instance of the PlugInDescriptionAttribute class

### Properties
- `DescriptionType` (DescriptionType, get) — 
- `Value` (String, get) — 

## PlugInInfo (class)

Contains detailed information about a Rhino plug-in.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_PlugInInfo.htm)

### Methods
#### `public Bitmap Icon(Size size)`

Returns the plug-in's icon in bitmap form.

**Parameters:**
- `size` (System.Drawing.Size) — [Missing <param name="size"/> documentation for "M:Rhino.PlugIns.PlugInInfo.Icon(System.Drawing.Size)"]

**Returns:** `Bitmap` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugInInfo.Icon(System.Drawing.Size)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugInInfo_Icon.htm)

#### `public bool IsLoadProtected(out bool loadSilently)`

Returns the load protection state for a plug-in

**Parameters:**
- `loadSilently` (System.Boolean) — The plug-in's load silently state.

**Returns:** `Boolean` — true if the plug-in is load protected, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugInInfo_IsLoadProtected.htm)

### Properties
- `Address` (String, get) — Returns the address of the organization or company that created the plug-in.
- `CommandNames` (String[], get) — Returns a plug-in's English command names.
- `Country` (String, get) — Returns the country of the organization or company that created the plug-in.
- `Description` (String, get) — Returns the plug-in's description.
- `Email` (String, get) — Returns the email address of the organization or company that created the plug-in.
- `Fax` (String, get) — Returns the fax number of the organization or company that created the plug-in.
- `FileName` (String, get) — Returns the plug-in's file name.
- `FileTypeDescriptions` (String[], get) — Returns the description of supported file types for file import and file export plug-in.
- `FileTypeExtensions` (String[], get) — Returns the file types extensions supported for file import and file export plug-in.
- `Id` (Guid, get) — Returns the plug-in's Id.
- `IsDotNet` (Boolean, get) — Returns true if the plug-in is based on .NET, false otherwise.
- `IsLoaded` (Boolean, get) — Returns true if the plug-in is loaded, false otherwise.
- `Name` (String, get) — Returns the plug-in's name.
- `Organization` (String, get) — Returns the organization or company that created the plug-in.
- `Phone` (String, get) — Returns the phone number of the organization or company that created the plug-in.
- `PlugInLoadTime` (PlugInLoadTime, get) — Returns the plug-in's load time.
- `PlugInType` (PlugInType, get) — Returns the plug-in type.
- `RegistryPath` (String, get) — Returns the plug-in's Windows Registry path.
- `ShipsWithRhino` (Boolean, get) — Returns true if the plug-in ships with Rhino, false otherwise.
- `UpdateUrl` (String, get) — Returns the web site, or URL, were an updated version of the plug-in can be found.
- `Version` (String, get) — Returns the plug-in's version.
- `WebSite` (String, get) — Returns the web site, or URL, of the organization or company that created the plug-in.

## PlugInLoadTime (enum)

Rhino plug-in load time enumeration.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_PlugInLoadTime.htm)

### Values
- `Disabled` = `0` — never load plug-in.
- `AtStartup` = `1` — Load when Rhino starts.
- `WhenNeeded` = `2` — (default) Load the first time a plug-in command used.
- `WhenNeededIgnoreDockingBars` = `6` — Load the first time a plug-in command used NOT when restoring docking control bars.
- `WhenNeededOrOptionsDialog` = `10` — When a plug-in command is used or the options dialog is shown.
- `WhenNeededOrTabbedDockBar` = `18` — When a plug-in command is used or when a tabbed dockbar is loaded.

## PlugInType (enum)

Rhino plug-in type enumeration.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_PlugInType.htm)

### Values
- `None` = `0`
- `Render` = `1`
- `FileImport` = `2`
- `FileExport` = `4`
- `Digitizer` = `8`
- `Utility` = `16`
- `DisplayPipeline` = `32`
- `DisplayEngine` = `64`
- `Any` = `127`

## PreviewNotification (class)

[Missing <summary> documentation for "T:Rhino.PlugIns.PreviewNotification"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_PreviewNotification.htm)

### Methods
#### `public void NotifyIntermediateUpdate(RenderWindow rw)`



**Parameters:**
- `rw` (Rhino.Render.RenderWindow) — [Missing <param name="rw"/> documentation for "M:Rhino.PlugIns.PreviewNotification.NotifyIntermediateUpdate(Rhino.Render.RenderWindow)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PreviewNotification_NotifyIntermediateUpdate.htm)

## RenderPlugIn (class)

A Rhino rendering plugin; applies materials, textures, and lights to a scene to produce rendered images.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_RenderPlugIn.htm)

### Constructors
- `protected RenderPlugIn()` — Initializes a new instance of the RenderPlugIn class

### Methods
#### `protected virtual bool AllowChooseContent(RenderContent content)`

Default implementation returns true which means the content can be picked from the content browser by the user. Override this method and return false if you don't want to allow a certain content contentType to be picked from the content browser while your render engine is current.

**Parameters:**
- `content` (Rhino.Render.RenderContent) — A render context.

**Returns:** `Boolean` — true if the operation was successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_AllowChooseContent.htm)

#### `protected bool AskUserForLicense(LicenseBuildType productBuildType, bool standAlone, string textMask, Object parentWindow, ValidateProductKeyDelegate validateProductKeyDelegate, OnLeaseChangedDelegate onLeaseChangedDelegate)`

(Inherited from PlugIn.)

**Parameters:**
- `productBuildType` (Rhino.PlugIns.LicenseBuildType) — [Missing <param name="productBuildType"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `standAlone` (System.Boolean) — [Missing <param name="standAlone"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `textMask` (System.String) — [Missing <param name="textMask"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `parentWindow` (System.Object) — [Missing <param name="parentWindow"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `validateProductKeyDelegate` (Rhino.PlugIns.ValidateProductKeyDelegate) — [Missing <param name="validateProductKeyDelegate"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]
- `onLeaseChangedDelegate` (Rhino.PlugIns.OnLeaseChangedDelegate) — [Missing <param name="onLeaseChangedDelegate"/> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.AskUserForLicense(Rhino.PlugIns.LicenseBuildType,System.Boolean,System.String,System.Object,Rhino.PlugIns.ValidateProductKeyDelegate,Rhino.PlugIns.OnLeaseChangedDelegate)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_AskUserForLicense.htm)

#### `public PersistentSettings CommandSettings(string name)`

(Inherited from PlugIn.)

**Parameters:**
- `name` (System.String) — [Missing <param name="name"/> documentation for "M:Rhino.PlugIns.PlugIn.CommandSettings(System.String)"]

**Returns:** `PersistentSettings` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.CommandSettings(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_CommandSettings.htm)

#### `protected virtual void CreateCommands()`

Called right after plug-in is created and is responsible for creating all of the commands in a given plug-in. The base class implementation Constructs an instance of every publicly exported command class in your plug-in's assembly.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_CreateCommands.htm)

#### `protected virtual void CreatePreview(CreatePreviewEventArgs args)`

Creates the preview bitmap that will appear in the content editor's thumbnail display when previewing materials and environments. If this function is not overridden or the PreviewImage is not set on the arguments, then the internal OpenGL renderer will generate a simulation of the content. This function is called with four different preview quality settings. The first quality level of RealtimeQuick is called on the main thread and needs to be drawn as fast as possible. This function is called with the other three quality settings on a separate thread and are meant for generating progressively refined preview.

**Parameters:**
- `args` (Rhino.Render.CreatePreviewEventArgs) — Event argument with several preview option state properties.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_CreatePreview.htm)

#### `protected virtual void CreateTexture2dPreview(CreateTexture2dPreviewEventArgs args)`

Creates the preview bitmap that will appear in the content editor's thumbnail display when previewing textures in 2d (UV) mode. If this function is not overridden or the PreviewImage is not set on the arguments, then the internal OpenGL renderer will generate a simulation.

**Parameters:**
- `args` (Rhino.Render.CreateTexture2dPreviewEventArgs) — Event argument with several preview option state properties.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_CreateTexture2dPreview.htm)

#### `public static bool CurrentRendererSupportsFeature(RenderPlugIn.RenderFeature feature)`



**Parameters:**
- `feature` (Rhino.PlugIns.RenderPlugIn.RenderFeature) — [Missing <param name="feature"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.CurrentRendererSupportsFeature(Rhino.PlugIns.RenderPlugIn.RenderFeature)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.RenderPlugIn.CurrentRendererSupportsFeature(Rhino.PlugIns.RenderPlugIn.RenderFeature)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_CurrentRendererSupportsFeature.htm)

#### `protected virtual string CustomChannelName(Guid id)`

Return the localized name of your custom channel.

**Parameters:**
- `id` (System.Guid) — [Missing <param name="id"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.CustomChannelName(System.Guid)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.PlugIns.RenderPlugIn.CustomChannelName(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_CustomChannelName.htm)

#### `public virtual bool DisplayHelp(IntPtr windowHandle)`

Called by Rhino if AddToHelpMenu is true and menu item associated with this plug-in is selected.

**Parameters:**
- `windowHandle` (System.IntPtr) — Native Window handle of the active Rhino interface.

**Returns:** `Boolean` — true = Help displayed successfully, false = Error displaying help

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_DisplayHelp.htm)

#### `protected virtual void DocumentPropertiesDialogPages(RhinoDoc doc, List<OptionsDialogPage> pages)`

Override this function if you want to extend the document properties sections of the options dialog. This function is called whenever the user brings up the Options dialog.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — document that the pages are set up for
- `pages` (System.Collections.Generic.List<OptionsDialogPage>) — list of pages to add your custom options dialog page(s) to.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_DocumentPropertiesDialogPages.htm)

#### `public virtual bool EnableAssignMaterialButton()`

Called to enable/disable the "Material" button located on the "Material" tab in the Properties and Layer dialog boxes. The default return value is false which will disable the button. If the button is disabled then the OnAssignMaterial function is never called.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.RenderPlugIn.EnableAssignMaterialButton"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_EnableAssignMaterialButton.htm)

#### `public virtual bool EnableCreateMaterialButton()`

Called to enable/disable the "New" button located on the "Material" in the Properties and Layer dialog boxes. The default return value is false which will disable the button. If the button is disabled then the OnEditMaterial function is never called.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.RenderPlugIn.EnableCreateMaterialButton"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_EnableCreateMaterialButton.htm)

#### `public virtual bool EnableEditMaterialButton(RhinoDoc doc, Material material)`

Called to enable/disable the "Edit" button located on the "Material" in the Properties and Layer dialog boxes. The default return value is false which will disable the button. If the button is disabled then the OnEditMaterial function is never called.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.EnableEditMaterialButton(Rhino.RhinoDoc,Rhino.DocObjects.Material)"]
- `material` (Rhino.DocObjects.Material) — [Missing <param name="material"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.EnableEditMaterialButton(Rhino.RhinoDoc,Rhino.DocObjects.Material)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.RenderPlugIn.EnableEditMaterialButton(Rhino.RhinoDoc,Rhino.DocObjects.Material)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_EnableEditMaterialButton.htm)

#### `public Command[] GetCommands()`

All of the commands associated with this plug-in.

**Returns:** `Command[]` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.GetCommands"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetCommands.htm)

#### `protected bool GetLicense(LicenseBuildType productBuildType, ValidateProductKeyDelegate validateProductKeyDelegate, OnLeaseChangedDelegate leaseChangedDelegate)`

Verifies that there is a valid product license for your plug-in, using the Rhino licensing system. If the plug-in is installed as a standalone node, the locally installed license will be validated. If the plug-in is installed as a network node, a loaner license will be requested by the system's assigned Zoo server. If the Zoo server finds and returns a license, then this license will be validated. If no license is found, then the user will be prompted to provide a license key, which will be validated.

**Parameters:**
- `productBuildType` (Rhino.PlugIns.LicenseBuildType) — The product build contentType required by your plug-in.
- `validateProductKeyDelegate` (Rhino.PlugIns.ValidateProductKeyDelegate) — Since the Rhino licensing system knows nothing about your product license, you will need to validate the product license provided by the Rhino licensing system. This is done by supplying a callback function, or delegate, that can be called to perform the validation.
- `leaseChangedDelegate` (Rhino.PlugIns.OnLeaseChangedDelegate) — Called by the ZooClient when the cloud zoo lease is changed.

**Returns:** `Boolean` — true if a valid license was found. false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetLicense.htm)

#### `protected bool GetLicense(LicenseCapabilities licenseCapabilities, string textMask, ValidateProductKeyDelegate validateProductKeyDelegate, OnLeaseChangedDelegate leaseChangedDelegate)`

Verifies that there is a valid product license for your plug-in, using the Rhino licensing system. If the plug-in is installed as a standalone node, the locally installed license will be validated. If the plug-in is installed as a network node, a loaner license will be requested by the system's assigned Zoo server. If the Zoo server finds and returns a license, then this license will be validated. If no license is found, then the user will be prompted to provide a license key, which will be validated.

**Parameters:**
- `licenseCapabilities` (Rhino.PlugIns.LicenseCapabilities) — In the event that a license was not found, or if the user wants to change the way your plug-in is licenses, then provide what capabilities your license has by using this enumeration flag.
- `textMask` (System.String) — In the event that the user needs to be asked for a license, then you can provide a text mask, which helps the user to distinguish between proper and improper user input of your license code. Note, if you do not want to use a text mask, then pass in a null value for this parameter. For more information on text masks, search MSDN for the System.Windows.Forms.MaskedTextBox class.
- `validateProductKeyDelegate` (Rhino.PlugIns.ValidateProductKeyDelegate) — Since the Rhino licensing system knows nothing about your product license, you will need to validate the product license provided by the Rhino licensing system. This is done by supplying a callback function, or delegate, that can be called to perform the validation.
- `leaseChangedDelegate` (Rhino.PlugIns.OnLeaseChangedDelegate) — Called by the ZooClient when the cloud zoo lease is changed.

**Returns:** `Boolean` — true if a valid license was found. false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetLicense_1.htm)

#### `protected bool GetLicenseOwner(out string registeredOwner, out string registeredOrganization)`

Get the customer name and organization used when entering the product license.

**Parameters:**
- `registeredOwner` (System.String) — [Missing <param name="registeredOwner"/> documentation for "M:Rhino.PlugIns.PlugIn.GetLicenseOwner(System.String@,System.String@)"]
- `registeredOrganization` (System.String) — [Missing <param name="registeredOrganization"/> documentation for "M:Rhino.PlugIns.PlugIn.GetLicenseOwner(System.String@,System.String@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.GetLicenseOwner(System.String@,System.String@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetLicenseOwner.htm)

#### `public virtual Object GetPlugInObject()`

(Inherited from PlugIn.)

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.GetPlugInObject"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_GetPlugInObject.htm)

#### `public List<Guid> GetRenderSettingsSections()`

This function returns a list of Guids for the render settings pages that should be displayed.

**Returns:** `List<Guid>` — Return a Id list of the Render settings sections that will be displayed

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_GetRenderSettingsSections.htm)

#### `public Bitmap Icon(Size size)`

Returns the plug-in's icon in bitmap form.

**Parameters:**
- `size` (System.Drawing.Size) — The desired size in pixels.

**Returns:** `Bitmap` — The icon if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_Icon.htm)

#### `protected virtual void InitializeDecalProperties(ref List<NamedValue> properties)`

Initialize your custom decal properties here. The input list will be empty - add your default named property values and return.

**Parameters:**
- `properties` (System.Collections.Generic.List<NamedValue>) — A list of named values that will be stored on the object the input values are the current ones, you should modify the values after the dialog closes.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_InitializeDecalProperties.htm)

#### `public virtual bool IsTextureSupported(RenderTexture texture)`

Returns true if this renderer can render the texture natively without needing it to be baked into a bitmap, false otherwise. By default, returns false for all textures.

**Parameters:**
- `texture` (Rhino.Render.RenderTexture) — [Missing <param name="texture"/> documentation for "M:Rhino.PlugIns.PlugIn.IsTextureSupported(Rhino.Render.RenderTexture)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.IsTextureSupported(Rhino.Render.RenderTexture)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_IsTextureSupported.htm)

#### `[ObsoleteAttribute("Use ObjectPropertiesPages(ObjectPropertiesPageCollection collection) instead")] protected virtual void ObjectPropertiesPages(List<ObjectPropertiesPage> pages)`

Override this function is you want to extend the object properties dialog

**Parameters:**
- `pages` (System.Collections.Generic.List<ObjectPropertiesPage>) — [Missing <param name="pages"/> documentation for "M:Rhino.PlugIns.PlugIn.ObjectPropertiesPages(System.Collections.Generic.List{Rhino.UI.ObjectPropertiesPage})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ObjectPropertiesPages_1.htm)

#### `protected virtual void ObjectPropertiesPages(ObjectPropertiesPageCollection collection)`

Override this function is you want to extend the object properties dialog. This method will be called each time a new document is created for each instance of a object properties panel. On Windows there will be a single panel per document but on Mac there may be many properties panel per document.

**Parameters:**
- `collection` (Rhino.UI.ObjectPropertiesPageCollection) — Add custom pages by calling collection.Add

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ObjectPropertiesPages.htm)

#### `public virtual bool OnAssignMaterial(IntPtr parent, RhinoDoc doc, ref Material material)`

This function is called by the Object Properties and Layer Control dialogs when the "Material" button is pressed in the "Render" tab. This is only called if EnableAssignMaterialButton returns true.

**Parameters:**
- `parent` (System.IntPtr) — [Missing <param name="parent"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.OnAssignMaterial(System.IntPtr,Rhino.RhinoDoc,Rhino.DocObjects.Material@)"]
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.OnAssignMaterial(System.IntPtr,Rhino.RhinoDoc,Rhino.DocObjects.Material@)"]
- `material` (Rhino.DocObjects.Material) — [Missing <param name="material"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.OnAssignMaterial(System.IntPtr,Rhino.RhinoDoc,Rhino.DocObjects.Material@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.RenderPlugIn.OnAssignMaterial(System.IntPtr,Rhino.RhinoDoc,Rhino.DocObjects.Material@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_OnAssignMaterial.htm)

#### `public virtual bool OnCreateMaterial(IntPtr parent, RhinoDoc doc, ref Material material)`

This function is called by the Object Properties and Layer Control dialogs when the "New" button is pressed in the "Material" tab. This is only called if EnableCreateMaterialButton returns true.

**Parameters:**
- `parent` (System.IntPtr) — [Missing <param name="parent"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.OnCreateMaterial(System.IntPtr,Rhino.RhinoDoc,Rhino.DocObjects.Material@)"]
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.OnCreateMaterial(System.IntPtr,Rhino.RhinoDoc,Rhino.DocObjects.Material@)"]
- `material` (Rhino.DocObjects.Material) — [Missing <param name="material"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.OnCreateMaterial(System.IntPtr,Rhino.RhinoDoc,Rhino.DocObjects.Material@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.RenderPlugIn.OnCreateMaterial(System.IntPtr,Rhino.RhinoDoc,Rhino.DocObjects.Material@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_OnCreateMaterial.htm)

#### `public virtual bool OnEditMaterial(IntPtr parent, RhinoDoc doc, ref Material material)`

This function is called by the Object Properties and Layer Control dialogs when the "Edit" button is pressed in the "Material" tab. This is only called if EnableEditMaterialButton returns true. A return value of true means the material has been updated.

**Parameters:**
- `parent` (System.IntPtr) — [Missing <param name="parent"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.OnEditMaterial(System.IntPtr,Rhino.RhinoDoc,Rhino.DocObjects.Material@)"]
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.OnEditMaterial(System.IntPtr,Rhino.RhinoDoc,Rhino.DocObjects.Material@)"]
- `material` (Rhino.DocObjects.Material) — [Missing <param name="material"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.OnEditMaterial(System.IntPtr,Rhino.RhinoDoc,Rhino.DocObjects.Material@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.RenderPlugIn.OnEditMaterial(System.IntPtr,Rhino.RhinoDoc,Rhino.DocObjects.Material@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_OnEditMaterial.htm)

#### `protected virtual LoadReturnCode OnLoad(ref string errorMessage)`

Is called when the plug-in is being loaded.

**Parameters:**
- `errorMessage` (System.String) — If a load error is returned and this string is set. This string is the error message that will be reported back to the user.

**Returns:** `LoadReturnCode` — An appropriate load return code. The default implementation returns Success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_OnLoad.htm)

#### `protected virtual void OnSetCurrent(bool current)`

This plug-in (has become)/(is no longer) the current render plug-in

**Parameters:**
- `current` (System.Boolean) — If true then this plug-in is now the current render plug-in otherwise it is no longer the current render plug-in.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_OnSetCurrent.htm)

#### `protected virtual void OnShutdown()`

(Inherited from PlugIn.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_OnShutdown.htm)

#### `protected virtual void OptionsDialogPages(List<OptionsDialogPage> pages)`

Override this function if you want to extend the options dialog. This function is called whenever the user brings up the Options dialog.

**Parameters:**
- `pages` (System.Collections.Generic.List<OptionsDialogPage>) — list of pages to add your custom options dialog page(s) to.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_OptionsDialogPages.htm)

#### `protected virtual RenderPlugIn.PreviewRenderTypes PreviewRenderType()`

Tell what kind of preview rendering your renderer supports.

**Returns:** `RenderPlugIn.PreviewRenderTypes` — One of PreviewRenderTypes

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_PreviewRenderType.htm)

#### `protected virtual void ReadDocument(RhinoDoc doc, BinaryArchiveReader archive, FileReadOptions options)`

Called whenever a Rhino document is being loaded and plug-in user data was encountered written by a plug-in with this plug-in's GUID.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — A Rhino document that is being loaded.
- `archive` (Rhino.FileIO.BinaryArchiveReader) — OpenNURBS file archive object Rhino is using to read this file. Use BinaryArchiveReader.Read*() functions to read plug-in data. If any BinaryArchive.Read*() functions throws an exception then archive.ReadErrorOccurve will be true and you should immediately return.
- `options` (Rhino.FileIO.FileReadOptions) — Describes what is being written.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ReadDocument.htm)

#### `protected bool RegisterCommand(Command command)`

(Inherited from PlugIn.)

**Parameters:**
- `command` (Rhino.Commands.Command) — [Missing <param name="command"/> documentation for "M:Rhino.PlugIns.PlugIn.RegisterCommand(Rhino.Commands.Command)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.RegisterCommand(Rhino.Commands.Command)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_RegisterCommand.htm)

#### `protected virtual void RegisterCustomRenderSaveFileTypes(CustomRenderSaveFileTypes saveFileTypes)`

Override this method to add custom file types to the render window save file dialog.

**Parameters:**
- `saveFileTypes` (Rhino.PlugIns.CustomRenderSaveFileTypes) — [Missing <param name="saveFileTypes"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.RegisterCustomRenderSaveFileTypes(Rhino.PlugIns.CustomRenderSaveFileTypes)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_RegisterCustomRenderSaveFileTypes.htm)

#### `protected virtual void RegisterRenderPanels(RenderPanels panels)`

Override this method and call RegisterPanel(PlugIn, RenderPanelType, Type, String, Boolean, Boolean) to add custom render UI to the render output window.

**Parameters:**
- `panels` (Rhino.Render.RenderPanels) — [Missing <param name="panels"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.RegisterRenderPanels(Rhino.Render.RenderPanels)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_RegisterRenderPanels.htm)

#### `protected virtual void RegisterRenderTabs(RenderTabs tabs)`

Override this method and call RegisterTab(PlugIn, Type, Guid, String, Icon) to add custom tabs to the render output window

**Parameters:**
- `tabs` (Rhino.Render.RenderTabs) — [Missing <param name="tabs"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.RegisterRenderTabs(Rhino.Render.RenderTabs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_RegisterRenderTabs.htm)

#### `protected abstract Result Render(RhinoDoc doc, RunMode mode, bool fastPreview)`

Called by Render and RenderPreview commands if this plug-in is set as the default render engine.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — A document.
- `mode` (Rhino.Commands.RunMode) — A command running mode.
- `fastPreview` (System.Boolean) — If true, lower quality faster render expected.

**Returns:** `Result` — If true, then the renderer is required to construct a rapid preview and not the high-quality final result.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_Render.htm)

#### `protected virtual IEnumerable<RenderContentSerializer> RenderContentSerializers()`

Called by Rhino when it is time to register RenderContentSerializer derived classes. Override this method and return an array of an instance of each serialize custom content object you wish to add.

**Returns:** `IEnumerable<RenderContentSerializer>` — List of RenderContentSerializer objects to register with the Rhino render content browsers.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_RenderContentSerializers.htm)

#### `protected virtual OptionsDialogPage RenderOptionsDialogPage(RhinoDoc doc)`

Override this method to replace the render properties page in the Rhino document properties dialog. The default implementation returns null which means just use the default Rhino page.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — The document properties to edit.

**Returns:** `OptionsDialogPage` — Return null to use the default Rhino page or return a page derived from OptionsDialogPage to replace the default page.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_RenderOptionsDialogPage.htm)

#### `public virtual void RenderSettingsCustomSections(List<ICollapsibleSection> sections)`

Override this function to provide custom sections for the render settings panel that are displayed when your plug-in is the current render plug-in.

**Parameters:**
- `sections` (System.Collections.Generic.List<ICollapsibleSection>) — Create your sections and return a list of them

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_RenderSettingsCustomSections.htm)

#### `protected virtual List<Guid> RenderSettingsSections()`

Override this method to provide the UUIDs of all sections that should be displayed in the Render Settings tab when this is the current renderer.The default implementation adds all the RDK's built-in Render Settings sections. These UUIDs start with the prefix uuidRenderSettingsSection'. They can be found in RhRdkUuids.h

**Remarks:** You should add \e all sections you would want to appear in any context.

**Returns:** `List<Guid>` — Return a Id list of content types to display in content browsers

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_RenderSettingsSections.htm)

#### `protected virtual Result RenderWindow(RhinoDoc doc, RunMode modes, bool fastPreview, RhinoView view, Rectangle rect, bool inWindow)`

This function is obsolete and only exists for legacy purposes. Do not override this function - prefer overriding the version with the blowup parameter.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.RenderWindow(Rhino.RhinoDoc,Rhino.Commands.RunMode,System.Boolean,Rhino.Display.RhinoView,System.Drawing.Rectangle,System.Boolean)"]
- `modes` (Rhino.Commands.RunMode) — [Missing <param name="modes"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.RenderWindow(Rhino.RhinoDoc,Rhino.Commands.RunMode,System.Boolean,Rhino.Display.RhinoView,System.Drawing.Rectangle,System.Boolean)"]
- `fastPreview` (System.Boolean) — [Missing <param name="fastPreview"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.RenderWindow(Rhino.RhinoDoc,Rhino.Commands.RunMode,System.Boolean,Rhino.Display.RhinoView,System.Drawing.Rectangle,System.Boolean)"]
- `view` (Rhino.Display.RhinoView) — [Missing <param name="view"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.RenderWindow(Rhino.RhinoDoc,Rhino.Commands.RunMode,System.Boolean,Rhino.Display.RhinoView,System.Drawing.Rectangle,System.Boolean)"]
- `rect` (System.Drawing.Rectangle) — [Missing <param name="rect"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.RenderWindow(Rhino.RhinoDoc,Rhino.Commands.RunMode,System.Boolean,Rhino.Display.RhinoView,System.Drawing.Rectangle,System.Boolean)"]
- `inWindow` (System.Boolean) — [Missing <param name="inWindow"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.RenderWindow(Rhino.RhinoDoc,Rhino.Commands.RunMode,System.Boolean,Rhino.Display.RhinoView,System.Drawing.Rectangle,System.Boolean)"]

**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.PlugIns.RenderPlugIn.RenderWindow(Rhino.RhinoDoc,Rhino.Commands.RunMode,System.Boolean,Rhino.Display.RhinoView,System.Drawing.Rectangle,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_RenderWindow.htm)

#### `protected virtual Result RenderWindow(RhinoDoc doc, RunMode modes, bool fastPreview, RhinoView view, Rectangle rect, bool inWindow, bool blowup)`



**Parameters:**
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.RenderWindow(Rhino.RhinoDoc,Rhino.Commands.RunMode,System.Boolean,Rhino.Display.RhinoView,System.Drawing.Rectangle,System.Boolean,System.Boolean)"]
- `modes` (Rhino.Commands.RunMode) — [Missing <param name="modes"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.RenderWindow(Rhino.RhinoDoc,Rhino.Commands.RunMode,System.Boolean,Rhino.Display.RhinoView,System.Drawing.Rectangle,System.Boolean,System.Boolean)"]
- `fastPreview` (System.Boolean) — [Missing <param name="fastPreview"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.RenderWindow(Rhino.RhinoDoc,Rhino.Commands.RunMode,System.Boolean,Rhino.Display.RhinoView,System.Drawing.Rectangle,System.Boolean,System.Boolean)"]
- `view` (Rhino.Display.RhinoView) — [Missing <param name="view"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.RenderWindow(Rhino.RhinoDoc,Rhino.Commands.RunMode,System.Boolean,Rhino.Display.RhinoView,System.Drawing.Rectangle,System.Boolean,System.Boolean)"]
- `rect` (System.Drawing.Rectangle) — [Missing <param name="rect"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.RenderWindow(Rhino.RhinoDoc,Rhino.Commands.RunMode,System.Boolean,Rhino.Display.RhinoView,System.Drawing.Rectangle,System.Boolean,System.Boolean)"]
- `inWindow` (System.Boolean) — [Missing <param name="inWindow"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.RenderWindow(Rhino.RhinoDoc,Rhino.Commands.RunMode,System.Boolean,Rhino.Display.RhinoView,System.Drawing.Rectangle,System.Boolean,System.Boolean)"]
- `blowup` (System.Boolean) — [Missing <param name="blowup"/> documentation for "M:Rhino.PlugIns.RenderPlugIn.RenderWindow(Rhino.RhinoDoc,Rhino.Commands.RunMode,System.Boolean,Rhino.Display.RhinoView,System.Drawing.Rectangle,System.Boolean,System.Boolean)"]

**Returns:** `Result` — [Missing <returns> documentation for "M:Rhino.PlugIns.RenderPlugIn.RenderWindow(Rhino.RhinoDoc,Rhino.Commands.RunMode,System.Boolean,Rhino.Display.RhinoView,System.Drawing.Rectangle,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_RenderWindow_1.htm)

#### `protected virtual void ResetMessageBoxes()`

(Inherited from PlugIn.)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ResetMessageBoxes.htm)

#### `protected bool ReturnLicense()`

Returns, or releases, a product license that was obtained from the Rhino licensing system. Note, most plug-ins do not need to call this as the Rhino licensing system will return all licenses when Rhino shuts down.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PlugIns.PlugIn.ReturnLicense"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ReturnLicense.htm)

#### `public void SaveSettings()`

Write settings to disk which will raise a SettingsSaved event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_SaveSettings.htm)

#### `protected void SetLicenseCapabilities(string textMask, LicenseCapabilities capabilities, Guid licenseId)`

(Inherited from PlugIn.)

**Parameters:**
- `textMask` (System.String) — [Missing <param name="textMask"/> documentation for "M:Rhino.PlugIns.PlugIn.SetLicenseCapabilities(System.String,Rhino.PlugIns.LicenseCapabilities,System.Guid)"]
- `capabilities` (Rhino.PlugIns.LicenseCapabilities) — [Missing <param name="capabilities"/> documentation for "M:Rhino.PlugIns.PlugIn.SetLicenseCapabilities(System.String,Rhino.PlugIns.LicenseCapabilities,System.Guid)"]
- `licenseId` (System.Guid) — [Missing <param name="licenseId"/> documentation for "M:Rhino.PlugIns.PlugIn.SetLicenseCapabilities(System.String,Rhino.PlugIns.LicenseCapabilities,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_SetLicenseCapabilities.htm)

#### `protected virtual bool ShouldCallWriteDocument(FileWriteOptions options)`

Called whenever a Rhino is about to save a .3dm file. If you want to save plug-in document data when a model is saved in a version 5 .3dm file, then you must override this function to return true and you must override WriteDocument().

**Parameters:**
- `options` (Rhino.FileIO.FileWriteOptions) — The file write options, such as "include preview image" and "include render meshes".

**Returns:** `Boolean` — true if the plug-in wants to save document user data in the version 5 .3dm file. The default returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_ShouldCallWriteDocument.htm)

#### `protected virtual bool ShowDecalProperties(ref List<NamedValue> properties)`

Override this function to handle showing a modal dialog with your plug-in's custom decal properties. You will be passed the current properties for the object being edited. The defaults will be set in InitializeDecalProperties.

**Parameters:**
- `properties` (System.Collections.Generic.List<NamedValue>) — A list of named values that will be stored on the object the input values are the current ones, you should modify the values after the dialog closes.

**Returns:** `Boolean` — true if the user pressed "OK", otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_ShowDecalProperties.htm)

#### `public virtual void SunCustomSections(List<ICollapsibleSection> sections)`

Override this function to provide custom sections for the sun panel that are displayed when your plug-in is the current render plug-in.

**Parameters:**
- `sections` (System.Collections.Generic.List<ICollapsibleSection>) — Create your sections and return a list of them

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_SunCustomSections.htm)

#### `protected virtual List<FileType> SupportedOutputTypes()`

Returns a list of output types which your renderer can write. The default implementation returns BMP, JPG, PNG, TIF, TGA, HDR, EXR and RIMAGE.

**Returns:** `List<FileType>` — A list of file types.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_SupportedOutputTypes.htm)

#### `protected virtual bool SupportsFeature(RenderPlugIn.RenderFeature feature)`

Determines if your renderer supports a specific feature.

**Parameters:**
- `feature` (Rhino.PlugIns.RenderPlugIn.RenderFeature) — A feature to be controlled.

**Returns:** `Boolean` — true if the renderer indeed supports the feature.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_SupportsFeature.htm)

#### `protected virtual List<Guid> UiContentTypes()`

Override this method to provide the UUIDs of all content types that should be presented to the user in the types combo box or the[+] button types menu.The default implementation adds only RDK's built-in types. Rhino automatically adds types in the most efficient way to minimize the list length. If you override this method, you may call the base class first to add the built-in types, a separator will be inserted at the end of the standard list followed by your own types. You may omit the base class call and only chosen types yourself, followed by a separator and your own types. A 'More Types...' item is automatically added when needed by Rhino. Specify a separator by adding uuidUiContentType_Separator.

**Remarks:** You should add \e all types you would want to appear in any context, regardless of their content kind. Rhino ensures that only types that make sense will actually be presented to the user in a given context.

**Returns:** `List<Guid>` — Return a Id list of content types to display in content browsers

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_RenderPlugIn_UiContentTypes.htm)

#### `protected virtual void WriteDocument(RhinoDoc doc, BinaryArchiveWriter archive, FileWriteOptions options)`

Called when Rhino is saving a .3dm file to allow the plug-in to save document user data.

**Parameters:**
- `doc` (Rhino.RhinoDoc) — The Rhino document instance that is being saved.
- `archive` (Rhino.FileIO.BinaryArchiveWriter) — OpenNURBS file archive object Rhino is using to write the file. Use BinaryArchiveWriter.Write*() functions to write plug-in data. OR use the ArchivableDictionary If any BinaryArchiveWriter.Write*() functions throw an exception, then archive.WriteErrorOccured will be true and you should immediately return. Setting archive.WriteErrorOccured to true will cause Rhino to stop saving the file.
- `options` (Rhino.FileIO.FileWriteOptions) — The file write options, such as "include preview image" and "include render meshes".

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PlugIns_PlugIn_WriteDocument.htm)

### Properties
- `AddToHelpMenu` (Boolean, get) — Called by Rhino to determine if the plug-in name should be added to the Rhino Help/Plug-ins menu.
- `Assembly` (Assembly, get) — Source assembly for this plug-in.
- `Description` (String, get) — Returns the description of the plug-in, as found in the plug-in's assembly attributes.
- `Id` (Guid, get) — Returns the Guid, or unique Id, of the plug-in.
- `InitialChannelToDisplay` (Guid, get) — 
- `LicenseId` (Guid, get) — (Inherited from PlugIn.)
- `LoadAtStartup` (Boolean, get) — Obsolete. (Inherited from PlugIn.)
- `LoadTime` (PlugInLoadTime, get) — Plug-ins are typically loaded on demand when they are first needed. You can change this behavior to load the plug-in at during different stages in time by overriding this property.
- `LocalPlugInName` (String, get) — Optionally override this to provide a localized plug-in name
- `Name` (String, get) — Returns the name of the plug-in, as found in the plug-in's assembly attributes.
- `PerferBasicContent` (Boolean, get/set) — Set to true if you would like Rhino to quickly create a basic render content in response to 'Create New' commands. Set to false if you would prefer Rhino to display the render content chooser dialog.
- `Settings` (PersistentSettings, get) — Persistent plug-in settings.
- `SettingsDirectory` (String, get) — Get the plug-in's settings directory. This is the directory where the plug-in's persistent settings files are saved. This directory will be located in the user's profile folder. Note, this does not verify the directory exists.
- `SettingsDirectoryAllUsers` (String, get) — Get the plug-in's "all users" settings directory. This directory will be located in the system's program data folder. Note, this does not verify the directory exists.
- `SupportedChannels` (Guid[], get) — Override to communicate that the renderer supports more channels beside the default channels RGBA, Depth, Normal, Albedo. See RenderWindow.StandardChannels. RenderWindow.ChannelId can be used to get the GUIDs for the channels to support
- `SupportsEditProperties` (Boolean, get) — Override this property and return true if your plug-in supports decals and overrides ShowDecalProperties(List<NamedValue>)
- `Version` (String, get) — Returns the version of the plug-in, as found in the plug-in's assembly attributes.
- `WindowPositionSettings` (PersistentSettings, get) — (Inherited from PlugIn.)

### Events
#### `SettingsSaved` (`System.EventHandler<PersistentSettingsSavedEventArgs>`)

**Signature:** `public event EventHandler<PersistentSettingsSavedEventArgs> SettingsSaved`

This event is raised when an instance of Rhino has modified the external settings file associated with this plug-in's Settings property.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_PlugIns_PlugIn_SettingsSaved.htm)

## RenderPlugIn.PreviewRenderTypes (enum)

[Missing <summary> documentation for "T:Rhino.PlugIns.RenderPlugIn.PreviewRenderTypes"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_RenderPlugIn_PreviewRenderTypes.htm)

### Values
- `None` = `0`
- `ThreeSeparateImages` = `1`
- `SingleImage` = `2`
- `Progressive` = `3`

## RenderPlugIn.RenderFeature (enum)

[Missing <summary> documentation for "T:Rhino.PlugIns.RenderPlugIn.RenderFeature"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_RenderPlugIn_RenderFeature.htm)

### Values
- `Materials` = `0`
- `Environments` = `1`
- `Textures` = `2`
- `PostEffects` = `3`
- `Sun` = `4`
- `CustomRenderMeshes` = `5`
- `Decals` = `6`
- `GroundPlane` = `7`
- `SkyLight` = `8`
- `CustomDecalProperties` = `9`
- `LinearWorkflow` = `10`
- `Exposure` = `11`
- `ShadowOnlyGroundPlane` = `12`
- `RenderBlowup` = `13`
- `RenderWindow` = `14`
- `RenderInWindow` = `15`
- `FocalBlur` = `17`
- `RenderArctic` = `18`
- `RenderViewSource` = `19`
- `CustomSkylightEnvironment` = `20`
- `CustomReflectionEnvironment` = `21`
- `RenderChannels` = `22`
- `LightMaterials` = `23`

## ValidateProductKeyDelegate (delegate)

Validates a product key or license.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_ValidateProductKeyDelegate.htm)

## ValidateResult (enum)

ValidateProductKeyDelegate result code.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_ValidateResult.htm)

### Values
- `Success` = `1` — The product key or license is validated successfully.
- `ErrorShowMessage` = `0` — There was an error validating the product key or license, the license manager show an error message.
- `ErrorHideMessage` = `-1` — There was an error validating the product key or license. The validating delegate will show an error message, not the license manager.

## VerifyLicenseKeyDelegate (delegate)

Called by Rhino to verify a license key. For details, see http://developer.rhino3d.com/guides/rhinocommon/rhinocommon-zoo-plugins/

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_VerifyLicenseKeyDelegate.htm)

## VerifyPreviousVersionLicenseDelegate (delegate)

Called by GetLicense/AskUserForLicense to verify that a previous version license.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_VerifyPreviousVersionLicenseDelegate.htm)

## WriteFileResult (enum)

[Missing <summary> documentation for "T:Rhino.PlugIns.WriteFileResult"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PlugIns_WriteFileResult.htm)

### Values
- `Cancel` = `-1`
- `Failure` = `0`
- `Success` = `1`

