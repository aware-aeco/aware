# layout-entities-reverse-each

Lifecycle: single

The #reverse_each method iterates through all of the Layout::Entitys in reverse order. When iterating over a LayerInstance's Layout::Entities, it is not necessary to provide a flags Hash. When iterating over a Page's Layout::Entities, the flags Hash is optional; providing no Hash will result in iterating over all Layout::Entitys, including those on hidden or locked Layers. Valid symbols for the Hash are :skip_hidden and :skip_locked.
