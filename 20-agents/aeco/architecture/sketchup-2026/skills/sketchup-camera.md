---
name: yard-sketchup-camera
description: Sketchup::Camera API reference (YARD)
---

# Sketchup::Camera API reference

The Camera class contains methods for creating and manipulating a camera. The camera in SketchUp is the “point of view” from which you look at the model.

## Methods

- `initialize` — Returns a new camera with eye (where the camera is) and targets (where the camera is looking).
- `aspect_ratio` — The #aspect_ratio method is used to retrieve the aspect ratio of the Camera.
- `aspect_ratio=` — The #aspect_ratio= method is used to set the aspect ratio for a Camera. Changing this value will cause SketchUp to show gray bars over the screen to show the resulting view.
- `center_2d` — The #center_2d method returns a point with the x and y offset of the camera when it's in two-point perspective or math photo mode. When the camera is in two-point perspective and the user pans around, the x and y values will change. These values are in normalized device coordinates, so for instance, the range [-1.0, 1.0] spans the full width or height of the screen.
- `description` — The #description method is used to retrieve the description for a Camera.
- `description=` — The #description= method is used to set the description for the Camera.
- `direction` — The #direction method is used to retrieve a Vector3d object in the direction that the Camera is pointing.
- `eye` — The #eye method is used to retrieve the eye Point3d object for the Camera.
- `focal_length` — The focal_length method is used to get the focal length in millimeters of perspective Camera.
- `focal_length=` — The #focal_length= method allows you to set the field of view by specifying a focal length in millimeters. Focal length must be between 1 and 3000, inclusive.
- `fov` — The #fov method retrieves the field of view of the Camera.
- `fov=` — The #fov= method sets the field of view for a Camera. Field of view must be between 1 and 120 degrees, inclusive.
- `fov_is_height?` — The #fov_is_height? method indicates whether the field of view is measured vertically, as opposed horizontally.
- `height` — The #height method retrieves the height of a Camera.
- `height=` — The #height= method is used to set the height for the Camera in inches.
- `image_width` — The #image_width method returns the width of the image, as used to calculate the #focal_length. This value has no effect on how the view is displayed in SketchUp.
- `image_width=` — The #image_width= method is used to set the width of the image, as used to calculate the #focal_length. This value has no effect on how the view is displayed in SketchUp.
- `is_2d?` — The #is_2d? method indicates whether the camera mode is two-point perspective or match photo mode, as opposed to a normal perspective or parallel projection camera.
- `perspective=` — The #perspective= method is used to set whether or not this is a perspective camera or an orthographic camera.
- `perspective?` — The #perspective? method is used to determine whether a camera is a perspective or orthographic camera.
- `scale_2d` — The #scale_2d method returns a float indicating the scaling factor of 2 point perspective cameras.
- `set` — The #set method sets the camera orientation. You have to set the camera eye, target and up parameters at the same time to make sure that you have a valid camera definition.
- `target` — The #target method retrieves Point3d that the camera is pointing at.
- `up` — The #up method is used to retrieve the up vector for the camera. This is
- `xaxis` — The #xaxis method is used to retrieve the x axis of the camera coordinate system.
- `yaxis` — The #yaxis method retrieves the y axis of the camera coordinate system.
- `zaxis` — The #zaxis method retrieves the z axis of the camera coordinate system.
