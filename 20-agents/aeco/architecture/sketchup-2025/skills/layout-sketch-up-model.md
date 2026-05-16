---
name: yard-layout-sketch-up-model
description: Layout::SketchUpModel API reference (YARD)
---

# Layout::SketchUpModel API reference

A SketchUp Model entity. This is an instance of a SketchUp Model that is inserted into a .layout file. You can change the render mode, line weight, and set the current scene for the SketchUp Model with this interface.

## Methods

- `initialize` — The #initialize method creates a new Layout::SketchUpModel.
- `camera_modified?` — The #camera_modified? method returns whether the camera of the Layout::SketchUpModel has been modified.
- `clip_mask` — The #clip_mask method returns the clip mask entity for the Layout::SketchUpModel, or nil if it does not have one. clip_mask can be a Rectangle, Ellipse, or Path.
- `clip_mask=` — clip_mask may be nil as of LayOut 2020.1.
- `current_scene` — The #current_scene method returns the index of the most recently selected scene of the Layout::SketchUpModel.
- `current_scene=` — LayOut automatically adds the scene “Last Saved SketchUp View” to each Layout::SketchUpModel. This means that the Sketchup::Model's scenes start with index 1.
- `current_scene_modified?` — The #current_scene_modified? method returns whether the most recently selected scene of the Layout::SketchUpModel has been modified.
- `dash_scale` — The #dash_scale method returns the dash scale for the Layout::SketchUpModel. A scale value of 0.0 means the dashes are scaled based on the line weight.
- `dash_scale=` — The #dash_scale= method sets the dash scale for the Layout::SketchUpModel. A scale value of 0.0 or lower will “auto” scale the dashes based on the line weight.
- `display_background=` — The #display_background= method sets whether the background is displayed for the Layout::SketchUpModel.
- `display_background?` — The #display_background? method returns whether the background is displayed for the Layout::SketchUpModel.
- `effects_modified?` — The #effects_modified? method returns whether the shadow or fog settings of the Layout::SketchUpModel have been modified.
- `entities` — The #entities method returns the Group that represents the Layout::SketchUpModel in its exploded form. The Group will contain a Image for raster and hybrid-rendered models, and will contain a Group of LayOut entities for vector and hybrid-rendered models.
- `layers_modified?` — In SketchUp 2020, SketchUp “layers” were renamed to “tags”. For consistency with the SketchUp API, this will continue to refer to “tags” as “layers”.
- `line_weight` — The #line_weight method returns the line weight for the Layout::SketchUpModel.
- `line_weight=` — The #line_weight= method sets the line weight for the Layout::SketchUpModel. Line weight must be at least 0.01.
- `model_to_paper_point` — The #model_to_paper_point method converts the Geom::Point3d in the Layout::SketchUpModel to a Geom::Point2d in paper space.
- `output_entities` — The #output_entities method returns the Group that represents the Layout::SketchUpModel in its exported form. The Group will contain a Image for raster and hybrid-rendered models, and will contain a Group of LayOut entities for vector and hybrid-rendered models. This takes into account the output resolution set in the document's PageInfo, and the render mode override set on the document.
- `perspective=` — The #perspective= method sets whether the Layout::SketchUpModel's view is perspective or orthographic.
- `perspective?` — The #perspective? method returns whether the Layout::SketchUpModel's view is perspective or orthographic.
- `preserve_scale_on_resize=` — The #preserve_scale_on_resize= method sets whether the scale is preserved when the Layout::SketchUpModel is resized.
- `preserve_scale_on_resize?` — The #preserve_scale_on_resize? method returns whether the scale is preserved when the Layout::SketchUpModel is resized.
- `render` — The #render method renders the Layout::SketchUpModel. If the model belongs to a Document, then the render will be performed at the quality set in document.page_info (see Document and PageInfo). Otherwise, the render will be performed at Low quality.
- `render_mode` — The #render_mode method returns the render mode of the Layout::SketchUpModel.
- `render_mode=` — The #render_mode= method sets the render mode of the Layout::SketchUpModel.
- `render_needed?` — The #render_needed? method returns whether the Layout::SketchUpModel needs to be rendered.
- `reset_camera` — The #reset_camera method resets the Layout::SketchUpModel's camera to the scene's setting.
- `reset_effects` — The #reset_effects method resets the Layout::SketchUpModel's shadow and fog settings to the scene's settings.
- `reset_layers` — In SketchUp 2020, SketchUp “layers” were renamed to “tags”. For consistency with the SketchUp API, this will continue to refer to “tags” as “layers”.
- `reset_style` — The #reset_style method resets the Layout::SketchUpModel's style to the scene's setting.
- `scale` — The #scale method returns the scale of the Layout::SketchUpModel.
- `scale=` — The #scale= method sets the scale of the Layout::SketchUpModel. Scale must be at least 0.0000001, and the view must be orthographic.
- `scenes` — The #scenes method returns an array of scene names that are available for the Layout::SketchUpModel. The first scene will always be the default scene, called “Last saved SketchUp View”.
- `style_modified?` — The #style_modified? method returns whether the style of the Layout::SketchUpModel has been modified.
- `view` — The #view method returns the standard view of the Layout::SketchUpModel.
- `view=` — The #view= method sets the standard view of the Layout::SketchUpModel.
