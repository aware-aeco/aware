# gh-relay-safe-delete

Lifecycle: single

Safely delete this relay and reconnect all wires.  This method does not trigger a new solution, nor does  it modify the document undo stack. Instead it returns a  list of all undo actions.
