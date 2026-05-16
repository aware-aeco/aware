---
name: yard-top-level-namespace
description: Top Level Namespace API reference (YARD)
---

# Top Level Namespace API reference

Call this function at the end of a file that you are loading to let the system know that you have loaded it.

## Methods

- `add_separator_to_menu` — 
- `file_loaded` — Call this function at the end of a file that you are loading to let the system know that you have loaded it.
- `file_loaded?` — Use in combination with #file_loaded to create load guards for code you don't want to reload. Especially useful to protect your UI setup from creating duplicate menus and toolbars.
- `inputbox` — This is a wrapper for UI.inputbox.  You call it exactly the same as UI.inputbox.  UI.inputbox will raise an exception if it can't convert the string entered for one of the values into the right type. This method will trap the exception and display an error dialog and then prompt for the values again.
- `require_all` — This adds the path given to $LOAD_PATH which can affect other extensions.
- `show_ruby_panel` — Use SKETCHUP_CONSOLE.show instead.
