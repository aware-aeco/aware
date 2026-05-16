---
name: yard-u-i-html-dialog
description: UI::HtmlDialog API reference (YARD)
---

# UI::HtmlDialog API reference

The window size is not guaranteed to be pixel perfect in all SketchUp versions and operating systems. Prefer responsive designs that can take up some fluctuations in size.

## Methods

- `initialize` — Prefix preference_key with something unique to your extension.
- `add_action_callback` — When an HtmlDialog is closed, all callbacks to that instance are cleared. Attach or re-attach them before you show the dialog.
- `bring_to_front` — The #bring_to_front method is used to bring the window to the front, putting it on top of other windows even if its minimized.
- `center` — The #center method is used to center the HtmlDialog relative to the active model window. If there is no active model window, this function doesn't do anything.
- `close` — The #close method is used to close a dialog box.
- `execute_script` — The #execute_script method is used to execute a JavaScript string on the html dialog asynchronously.
- `get_content_size` — The #get_content_size method is used to get the content size of the HtmlDialog, in logical pixels.
- `get_position` — The #get_position method is used to get the position of the HtmlDialog relative to the screen, in logical pixels.
- `get_size` — The #get_size method is used to get the outer frame size of the HtmlDialog, in logical pixels.
- `set_can_close` — The #set_can_close method is used to attach a block that is executed just before closing, this block has to return a boolean, if the block returns false the close will be canceled.
- `set_content_size` — The #set_content_size method is used to set the content size of the HtmlDialog, in logical pixels.
- `set_file` — The #set_file method is used to identify a local HTML file to display in the HtmlDialog.
- `set_html` — The #set_html method is used to load a HtmlDialog with a string of provided HTML.
- `set_on_closed` — For saving state before window closes use #set_can_close instead.
- `set_position` — The #set_position method is used to set the position of the HtmlDialog relative to the screen, in pixels.
- `set_size` — The #set_size method is used to set the outer frame size of the HtmlDialog, in pixels.
- `set_url` — The #set_url method is used to load a HtmlDialog with the content at a specific URL. This method allows you to load web sites in a HtmlDialog.
- `show` — The #show method is used to display a non-modal dialog box.
- `show_modal` — The #show_modal method is used to display a modal dialog box.
- `visible?` — The #visible? method is useful to tell if the dialog is shown and still alive, if the dialog is minimized or not visible on the screen this will still return true.
