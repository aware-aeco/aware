---
name: yard-sketchup-component-definition
description: Sketchup::ComponentDefinition API reference (YARD)
---

# Sketchup::ComponentDefinition API reference

The ComponentDefinition class is used to define the contents for a SketchUp component. Components are a collection of entities that can be instanced and reused multiple times throughout a model. For example, you could draw a chair once, turn it into a component, and then use 6 instances of it to surround a table. Edits to the original “definition” will then propagate across all of its instances.

## Methods

- `<=>` — The <=> method is used to compare two ComponentDefinition objects for sorting. The comparison is done based on the component name.
- `==` — The == method is used to test if two ComponentDefinition objects are the same (based on their address in memory).
- `add_classification` — The add_classification method is used to add a given classification to the component.
- `add_observer` — The add_observer method is used to add an observer to the current object.
- `behavior` — The behavior method is used to retrieve the Behavior object associated with a component definition.
- `count_instances` — The count_instances method is used to count the number of unique component instances in a model using this component definition. This does not represent the total number of instances placed in the model as it doesn't take into account instances inside unused definitions.
- `count_used_instances` — The count_used_instances method is used to count the total number of component instances in a model using this component definition. This method takes into account the full hierarchy of the model.
- `description` — The description method is used to retrieve the description of the component definition.
- `description=` — The description= method is used to set the description for the component definition.
- `entities` — The entities method retrieves a collection of all the entities in the component definition
- `get_classification_value` — The get_classification_value method is used to retrieve the value from a classification attribute given a key path.
- `group?` — The group? method is used to determine if this component definition is used to hold the elements of a group.
- `guid` — The guid method is used to retrieve the unique identifier of this component definition. The guid changes after the component definition is modified and the component edit is exited.
- `hidden?` — The hidden method is used to determine if this component definition should be hidden on the component browser.
- `image?` — The image? method is used to determine if this component definition is used to define an image.
- `insertion_point` — SketchUp 2020.0 removed the insertion point feature. The getter will always return the origin point and the setter becomes a no-op.
- `insertion_point=` — SketchUp 2020.0 removed the insertion point feature. The getter will always return the origin point and the setter becomes a no-op.
- `instances` — The instances method is used to return any array of ComponentInstancesfor this ComponentDefinition.
- `internal?` — The internal? method is used to determine if the component definition is internal to the Component Browser
- `invalidate_bounds` — Invalidates the bounding box of your definition. This command forces the update of the bounding box of definition while inside an operation. See Model.start_operation for how to start an operation.
- `live_component?` — These components are parametrically generated and API users should not modify them.
- `load_time` — The #load_time method gets the load time of the component definition. For an internal component definition, this is the time that it was created. For an external component definition, this is the time that it was added to the model.
- `name` — The name method retrieves the name of the component definition.
- `name=` — The #name= method is used to set the name of the component definition.
- `path` — The path method is used to retrieve the path where the component was loaded.
- `refresh_thumbnail` — The refresh_thumbnail method is used to force SketchUp to regenerate the thumbnail image that appears in the component browser. This is useful if you've used the API to change the geometry of your component and would like the thumbnail to match.
- `remove_classification` — The remove_classification method is used to remove a given classification from the component.
- `remove_observer` — The remove_observer method is used to remove an observer from the current object.
- `save_as` — The #save_as method is used to save your definition as a SketchUp file at the specified file destination.
- `save_copy` — The #save_copy method is used to save your definition as a SketchUp file without changing the file path it is already associated with.
- `save_thumbnail` — Saves a component thumbnail image. The image format is specified by the file extension of filePath. Supported image formats are bmp, jpg, png, tif, pct, and gif.
- `set_classification_value` — The set_classification_value method is used to set the value of a classification attribute given a key path.
- `thumbnail_camera` — The #thumbnail_camera method is used to retrieve a camera representing the thumbnail associated with the component definition.
- `thumbnail_camera=` — The #thumbnail_camera= method is used to set the camera for the thumbnail associated with the component definition.
