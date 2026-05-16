---
name: yard-sketchup-model-observer-abstract
description: Sketchup::ModelObserver   Abstract API reference (YARD)
---

# Sketchup::ModelObserver   Abstract API reference

To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the model.

## Methods

- `onActivePathChanged` — The #onActivePathChanged method is invoked when the user opens or closes a ComponentInstance or Group for editing.
- `onAfterComponentSaveAs` — The #onAfterComponentSaveAs method is invoked when the user context-clicks > Save As on a component instance. It is called just after the component is written to disk, so you can restore the component to some state before returning control to the user.
- `onBeforeComponentSaveAs` — The #onBeforeComponentSaveAs method is invoked when the user context-clicks > Save As on a component instance. It is called just before the component is written to disk, so you can make changes within the handler and it will make it into the save.
- `onDeleteModel` — The #onDeleteModel method is invoked when a model is deleted.
- `onEraseAll` — The #onEraseAll method is invoked when everything in a model is erased.
- `onExplode` — The method is invoked whenever a component anywhere in this model is exploded. This is an easier way to watch explode events vs. attaching an InstanceObserver to every instance in the model.
- `onPidChanged` — This callback is useful for tracking changes to entities that result in new PIDs, such as grouping or other modifications that result in new entities.
- `onPlaceComponent` — The #onPlaceComponent method is invoked when a component is “placed” into the model, meaning it is dragged from the Component Browser.
- `onPostSaveModel` — The #onPostSaveModel method is invoked after a model has been saved to disk.
- `onPreSaveModel` — The #onPreSaveModel method is invoked before a model is saved to disk.
- `onSaveModel` — The #onSaveModel method is invoked after a model has been saved to disk.
- `onTransactionAbort` — The #onTransactionAbort method is invoked when a transaction is aborted.
- `onTransactionCommit` — The #onTransactionCommit method is invoked when a transaction is completed.
- `onTransactionEmpty` — The #onTransactionEmpty method is invoked when a transaction (aka an undoable operation) starts and then is committed without anything being altered in between.
- `onTransactionRedo` — The #onTransactionRedo method is invoked when the user “redoes” a transaction (aka undo operation.) You can programmatically fire a redo by calling Sketchup.sendAction(“editRedo”).
- `onTransactionStart` — The #onTransactionStart method is invoked when a transaction (aka an undoable operation) starts.
- `onTransactionUndo` — The method is invoked when the user “undoes” a transaction (aka undo operation.) You can programmatically fire an undo by calling Sketchup.sendAction(“editUndo”).
