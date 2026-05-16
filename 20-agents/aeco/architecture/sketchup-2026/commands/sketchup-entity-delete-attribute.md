# sketchup-entity-delete-attribute

Lifecycle: single

In SketchUp 2018, special attribute dictionaries have been added. The name of these dictionaries are “SU_InstanceSet” and “SU_DefinitionSet”. The dictionaries cannot be deleted via ruby and an ArgumentError will be raised. The key/value pairs in the dictionary can be deleted safely.
