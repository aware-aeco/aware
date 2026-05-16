# layout-document-add-entity

Lifecycle: single

The #add_entity method adds an Entity to the Layout::Document and places it on the given Layer and Page. If layer is a shared Layer then page may be ommitted. The Entity must not already belong to a Layout::Document. If the Entity is a Group, then the Group along with all of its children will be added to the Layout::Document.
