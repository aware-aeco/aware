---
name: yard-sketchup-definition-list
description: Sketchup::DefinitionList API reference (YARD)
---

# Sketchup::DefinitionList API reference

A DefinitionList object holds a list of all of the ComponentDefinition objects in a model. This class contains methods for  adding and retrieving definitions from the list.

## Methods

- `[]` — The [] method is used to retrieve a component definition from the list. You can give an integer index in the range 0 to length, a string which represents the GUID (a unique internal identifier), or a string that is the name of the definition.
- `add` — The add method is used to add a new component definition to the definition list with the given name.
- `add_observer` — The add_observer method is used to add an observer to the current object.
- `[]` — The [] method is used to retrieve a component definition from the list. You can give an integer index in the range 0 to length, a string which represents the GUID (a unique internal identifier), or a string that is the name of the definition.
- `count` — Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module. Prior to that the #count method is an alias for #length.
- `each` — Don't remove content from this collection while iterating over it with #each. This would change the size of the collection and cause elements to be skipped as the indices change. Instead copy the current collection to an array using to_a and then use each on the array, when removing content.
- `import` — The #import method is used to import a (non-SketchUp) 3d model file as a definition.
- `length` — The #length method is used to retrieve number of component definitions in the list.
- `load` — The #load method is used to load a component from a file.
- `load_from_url` — The #load_from_url method loads a component from a location specified by string url.
- `purge_unused` — The purge_unused method is used to remove the unused component definitions.
- `remove` — The #remove method is used to remove a component definition from the definition list with the given component definition. This will remove all instances of the definition.
- `remove_observer` — The remove_observer method is used to remove an observer from the current object.
- `size` — The #size method is an alias for #length.
- `unique_name` — The unique_name is used to generate a unique name for a definition based on a base_name string. For example, a base_name of “Joe” might return “Joe #2”
