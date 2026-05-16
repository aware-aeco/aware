---
name: yard-sketchup-group
description: Sketchup::Group API reference (YARD)
---

# Sketchup::Group API reference

A Group class contains methods for manipulating groups of entities.

## Methods

- `add_observer` — The add_observer method is used to add a ComponentInstance observer to the group.
- `copy` — The copy method is used to create a new Group object that is a copy of the group.
- `definition` — The definition method is used to retrieve the component definition for this group.
- `description` — The description method is used to retrieve the description for the group.
- `description=` — The description= method is used to set the description for the group.
- `entities` — The entities method is used to retrieve a collection of entities in the group.
- `equals?` — The equals? method is used to determine if a group is geometrically equivalent to another group.
- `explode` — The explode method is used to explode the group into individual entities.
- `glued_to` — The #glued_to method is used to retrieve the entity that this group is glued to.
- `glued_to=` — The #glued_to= method glues this group to a drawing element. When moving this other drawing elment with the Move tool, the glued group moves with it.
- `guid` — The guid method is used to get the base 64 encoded unique id for this SketchUp object.
- `intersect` — This method is not available in SketchUp Make.
- `local_bounds` — In favor of `group.definition.bounds`.
- `locked=` — The locked= method is used to lock a group.
- `locked?` — The locked? method is used to determine if a group is locked.
- `make_unique` — The #make_unique method is used to force a group to have a unique definition. If the group is already unique in the model, nothing happens.
- `manifold?` — The manifold? method is used to determine if a group is manifold.
- `move!` — Despite the name being similar to #transform!, this method closer corresponds to #transformation=.
- `name` — The name method is used to retrieve the name of the group.
- `name=` — The #name= method is used to set the name for the group.
- `outer_shell` — The outer_shell method is used to compute the outer shell of the two groups representing manifold solid volumes (this || arg).  If the specified objects (this and arg) do not represent manifold volumes, this method fails.
- `remove_observer` — The remove_observer method is used to remove a ComponentInstance observer from the group.
- `show_differences` — The show_differences method is used to determine if a group is geometrically equivalent to another group and in addition move the non- matching and matching geometry to new layers.
- `split` — This method is not available in SketchUp Make.
- `subtract` — This method is not available in SketchUp Make.
- `to_component` — The to_component method is used to convert the group to a component instance.
- `transform!` — The transform! method is used to apply a transformation to a group.
- `transformation` — The transformation method is used to retrieve the transformation for the group.
- `transformation=` — The #transformation= method is used to set the transformation of this group
- `trim` — Trimming object group2 using group1 results in a new trimmed version of group2. If the trim is successful the original group2 is erased and a newly trimmed version is created. This new version, derived from the trimming operation, will possess a new GUID and will be returned.
- `union` — This method is not available in SketchUp Make.
- `volume` — The volume method is used to compute the volume of this group if and only if this group is manifold.
