---
name: yard-sketchup-options-provider
description: Sketchup::OptionsProvider API reference (YARD)
---

# Sketchup::OptionsProvider API reference

An OptionsProvider class provides various kinds of options on a Model. You get an OptionsProvider from the OptionsManager. The options are given as name/value pairs.

## Methods

- `[]` — The [] method is used to get a value by name or index of the key.
- `[]=` — The []= method is used to set the value of a specific key.
- `add_observer` — The add_observer method is used to add an observer to the current object.
- `count` — Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module. Prior to that the #count method is an alias for #length.
- `each` — The #each method is used to iterate through all of the options.
- `each_key` — The #each_key method is used to iterate through all of the attribute keys.
- `each_pair` — The #each method is used to iterate through all of the options.
- `each_value` — The #each_value method is used to iterate through all of the attribute values.
- `has_key?` — The #has_key? method is an alias for #key?.
- `key?` — The #key? method is used to determine if the options provider has a specific key.
- `keys` — The #keys method is used to retrieve an array with all of the attribute keys.
- `length` — The #length method is an alias of #size.
- `name` — The name method is used to retrieve the name of an options provider.
- `remove_observer` — The remove_observer method is used to remove an observer from the current object.
- `size` — The #size method is used to retrieve the size (number of elements) of an options provider.
