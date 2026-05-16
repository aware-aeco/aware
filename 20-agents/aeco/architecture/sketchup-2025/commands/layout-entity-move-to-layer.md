# layout-entity-move-to-layer

Lifecycle: single

The #move_to_layer method moves the Layout::Entity to the given Layer. If the Layer is non-shared and the Layout::Entity is currently on a shared Layer, an array of Pages must be provided to move the Layout::Entity to. In all other cases, passing in an array of Pages is not necessary. The Layout::Entity must belong to the same Document as the the Layer and the Pages.
