---
name: yard-sketchup-model
description: Sketchup::Model API reference (YARD)
---

# Sketchup::Model API reference

This is the interface to a SketchUp model. The model is the 3D drawing that the user is working with, and it serves as the “entry point” for most Ruby API interactions. The Sketchup.active_model method gives you a handle to the current model, and from there you can use the model-level methods to start getting information and making changes.

## Methods

- `abort_operation` — Never abort a transparent operation. Doing so would abort the operation it chains to. Instead, try to clean up and simply commit in order to make sure the operation is closed.
- `active_entities` — Returns an Entities object which contains the entities in the open group or component instance. If no group or component is open for editing then this will be the same as #entities.
- `active_layer` — The #active_layer method retrieves the active Layer.
- `active_layer=` — The #active_layer= method sets the active Layer object.
- `active_path` — Returns an array containing the sequence of entities the user has double-clicked on for editing. This allows one to determine whether they are in component edit mode and where in the model they are.
- `active_path=` — An instance path can only be opened if the instances are not locked. This also include instances of the same component definition that are not on the given path. A definition cannot be edited if any of its instances are locked.
- `active_view` — The #active_view method returns the active View object for this model.
- `add_note` — Add a text note to the Model.  The position of the note is given as relative window positions between 0 and 1.  For example, the following command would create a note that start 1/10 of the ways down the screen from the upper left corner of the window.
- `add_observer` — The add_observer method is used to add an observer to the current object.
- `attribute_dictionaries` — The #attribute_dictionaries method retrieves the AttributeDictionaries object that is associated with the Model.
- `attribute_dictionary` — Returns the Sketchup::AttributeDictionary object that is specified by name.  If the model does not have an attribute dictionary that corresponds to name, returns either nil, or a creates an attribute dictionary.  If the optional second argument is true, and there is no attribute dictionary that corresponds to name, a new attribute dictionary is created.
- `axes` — The #axes method returns the drawing axes for the model.
- `behavior` — The behavior method retrieves the behavior of the model.
- `bounds` — The #bounds method is used to retrieve the Geom::BoundingBox bounding the contents of a Sketchup::Model.
- `classifications` — The #classifications method is used to retrieve the Classifications object for this model.
- `close` — As of SketchUp 2024.0 this method will ensure the next model window gets focus if there is one. Before that `Sketchup.active_model` might return `nil` after calling this method even though more models where open.
- `close_active` — Before SketchUp 2014 this method had a bug where it didn't create an undo operation and that could lead to corrupted geometry when undo/redo was used after invoking this method.
- `commit_operation` — The commit_operation method commits an operation for undo.
- `definitions` — The #definitions method retrieves a definition list containing all of the component definitions in the model.
- `description` — The description method retrieves a description of the model as found in the Model Info > Files panel.
- `description=` — The #description= method sets the description of the model.
- `drawing_element_visible?` — The #drawing_element_visible? method reports whether the given drawing element in an instance path is visible given the current model options.
- `edit_transform` — Returns the transformation of the current component edit session. If a user has double-clicked to edit a component's geometry, this will return the transformation of that component, relative to its parent's origin. This allows one to correctly calculate “local” transformations of a given entity regardless of whether the user is in edit mode.
- `entities` — This does not return a collection of all the entities in the model, only the top level node of the model hierarchy. To get to all entities in a model you must recursivly traverse the model.
- `environments` — The #environments method is used to retrieve the Environments object for this model.
- `export` — The export method is used to export a given file format. It knows which format to export based on the file extension you place on the file name. For example, a filename of “thing.obj” will export an OBJ file, whereas “thing.dae” will export a COLLADA file.
- `find_entity_by_id` — Finds and returns entities by their entityID or GUID.
- `find_entity_by_persistent_id` — Finds and returns entities by their persistent id.
- `georeferenced?` — This methods determines if the model is georeferenced.
- `get_attribute` — The get_attribute method gets the value of an attribute that in the AttributeDictionary with the given name. If no value is associated with key, or if the model does not have an attribute dictionary specified by name, the optional third parameter will be returned.
- `get_datum` — the get_datum method retrieves the datum, in the form of a string, used in UTM conversions.
- `get_product_family` — Returns a value which indicates the product family of the installed SketchUp application.
- `guid` — The guid method retrieves the globally unique identifier, in the form of a string, for the Model. The guid will change after the model is modified and saved. The Model guid is stored with the SketchUp file; it will not change if the file is moved to another computer.
- `import` — The import method is used to load a file by recognizing the file extension and calling appropriate importer.
- `instance_path_from_pid_path` — The #instance_path_from_pid_path method returns a instance path given a string with persistent ids representing the path to the entity.
- `latlong_to_point` — The latlong_to_point method converts a latitude and longitude to a Point3d object in the model. It does not actually work with a LatLong object, but operates on a 2-element array. The returned point will always be on the ground (z=0).
- `layers` — The #layers method retrieves a collection of all Layers objects in the model.
- `line_styles` — The #line_styles method returns the line styles manager.
- `list_datums` — This method retrieves an Array of all of the datums recognized by SketchUp.
- `materials` — The #materials method returns a collection of all of the materials in the model.
- `mipmapping=` — This method can be used to turn mipmapping on or off.
- `mipmapping?` — This method can be used to find out if mipmapping is on or off.
- `modified?` — The modified? method determines if the Model has been modified since the last save.
- `name` — The #name method retrieves the name of the model.
- `name=` — The #name= method sets the string name of the model.
- `number_faces` — Returns the number faces in a model.
- `options` — The #options method retrieves the options manager that defines the options settings for the model.
- `overlays` — 
- `pages` — The #pages method retrieves a Pages object containing all of the pages in the model.
- `path` — The path method retrieves the path of the file from which the model was opened.
- `place_component` — The place_component method places a new component in the Model using the component placement tool.
- `point_to_latlong` — The point_to_latlong method converts a point in the model to a LatLong so that you can get its latitude and longitude.
- `point_to_utm` — This method converts a Point3d object in the Model to UTM coordinates.
- `raytest` — The #raytest method is used to cast a ray (line) through the model and return the first thing that the ray hits.
- `remove_observer` — The remove_observer method is used to remove an observer from the current object.
- `rendering_options` — The #rendering_options method retrieves the RenderingOptions object for this model.
- `save` — A bug in SketchUp 2016 and older caused the .skb backup file written during save to be empty. The .skp file was however valid.
- `save_copy` — This method is used to save the copy of the current model to a file.
- `save_thumbnail` — The save_thumbnail method is used to save a thumbnail image to a file. The image format is specified by the file extension of filename.  Supported formats are bmp, jpg, png, tif, pct, and gif.
- `select_tool` — If you call select_tool from within the initialize method of a custom tool, the call will be ignored.
- `selection` — This method retrieves a Selection object for the model, containing the currently selected entities. The entries in the selection list are not necessarily in the same order in which the user selected them.
- `set_attribute` — This method is used to set the value of an attribute in an attribute dictionary with the given name.
- `set_datum` — This method sets the datum used in conversions between the internal coordinate system and UTM.
- `shadow_info` — This method is used to retrieve the shadow information for the Model.
- `start_operation` — Operations in SketchUp are sequential and cannot be nested. If you start a new Ruby operation while another is still open, you will implicitly close the first one.
- `styles` — The #styles method retrieves the styles associated with the model.
- `tags` — The tags method retrieves the string tags of the model.
- `tags=` — The tags= method sets the string tags of the model.
- `title` — The #title method retrieves the name of the model. If the model is saved on disk, returns the file name without extension. Otherwise returns an empty string.
- `tools` — The #tools method is used to retrieve the current Tools object.
- `utm_to_point` — The utm_to_point method converts a position given in UTM coordinates to a Point3d in the Model.
- `valid?` — Determine if a model is a valid Sketchup::Model object. Returns false if the model has been closed.
