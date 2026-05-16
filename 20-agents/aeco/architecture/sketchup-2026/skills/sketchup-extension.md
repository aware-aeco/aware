---
name: yard-sketchup-extension
description: SketchupExtension API reference (YARD)
---

# SketchupExtension API reference

The SketchupExtension class contains methods allowing you to create and manipulate SketchUp extensions. Extensions are Ruby scripts that can be loaded and unloaded using the Extension manager (Extensions panel of the Extension Manager dialog box). Generally you should register your ruby scripts as an extension to give SketchUp users the ability to disable it through the user interface.

## Methods

- `initialize` — It is recommended to omit the file extension provided in the path argument. SketchUp will resolve the file extension to .rbe, .rbs or .rb.
- `check` — Loads the extension, meaning the underlying ruby script is immediately interpreted. This is the equivalent of checking the extension's checkbox in the Extension Manager.
- `copyright` — The copyright method returns the copyright string which appears beneath an extension inside the Extensions Manager dialog.
- `copyright=` — The copyright= method sets the copyright string which appears beneath an extension inside the Extensions Manager dialog.
- `creator` — The creator method returns the creator string which appears beneath an extension inside the Extensions Manager dialog.
- `creator=` — The creator= method sets the creator string which appears beneath an extension inside the Extensions Manager dialog.
- `description` — The description method returns the long description which appears beneath an extension inside the Extensions Manager dialog.
- `description=` — The description= method sets the long description which appears beneath an extension inside the Extensions Manager dialog.
- `extension_path` — The extension_path method returns the file system path to the extension's outer rb file.
- `id` — The id method returns the Extension Warehouse ID string.
- `load_on_start?` — Returns whether the extension is set to load when SketchUp starts up.
- `loaded?` — Returns whether the extension is currently loaded, meaning the actual ruby script that implements the extension has been evaluated.
- `name` — The name method returns the name which appears for an extension inside the Extensions Manager dialog.
- `name=` — The name= method sets the name which appears for an extension inside the Extensions Manager dialog.
- `registered?` — Returns whether the extension has been registered via Sketchup.register_extension.
- `uncheck` — Unloads the extension. This is the equivalent of unchecking the extension's checkbox in the Extension Manager > Extensions list.
- `version` — The version method returns the version which appears beneath an extension inside the Extensions Manager dialog.
- `version=` — The version method sets the version which appears beneath an extension inside the Extensions Manager dialog.
- `version_id` — The version_id method returns the Extension Warehouse Version ID string.
