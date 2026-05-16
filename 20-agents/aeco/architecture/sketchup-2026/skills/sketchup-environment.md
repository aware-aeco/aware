---
name: yard-sketchup-environment
description: Sketchup::Environment API reference (YARD)
---

# Sketchup::Environment API reference

An Environment object represents an environment in the model. Environments are used to control the background and lighting of the model. Environments can be used as skydomes, for reflections, and to link the sun to the environment.

## Methods

- `description` — The #description method gets the description for an Sketchup::Environment.
- `description=` — The #description= method sets the description for an Sketchup::Environment.
- `linked_sun=` — The #linked_sun= method is used to set if the Sketchup::Environment is linked to the sun. Shadow lighting is used to create realistic shadows in the scene, enhancing the visual quality.
- `linked_sun?` — The #linked_sun? method is used to determine if the Sketchup::Environment is linked to the sun. This function returns a boolean value indicating whether the shadow light feature is currently enabled in the environment. Shadow lighting is used to create realistic shadows in the scene, enhancing the visual quality.
- `linked_sun_position` — The #linked_sun_position method is used to get the position of the sun linked to the Sketchup::Environment. The position is a Geom::Point3d where the x must be in range [0.0, 1.0] and y must be in range [-1.0, 1.0].
- `linked_sun_position=` — The #linked_sun_position= method is used to set the position of the sun linked to the Sketchup::Environment. The position is a Geom::Point3d where the x must be in range [0.0, 1.0] and y must be in range [-1.0, 1.0].
- `name` — The #name method retrieves the name of the Sketchup::Environment. This is the unique internal name of the object which should be used for retrieving the Sketchup::Environment from the model's Sketchup::Environments.
- `name=` — The #name= method sets the name for an Sketchup::Environment.
- `path` — The #path method is used to get the file name of the image or SKE file used for the Sketchup::Environment.
- `reflection_exposure` — Reflection exposure is a value between 0.0 and 10.0, where 0.0 is no exposure and 10.0 is full exposure.
- `reflection_exposure=` — Reflection exposure is a value between 0.0 and 10.0, where 0.0 is no exposure and 10.0 is full exposure.
- `rotation` — The #rotation method is used to get the vertical rotation angle in degrees to apply to the Sketchup::Environment.
- `rotation=` — Rotation is a value between 0.0 and 360.0 degrees.
- `skydome_exposure` — Skydome exposure is a value between 0.0 and 20.0, where 0.0 is no exposure and 20.0 is full exposure.
- `skydome_exposure=` — Skydome exposure is a value between 0.0 and 20.0, where 0.0 is no exposure and 20.0 is full exposure.
- `thumbnail` — The #thumbnail method is used to get the thumbnail image of the Sketchup::Environment.
- `use_as_skydome=` — The #use_as_skydome= method is used to set if the Sketchup::Environment is used as a skydome.
- `use_as_skydome?` — The #use_as_skydome? method is used to determine if the Sketchup::Environment is used as a skydome.
- `use_for_reflections=` — The #use_for_reflections= method is used to set if the Sketchup::Environment is used for reflections.
- `use_for_reflections?` — The #use_for_reflections? method is used to determine if the Sketchup::Environment is used for reflections.
- `write_hdr` — The #write_hdr method writes the HDR, EXR or SKE image of the environment to a file in its original file type.
