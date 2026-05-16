# sketchup-entities-build

Lifecycle: single

While using #build it is important to not add or remove vertices by other means than the builder. Also don't modify the position of the vertices in the Sketchup::Entities collection. Doing so can break the vertex-cache that de-duplicates the vertices.
