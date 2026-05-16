---
name: yard-sketchup-layer
description: Sketchup::Layer API reference (YARD)
---

# Sketchup::Layer API reference

As of SketchUp 2020 “Layers” were renamed to “Tags” in the UI. The API retains the use of “Layer” for compatibility and is synonymous with “Tag”.

## Methods

- `<=>` — The #<=> method is used to compare two layers based on their names. This enables the Ruby Array#sort method to sort SketchUp layers.
- `==` — The #== method is used to determine if two layers are the same.
- `color` — The #color method is used to retrieve the color of the layer.
- `color=` — The #color= method is used to set the color of a layer.
- `display_name` — The display name and internal name of layers should share the same value except for the Layer0. From version 2020.0 onwards the display name of Layer0 is “Untagged” and it is localized.
- `folder` — The #folder method is used to return the parent layer folder of a layer.
- `folder=` — The #folder= method is used to set the parent layer folder of a layer.
- `line_style` — The #line_style method retrieves the line style on this layer.
- `line_style=` — The #line_style= method lets you set a specific line style to a layer
- `name` — The internal layer and display name of layers should share the same value except for the Layer0. From version 2020.0 onwards the display name of Layer0 is “Untagged” and it is localized.
- `name=` — The #name= method is used to set the name of a layer.
- `page_behavior` — The #page_behavior method is used to retrieve the visibility behavior of the layer for new pages and existing pages. For example, you may want your layer to be visible or hidden by default in any new pages (aka Scenes) created by the user.
- `page_behavior=` — The #page_behavior= method is used to control the layer's visibility behavior on existing and new pages.
- `visible=` — The #visible= method is used to set if the layer is visible.
- `visible?` — The #visible? method is used to determine if the layer is visible.
