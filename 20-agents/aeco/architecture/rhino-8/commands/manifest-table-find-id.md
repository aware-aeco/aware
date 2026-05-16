# manifest-table-find-id

Lifecycle: single

Uses the guid to find a model component. Deleted objects cannot be found by id.             The guid is the value that is stored in the .Id property.             In a single document, no two active objects have the same guid. If an object is             replaced with a new object, then the guid  persists. For example, if the _Move command             moves an object, then the moved object inherits its guid from the starting object.             If the Copy command copies an object, then the copy gets a new guid. This guid persists             through file saving/opening operations. This function will not find grip objects.
