---
name: yard-layout-document
description: Layout::Document API reference (YARD)
---

# Layout::Document API reference

This is the interface to a LayOut document. A Document is the 2D drawing that the user is working with, and it serves as the “entry point” for most Ruby API interactions. The Document.open method gives you a handle to a Document, and from there you can use the document-level methods to start getting information and making changes.

## Methods

- `initialize` — The #initialize method creates a new Layout::Document. Passing a path to an existing Layout::Document will use that file as a template. The new Layout::Document won't have a path until it is saved for the first time.
- `open` — The open method creates a new Layout::Document by loading an existing .layout file.
- `==` — The #== method checks to see if the two Layout::Documents are equal. This checks whether the Ruby Objects are pointing to the same internal object.
- `add_entity` — The #add_entity method adds an Entity to the Layout::Document and places it on the given Layer and Page. If layer is a shared Layer then page may be ommitted. The Entity must not already belong to a Layout::Document. If the Entity is a Group, then the Group along with all of its children will be added to the Layout::Document.
- `auto_text_definitions` — The #auto_text_definitions method returns an array of AutoTextDefinition's in the Layout::Document.
- `export` — The #export method exports the Layout::Document to a given file format. It knows which format to export based on the file extension you place on the file name. For example, a filename of “thing.pdf” will export a PDF file, whereas “thing.png” will export a set of PNG images.
- `grid` — The #grid method returns the Grid for a Layout::Document.
- `grid_snap_enabled=` — The #grid_snap_enabled= method sets whether or not grid snap is enabled in the Layout::Document.
- `grid_snap_enabled?` — The #grid_snap_enabled? method returns whether or not grid snap is enabled in the Layout::Document.
- `layers` — The #layers method returns the Layers of the Layout::Document.
- `object_snap_enabled=` — The #object_snap_enabled= method enables or disables inference in the Layout::Document.
- `object_snap_enabled?` — The #object_snap_enabled? method returns whether or not inference is enabled in the Layout::Document.
- `page_info` — The #page_info method returns a reference to the PageInfo settings of the Layout::Document.
- `pages` — The #pages method returns the Pages of the Layout::Document.
- `path` — The #path method returns the full path of the Layout::Document file. An empty string is returned for a new Layout::Document (one which has not been saved and opened).
- `precision` — The #precision method returns the precision for the Layout::Document.
- `precision=` — LayOut only allows for a finite set of precision values for each units setting, so it will set the precision to the closest valid setting for the specified units. See the “Units” section of LayOut's “Document Setup” dialog for a reference of the available precisions for each units setting.
- `remove_entity` — The #remove_entity method removes an Entity from the Layout::Document. If entity is a Group, then the Group and all of its children will be removed from the Layout::Document.
- `render_mode_override` — The #render_mode_override method returns the override setting for output render modes of SketchUpModels in the Layout::Document.
- `render_mode_override=` — The #render_mode_override= method sets the override setting for output render modes of SketchUpModels in the Layout::Document. Setting this to NO_OVERRIDE will prevent overriding the individual SketchUpModel render mode setting during export. This override will only affect raster rendered SketchUpModels, if a viewport is set to vector or hybrid, it will retain that render mode as its output render mode.
- `save` — The #save method saves the Layout::Document to a file at the given path. Passing an empty path string will save the Layout::Document at its current path.
- `shared_entities` — The #shared_entities method returns the Entities that exist on shared Layers in the Layout::Document.
- `time_created` — The #time_created method returns the time when the Layout::Document was created.
- `time_modified` — The #time_modified method returns the last time the Layout::Document was modified.
- `time_published` — The #time_published method returns the time when the Layout::Document was published.
- `units` — The #units method returns the units for the Layout::Document.
- `units=` — The #units= method sets the units for the Layout::Document.
