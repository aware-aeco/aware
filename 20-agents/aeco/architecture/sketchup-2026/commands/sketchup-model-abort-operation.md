# sketchup-model-abort-operation

Lifecycle: single

Never abort a transparent operation. Doing so would abort the operation it chains to. Instead, try to clean up and simply commit in order to make sure the operation is closed.
