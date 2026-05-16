---
name: yard-u-i-toolbar
description: UI::Toolbar API reference (YARD)
---

# UI::Toolbar API reference

The Toolbar class contains methods to create and manipulate SketchUp toolbars in Ruby. Toolbars are collections of buttons that you can use to activate custom Tools or ruby scripts.

## Methods

- `new` — The new method creates a new Toolbar object.
- `add_item` — The add_item method is used to add an item to the toolbar.
- `add_separator` — The add_separator method is used to add a line separator to the toolbar.
- `count` — The #count method is inherited from the Enumerable mix-in module.
- `each` — The #each method is used to iterate through all of the commands attached to a toolbar.
- `get_last_state` — The get_last_state method is used to determine if the toolbar was hidden or visible in the previous session of SketchUp.
- `hide` — The hide method is used to hide the toolbar on the user interface.
- `length` — The #length method returns the number of items in the toolbar.
- `name` — The name method returns the name of the toolbar.
- `restore` — The restore method is used to reposition the toolbar to its previous location and show if not hidden.
- `show` — The show method is used to display the toolbar in the user interface.
- `size` — The #size method is an alias of #length.
- `visible?` — The visible? method is used to find out if a toolbar is currently visible.
