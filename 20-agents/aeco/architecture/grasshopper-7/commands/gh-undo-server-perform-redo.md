# gh-undo-server-perform-redo

Lifecycle: single

Performs a single Redo step if possible and migrates the record onto the undo stack.   This function may throw all kinds of exceptions, if you're calling it from a UI thread,   use a Try..Catch block to prevent crashes.
