# sketchup-materials-observer-abstract-on-material-undo-redo

Lifecycle: single

Due to a bug, this callback does not fire in SU6 or SU7. You can use the Sketchup::ModelObserver#onTransactionStart to capture all undo events.
