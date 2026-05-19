---
name: grasshopper-gh_io-userinterface
description: This skill encodes the grasshopper 8.0 surface of the GH_IO.UserInterface namespace — 2 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GH_ArchiveMessageViewer, GH_DeveloperDetails.
---

# GH_IO.UserInterface

Auto-generated from vendor docs for grasshopper 8.0. 2 types in this namespace.

## GH_ArchiveMessageViewer (class)

Viewer/Browser that displays a list of archive messages.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_GH_IO_UserInterface_GH_ArchiveMessageViewer.htm)

### Constructors
- `public GH_ArchiveMessageViewer()` — Constructor.

### Methods
#### `protected override void Dispose(bool disposing)`

Clean up any resources being used.

**Parameters:**
- `disposing` (System.Boolean) — true if managed resources should be disposed; otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_UserInterface_GH_ArchiveMessageViewer_Dispose.htm)

#### `public void SetArchive(GH_Archive nArchive)`

Set a new archive. The messages in this archive will be displayed on the form.

**Parameters:**
- `nArchive` (GH_IO.Serialization.GH_Archive)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_UserInterface_GH_ArchiveMessageViewer_SetArchive.htm)

## GH_DeveloperDetails (class)

Constains a set of static fields and properties regarding developer contact details.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_GH_IO_UserInterface_GH_DeveloperDetails.htm)

### Properties
- `DefaultDeveloperContact` (String, get) — Gets the default developer contact details.
- `DeveloperContact` (String, get/set) — Gets or sets the local developer contact info (e-mail). By default, the developer email is DefaultDeveloperContact; This email address is used in the Message Report viewer when sending a support e-mail.

