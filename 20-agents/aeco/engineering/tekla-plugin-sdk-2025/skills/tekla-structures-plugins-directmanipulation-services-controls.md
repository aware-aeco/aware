---
name: tekla-plugin-sdk-tekla-structures-plugins-directmanipulation-services-controls
description: This skill encodes the tekla-plugin-sdk 2025.0 surface of the Tekla.Structures.Plugins.DirectManipulation.Services.Controls namespace — 7 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ButtonControl, CheckBoxControl, DropDownListControl, LabelControl, RadioButtonControl, TextBoxControl, ValueBoxControl.
---

# Tekla.Structures.Plugins.DirectManipulation.Services.Controls

Auto-generated from vendor docs for tekla-plugin-sdk 2025.0. 7 types in this namespace.

## ButtonControl (class)

The ButtonControl class represents a control that can contain text and images, and can be clicked using the mouse or the keyboard.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Controls.ButtonControl)

### Events
#### `Clicked` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Clicked`

Event to be called when the button is clicked.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Controls.ButtonControl.Clicked)

## CheckBoxControl (class)

The CheckBoxControl class represents a check box.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Controls.CheckBoxControl)

### Properties
- `IsChecked` (bool, get) — Gets a value indicating whether this control is currently in a checked state.

### Events
#### `StateChanged` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> StateChanged`

Event to be called when the state of the control has changed.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Controls.CheckBoxControl.StateChanged)

## DropDownListControl (class)

The DropDownListControl class represents a control that allows the user to select a single item from a list that is displayed when the user clicks the control.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Controls.DropDownListControl)

### Methods
#### `void AddItem(object item)`

Adds an item to the list of items to choose from.

**Parameters:**
- `item` (object) — The item.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#M%3ATekla.Structures.Plugins.DirectManipulation.Services.Controls.DropDownListControl.AddItem%28System.Object%29)

### Properties
- `Items` (System.Collections.Generic.IList<object>, get) — Gets the items.
- `SelectedItem` (object, get/set) — Gets or sets the selected item.

### Events
#### `StateChanged` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> StateChanged`

Event to be called when the state of the control has changed.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Controls.DropDownListControl.StateChanged)

## LabelControl (class)

The LabelControl class represents a control that can be used to display label content.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Controls.LabelControl)

## RadioButtonControl (class)

The RadioButtonControl class represents a radio button.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Controls.RadioButtonControl)

### Properties
- `Group` (string, get/set) — Gets or sets the group.
- `IsChecked` (bool, get/set) — Gets or sets the checked value.

### Events
#### `StateChanged` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> StateChanged`

Event to be called when the state of the control has changed.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Controls.RadioButtonControl.StateChanged)

## TextBoxControl (class)

The TextBoxControl class represents a control that can be used to display or edit unformatted text.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Controls.TextBoxControl)

### Properties
- `Title` (string, set) — Sets the title of the control.

### Events
#### `GotKeyboardFocus` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> GotKeyboardFocus`

Event to be taken when the control has gotten keyboard focus.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Controls.TextBoxControl.GotKeyboardFocus)

#### `LostKeyboardFocus` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> LostKeyboardFocus`

Event to be taken when the control has lost keyboard focus.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Controls.TextBoxControl.LostKeyboardFocus)

#### `StateChanged` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> StateChanged`

Event to be called when the state of the control has changed.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Controls.TextBoxControl.StateChanged)

## ValueBoxControl (class)

The ValueBoxControl class represents a control that can be used to display or edit distances and angles.

[Vendor docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#T:Tekla.Structures.Plugins.DirectManipulation.Services.Controls.ValueBoxControl)

### Properties
- `IsInErrorState` (bool, get) — Gets the value for indicating whether the control is in an error state.
- `Text` (string, get/set) — Text property of ValueBoxControl.
- `Title` (string, set) — Sets the title of the control.
- `Value` (double?, get/set) — Gets or sets the value of the item in the value box.

### Events
#### `GotKeyboardFocus` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> GotKeyboardFocus`

Event to be called when the control has gotten keyboard focus.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Controls.ValueBoxControl.GotKeyboardFocus)

#### `LostKeyboardFocus` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> LostKeyboardFocus`

Event to be called when the control has lost keyboard focus.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Controls.ValueBoxControl.LostKeyboardFocus)

#### `StateChanged` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> StateChanged`

Event to be called when the state of the control has changed.

[Docs](https://developer.tekla.com/documentation/direct-manipulation-api-plugins#E%3ATekla.Structures.Plugins.DirectManipulation.Services.Controls.ValueBoxControl.StateChanged)

