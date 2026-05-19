---
name: tedds-tekla-structural-interopassemblies-tedds
description: This skill encodes the tedds 25.0 surface of the Tekla.Structural.InteropAssemblies.Tedds namespace — 15 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: IApplication, IBimProject, ILibraryCalcItem, IProjectFileItem, IProjectFolderItem, IProjectItem, IProjectItems, ITeddsDocument, and 7 more types.
---

# Tekla.Structural.InteropAssemblies.Tedds

Auto-generated from vendor docs for tedds 25.0. 15 types in this namespace.

## IApplication (interface)

Interface for the Main application

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-tedds-iapplication-41831)

### Methods
#### `void Activate()`

Activates the user interface of the application

[Docs](https://developer.tekla.com/topic/en/20/45/af0684849d8f93ec5a6a39b618034ad7b37f7a5ab59970268dabd913d9d21f4a)

#### `void Move(int Left, int Top)`

Move the applications main window

**Parameters:**
- `Left` (int) — New position of left edge of window in screen co-ordinates
- `Top` (int) — New position of top edge of window in screen co-ordinates

[Docs](https://developer.tekla.com/topic/en/20/45/e2494933c1fd5b4a31a2fcde3ba43eec6ecc4a463047910485860aa5afefea6f)

#### `void Quit(bool SaveChanges)`

Exit the application

**Parameters:**
- `SaveChanges` (bool) — Determines whether changes to unsaved documents should be saved or discarded

[Docs](https://developer.tekla.com/topic/en/20/45/1acbb71ed797106f3b430841e1721be0b1179e63fcb1fefeb4cc2bd40406043f)

#### `void Resize(int Width, int Height)`

Size the applications main window

**Parameters:**
- `Width` (int) — Width of window in pixels
- `Height` (int) — Height of window in pixels

[Docs](https://developer.tekla.com/topic/en/20/45/0744c98b7353e04e55c9605b2d6150f965d126e1ea28c19e91517173ce9f0ceb)

### Properties
- `ActiveDocument` (ITeddsDocument, get) — Returns active document
- `Build` (string, get) — Returns the version build number of the Tedds software
- `Documents` (ITeddsDocuments, get) — Returns A Documents collection which represents all the open documents
- `Height` (int, get/set) — Height of the applications main window
- `Left` (int, get/set) — Position of the left edge of the applications main window
- `Project` (ITeddsProject, get) — Returns the active project
- `Top` (int, get/set) — Position of the top edge of the applications main window
- `Version` (string, get) — Returns the version number of the Tedds software
- `VersionLibraryMajor` (int, get) — Returns the major version number of the Tedds Engineering Library
- `VersionLibraryMinor` (int, get) — Returns the minor version number of the Tedds Engineering Library
- `VersionMajor` (int, get) — Returns the major version number of the Tedds software
- `VersionMinor` (int, get) — Returns the minor version number of the Tedds software
- `Visible` (bool, get/set) — Visibility status of the applications user interface
- `Width` (int, get/set) — Width of the applications main window
- `WindowHandle` (int, get) — Returns the window handle of the applications main window
- `WindowState` (TdsWindowState, get/set) — View state of the applications main window

## IBimProject (interface)

BIM project

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-tedds-ibimproject-41832)

### Properties
- `ModelId` (string, get/set) — Model identifier
- `ProjectId` (string, get) — Project identifier
- `ServiceId` (string, get) — Service provider identifier

## ILibraryCalcItem (interface)

Interface for a library calc item

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-tedds-ilibrarycalcitem-41833)

### Properties
- `CalcName` (string, get/set) — Calculation name
- `FileName` (string, get/set) — Calc library file name including the path which may contain placeholders
- `ItemName` (string, get/set) — Calc library item name

## IProjectFileItem (interface)

Interface for a project file item

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-tedds-iprojectfileitem-41834)

### Methods
#### `ITeddsDocument Open()`

Open the referenced file and return the ITeddsDocument interface

**Returns:** `ITeddsDocument` — Opened document

[Docs](https://developer.tekla.com/topic/en/20/45/8709ba851fc346ac762a3f2277591629d38fa02471af293b8d8e210a6e0f3c39)

### Properties
- `FullName` (string, get) — Returns the full path and file name of the referenced file
- `IsFolder` (bool, get) — Returns whether the object is a folder
- `Name` (string, get/set) — File name (excluding file extension)
- `ParentFolder` (IProjectFolderItem, get) — Parent folder in the project hierarchy
- `Path` (string, get) — Returns the path of the referenced file

## IProjectFolderItem (interface)

Interface for a project folder

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-tedds-iprojectfolderitem-41835)

### Properties
- `IsFolder` (bool, get) — Returns whether the object is a folder
- `Items` (IProjectItems, get) — Returns the collection of child project item in the folder
- `Name` (string, get/set) — Folder name
- `ParentFolder` (IProjectFolderItem, get) — Parent folder in the project hierarchy

## IProjectItem (interface)

Interface for a project item

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-tedds-iprojectitem-41836)

