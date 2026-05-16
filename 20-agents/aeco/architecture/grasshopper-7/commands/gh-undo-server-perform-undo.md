# gh-undo-server-perform-undo

Lifecycle: single

Performs a single Undo step when possible and migrates the record onto the redo stack.   This function may throw all kinds of exceptions, if you're calling it from a UI thread,   use a Try..Catch block to prevent crashes.
