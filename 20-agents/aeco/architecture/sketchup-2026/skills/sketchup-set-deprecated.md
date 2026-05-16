---
name: yard-sketchup-set-deprecated
description: Sketchup::Set      Deprecated API reference (YARD)
---

# Sketchup::Set      Deprecated API reference

In SketchUp 2014 this class was changed from Set to Sketchup::Set in order to avoid conflict with the Ruby Standard Library. The Sketchup::Set class is deprecated and new extensions should make use of Ruby's Set class unless they need backward compatibility.

## Methods

- `clear` — The clear method is used to clear all objects out of the set.
- `contains?` — The #contains? method is an alias for #include?.
- `delete` — The delete object is used to delete or remove an object from the set.
- `each` — The each method is used to iterate through all of the objects in the set.
- `empty?` — The empty? method is used to determine whether the set is empty.
- `include?` — The #include? method is used to determine if the set includes a particular object.
- `insert` — The insert method is used to insert an object into the set.
- `length` — The #length method is an alias for #size.
- `size` — The #size method is used to determine the number of objects in the set.
- `to_a` — The to_a method is used to get an Array of the entities in your Set.
