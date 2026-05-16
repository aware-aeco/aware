---
name: yard-sketchup
description: Sketchup API reference (YARD)
---

# Sketchup API reference

The Sketchup module contains a number of important utility methods for use in your Ruby scripts. Many of the classes in the API are implemented beneath this module. You can think of the Sketchup module as the “root” of the application tree. Most ruby calls start from the currently active model, and this is accessed via the Sketchup.active_model method.

## Methods

- `active_model` — The active_model method returns the currently active SketchUp model. On the PC, this is the only model that one can have access to via the API, but Macintosh versions of SketchUp can have multiple models open at once, in which case the method will return the model that the user currently has focused.
- `add_observer` — The add_observer method is used to add an observer to the current object.
- `app_name` — The app_name method is used to retrieve the current application name.
- `break_edges=` — The break_edges= method can be used to disable or enable the break edges feature. Break edges is the SketchUp 7 feature that automatically splits edges that the user draws which cross over one another.
- `break_edges?` — The break_edges? method indicates whether the break edges feature is currently turned on. Break edges is the SketchUp 7 feature that automatically splits edges that the user draws which cross over one another. This feature is always on by default and cannot be disabled by the user via the user interface.
- `create_texture_writer` — The create_texture_writer method is used to create a TextureWriter object.
- `debug_mode=` — Changing this value within your extension can cause problems for other extension developers who rely on the debug information for their own work. Only use this locally; never change this value in an extension you publish.
- `debug_mode?` — The debug_mode? controls whether SketchUp will output warnings to the console when it detects incorrect usage of the API.
- `display_name_from_action` — This method has been non-functional on Mac since SketchUp 8.
- `extensions` — Returns the ExtensionsManager where you can find all registered SketchupExtension objects.
- `file_new` — The file_new method is used to create a new file.
- `find_support_file` — The find_support_files method is used to retrieve the relative path and name of a file within the SketchUp installation directory.
- `find_support_files` — The find_support_files method is used to retrieve the path and name of all matching files within the SketchUp installation directory.
- `fix_shadow_strings=` — The fix_shadow_strings= method lets you control whether shadow rendering attempts to fix an artifact commonly referred to as “strings”.  The fix is actually very model dependent and not controllable from the UI, so this method can be used to control it.
- `fix_shadow_strings?` — The fix_shadow_strings? method indicates whether the a fix for a shadow rendering artifact commonly referred to as “strings” is enabled.  The fix is actually very model dependent and not controllable from the UI, so this method can be used to test it.
- `focus` — The focus method is used to focus the active model window.
- `format_angle` — The format_angle method takes a number as an angle in radians and formats it into degrees. For example, format_angle(Math::PI) will return 180.0.
- `format_area` — The format_area method formats a number as an area using the current units settings.
- `format_degrees` — The format_degrees method formats a number as an angle given in degrees. For example, 10 becomes 10.0. This is the equivalent to a to_f call.
- `format_length` — The format_length method formats a number as a length using the current units settings.
- `format_volume` — The format_volume method formats a number as a volume using the current units settings.
- `get_datfile_info` — The get_datfile_info method is used to retrieve the value for the given key from Sketchup.dat.
- `get_i18n_datfile_info` — The get_i18n_datfile_info method is used to retrieve the value for the given key from the internationalization file that SketchUp uses to work in multiple languages.
- `get_locale` — The os_language method returns the language code for the language SketchUp is running in. This is an alias for the get_locale method.
- `get_resource_path` — The get_resource_path is used to retrieve the directory where “resource” files are stored by SketchUp. Resource files include things like language localization files.
- `get_shortcuts` — The get_shortcuts method retrieves an array of all keyboard shortcuts currently registered with SketchUp. Each shortcut is returned as a string with the shortcut and the command separated by a tab, similar to “Ctrl+A\tEdit/Select All”
- `install_from_archive` — Installs the contents of a ZIP archive file into SketchUp's Plugins folder. If the ZIP contains subfolders, these will be preserved. This allows for a Ruby API plugin or Extension developer to distribute their plugin as a single file regardless of how many asset files must be included.
- `is_64bit?` — This methods indicates whether the host SketchUp application is 64bit. Useful for extensions that ship with binaries and need to determine which versions to load.
- `is_online` — The is_online method is used to verify a connection to the Internet. This method can take some time to execute, so be careful not to call it more often than you need.
- `is_pro?` — In SketchUp Make this method will return true during the Pro trial period and revert to false when the trial period is over.
- `is_valid_filename?` — The is_valid_filename? method is used to determine whether a filename contains illegal characters.
- `load` — The load method is used to load Ruby files. Unlike Ruby's own load method it also supports SketchUp's encrypted .rbe files.
- `open_file` — The open_file method is used to open a SketchUp model.
- `os_language` — The os_language method returns the language code for the language SketchUp is running in. This is an alias for the get_locale method.
- `parse_length` — The parse_length method parses a string as a length.
- `platform` — This methods returns a symbol indicating the current platform.
- `plugins_disabled=` — The plugins_disabled= method lets you control whether SketchUp will load Ruby scripts from the plugins directory at startup time. This is primarily a trouble-shooting method. If you are having strange behavior in SketchUp that you suspect is from a bad script, you can type Sketchup.plugins_disabled=true into the Ruby console and restart SketchUp to see if the problem is fixed.
- `plugins_disabled?` — The plugins_disabled? method indicates whether Ruby scripts in the plugins directory will be loaded at startup time.
- `quit` — The quit method is used to terminate the application. This will pop-up the usual model save prompts if there are unsaved models open. User can cancel the model save, in which case the application will not terminate.
- `read_default` — Be aware that the method is not capable of handling Length objects. You can convert the value to a Float before writing and convert back to Length when reading the value. Don't store the value as a String as this rounds the value and formats it in a way that can't be read if the system setting for decimal separator changes.
- `redo` — The redo method is used redo the last transaction on the redo stack.
- `register_extension` — It is recommended to set load_on_start to true unless you have a very good reason not to.
- `register_importer` — The register_importer method is used to register an importer with SketchUp.
- `remove_observer` — The remove_observer method is used to remove an observer from the current object.
- `require` — The require method is used to load Ruby files once. Unlike Ruby's own require method it also supports SketchUp's encrypted .rbe files.
- `resize_viewport` — In SketchUp 2024.0 and later this method doesn't behave correctly in all cases on Windows. The passed values are internally converted to logical pixels, rounded and converted back to physical pixels. This means certain sizes, such as 1000 px at 150% scaling, cannot be accurately set.
- `save_thumbnail` — The save_thumbnail method is used to generate a thumbnail for any SKP file - not necessarily the loaded model.
- `send_action` — The send_action method sends a message to the message queue to perform some action asynchronously.
- `send_to_layout` — The send_to_layout method is used to open a file in LayOut.
- `set_status_text` — The set_status_text method is used to set the text appearing on the status bar within the drawing window.
- `status_text=` — The status_text= method is used to set the text appearing on the status bar within the drawing window.
- `temp_dir` — The temp_dir method is used to retrieve the OS temporary directory for the current user. You can use this directory to write temporary files that are not required to persist between SketchUp sessions.
- `template` — The template method is used to get the file name of the current template. Templates are the .skp files that are loaded when the user select File > New.
- `template=` — The template= method is used to set the file name of the current template. Templates are the .skp files that are loaded when the user select File > New.
- `template_dir` — The template_dir is used to retrieve the directory where templates are stored by the SketchUp install. Templates are the .skp files that are loaded when the user select File > New.
- `undo` — The undo method is used undo the last transaction on the undo stack.
- `vcb_label=` — The vcb_label= method is used to set the label that appears on the vcb, or the “value control box”, which is another word for the “measurements” text entry box that appears at the bottom on the SketchUp window.
- `vcb_value=` — The vcb_value= method is used to set the value that appears on the vcb, or the “value control box”, which is another word for the “measurements” text entry box that appears at the bottom on the SketchUp window.
- `version` — Gets the current version of sketchup in decimal form.
- `version_number` — Get the current version of sketchup as a whole number for comparisons. The number returned has the major, minor, and build values packed into an integer value as follows:
- `write_default` — Be aware that the method is not capable of handling Length objects. You can convert the value to a Float before writing and convert back to Length when reading the value. Don't store the value as a String as this rounds the value and formats it in a way that can't be read if the system setting for decimal separator changes.
