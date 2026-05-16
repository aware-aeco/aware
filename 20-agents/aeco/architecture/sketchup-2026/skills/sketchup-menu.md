---
name: yard-sketchup-menu
description: Sketchup::Menu API reference (YARD)
---

# Sketchup::Menu API reference

An interface to a menu.

## Methods

- `add_item` — The #add_item method is used to add a menu item to the specified menu.
- `add_separator` — The #add_separator method is used to add a menu separator to a menu.
- `add_submenu` — The #add_submenu method is used to add a sub-menu to a menu.
- `set_validation_proc` — The #set_validation_proc method is used to specify the menu validation procedure. Your procedure should return either MF_ENABLED, MF_DISABLED, MF_CHECKED, MF_UNCHECKED, or MF_GRAYED.
