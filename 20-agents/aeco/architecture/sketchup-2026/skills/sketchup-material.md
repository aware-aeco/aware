---
name: yard-sketchup-material
description: Sketchup::Material API reference (YARD)
---

# Sketchup::Material API reference

The Material class represents a Texture or Color that can be applied to Drawingelements. It is most often applied to Faces.

## Methods

- `<=>` — The #<=> method is used to compare two materials based on ##display_name.
- `==` — The #== method is used to test if two materials are the same.
- `alpha` — The #alpha method is used to get the opacity of the material.
- `alpha=` — The #alpha= method is used to set the opacity of the material.
- `ao_enabled=` — 
- `ao_enabled?` — There is no setter for this property. Instead it's dictated whether a #ao_texture is set.
- `ao_strength` — Returns A value between 0.0 and 1.0.
- `ao_strength=` — 
- `ao_texture` — 
- `ao_texture=` — Copies another texture to this texture.
- `color` — The alpha value of the Color object is not used for the material's transparency. This is controlled by the #alpha property in order for textured materials to also have transparency.
- `color=` — The alpha value of the Color object is not used for the material's transparency. This is controlled by the #alpha property in order for textured materials to also have transparency.
- `colorize_deltas` — The #colorize_deltas method retrieves the HLS delta for colorized materials.
- `colorize_type` — This value is only relevant when the #materialType is set to MATERIAL_COLORIZED_TEXTURED.
- `colorize_type=` — This value is only relevant when the #materialType is set to MATERIAL_COLORIZED_TEXTURED.
- `display_name` — The #display_name method retrieves the name that is displayed within SketchUp for the material.
- `materialType` — The #materialType method retrieves the type of the material.
- `metallic_factor` — Returns A value between 0.0 and 1.0.
- `metallic_factor=` — 
- `metallic_texture` — 
- `metallic_texture=` — Copies another texture to this texture.
- `metalness_enabled=` — 
- `metalness_enabled?` — 
- `name` — The #name method retrieves the name of the material. This is the unique internal name of the object which should be used for retrieving the material from the model's material list.
- `name=` — Since SketchUp 2018 this method will raise an ArgumentError if the name is not unique.
- `normal_enabled=` — 
- `normal_enabled?` — 
- `normal_scale` — Returns A value larger than or equal to 0.0.
- `normal_scale=` — 
- `normal_style` — Material Normal Styles:
- `normal_style=` — Material Normal Styles:
- `normal_texture` — 
- `normal_texture=` — Copies another texture to this texture.
- `owner_type` — The #owner_type method is used to determine what owns the material.
- `roughness_enabled=` — 
- `roughness_enabled?` — 
- `roughness_factor` — Returns A value between 0.0 and 1.0.
- `roughness_factor=` — 
- `roughness_texture` — 
- `roughness_texture=` — Copies another texture to this texture.
- `save_as` — You must remember to append “.skm” to the filename as this will not be done automatically.
- `texture` — The #texture method retrieves the texture of the material.
- `texture=` — The #texture= method sets the texture for the material.
- `use_alpha?` — that this is not affected by the alpha value of the #color object. Only the #alpha value will make this method return true.
- `workflow` — Material Workflows:
- `write_thumbnail` — The #write_thumbnail method writes a bitmap thumbnail to the given file name.
