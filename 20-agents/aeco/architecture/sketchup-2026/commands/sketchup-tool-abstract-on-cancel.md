# sketchup-tool-abstract-on-cancel

Lifecycle: single

When something is undone #onCancel is called before the undo is actually executed. If you need to do something with the model after an undo use ModelObserver#onTransactionUndo.
