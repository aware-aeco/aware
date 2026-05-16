---
name: yard-u-i-command
description: UI::Command API reference (YARD)
---

# UI::Command API reference

The Command class is the preferred class for adding tools to the menus and Ruby toolbars. For example, you could add a menu item and pass it a code block directly, or you could first create a Command.

## Methods

- `new` — Prior to SketchUp 2019 it was not possible to sub-class UI::Command due to a bug in how SketchUp initialized the class.
- `extension` — This is an advanced feature that extension developers normally won't have to deal with. It's purpose is to address scenarios when SketchUp isn't able to automatically infer which extension the command belongs to.
- `extension=` — This is an advanced feature that extension developers normally won't have to deal with. It's purpose is to address scenarios when SketchUp isn't able to automatically infer which extension the command belongs to. These scenarios are for example an extension using a library to add its commands or command manager extensions.
- `get_validation_proc` — The #get_validation_proc method returns the command's validation proc.
- `large_icon` — The large_icon method returns the icon file for the command's large icon.
- `large_icon=` — The large_icon= method is used to identify the icon file for the command's large icon. large icons should be 32x32 pixel images for best display quality.
- `menu_text` — The menu_text method returns the menu item name for the command.
- `menu_text=` — The menu_text= method is used to set the menu item name for the command.
- `proc` — The #proc method returns the command's proc that is called when the command is invoked.
- `set_validation_proc` — Avoid disabling an command as it often isn't obvious to the user why it is disabled. Prefer keeping the command enabled but show an error message if pressed when it cannot be used.
- `small_icon` — The small_icon method returns the icon file for the command's small icon.
- `small_icon=` — The small_icon= method is used to identify the icon file for the command's small icon. Small icons should be 24x24 pixel images for best display quality.
- `status_bar_text` — The status_bar_text method returns the status bar text for the command.
- `status_bar_text=` — The status_bar_text= method is used to set the status bar text for the command. This should be a description what the command does.
- `tooltip` — The tooltip method returns command item's tooltip text.
- `tooltip=` — The tooltip text should repeat the commands' title text. For the command description, use #status_bar_text.
