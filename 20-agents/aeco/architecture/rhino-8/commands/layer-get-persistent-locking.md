# layer-get-persistent-locking

Lifecycle: single

The persistent locking setting is used for layers that can be locked by             a "parent" object. A common case is when a layer is a child layer             (Layer.ParentI is not nil). In this case, when a parent layer is locked,             then child layers are also locked. The persistent locking setting             determines what happens when the parent is unlocked again.
