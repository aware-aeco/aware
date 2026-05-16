# layout-entity-move-to-group

Lifecycle: single

The #move_to_group method moves the Layout::Entity into a Group. If the Layout::Entity is already in a Group, it will be removed from that Group prior to being added to the new one. If this action results in the old Group containing only one Layout::Entity, the old Group will be collapsed and the remaining Layout::Entity will be moved to the old Group's parent.
