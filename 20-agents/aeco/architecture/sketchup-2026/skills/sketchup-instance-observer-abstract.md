---
name: yard-sketchup-instance-observer-abstract
description: Sketchup::InstanceObserver   Abstract API reference (YARD)
---

# Sketchup::InstanceObserver   Abstract API reference

To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the objects of interests.

## Methods

- `onClose` — The #onClose method is called when an instance is “closed,” meaning an end user was editing a component's geometry and then exited back into the parent's editing space.
- `onOpen` — The #onOpen method is called when an instance is “opened,” meaning an end user has double clicked on it to edit its geometry. This is particularly useful if your plugin is dynamically drawing geometry or performing transformations in global space, since when in “edit component” mode all transformations and positions are returned in relation to the current component's origin.
