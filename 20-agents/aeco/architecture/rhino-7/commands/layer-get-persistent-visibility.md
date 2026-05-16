# layer-get-persistent-visibility

Lifecycle: single

The persistent viability setting is used for layers whose visibility can             be changed by a "parent" object. A common case is when a layer is a             child layer (ParentId is not nil). In this case, when a parent layer is             turned off, then child layers are also turned off. The persistent             visibility setting determines what happens when the parent is turned on             again.