### Methods
#### `void Remove()`

Remove project item from the project

[Docs](https://developer.tekla.com/topic/en/20/45/6823224116e6c65be115e853c5da1bfe4de5eafdb5058ca68a887e6566bcd086)

### Properties
- `IsFolder` (bool, get) — Returns whether the object is a folder
- `Name` (string, get/set) — Name of the project item
- `ParentFolder` (IProjectFolderItem, get) — Parent folder in the project hierarchy

## IProjectItems (interface)

Interface for a collection of Project items

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-tedds-iprojectitems-41837)

### Methods
#### `IProjectFileItem AddDocument(ITeddsDocument Document)`

Add an existing open document to the end of the collection of project items

**Parameters:**
- `Document` (ITeddsDocument) — Document to add

**Returns:** `IProjectFileItem` — New project file object

[Docs](https://developer.tekla.com/topic/en/20/45/d9162bd93cb1943d2e30b75278a028c2388e9fda3a7fb8eea3a44bb308ac3747)

#### `IProjectFileItem AddFile(string FilePath)`

Add a reference to a Tedds document file to the end of the collection of project items

**Parameters:**
- `FilePath` (string) — Full file path of the file to add

**Returns:** `IProjectFileItem` — New project file object

[Docs](https://developer.tekla.com/topic/en/20/45/948a05bcc216da51464939cae5218447eda15f85a15a86dae7d1f1eb750c92ca)

#### `IProjectFolderItem AddFolder(string Name)`

Add a new folder to the end of the collection of project items

**Parameters:**
- `Name` (string) — Name of the folder

**Returns:** `IProjectFolderItem` — New project folder object

[Docs](https://developer.tekla.com/topic/en/20/45/f6fa5a8682001a1e694926f8c31413572e3c9667a95a0c02514408c2a56948b5)

#### `void AddFromPath(string FilePath)`

Add a single Tedds project file or a complete folder hierarchy from a full file path

**Parameters:**
- `FilePath` (string) — Full path to a file or folder

[Docs](https://developer.tekla.com/topic/en/20/45/c3cbc28e95402a077dfc2000cc1f079458a4dfb8a224091fd1829f371e0c14cb)

#### `IEnumerator GetEnumerator()`

Enumeration interface for iterating the items in the collection

**Returns:** `IEnumerator` — Enumeration interface

[Docs](https://developer.tekla.com/topic/en/20/45/b8ec794715b4ea12de62b8e45f468a7f59bf11908c2e701720e095b30a8ee7a8)

#### `void Remove(object Item)`

Remove a project item from the collection

**Parameters:**
- `Item` (object) — Item Identifier of the project item to remove which is either a one based index of the item in the

[Docs](https://developer.tekla.com/topic/en/20/45/3f9a6fc64aed8addaa5c1d92c1a6243a15f0780c2e09812e2f85fe46b10b00ce)

### Properties
- `Count` (int, get) — Return the number of items in the collection
- `this[object]` (IProjectItem, get) — Returns a single item in the collection

## ITeddsDocument (interface)

Interface for an open Document

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-tedds-iteddsdocument-41838)

### Methods
#### `void Activate()`

Activate this document

[Docs](https://developer.tekla.com/topic/en/20/45/b72ce91ea2daab02e20725fd1b22c289fddebba79f7777b5e1ef1b64efaa5454)

#### `void Calculate()`

Calculate document

[Docs](https://developer.tekla.com/topic/en/20/45/96565629bd31e7a5c2bd4002c8fad2bf50cff944022568d1d597ed1ca912e81d)

#### `TdsCalcStatus Calculate2()`

Calculate the document and return the calculation status code

**Returns:** `TdsCalcStatus` — Calculation status code

[Docs](https://developer.tekla.com/topic/en/20/45/ab9a16f635d573a4fc1e06b05a63b116b5960ae0569e0e91e58340668e182645)

#### `void Close(TdsSaveOptions SaveChanges)`

Close the document

**Parameters:**
- `SaveChanges` (TdsSaveOptions) — Determines whether changes should be saved or discarded

[Docs](https://developer.tekla.com/topic/en/20/45/0724db0ab7e28031bfb8e4e94f0cd2bd38efcc782ffe9379f2ed90e3899640f3)

#### `void Save()`

Save changes to the document

[Docs](https://developer.tekla.com/topic/en/20/45/98e1cb2125038836139c70819aaa8cae76dfa7f311a3b49a359cd9823cd8c91e)

#### `void SaveAs(object FileName)`

Save document to a new file location

**Parameters:**
- `FileName` (object) — Full path of file location

[Docs](https://developer.tekla.com/topic/en/20/45/608135ecfcc6798c77a31e28dc96da4bbd4f2c5878e1fd1163f7f8d5ca156523)

#### `void SaveAsPdf(object FileName)`

Save the document in Adobe PDF format

**Parameters:**
- `FileName` (object) — Full path and filename of location to save file

[Docs](https://developer.tekla.com/topic/en/20/45/92bbbb7d8f2a0245f6c997d8c3b8302553b7a691691a2a26fd9cf3762273008c)

### Properties
- `BimObjectId` (string, get/set) — Returns a BIM object identifier associated with the document
- `CalcItem` (object, get) — Returns a CalcItem object which represents the calculation associated with the document
- `FullName` (string, get) — Returns the document full path and file name
- `Name` (string, get) — Returns the documents name (excluding file extension)
- `Path` (string, get) — Return documents file path (excluding file name)
- `Saved` (bool, get/set) — Saved status of document
- `VariablesXml` (string, get/set) — Variables defined in the document in the Tedds XML schema format

## ITeddsDocuments (interface)

Interface for a collection of open documents

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-tedds-iteddsdocuments-41839)

### Methods
#### `ITeddsDocument Add()`

Add a new document

**Remarks:** Adding a new document will prompt the user to choose the calculation they want to use and the document will be calculated.

**Returns:** `ITeddsDocument` — The new document

[Docs](https://developer.tekla.com/topic/en/20/45/6668469fd04a141cd73e6529dad1efe590488f4d0bdaa6a784e210e293e14303)

#### `ITeddsDocument Add2(string libraryFileName, string libraryItemName, string VariablesXml)`

Add a new document using the specified Calc Item and input variables

**Parameters:**
- `libraryFileName` (string) — Name of the Calc Library file in which the Calc Item exists
- `libraryItemName` (string) — Short name of the Calc Item which is used to start the calculation
- `VariablesXml` (string) — Input variables for the document which have been created using an existing document or via the Tedds Calculator API.

**Returns:** `ITeddsDocument` — The new document

[Docs](https://developer.tekla.com/topic/en/20/45/52340a54a523df5fdc8cb0ca66f6b53c9d4b29ef1801af9f43a414a1153b48f3)

#### `ITeddsDocument Add3(string documentName, string libraryFileName, string libraryItemName, string VariablesXml, string outputRtf)`

Add a new document using the specified Calc Item, input variables and pre-calculated RTF

**Remarks:** The input variables XML and the output RTF can be directly retrieved by using the Tedds Calculator API.

**Parameters:**
- `documentName` (string) — Name for the new document
- `libraryFileName` (string) — Name of the Calc Library file in which the Calc Item exists
- `libraryItemName` (string) — Short name of the Calc Item which is used to start the calculation
- `VariablesXml` (string) — Input variables for the document which have been created using an existing document or via the Tedds Calculator API .
- `outputRtf` (string) — Output in RTF format which has been retrieved from an existing document or via the Tedds Calculator API.

**Returns:** `ITeddsDocument` — The new document

[Docs](https://developer.tekla.com/topic/en/20/45/8100f1a6190a880e4f0e983d9526421c00fce8b535031f5d4907624881e6269b)

#### `void Close(bool SaveChanges)`

Close all documents

**Parameters:**
- `SaveChanges` (bool) — false to discard unsaved changes; otherwise save changes

[Docs](https://developer.tekla.com/topic/en/20/45/571eec996e9813b9de40df102b3714e8411d30cea66a20bd60ae4a02ab117a43)

#### `IEnumerator GetEnumerator()`

Enumeration interface for iterating the items in the collection

**Returns:** `IEnumerator` — Enumeration interface

[Docs](https://developer.tekla.com/topic/en/20/45/e752a76a0ddcc4bf3d07e87f81b98a0244b2b5d6e7c9ae6dff083f32dd92d25f)

#### `ITeddsDocument Open(string FilePath)`

Open an existing document

**Parameters:**
- `FilePath` (string) — Full path and file name

**Returns:** `ITeddsDocument` — Open document

[Docs](https://developer.tekla.com/topic/en/20/45/b234bdbd437e001a0180baabb7f5d3333b3123c55ead97950944871041ce3c4e)

#### `void Save()`

Saved changes to all open documents

[Docs](https://developer.tekla.com/topic/en/20/45/dd7411ea67dd1c4b29dc84eb203fde9aec82902a2ae0a156504af837f34f6535)

### Properties
- `Count` (int, get) — Returns the number of documents in the collection
- `this[object]` (ITeddsDocument, get) — Returns a single document in the collection

## ITeddsProject (interface)

Interface for a project

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-tedds-iteddsproject-41840)

### Methods
#### `void Calculate()`

Calculate all documents in the project

[Docs](https://developer.tekla.com/topic/en/20/45/2838496f69be1f7a83cba03368fa67a812931523f69728bda5db4093d59142ba)

#### `void Close(object SaveChanges)`

Close the active project

**Parameters:**
- `SaveChanges` (object) — false to discard unsaved project file changes; otherwise save changes

[Docs](https://developer.tekla.com/topic/en/20/45/2c87f3248346ed285fe563afde1337da50b9b562006f59af90b5a415dd65851a)

#### `void New(string Name)`

Create a new project

**Parameters:**
- `Name` (string) — Name for the new project

[Docs](https://developer.tekla.com/topic/en/20/45/f450f22c6de708cac674cd161b16011a1453cbc42c965b2b908403c9c1dd45d9)

#### `void Open(string FileName)`

Open an existing project

**Parameters:**
- `FileName` (string) — Full path to project file

[Docs](https://developer.tekla.com/topic/en/20/45/c4360567d5ecb42097b5757d2e677f3db24eca2067c6f85ae3b0ac67f6445a24)

#### `void Save()`

Save the active project file

[Docs](https://developer.tekla.com/topic/en/20/45/690c80c88800cd7ac0044f0fc7ea5e87100a46e94d76e52b25518402b2457a54)

#### `void SaveAs(object FileName)`

Save the active project file to a new location

**Parameters:**
- `FileName` (object) — Full path to project file

[Docs](https://developer.tekla.com/topic/en/20/45/4322db7d765395e3247fe7c22aa17849757bf97adc8ef419629dcbd837fbf1f8)

#### `string[] SaveAsPdf(string folderName, TdsProjectSavePdf options)`

Save all the documents in the project in PDF format

**Parameters:**
- `folderName` (string) — Path of location where to save files
- `options` (TdsProjectSavePdf) — Save project to PDF options

**Returns:** `string[]` — Array of full file paths for all the output PDF files created

[Docs](https://developer.tekla.com/topic/en/20/45/bb489743c62a33e414db7190b50da208c87078375f088a5688b75c6ef00cdfcd)

#### `void SendToWord(bool LoadTedds)`

Export the project to Microsoft Word or Tedds for Word

**Parameters:**
- `LoadTedds` (bool) — false to just load Microsoft Word; otherwise also load the Tedds for Word Add-In

[Docs](https://developer.tekla.com/topic/en/20/45/7e32f74ab249fcacf436e6678d913be8bb4c8810103a12b4daa0e2e4088af50f)

#### `void SetBimProject(string serviceId, string projectId)`

Link the Tedds project to a BIM project

**Parameters:**
- `serviceId` (string) — Service identifier
- `projectId` (string) — Project identifier

[Docs](https://developer.tekla.com/topic/en/20/45/0a95195fed20f819a0532d7984d1b6a196a1e67c9596a66a32e725b1fc065716)

### Properties
- `BimProject` (IBimProject, get) — Linked BIM project
- `FullName` (string, get) — Full path of project file including file name
- `IsOpen` (bool, get) — Is a project currently open
- `Name` (string, get) — Project name
- `Path` (string, get) — Project file path (excludes file name)
- `RootFolder` (IProjectFolderItem, get) — Root folder in the project tree
- `Saved` (bool, get/set) — Saved status of project file

## TdsCalcStatus (enum)

Calculation status

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-tedds-tdscalcstatus-41841)

### Values
- `Aborted` = `-1` — Calculation process was aborted
- `Interrupted` = `2` — The calculation process was interrupted by the user
- `NotInitialized` = `0` — The Tedds Calculator has not been initialized
- `Ok` = `1` — Calculating completed successfully

## TdsProjectSavePdf (enum)

Save project to PDF options

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-tedds-tdsprojectsavepdf-41842)

### Values
- `HideOptions` = `4` — Hide options user interface
- `HideProgress` = `8` — Hide progress
- `InsertDocPageBreaks` = `16` — Insert document page breaks
- `OneFilePerDocument` = `1` — Save each document as a separate PDF
- `ToDocumentFolder` = `2` — Save PDFs to document folder

## TdsSaveOptions (enum)

File save options

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-tedds-tdssaveoptions-41843)

### Values
- `DoNotSaveChanges` = `0` — Discard unsaved changes
- `PromptToSaveChanges` = `-2` — Prompt the user to confirm saving any unsaved changes
- `SaveChanges` = `-1` — Save any unsaved changes

## TdsWindowState (enum)

Window state

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-tedds-tdswindowstate-41844)

### Values
- `Maximize` = `1` — Maximize
- `Minimize` = `2` — Minimize
- `Normal` = `0` — Normal

## _IApplicationEvents (interface)

Interface for Tedds application events

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-tedds-iapplicationevents-41845)

### Methods
#### `void DocumentExported(ITeddsDocument document, string fileName)`

A document has been exported

**Parameters:**
- `document` (ITeddsDocument) — The document that was exported
- `fileName` (string) — The filename of the document that was exported

[Docs](https://developer.tekla.com/topic/en/20/45/783978bfae273b231e878a27dbe58cdc1cf878d71910c3ec33d41ff565187f88)

#### `void ProjectBimProjectModified(ITeddsProject project)`

A project's BIM project modified

**Parameters:**
- `project` (ITeddsProject) — The project that was modified

[Docs](https://developer.tekla.com/topic/en/20/45/d6ae6e5b78aa170fac8d93c3aea4deb3287c4596f0a3f92ff12ba1e80106ba2f)

#### `void ProjectClosed()`

A project has been closed

[Docs](https://developer.tekla.com/topic/en/20/45/f70a6fb6bd6f50e3ee15b827a4c7fa07619ca8eb6eacec36700019a060bdf154)

#### `void ProjectCreated(ITeddsProject project)`

A new project has been created

**Parameters:**
- `project` (ITeddsProject) — The new project

[Docs](https://developer.tekla.com/topic/en/20/45/364e05a164d560e8ea2757032d1208118e2494e464739b490301c86d7689733c)

#### `void ProjectOpened(ITeddsProject project)`

A project has been opened

**Parameters:**
- `project` (ITeddsProject) — The project that was opened

[Docs](https://developer.tekla.com/topic/en/20/45/91f6856f2ed839f15f5b96314d914eb12180318e87b2deb35708bdcd3b7393ea)

