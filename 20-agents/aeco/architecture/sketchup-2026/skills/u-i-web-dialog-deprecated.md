---
name: yard-u-i-web-dialog-deprecated
description: UI::WebDialog      Deprecated API reference (YARD)
---

# UI::WebDialog      Deprecated API reference

Please use HtmlDialog that was introduced in SketchUp 2017.

## Methods

- `initialize` — Since SU2017 the position and size of the dialog is DPI aware - it will scale to the DPI of the monitor automatically. Specify units as they would be on a traditional low-DPI monitor.
- `add_action_callback` — The add_action_callback method establishes a Ruby callback method that your web dialog can call to perform some function.
- `allow_actions_from_host` — By default, actions are only allowed on the host where the webdialog is displayed. The allow_actions_from_host method is used to selectively allow actions to take place on a host remote from the host where the webdialog exists. If the webdialog is local, no remote host is allowed unless you use this method.
- `bring_to_front` — The bring_to_front method is used to bring the webdialog to the front of all the windows on the desktop. See show_modal for how to ensure that your WedDialogs are on top.
- `close` — The close method is used to close the webdialog.
- `execute_script` — The execute_script method is used to execute a JavaScript string on the web dialog.
- `get_default_dialog_color` — The get_default_dialog_color method is used to get the default dialog color for the web dialog.
- `get_element_value` — The get_element_value method is used to get a value, with a given element_id, from the web dialog's DOM.
- `max_height` — The max_height method is used to get the maximum height that the user is allowed to resize the dialog to.
- `max_height=` — As of SU2017 this will automatically scale the height by the same factor as UI.scale_factor.
- `max_width` — The max_width method is used to get the maximum width that the user is allowed to resize the dialog to.
- `max_width=` — As of SU2017 this will automatically scale the width by the same factor as UI.scale_factor.
- `min_height` — The min_width method is used to get the minimum height that the user is allowed to resize the dialog to.
- `min_height=` — As of SU2017 this will automatically scale the height by the same factor as UI.scale_factor.
- `min_width` — The min_width method is used to get the minimum width that the user is allowed to resize the dialog to.
- `min_width=` — As of SU2017 this will automatically scale the width by the same factor as UI.scale_factor.
- `navigation_buttons_enabled=` — The navigation_buttons_enabled= method is used to set whether the home, next, and back buttons are visible at the top of the WebDialog on the mac. This method has no use on the PC, as these buttons are never displayed.
- `navigation_buttons_enabled?` — The navigation_buttons_enabled? method is used to get whether the home, next, and back buttons are visible at the top of the WebDialog on the mac. This method has no use on the PC, as these buttons are never displayed.
- `post_url` — The post_url method is used to send the data to a url using the HTTP POST method.
- `screen_scale_factor` — The screen_scale_factor method returns the ratio of screen pixels to logical window units (called 'points' on Mac) for the screen this WebDialog is currently in. On a retina screen Mac, this ratio will be greater than 1.0. On Windows this always return 1.0.
- `set_background_color` — The set_background_color method is used to set the background color for the webdialog.
- `set_file` — The #set_file method is used to identify a local HTML file to display in the webdialog.
- `set_full_security` — The set_full_security method is used to place the WebDialog into a higher security mode where remote URLs and plugins (such as Flash) are not allowed inside the browser. This defaults to false when a new WebDialog is created.
- `set_html` — The set_html method is used to load a webdialog with a string of provided HTML.
- `set_on_close` — #close method should not be called from the #set_on_close callback. That would make it trigger itself recursively.
- `set_position` — As of SU2017 this will automatically scale the x and y by the same factor as UI.scale_factor.
- `set_size` — As of SU2017 this will automatically scale the width and height by the same factor as UI.scale_factor.
- `set_url` — The set_url method is used to load a webdialog with the content at a specific URL. This method allows you to load web sites in a webdialog.
- `show` — The show method is used to display a non-modal dialog box.
- `show_modal` — The show_modal method is used to display a modal dialog box. In SketchUp 6 and 7, this behaves differently on Mac vs. PC. On the PC, it shows a truly modal dialog, meaning so long as the WebDialog is visible, no input can be performed elsewhere inside SketchUp. On the Mac, “modal” WebDialogs do not behave this way, but instead are “always on top” of other windows.
- `visible?` — The visible? method is used to tell if the webdialog is currently shown.
- `write_image` — The write_image method is used to grab a portion of the web dialog screen and save the image to the given file path.
