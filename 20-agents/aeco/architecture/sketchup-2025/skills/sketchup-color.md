---
name: yard-sketchup-color
description: Sketchup::Color API reference (YARD)
---

# Sketchup::Color API reference

The Color class is used to create and manipulate colors within SketchUp Models. The class can also be used the same way with LayOut documents.

## Methods

- `initialize` — The new method is used to create a new Color object.
- `names` — The names method is used to retrieve an array of all color names recognized by SketchUp.
- `==` — The #== method checks to see if the two Sketchup::Colors are equal. This checks whether the RGBA values are the same. In versions prior to SketchUp 2018 two color objects with the same values would be considered different.
- `alpha` — The #alpha method is used to retrieve the opacity of the color. A value of 0 is transparent, 255 is opaque.
- `alpha=` — The #alpha= method is used to set the opacity of the color. A value of 0 is transparent, 255 is opaque.
- `blend` — The #blend method is used to blend two colors.
- `blue` — The #blue method is used to retrieve the blue value of a color.
- `blue=` — The #blue= method is used to set the blue value of a color.
- `green` — The #green method is used to retrieve the green value of a color.
- `green=` — The #green= method is used to set the green component of a RGB Color.
- `red` — The #red method is used to retrieve the red component of a RGB Color.
- `red=` — The #red= method is used to set the red component of a RGB Color.
- `to_a` — The #to_a method is used to convert a Color object to an Array object. The returned array will contain 4 integer values (RGBA) between 0 and 255.
- `to_i` — The #to_i method is used to convert a Color object to an 32 bit integer.
- `to_s` — The #to_s method returns a string representation of the Sketchup::Color object, in the form of “Color(255, 255, 255, 255)”.
