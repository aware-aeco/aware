# get-object-enable-clear-objects-on-entry

Lifecycle: single

By default the picked object list is cleared when GetObject.GetObjects() is called.             If you are reusing a GetObject class and do not want the existing object list             cleared when you call Input, then call EnableClearObjectsOnEntry(false) before             calling GetObjects().
