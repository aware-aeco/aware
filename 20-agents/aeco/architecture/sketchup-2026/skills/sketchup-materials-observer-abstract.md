---
name: yard-sketchup-materials-observer-abstract
description: Sketchup::MaterialsObserver   Abstract API reference (YARD)
---

# Sketchup::MaterialsObserver   Abstract API reference

To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the objects of interests.

## Methods

- `onMaterialAdd` — The #onMaterialAdd method is invoked whenever a new material is added.
- `onMaterialChange` — The #onMaterialChange method is invoked whenever a material's texture image is altered.
- `onMaterialRefChange` — The #onMaterialRefChange method is invoked whenever the number of entities that a material is painted on changes. This could be due to the user manually painting something, but it could also be when faces are split, pasted, push-pulled, deleted, etc.
- `onMaterialRemove` — The #onMaterialRemove method is invoked whenever a material is deleted.
- `onMaterialSetCurrent` — The #onMaterialSetCurrent method is invoked whenever a different material is selected in the Materials dialog.
- `onMaterialUndoRedo` — Due to a bug, this callback does not fire in SU6 or SU7. You can use the Sketchup::ModelObserver#onTransactionStart to capture all undo events.
