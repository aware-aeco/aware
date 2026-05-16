---
name: yard-sketchup-component-instance
description: Sketchup::ComponentInstance API reference (YARD)
---

# Sketchup::ComponentInstance API reference

The ComponentInstance class is used to represent component instances of a component definition or components that have been dragged from the Component Browser and placed (thus, instanced) within the Model. Therefore, the ComponentInstance class contains a reference to a corresponding ComponentDefinition object and a Transformation object (which contains the location of the component in the Drawing Window).

## Methods

- `add_observer` — The add_observer method is used to add an observer to the current object.
- `definition` — The definition method is used to retrieve the component definition for this component instance.
- `definition=` — The definition= method is used to set the component definition for this component.
- `equals?` — The equals? method is used to determine if a component instance is geometrically equivalent to another instance.
- `explode` — The explode method is used to explode the component instance into separate entities.
- `glued_to` — The #glued_to method is used to retrieve the entity that this instance is glued to.
- `glued_to=` — The #glued_to= method glues this instance to a drawing element. When moving this other drawing elment with the Move tool, the glued instance moves with it.
- `guid` — The guid method is used to get the base 64 encoded unique id for this SketchUp object.
- `intersect` — This method is not available in SketchUp Make.
- `locked=` — The locked= method is used to lock a component instance.
- `locked?` — The locked? method is used to determine if a component instance is locked.
- `make_unique` — The #make_unique method is used to create a component definition for this instance that is not used by any other instances. If the component is already unique in the model, nothing happens.
- `manifold?` — The manifold? method is used to determine if an instance is manifold.
- `move!` — Despite the name being similar to #transform!, this method closer corresponds to #transformation=.
- `name` — The name method is used to get the name of this instance.
- `name=` — The name method is used to set the name of this instance.
- `outer_shell` — The outer_shell method is used to compute the outer shell of the two instances representing manifold solid volumes (this || arg). If the specified objects (this and arg) do not represent manifold volumes, this method fails.
- `remove_observer` — The remove_observer method is used to remove an observer from the current object.
- `show_differences` — The show_differences method is used to determine if a component instance is geometrically equivalent to another instance and in addition move the non- matching and matching geometry to new layers.
- `split` — This method is not available in SketchUp Make.
- `subtract` — This method is not available in SketchUp Make.
- `transform!` — Apply a Geom::Transformation to a component instance.
- `transformation` — The transformation method is used to retrieve the transformation of this instance.
- `transformation=` — The #transformation= method is used to set the transformation of this component instance.
- `trim` — Trimming object instance2 using instance1 results in a new trimmed version of instance2. If the trim is successful the original instance2 is erased and a newly trimmed version is created. This new version, derived from the trimming operation, will possess a new GUID and will be returned.
- `union` — This method is not available in SketchUp Make.
- `volume` — The volume method is used to compute the volume of this instance if and only if this instance is manifold.
