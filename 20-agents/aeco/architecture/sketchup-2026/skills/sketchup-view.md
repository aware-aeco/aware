---
name: yard-sketchup-view
description: Sketchup::View API reference (YARD)
---

# Sketchup::View API reference

This class contains methods to manipulate the current point of view of the model. The drawing methods here (#draw_line, #draw_polyline, etc) are meant to be invoked within a tool's Tool#draw method. Calling them outside Tool#draw will have no effect.

## Methods

- `add_observer` — The add_observer method is used to add an observer to the current object.
- `animation=` — The #animation= method is used to set an animation that is displayed for a view. See Animation for details on how to create an animation object.
- `average_refresh_time` — The average_refresh_time is used to set the average time used to refresh the current model in the view. This can be used to estimate the frame rate for an animation.
- `camera` — The camera method is used to retrieve the camera for the view.
- `camera=` — The #camera= method is used to set the camera for the view. If a transition time is given, then it will animate the transition from the current camera to the new one.
- `center` — The #center method is used to retrieve the coordinates of the center of the view in pixels.
- `corner` — If the view uses a Camera with a fixed aspect ratio, then the corners are the corners of the viewing are of the camera which might be different than the actual corners of the view itself.
- `device_height` — The #device_height method is used to retrieve the height of the viewport for the view in physical pixels.
- `device_width` — The #device_width method is used to retrieve the width of the viewport for the view in physical pixels.
- `draw` — If you draw outside the model bounds you need to implement Tool#getExtents which returns a bounding box large enough to include the points you draw. Otherwise your drawing will be clipped.
- `draw2d` — Prior to SketchUp 2025.0 this method accepted the points as physical screen coordinates. As of SketchUp 2025.0 the points are expected to be in logical screen coordinates. Older versions need to apply the scaling factor from UI.scale_factor to the points before passing them to this method.
- `draw_lines` — The draw_lines method is used to draw disconnected lines.
- `draw_lines` — The draw_lines method is used to draw disconnected lines.
- `draw_points` — Prior to SketchUp 2025.0 this method accepted the size as physical pixels. As of SketchUp 2025.0 the points are expected to be in logical pixels. Older versions need to apply the scaling factor from UI.scale_factor to the size before passing them to this method.
- `draw_polyline` — The draw_polyline method is used to draw a series of connected line segments from pt1 to pt2 to pt3, and so on.
- `draw_text` — Under Windows the font name must be less than 32 characters - due to system limitations.
- `drawing_color=` — The drawing_color method is used to set the color that is used for drawing to the view.
- `dynamic=` — This method is no longer doing anything.
- `field_of_view` — The field_of_view method is used get the view's field of view setting, in degrees.
- `field_of_view=` — The field_of_view= method is used set the view's field of view setting, in degrees.
- `graphics_engine` — The #graphics_engine method is used query the type of the graphics engine that's currently used by this view.
- `guess_target` — The guess_target method is used to guess at what the user is looking at when you have a perspective view.
- `inference_locked?` — The inference_locked? method is used to determine if inference locking is on for the view.
- `inputpoint` — The #inputpoint method is used to retrieve an InputPoint.
- `invalidate` — This is the preferred method to update the viewport. Use this before trying to use #refresh.
- `last_refresh_time` — The last_refresh_time method is used to retrieve the time for the last full view refresh.
- `line_stipple=` — The line_stipple= method is used to set the line pattern to use for drawing. The stipple pattern is given as a string. Valid strings are:
- `line_width=` — As of SU2017 this will automatically scale the line width by the same factor as UI.scale_factor.
- `load_texture` — Avoid loading and releasing textures within the Tool#draw event as that is not efficient.
- `lock_inference` — The #lock_inference method is used to lock or unlock an inference.
- `model` — The model method is used to retrieve the model for the current view.
- `pick_helper` — The #pick_helper method is used to retrieve a pick helper for the view.
- `pickray` — The #pickray method is used to retrieve a ray passing through a given screen position in the viewing direction.
- `pixels_to_model` — As of SU2017 this will automatically scale the pixel-size by the same factor as UI.scale_factor.
- `refresh` — This method might impact performance and if used incorrectly cause instability or crashes. Don't use this unless you have verified that you cannot use #invalidate instead.
- `release_texture` — Releases a texture loaded via #load_texture, freeing up it's memory. It's good practice to do so whenever there is no longer any need for the resource.
- `remove_observer` — The remove_observer method is used to remove an observer from the current object.
- `screen_coords` — Prior to SketchUp 2025.0 this method returned the points as physical screen coordinates. As of SketchUp 2025.0 the points are returned in logical screen coordinates.
- `set_color_from_line` — Set the drawing color for the view based on the direction of a line that you want to draw. These colors will match the axes colors in the SketchUp model (typically blue for straight up and down, etc.)
- `show_frame` — The show_frame method is used to show a frame of an Animation object in the current view.
- `text_bounds` — Under Windows the font name must be less than 32 characters - due to system limitations.
- `tooltip=` — Set a tooltip to display in the view. This is useful for displaying tooltips in a tool that you write in Ruby.
- `vpheight` — Prior to SketchUp 2025.0 this method returned the size as physical screen coordinates. As of SketchUp 2025.0 the size are returned in logical screen coordinates.
- `vpwidth` — Prior to SketchUp 2025.0 this method returned the size as physical screen coordinates. As of SketchUp 2025.0 the size are returned in logical screen coordinates.
- `write_image` — The #write_image method is used to write the current view to an image file.
- `zoom` — The zoom method is used to zoom in or out by some zoom factor.
- `zoom_extents` — The zoom_extents method is used to zoom to the extents about the entire model, as if the user has selected the zoom extents command from the menu.
