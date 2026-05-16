---
name: yard-sketchup-page
description: Sketchup::Page API reference (YARD)
---

# Sketchup::Page API reference

The Page class contains methods to extract information and modify the properties of an individual page.

## Methods

- `active_section_planes` — The #active_section_planes method is used to retrieve the active section plane for the Sketchup::Page.
- `axes` — The axes method returns the drawing axes for the page.
- `camera` — The #camera method retrieves the camera for a particular page.
- `delay_time` — The delay_time method retrieves the amount of time, in seconds, that a page will be displayed before transition to another page during a tour.
- `delay_time=` — The delay_time= method sets the amount of time, in seconds, that a page will be displayed before transitioning to another page during a tour. If you set the delay for a page to be -1, the default delay time will be used.
- `description` — The description method retrieves the description for a page as found in the Scenes manager dialog.
- `description=` — The description method sets the description for a page as found in the Scenes manager dialog.
- `environment` — The #environment method is used to retrieve the Environment for the scene.
- `environment=` — The #environment= method is used to set the Environment for the scene.
- `get_drawingelement_visibility` — The #get_drawingelement_visibility method is used to get the visibility of a drawing element on a particular page.
- `hidden_entities` — The hidden_entities method retrieves all hidden entities within a page.
- `include_in_animation=` — The #include_in_animation= method controls whether the page should be included when exporting an animation from the model.
- `include_in_animation?` — The #include_in_animation? method determines whether the page should be included when exporting an animation from the model.
- `label` — The label method retrieves the label for a page from the page tab.
- `layer_folders` — The #layer_folders method retrieves the hidden layer folders associated with a page.
- `layers` — The #layers method retrieves layers that don't use their default visibility on this page.
- `name` — The name method retrieves the name for a page from the page tab.
- `name=` — The #name= method sets the name for a page's tab. If the name is already used by another page, a unique name is created.
- `rendering_options` — Most rendering options of a scene are also present in Style and are governed by the selected style. Those options should not be changed from the scene. The ones not related to Style like fog (DisplayFog, FogColor) are safe to be changed from the scene.
- `set_drawingelement_visibility` — The #set_drawingelement_visibility method is used to change the visibility of a drawing element on a particular page. Only drawing elements on the root of the model, as well as nested instances of components, groups, and images are controlled by Page visibility.
- `set_visibility` — The #set_visibility method sets the visibility for a layer or layer folder on a page.
- `shadow_info` — While certain shadow settings, such as those available in the Shadows panel, can be controlled on a per-page basis, global settings like north angle, latitude, and longitude are managed at the model level and are not page-specific.
- `style` — The style method retrieves the style associated with the page.
- `transition_time` — Get the amount of time that it takes to transition to this page during a slideshow or animation export.  If this value is -1, it means to use the default transition time.
- `transition_time=` — The transition_time= method is used to set the transition time.
- `update` — The #update method performs an update on the page properties based on the current view that the user has. What properties of the Page get updated are controlled via an integer whose bits corresponds to different properties. These flags can be used individually or combined using bitwise OR.
- `use_axes=` — The use_axes= method sets the page's axes property.
- `use_axes?` — The use_axes? method determines whether you are storing the axes property with the page.
- `use_camera=` — The use_camera= method sets the page's camera property.
- `use_camera?` — The use_camera? method determines whether you are storing the camera property with the page.
- `use_environment=` — The #use_environment= method is used to set if the Environment settings are used in the scene.
- `use_environment?` — The #use_environment? method is used to determine if the Environment settings are used in the scene.
- `use_hidden=` — The functionality is replaced by #use_hidden_geometry= and #use_hidden_objects= in SketchUp 2020.1.
- `use_hidden?` — The functionality is replaced by #use_hidden_geometry? and #use_hidden_objects? in SketchUp 2020.1.
- `use_hidden_geometry=` — Sets the page's use hidden geometry property.
- `use_hidden_geometry?` — Returns the use hidden geometry property from the page.
- `use_hidden_layers=` — The use_hidden_layers= method sets the page's hidden layers property.
- `use_hidden_layers?` — The use_hidden_layers? method determines whether you are storing the hidden layers property with the page.
- `use_hidden_objects=` — Sets the page's use hidden objects property.
- `use_hidden_objects?` — Returns the use hidden objects property from the page.
- `use_rendering_options=` — The use_rendering_optoins= method sets the page's display settings property.
- `use_rendering_options?` — The use_rendering_options? method determines whether you are storing the rendering options property with the page.
- `use_section_planes=` — The use_section_planes= method sets the page's section planes property.
- `use_section_planes?` — The use_section_planes? method determines whether you are storing the section planes property with the page.
- `use_shadow_info=` — The use_shadow_info= method sets the page's shadow info property.
- `use_shadow_info?` — The use_shadow_info? method determines whether you are storing the shadow info property with the page.
- `use_style=` — The use_style= method sets the style to be used by the page.
- `use_style?` — The use_style? method determines whether storing a style with the page.
