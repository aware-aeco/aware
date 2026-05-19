---
name: sketchup-ui
description: This skill encodes the sketchup 2026.0 surface of the UI namespace — 5 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Command, Notification, HtmlDialog, Toolbar, WebDialog.
---

# UI

Auto-generated from vendor docs for sketchup 2026.0. 5 types in this namespace.

## Command (class)

The Command class is the preferred class for adding tools to the menus and Ruby toolbars. For example, you could add a menu item and pass it a code block directly, or you could first create a Command. Using Commands gives you greater control over how the item works in the UI, and it allows multiple spots in the UI to call the same code. For example, You might want a toolbar button and a context-click menu item to both point to the same command, and to control the tooltip and its “graying” from a single spot in your code.

[Vendor docs](https://ruby.sketchup.com/UI/Command.html)

### Constructors
- `Command(menutext) { ... }` — Note: Prior to SketchUp 2019 it was not possible to sub-class UI::Command due to a bug in how SketchUp initialized the class. The new method is used to create a new command.

### Methods
#### `get_validation_proc => Proc?`

The #get_validation_proc method returns the command's validation proc.

**Remarks:** The #get_validation_proc method returns the command's validation proc.

**Returns:** `Proc, nil` — 

[Docs](https://ruby.sketchup.com/UI/Command.html#get_validation_proc-instance_method)

#### `proc => Proc`

The #proc method returns the command's proc that is called when the command is invoked.

**Remarks:** The #proc method returns the command's proc that is called when the command is invoked.

**Returns:** `Proc` — 

[Docs](https://ruby.sketchup.com/UI/Command.html#proc-instance_method)

#### `set_validation_proc { ... } => UI::Command`

Note: Avoid disabling an command as it often isn't obvious to the user why it is disabled.

**Remarks:** Note: Avoid disabling an command as it often isn't obvious to the user why it is disabled. Prefer keeping the command enabled but show an error message if pressed when it cannot be used. The #set_validation_proc method allows you to change whether the command is enabled, checked, etc. For instance, the command toggling a dialog window may be displayed as checked while the dialog is open.

**Returns:** `UI::Command` — 

[Docs](https://ruby.sketchup.com/UI/Command.html#set_validation_proc-instance_method)

### Properties
- `extension` (SketchupExtension, nil, get/set) — Note: This is an advanced feature that extension developers normally won't have to deal with.
- `large_icon` (String, get/set) — The large_icon method returns the icon file for the command's large icon.
- `menu_text` (String, get/set) — The menu_text method returns the menu item name for the command.
- `small_icon` (String, get/set) — The small_icon method returns the icon file for the command's small icon.
- `status_bar_text` (String, get/set) — The status_bar_text method returns the status bar text for the command.
- `tooltip` (String, get/set) — The tooltip method returns command item's tooltip text.

## HtmlDialog (class)

Note: The window size is not guaranteed to be pixel perfect in all SketchUp versions and operating systems. Prefer responsive designs that can take up some fluctuations in size. The Ruby HtmlDialog class allows you to create and interact with HTML dialog boxes from Ruby. This is the best way to generate complex, embedded UIs inside SketchUp, but it does generally require HTML and JavaScript expertise. If your goal is to simple display a website to your users, consider using #openURL, which will show them a web page in their default browser rather than inside a dialog in SketchUp. The left, top, width, height etc. dimensions of the dialog are in logical units. This means you provide the units as if they where on a monitor with “normal” DPI. The units given will be multiplied by the same factor as returned by scale_factor. For usage examples, including how to migrate from the old WebDialog class, see github.com/SketchUp/htmldialog-examples. You may use the Trimble Modus framework for a look and feel of your dialog that matches that of SketchUp's dialogs. HtmlDialog uses the following versions of CEF (Chromium Embedded Framework): SketchUp 2025.0 CEF 128 SketchUp 2024.0 CEF 112 SketchUp 2021.1 CEF 88 SketchUp 2019.0 CEF 64 SketchUp 2018.0 CEF 56 SketchUp 2017.0 CEF 52 Starting with SketchUp 2026.1, HtmlDialog exposes a small set of built-in callbacks (mirroring the legacy skp: protocol used by WebDialog). sketchup.launchEW(id) – Opens the Extension Warehouse dialog and show a specified extension's page. sketchup.installRBZ(url) – Downloads and installs a Ruby extension from the given .rbz URL. (User will be prompted just like using the Extension Manager.)

[Vendor docs](https://ruby.sketchup.com/UI/HtmlDialog.html)

### Constructors
- `HtmlDialog(properties)` — Note: Prefix preference_key with something unique to your extension. Note: If there is no reference kept to the HtmlDialog object, the window will close once the garbage collection runs. This behavior can be confusing in trivial test code but is usually not a concern in real life scenarios. Typically a persistent reference, e.g. an instance variable, should be kept to bring the dialog to front, rather than creating a duplicate, if the user should request it a second time. The new method is used to create a new HtmlDialog. In SketchUp 2021.1 use_content_size was added. When set to true, width, height, min_width, max width, min_height, max_height will represent the size of the content area of the window. This excludes the titlebar and the window frame. When use_content_size is set to false (the default value), the size dimensions will represent the outer frame size. The properties hash accepts an optional key style where the value is one of: UI::HtmlDialog::STYLE_DIALOG HtmlDialog stays at the top of SketchUp. UI::HtmlDialog::STYLE_WINDOW HtmlDialog can go behind SketchUp and doesn't disappear when SketchUp looses focus. UI::HtmlDialog::STYLE_UTILITY HtmlDialog is shown with small titlebar and stays on top of SketchUp.

### Methods
#### `add_action_callback(callback_name) {|action_context, *args| ... } => Boolean`

Note: When an HtmlDialog is closed, all callbacks to that instance are cleared.

**Remarks:** Note: When an HtmlDialog is closed, all callbacks to that instance are cleared. Attach or re-attach them before you show the dialog. The #add_action_callback method establishes a Ruby callback method that your html dialog can call to perform some function. Use the sketchup.callback_method_name to invoke the callback method from your html dialog. Your JavaScript in the html dialog will invoke the callback with the same number of arguments. The call is asynchronous. JavaScript call might return before Ruby callback even called. Use onCompleted callback to get notified for completion. Basic types such as booleans, numbers, strings, arrays and hashes are automatically converted between Ruby and JavaScript.

**Parameters:**
- `callback_name` (String) — The name of the callback method to be invoked from the html dialog.

**Returns:** `Boolean` — true if action added successfully, false otherwise.

[Docs](https://ruby.sketchup.com/UI/HtmlDialog.html#add_action_callback-instance_method)

#### `bring_to_front => nil`

The #bring_to_front method is used to bring the window to the front, putting it on top of other windows even if its minimized.

**Remarks:** The #bring_to_front method is used to bring the window to the front, putting it on top of other windows even if its minimized.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/HtmlDialog.html#bring_to_front-instance_method)

#### `center => true`

The #center method is used to center the HtmlDialog relative to the active model window.

**Remarks:** The #center method is used to center the HtmlDialog relative to the active model window. If there is no active model window, this function doesn't do anything.

**Returns:** `true` — This always return true, never false.

[Docs](https://ruby.sketchup.com/UI/HtmlDialog.html#center-instance_method)

#### `close => nil`

The #close method is used to close a dialog box.

**Remarks:** The #close method is used to close a dialog box.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/HtmlDialog.html#close-instance_method)

#### `execute_script(script) => nil`

The #execute_script method is used to execute a JavaScript string on the html dialog asynchronously.

**Remarks:** The #execute_script method is used to execute a JavaScript string on the html dialog asynchronously.

**Parameters:**
- `script` (String) — The JavaScript script to execute on the HtmlDialog.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/HtmlDialog.html#execute_script-instance_method)

#### `get_content_size => Array(Integer, Integer)?`

The #get_content_size method is used to get the content size of the HtmlDialog, in logical pixels.

**Remarks:** The #get_content_size method is used to get the content size of the HtmlDialog, in logical pixels.

**Returns:** `Array(Integer, Integer), nil` — Content width and height of the HtmlDialog. A nil value is returned if the HtmlDialog is not visible.

[Docs](https://ruby.sketchup.com/UI/HtmlDialog.html#get_content_size-instance_method)

#### `get_position => Array(Integer, Integer)?`

The #get_position method is used to get the position of the HtmlDialog relative to the screen, in logical pixels.

**Remarks:** The #get_position method is used to get the position of the HtmlDialog relative to the screen, in logical pixels.

**Returns:** `Array(Integer, Integer), nil` — Left and top position of the dialog. A nil value is returned if the HtmlDialog is not visible.

[Docs](https://ruby.sketchup.com/UI/HtmlDialog.html#get_position-instance_method)

#### `get_size => Array(Integer, Integer)?`

The #get_size method is used to get the outer frame size of the HtmlDialog, in logical pixels.

**Remarks:** The #get_size method is used to get the outer frame size of the HtmlDialog, in logical pixels.

**Returns:** `Array(Integer, Integer), nil` — Outer frame width and height of the HtmlDialog. A nil value is returned if the HtmlDialog is not visible.

[Docs](https://ruby.sketchup.com/UI/HtmlDialog.html#get_size-instance_method)

#### `hide => nil`

Note: Modal dialogs cannot be hidden.

**Remarks:** Note: Modal dialogs cannot be hidden. Attempting to hide a modal dialog will raise an ArgumentError. The #hide method is used to hide a dialog box without closing it. This preserves the dialog's state and allows it to be shown again later without recreating the dialog.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/HtmlDialog.html#hide-instance_method)

#### `set_can_close => Boolean`

The #set_can_close method is used to attach a block that is executed just before closing, this block has to return a boolean, if the block returns false the close will be canceled.

**Remarks:** The #set_can_close method is used to attach a block that is executed just before closing, this block has to return a boolean, if the block returns false the close will be canceled.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/UI/HtmlDialog.html#set_can_close-instance_method)

#### `set_content_size(width, height) => nil`

The #set_content_size method is used to set the content size of the HtmlDialog, in logical pixels.

**Remarks:** The #set_content_size method is used to set the content size of the HtmlDialog, in logical pixels.

**Parameters:**
- `width` (Integer) — Content width of the HtmlDialog.
- `height` (Integer) — Content height of the HtmlDialog.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/HtmlDialog.html#set_content_size-instance_method)

#### `set_file(filename) => nil`

The #set_file method is used to identify a local HTML file to display in the HtmlDialog.

**Remarks:** The #set_file method is used to identify a local HTML file to display in the HtmlDialog.

**Parameters:**
- `filename` (String) — The filename for the HtmlDialog file (HTML file)

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/HtmlDialog.html#set_file-instance_method)

#### `set_html(html_string) => nil`

The #set_html method is used to load a HtmlDialog with a string of provided HTML.

**Remarks:** The #set_html method is used to load a HtmlDialog with a string of provided HTML.

**Parameters:**
- `html_string` (String) — A string of valid html to display in your HtmlDialog.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/HtmlDialog.html#set_html-instance_method)

#### `set_on_closed => Boolean`

Note: For saving state before window closes use #set_can_close instead.

**Remarks:** Note: For saving state before window closes use #set_can_close instead. The #set_on_closed method is used to attach a block that will be executed when a dialog is already in the process of closing, do any last minute operations within this block such as releasing resources or performing cleanup tasks.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/UI/HtmlDialog.html#set_on_closed-instance_method)

#### `set_position(left, top) => true`

The #set_position method is used to set the position of the HtmlDialog relative to the screen, in pixels.

**Remarks:** The #set_position method is used to set the position of the HtmlDialog relative to the screen, in pixels.

**Parameters:**
- `left` (Integer) — The number of pixels from the left.
- `top` (Integer) — The number of pixels from the top of the screen.

**Returns:** `true` — This always return true, never false.

[Docs](https://ruby.sketchup.com/UI/HtmlDialog.html#set_position-instance_method)

#### `set_size(width, height) => true`

The #set_size method is used to set the outer frame size of the HtmlDialog, in pixels.

**Remarks:** The #set_size method is used to set the outer frame size of the HtmlDialog, in pixels.

**Parameters:**
- `width` (Integer) — Outer frame width of the HtmlDialog.
- `height` (Integer) — Outer frame height of the HtmlDialog.

**Returns:** `true` — This always return true, never false.

[Docs](https://ruby.sketchup.com/UI/HtmlDialog.html#set_size-instance_method)

#### `set_url(url) => nil`

The #set_url method is used to load a HtmlDialog with the content at a specific URL.

**Remarks:** The #set_url method is used to load a HtmlDialog with the content at a specific URL. This method allows you to load web sites in a HtmlDialog.

**Parameters:**
- `url` (String) — The URL for a specific web site.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/HtmlDialog.html#set_url-instance_method)

#### `show => nil`

The #show method is used to display a non-modal dialog box.

**Remarks:** The #show method is used to display a non-modal dialog box.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/HtmlDialog.html#show-instance_method)

#### `show_modal => nil`

The #show_modal method is used to display a modal dialog box.

**Remarks:** The #show_modal method is used to display a modal dialog box.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/HtmlDialog.html#show_modal-instance_method)

#### `visible? => Boolean`

The #visible? method is useful to tell if the dialog is shown and still alive, if the dialog is minimized or not visible on the screen this will still return true.

**Remarks:** The #visible? method is useful to tell if the dialog is shown and still alive, if the dialog is minimized or not visible on the screen this will still return true.

**Returns:** `Boolean` — Returns true if the dialog is open.

[Docs](https://ruby.sketchup.com/UI/HtmlDialog.html#visible?-instance_method)

### Properties
- `STYLE_WINDOW` (Object, get) — 
- `STYLE_DIALOG` (Object, get) — 
- `STYLE_UTILITY` (Object, get) — 
- `CEF_VERSION` (Object, get) — 
- `CHROME_VERSION` (Object, get) — 

## Notification (class)

Notification objects allows you to show native notifications in the desktop. Notifications can have a message, icon and accept and/or dismiss buttons with callback blocks. Supported icon formats include: .bmp, .png, .jpg. Vector icons are supported as .svg on Windows and .pdf on Mac. Recommended icon size is 48x48 pixels. Icons larger than these sizes will be automatically downscaled to fit within the limits.

**Remarks:** Known Bugs: Prior to SketchUp 2026.0 oversized icons are cropped on Windows.

[Vendor docs](https://ruby.sketchup.com/UI/Notification.html)

### Constructors
- `Notification(sketchup_extension, message = nil, icon_name = nil, icon_tooltip = nil)` — Note: In order to insert line breaks into the message you need to use \r\n. From SketchUp 2019 and onwards \n also works on both Mac and Windows. Creates a new UI::Notification object.

### Methods
#### `on_accept(title, block) => Boolean`

Shows a button in the notification with the given title and callback block, both arguments are required.

**Remarks:** Shows a button in the notification with the given title and callback block, both arguments are required.

**Parameters:**
- `title` (String) — Sets the title of the button.
- `block` (Proc) — Sets the action callback, this will be called when the user clicks on the accept button.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/UI/Notification.html#on_accept-instance_method)

#### `on_accept_title => String`

Returns the accept's button title.

**Remarks:** Returns the accept's button title.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/UI/Notification.html#on_accept_title-instance_method)

#### `on_dismiss(title, block) => Boolean`

Shows a button in the notification with the given title and callback block.

**Remarks:** Shows a button in the notification with the given title and callback block. Both arguments are required. This callback is only called if you press the Dismiss button, not when the time runs out and the notification automatically disappears.

**Parameters:**
- `title` (String) — Sets the title of the button.
- `block` (Proc) — Sets the action callback, this will be called when the user clicks on the dismiss button.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/UI/Notification.html#on_dismiss-instance_method)

#### `on_dismiss_title => String`

Returns the dismiss's button title.

**Remarks:** Returns the dismiss's button title.

**Returns:** `String` — 

[Docs](https://ruby.sketchup.com/UI/Notification.html#on_dismiss_title-instance_method)

#### `show => Boolean`

Shows the notification.

**Remarks:** Shows the notification. If not interacted with, the notification will disappear without calling any callbacks.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/UI/Notification.html#show-instance_method)

### Properties
- `icon_name` (String, get/set) — Gets the icon name, this is the path that will be used to get the icon from the file system path.
- `icon_tooltip` (String, get/set) — Gets the icon Tooltip, this is the string that appear when the mouse is over the icon.
- `message` (String, get/set) — Gets the message as string.

## Toolbar (class)

The Toolbar class contains methods to create and manipulate SketchUp toolbars in Ruby. Toolbars are collections of buttons that you can use to activate custom Tools or ruby scripts. Also see the Command object for details on creating “commands” which can be called from your toolbars.

[Vendor docs](https://ruby.sketchup.com/UI/Toolbar.html)

### Methods
#### `add_item(command) => UI::Toolbar`

The add_item method is used to add an item to the toolbar.

**Remarks:** The add_item method is used to add an item to the toolbar.

**Parameters:**
- `command` (UI::Command) — A Command object representing the command to add to the toolbar.

**Returns:** `UI::Toolbar` — the toolbar where the command was just added

[Docs](https://ruby.sketchup.com/UI/Toolbar.html#add_item-instance_method)

#### `add_separator(arg) => UI::Toolbar`

The add_separator method is used to add a line separator to the toolbar.

**Remarks:** The add_separator method is used to add a line separator to the toolbar.

**Returns:** `UI::Toolbar` — the toolbar where the line separator was just added

[Docs](https://ruby.sketchup.com/UI/Toolbar.html#add_separator-instance_method)

#### `count => Integer`

The #count method is inherited from the Enumerable mix-in module.

**Remarks:** The #count method is inherited from the Enumerable mix-in module.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/UI/Toolbar.html#count-instance_method)

#### `each {|command| ... } => nil`

The #each method is used to iterate through all of the commands attached to a toolbar.

**Remarks:** The #each method is used to iterate through all of the commands attached to a toolbar.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/Toolbar.html#each-instance_method)

#### `get_last_state => Integer`

The get_last_state method is used to determine if the toolbar was hidden or visible in the previous session of SketchUp.

**Remarks:** The get_last_state method is used to determine if the toolbar was hidden or visible in the previous session of SketchUp. Valid states are TB_VISIBLE (1) for visible, TB_HIDDEN (0) for hidden, TB_NEVER_SHOWN (-1) for before never shown.

**Returns:** `Integer` — the last state of the toolbar (see constants above)

[Docs](https://ruby.sketchup.com/UI/Toolbar.html#get_last_state-instance_method)

#### `hide => nil`

The hide method is used to hide the toolbar on the user interface.

**Remarks:** The hide method is used to hide the toolbar on the user interface.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/Toolbar.html#hide-instance_method)

#### `length => Integer`

The #length method returns the number of items in the toolbar.

**Remarks:** The #length method returns the number of items in the toolbar.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/UI/Toolbar.html#length-instance_method)

#### `name => String`

The name method returns the name of the toolbar.

**Remarks:** The name method returns the name of the toolbar.

**Returns:** `String` — the name of the toolbar.

[Docs](https://ruby.sketchup.com/UI/Toolbar.html#name-instance_method)

#### `new(toolbarname) => UI::Toolbar`

The new method creates a new Toolbar object.

**Remarks:** The new method creates a new Toolbar object.

**Parameters:**
- `toolbarname` (String) — The name for the new toolbar.

**Returns:** `UI::Toolbar` — the newly created toolbar object

[Docs](https://ruby.sketchup.com/UI/Toolbar.html#new-class_method)

#### `restore => nil`

The restore method is used to reposition the toolbar to its previous location and show if not hidden.

**Remarks:** The restore method is used to reposition the toolbar to its previous location and show if not hidden.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/Toolbar.html#restore-instance_method)

#### `show => nil`

The show method is used to display the toolbar in the user interface.

**Remarks:** The show method is used to display the toolbar in the user interface.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/Toolbar.html#show-instance_method)

#### `size => Integer`

The #size method is an alias of #length.

**Remarks:** The #size method is an alias of #length.

**Returns:** `Integer` — 

[Docs](https://ruby.sketchup.com/UI/Toolbar.html#size-instance_method)

#### `visible? => Boolean`

The visible? method is used to find out if a toolbar is currently visible.

**Remarks:** The visible? method is used to find out if a toolbar is currently visible.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/UI/Toolbar.html#visible?-instance_method)

## WebDialog (class)

Deprecated. Please use HtmlDialog that was introduced in SketchUp 2017. The Ruby WebDialog class allows you to create and interact with DHTML dialog boxes from Ruby. If your goal is to simply display a website to your users, consider using openURL, which will show them a web page in their default browser rather than inside a dialog in SketchUp. See this blog post for a detailed, step-by-step example: sketchupapi.blogspot.com/2008/02/sharing-data-between-sketchup-ruby-and.html Under Windows the IE render mode is different in webdialogs than from what you see in the normal browser. It will by default pick an older render mode and different versions of SketchUp will use different modes. In order to reliably control the render mode of your webdialogs under Windows you need to insert a special META compatibility tag: Starting with SketchUp 2013, you can embed a special HTML link in your dialog that will launch Extension Warehouse and show a specified extension's page. This can be useful if your extension has a dependency on another one and you would like to direct the user to install that extension. For example, to launch an extension's page whose URL is: extensions.sketchup.com/en/content/advanced-camera-tools The link would be:

[Vendor docs](https://ruby.sketchup.com/UI/WebDialog.html)

### Constructors
- `WebDialog(dialog_title = "", scrollable = true, pref_key = nil, width = 250, height = 250, left = 0, top = 0, resizable = true)` — Note: Since SU2017 the position and size of the dialog is DPI aware - it will scale to the DPI of the monitor automatically. Specify units as they would be on a traditional low-DPI monitor. Note: The browser which is embedded inside the dialog depends on the user's OS. On Mac, Safari is embedded, while on the PC whatever version of Internet Explorer is installed will be embedded. The new method is used to create a new webdialog.

### Methods
#### `add_action_callback(callback_name) {|dialog, params| ... } => Object`

The add_action_callback method establishes a Ruby callback method that your web dialog can call to perform some function.

**Remarks:** The add_action_callback method establishes a Ruby callback method that your web dialog can call to perform some function. Use the skp:callback_method_name to invoke the callback method from your webdialog. Your JavaScript in the webdialog will invoke the callback method with a string representing arguments to the callback method. Note that you're sending data down to Ruby as a single string that's passed via the window.location bar. In Internet Explorer on PC, there is a length limit of 2038 characters for this bar, so if you're needing to pass large data down you might consider using get_element_value to pull in a longer string from a hidden input field in the HTML.

**Parameters:**
- `callback_name` (Object) — The name of the callback method to be invoked from the webdialog.

**Returns:** `Object` — nil

[Docs](https://ruby.sketchup.com/UI/WebDialog.html#add_action_callback-instance_method)

#### `allow_actions_from_host(hostname) => Boolean`

By default, actions are only allowed on the host where the webdialog is displayed.

**Remarks:** By default, actions are only allowed on the host where the webdialog is displayed. The allow_actions_from_host method is used to selectively allow actions to take place on a host remote from the host where the webdialog exists. If the webdialog is local, no remote host is allowed unless you use this method.

**Parameters:**
- `hostname` (String) — The name (domain) of the host that your webdialog can access safely.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/UI/WebDialog.html#allow_actions_from_host-instance_method)

#### `bring_to_front => UI::WebDialog`

The bring_to_front method is used to bring the webdialog to the front of all the windows on the desktop.

**Remarks:** The bring_to_front method is used to bring the webdialog to the front of all the windows on the desktop. See show_modal for how to ensure that your WedDialogs are on top.

**Returns:** `UI::WebDialog` — 

[Docs](https://ruby.sketchup.com/UI/WebDialog.html#bring_to_front-instance_method)

#### `close => nil`

The close method is used to close the webdialog.

**Remarks:** The close method is used to close the webdialog.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/WebDialog.html#close-instance_method)

#### `execute_script(script) => Boolean`

The execute_script method is used to execute a JavaScript string on the web dialog.

**Remarks:** The execute_script method is used to execute a JavaScript string on the web dialog.

**Parameters:**
- `script` (String) — The JavaScript script to execute on the webdialog.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/UI/WebDialog.html#execute_script-instance_method)

#### `get_default_dialog_color => String`

The get_default_dialog_color method is used to get the default dialog color for the web dialog.

**Remarks:** The get_default_dialog_color method is used to get the default dialog color for the web dialog.

**Returns:** `String` — a six digit hexidecimal number representing the color

[Docs](https://ruby.sketchup.com/UI/WebDialog.html#get_default_dialog_color-instance_method)

#### `get_element_value(element_id) => String`

The get_element_value method is used to get a value, with a given element_id, from the web dialog's DOM.

**Remarks:** The get_element_value method is used to get a value, with a given element_id, from the web dialog's DOM.

**Parameters:**
- `element_id` (String) — The name of the element in your HTML code.

**Returns:** `String` — a string containing the retrieved value.

[Docs](https://ruby.sketchup.com/UI/WebDialog.html#get_element_value-instance_method)

#### `navigation_buttons_enabled? => Boolean`

The navigation_buttons_enabled? method is used to get whether the home, next, and back buttons are visible at the top of the WebDialog on the mac.

**Remarks:** The navigation_buttons_enabled? method is used to get whether the home, next, and back buttons are visible at the top of the WebDialog on the mac. This method has no use on the PC, as these buttons are never displayed. On the mac, this defaults to true for new WebDialogs.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/UI/WebDialog.html#navigation_buttons_enabled?-instance_method)

#### `post_url(url, data) => nil`

The post_url method is used to send the data to a url using the HTTP POST method.

**Remarks:** The post_url method is used to send the data to a url using the HTTP POST method.

**Parameters:**
- `url` (String) — The url to send the data.
- `data` (String) — The data to be sent.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/WebDialog.html#post_url-instance_method)

#### `screen_scale_factor => Float`

The screen_scale_factor method returns the ratio of screen pixels to logical window units (called 'points' on Mac) for the screen this WebDialog is currently in.

**Remarks:** The screen_scale_factor method returns the ratio of screen pixels to logical window units (called 'points' on Mac) for the screen this WebDialog is currently in. On a retina screen Mac, this ratio will be greater than 1.0. On Windows this always return 1.0.

**Returns:** `Float` — screen scale factor

[Docs](https://ruby.sketchup.com/UI/WebDialog.html#screen_scale_factor-instance_method)

#### `set_background_color(color) => nil`

The set_background_color method is used to set the background color for the webdialog.

**Remarks:** The set_background_color method is used to set the background color for the webdialog.

**Parameters:**
- `color` (String) — A six digit hexidecimal color.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/WebDialog.html#set_background_color-instance_method)

#### `set_file(filename, path = nil) => nil`

The #set_file method is used to identify a local HTML file to display in the webdialog.

**Remarks:** The #set_file method is used to identify a local HTML file to display in the webdialog.

**Parameters:**
- `filename` (String) — The filename for the webdialog file (HTML file).
- `path` (String) — A path that filename is relative to.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/WebDialog.html#set_file-instance_method)

#### `set_full_security => UI::WebDialog`

The set_full_security method is used to place the WebDialog into a higher security mode where remote URLs and plugins (such as Flash) are not allowed inside the browser.

**Remarks:** The set_full_security method is used to place the WebDialog into a higher security mode where remote URLs and plugins (such as Flash) are not allowed inside the browser. This defaults to false when a new WebDialog is created.

**Returns:** `UI::WebDialog` — the updated WebDialog

[Docs](https://ruby.sketchup.com/UI/WebDialog.html#set_full_security-instance_method)

#### `set_html(html_string) => nil`

The set_html method is used to load a webdialog with a string of provided HTML.

**Remarks:** The set_html method is used to load a webdialog with a string of provided HTML.

**Parameters:**
- `html_string` (String) — A string of valid html to display in your webdialog.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/WebDialog.html#set_html-instance_method)

#### `set_on_close { ... } => nil`

Note: #close method should not be called from the #set_on_close callback.

**Remarks:** Note: #close method should not be called from the #set_on_close callback. That would make it trigger itself recursively. The set_on_close method is used to establish one or more activities to perform when the dialog closes (such as saving values stored in the dialog).

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/WebDialog.html#set_on_close-instance_method)

#### `set_position(left, top) => nil`

Note: As of SU2017 this will automatically scale the x and y by the same factor as UI.

**Remarks:** Note: As of SU2017 this will automatically scale the x and y by the same factor as UI.scale_factor. The set_position method is used to set the position of the webdialog relative to the screen, in pixels.

**Parameters:**
- `left` (Integer) — The number of pixels from the left.
- `top` (Integer) — The number of pixels from the top of the screen.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/WebDialog.html#set_position-instance_method)

#### `set_size(w, h) => nil`

Note: As of SU2017 this will automatically scale the width and height by the same factor as UI.

**Remarks:** Note: As of SU2017 this will automatically scale the width and height by the same factor as UI.scale_factor. The set_size method is used to set the size of the webdialog, in pixels.

**Parameters:**
- `w` (Integer) — Width of the webdialog.
- `h` (Integer) — Height of the webdialog.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/WebDialog.html#set_size-instance_method)

#### `set_url(url) => nil`

The set_url method is used to load a webdialog with the content at a specific URL.

**Remarks:** The set_url method is used to load a webdialog with the content at a specific URL. This method allows you to load web sites in a webdialog.

**Parameters:**
- `url` (String) — The URL for a specific web site.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/WebDialog.html#set_url-instance_method)

#### `show {|dialog| ... } => nil`

The show method is used to display a non-modal dialog box.

**Remarks:** The show method is used to display a non-modal dialog box.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/WebDialog.html#show-instance_method)

#### `show_modal {|dialog| ... } => nil`

The show_modal method is used to display a modal dialog box.

**Remarks:** The show_modal method is used to display a modal dialog box. In SketchUp 6 and 7, this behaves differently on Mac vs. PC. On the PC, it shows a truly modal dialog, meaning so long as the WebDialog is visible, no input can be performed elsewhere inside SketchUp. On the Mac, “modal” WebDialogs do not behave this way, but instead are “always on top” of other windows.

**Returns:** `nil` — 

[Docs](https://ruby.sketchup.com/UI/WebDialog.html#show_modal-instance_method)

#### `visible? => Boolean`

The visible? method is used to tell if the webdialog is currently shown.

**Remarks:** The visible? method is used to tell if the webdialog is currently shown.

**Returns:** `Boolean` — 

[Docs](https://ruby.sketchup.com/UI/WebDialog.html#visible?-instance_method)

#### `write_image(image_path, option, top_left_x, top_left_y, bottom_right_x, bottom_right_y) => Object`

The write_image method is used to grab a portion of the web dialog screen and save the image to the given file path.

**Remarks:** The write_image method is used to grab a portion of the web dialog screen and save the image to the given file path.

**Parameters:**
- `image_path` (String) — The destination path of the saved image.
- `option` (Integer) — Specifies what method to use when saving the image. For JPG/JPEG images, this specifies the image quality and can range from 1 to 100. For PNG images this specifies the compression algorithm: <4 (best speed), 4-8 (default), or >=9 (best compression).
- `top_left_x` (Integer) — The x coordinate of the upper left corner of the region to grab.
- `top_left_y` (Integer) — The x coordinate of the upper left corner of the region to grab.
- `bottom_right_x` (Integer) — The x coordinate of the lower right corner of the region to grab.
- `bottom_right_y` (Integer) — The x coordinate of the lower right corner of the region to grab.

[Docs](https://ruby.sketchup.com/UI/WebDialog.html#write_image-instance_method)

### Properties
- `max_height` (Integer, get/set) — The max_height method is used to get the maximum height that the user is allowed to resize the dialog to.
- `max_width` (Integer, get/set) — The max_width method is used to get the maximum width that the user is allowed to resize the dialog to.
- `min_height` (Integer, get/set) — The min_width method is used to get the minimum height that the user is allowed to resize the dialog to.
- `min_width` (Integer, get/set) — The min_width method is used to get the minimum width that the user is allowed to resize the dialog to.
- `navigation_buttons_enabled` (Object, set) — The navigation_buttons_enabled= method is used to set whether the home, next, and back buttons are visible at the top of the WebDialog on the mac.

