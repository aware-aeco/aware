# model-commit-changes

Lifecycle: single

Commits the changes made to the model database so far. One commit is something that a             user can later on undo with the undo command. A commit also launches the drawing of the             changed product model to the visible views. A dependent plug-in should never call CommitChanges,              since this would make undo very difficult for the user to do. However, non-dependent plug-ins              require a separate commit if new objects are created.
