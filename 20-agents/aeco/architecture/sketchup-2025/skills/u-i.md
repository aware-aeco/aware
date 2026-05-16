---
name: yard-u-i
description: UI API reference (YARD)
---

# UI API reference

The UI module contains a number of methods for creating simple UI elements from a SketchUp Ruby script.

## Methods

- `add_context_menu_handler` — The add_context_menu_handler method is used to register a block of code with SketchUp that will be called when a context menu is to be displayed. The context menu handler can then display the context menu with the items that you have added.
- `beep` — The beep method plays a system beep sound.
- `create_cursor` — Since SketchUp 2016 it is possible to provide vector images for the cursors. SVG format for Windows and PDF format for macOS. This is the recommended format to use since it will scale well with different DPI scaling.
- `get_clipboard_data` — Returns the plain text available on the clipboard, if there is any.
- `inputbox` — The method intelligently handles various types for default values and lists, automatically attempting to convert provided inputs to match the type of the default value. This ensures consistency in data types throughout the operation.
- `inspector_names` — The inspector_names method is used to returns the names of all the inspectors. Inspectors are another name for the various floating dialog windows that you can activate from withing SketchUp, such as the Materials window.
- `menu` — The 'Extensions' menu was named 'Plugins' prior to SketchUp 2015. For backward compatibility 'Plugins' still works.
- `messagebox` — Creates a dialog box containing static text with a series of buttons for the user to choose from.
- `model_info_pages` — The model_info_pages method is used to returns the names of all the available model info pages. These include UI windows such as Components, Credits, and Units.
- `openpanel` — The openpanel method is used to display the Open dialog box. The path that is returned can then be used inside code to open a text or image file. See the standard Ruby class File for examples of reading and writing from disk.
- `openURL` — The openURL method is used to open the default browser to a URL.
- `play_sound` — The play_sound method is used to play a sound file. Valid sound files include .wav and .mp3 files on the Mac and .wav files on the PC.
- `preferences_pages` — The preferences_pages method is used to returns the names of all the preferences pages. These include windows like Templates.
- `refresh_inspectors` — Tells SketchUp to refresh all inspectors such as the Component Browser and the Outliner. This is useful when you need to manually force a refresh after you've made a change to the document via Ruby. Generally, SketchUp will keep these in sync for you, but occasionally it does not, such as when model.start_operation has disabled UI updates.
- `refresh_toolbars` — Tells SketchUp to refresh all floating toolbars. This is useful when you need to manually force a refresh after you've made a change to the document via Ruby. Generally, SketchUp will keep these in sync for you, but occasionally it does not, such as when Sketchup::Model#start_operation has disabled UI updates. This only affects macOS, on Windows the toolbars are always refreshing.
- `savepanel` — The savepanel method is used to display the Save dialog box. The path that is returned can then be used inside code to save out a text or image file. See the standard Ruby class File for examples of reading and writing from disk.
- `scale_factor` — SU2017M0 will automatically scale up line width and text size, but will not scale up the points provided to Sketchup::View#draw2d.
- `select_directory` — The select_directory method is used to display the OS dialog for selecting one or several directories from the file system.
- `set_clipboard_data` — Copies plain text to the clipboard.
- `set_cursor` — The set_cursor method is used to change the cursor to a new cursor with a given cursor id. See UI.create_cursor and the Tool class for details on creating your own tools with arbitrary cursors.
- `set_toolbar_visible` — The set_toolbar_visible method is used to set whether a given toolbar is visible.  Note that the toolbars and their names are different on the Mac vs. PC, so be careful and be sure to test when using this method in a cross-platform script.
- `show_extension_manager` — The show_extension_manager method is used to display the Extension Manager dialog.
- `show_inspector` — The show_inspector method is used to display the inspector with the given name. You can get the list of valid inspectors with UI.inspector_names.
- `show_model_info` — The show_model_info method is used to display the model info dialog for a specific page. You can get the list of valid page names with model_info_pages.
- `show_preferences` — The show_preferences method is used to display a SketchUp preferences dialog. You can get the list of valid dialogs with preferences_pages.
- `start_timer` — The start_timer method is used to start a timer. This is an effective method to create a repeating snippet of code for arbitrary animation.
- `stop_timer` — The stop_timer method is used to stop a timer based on its id.
- `toolbar` — The toolbar method is used to get a Ruby toolbar by name. If the toolbar doesn't exist a new one will be created.
- `toolbar_names` — The toolbar_names method is used to return the name of all the available native toolbars (this differs between PC and Mac). These toolbar names do not include Ruby toolbars.
- `toolbar_visible?` — The toolbar_visible? method is used to determine whether a given toolbar is visible. Note that the toolbars and their names are different on the Mac vs. PC, so be careful and be sure to test when using this method in a cross-platform script.
