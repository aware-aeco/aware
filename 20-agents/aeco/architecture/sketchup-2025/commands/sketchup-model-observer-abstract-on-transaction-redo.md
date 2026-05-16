# sketchup-model-observer-abstract-on-transaction-redo

Lifecycle: single

The #onTransactionRedo method is invoked when the user “redoes” a transaction (aka undo operation.) You can programmatically fire a redo by calling Sketchup.sendAction(“editRedo”).
