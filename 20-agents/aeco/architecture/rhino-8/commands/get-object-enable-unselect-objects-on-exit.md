# get-object-enable-unselect-objects-on-exit

Lifecycle: single

By default any objects in the object list are unselected when GetObject.GetObjects()             exits with any return code besides Object. If you want to leave the objects             selected when non-object input is returned, then call EnableUnselectObjectsOnExit(false)             before calling GetObjects().
