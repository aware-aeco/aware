---
name: yard-sketchup-instance-path
description: Sketchup::InstancePath API reference (YARD)
---

# Sketchup::InstancePath API reference

The InstancePath class represent the instance path to a given entity within the model hierarchy.

## Methods

- `initialize` — 
- `==` — Returns `true` if the instances paths represent the same set of entities.
- `[]` — This method does not accept negative indices. For the exact behavior of an array, use {#to_a}.
- `each` — The yielded entities will start with the root and end with the leaf.
- `empty?` — 
- `include?` — Returns `true` if the instance path contain the given object.
- `leaf` — The leaf of an instance path is the last element which can be any entity that can be represented in the model. This is normally a Drawingelement, but could be a Vertex.
- `length` — #length is an alias of #size.
- `persistent_id_path` — The serialized version of an instance path is the persistent ids of its entities concatenated with a period.
- `root` — The root of an instance path is the element located closest to the model root. This will be a group or component instance. If you have a non-instance as a leaf with no other parent component this will return `nil`.
- `size` — 
- `to_a` — Returns an array representing the instance path.
- `transformation` — Returns the combined transformation up to the the leaf entity.
- `valid?` — An instance path is valid if it has at least one element and consist of groups and instances with exception of the leaf which can be any entity.
