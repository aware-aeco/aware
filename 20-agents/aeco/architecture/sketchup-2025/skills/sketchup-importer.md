---
name: yard-sketchup-importer
description: Sketchup::Importer API reference (YARD)
---

# Sketchup::Importer API reference

The Importer interface lets you build your own importers for SketchUp. To use this, you create a subclass of Importer and implement all of the methods described below. This will make your importer appear in the list that users see under File > Import, and you can use Ruby to do all of the work of opening the file and creating whatever you need inside SketchUp.

## Methods

- `description` — This method is called by SketchUp to determine the description that appears in the File > Import dialog's pulldown list of valid importers.
- `do_options` — This method is called by SketchUp when the user clicks on the “Options” button inside the File > Import dialog. You can use it to gather and store settings for your importer.
- `file_extension` — This method is called by SketchUp to determine a single file extension is associated with your importer. Only files that match this extension will be shown to the user as they browse their harddrive for things to import.
- `id` — This method is called by SketchUp to determine a unique identifier for your importer, typically something like “com.sketchup.importers.dxf”.
- `load_file` — This method is called by SketchUp after the user has selected a file to import. This is where you do the real work by opening the file via Ruby's File object and processing it in whatever way you need.
- `supports_options?` — This method is called by SketchUp to determine if the “Options” button inside the File > Import dialog should be enabled while your importer is selected.
