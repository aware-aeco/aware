---
name: yard-layout-style
description: Layout::Style API reference (YARD)
---

# Layout::Style API reference

References a collection of style attributes that determine the visual appearance of Entitys. Style attributes are those attributes which the user can manipulate in LayOut's inspector windows. For example, shape style attributes that define stroke and fill, or text style attributes that define the font for FormattedText. The Document maintains a default style for various types of Entitys, and it is possible to apply the style of one entity to another. Style objects are transient and do not belong to a Document.

## Methods

- `initialize` — The #initialize method creates a new Layout::Style.
- `arrow_type_filled?` — The arrow_type_filled? method returns whether the specified arrow type is filled or not.
- `dimension_rotation_alignment` — The #dimension_rotation_alignment method returns the rotational text alignment for LinearDimension text, or nil if the Layout::Style does not have a value for that setting.
- `dimension_rotation_alignment=` — The #dimension_rotation_alignment= method sets the rotational text alignment.
- `dimension_units` — The #dimension_units method returns the unit format and precision for dimensions, or nil if the Layout::Style does not have a value for that setting. Units may be for either LinearDimensions or AngularDimensions, but not both.
- `dimension_vertical_alignment` — The #dimension_vertical_alignment method returns the vertical text alignment for LinearDimension text, or nil if the Layout::Style does not have a value for that setting.
- `dimension_vertical_alignment=` — The #dimension_vertical_alignment= method sets the vertical text alignment for LinearDimension text.
- `end_arrow_size` — The #end_arrow_size method returns the size of the end arrow, or nil if the Layout::Style does not have a value for that setting.
- `end_arrow_size=` — The #end_arrow_size= method sets the size of the end arrow. The minimum size is 0.25.
- `end_arrow_type` — The #end_arrow_type method returns the type of end arrow, or nil if the Layout::Style does not have a value for that setting.
- `end_arrow_type=` — The #end_arrow_type= method sets the type of end arrow.
- `fill_color` — The #fill_color method returns the solid file color, or nil if the Layout::Style does not have a value for that setting.
- `fill_color=` — The #fill_color= method sets the solid fill color.
- `font_family` — The #font_family method returns the text font name, or nil if the Layout::Style does not have a value for that setting.
- `font_family=` — The #font_family= method sets the text font name.
- `font_size` — The #font_size method returns the font size, or nil if the Layout::Style does not have a value for that setting.
- `font_size=` — The #font_size= method sets the font size.
- `get_sub_style` — The #get_sub_style method returns the Layout::Style for a sub-entity from the Layout::Style. This would be used to get the current style of a LinearDimension's text, for example.
- `pattern_fill_origin` — The #pattern_fill_origin method returns the starting piont for the pattern fill, or nil if the Layout::Style does not have a value for that setting.
- `pattern_fill_origin=` — The #pattern_fill_origin= method sets the starting point for the pattern fill.
- `pattern_fill_path` — The #pattern_fill_path method returns the file path to the pattern fill image, or nil if the Layout::Style does not have a value for that setting.
- `pattern_fill_path=` — The #pattern_fill_path= method sets the path to the image to use for the pattern fill.
- `pattern_fill_rotation` — The #pattern_fill_rotation method returns the rotation of the pattern fill image in degrees, or nil if the Layout::Style does not have a value for that setting.
- `pattern_fill_rotation=` — The #pattern_fill_rotation= method sets the rotation in degrees of the pattern fill image.
- `pattern_fill_scale` — The #pattern_fill_scale method returns the pattern fill scale, or nil if the Layout::Style does not have a value for that setting.
- `pattern_fill_scale=` — The #pattern_fill_scale= method sets the pattern fill scale.
- `pattern_filled` — The #pattern_filled method returns whether the Layout::Style has a pattern fill, or nil if the Layout::Style does not have a value for that setting.
- `pattern_filled=` — The #pattern_filled= method sets whether the Layout::Style has a pattern fill.
- `set_dimension_units` — The #set_dimension_units method sets the unit format and precision for dimensions. Units may be for either LinearDimensions or AngularDimensions, but not both.
- `set_sub_style` — The #set_sub_style method adds a Layout::Style to apply to a Entity's sub-entity. This would be used to set the arrow type for extension lines of a LinearDimension, for example.
- `solid_filled` — The #solid_filled method returns whether the Layout::Style has a solid fill, or nil if the Layout::Style does not have a value for that setting.
- `solid_filled=` — The #solid_filled= method sets whether the Layout::Style has a solid fill.
- `start_arrow_size` — The #start_arrow_size method returns the size of the start arrow, or nil if the Layout::Style does not have a value for that setting.
- `start_arrow_size=` — The #start_arrow_size= method sets the size of the start arrow. The minimum size is 0.25.
- `start_arrow_type` — The #start_arrow_type method returns the type of start arrow, or nil if the Layout::Style does not have a value for that setting.
- `start_arrow_type=` — The #start_arrow_type= method sets the type of start arrow.
- `stroke_cap_style` — The #stroke_cap_style method returns the stroke cap style, or nil if the Layout::Style does not have a value for that setting.
- `stroke_cap_style=` — The #stroke_cap_style= method sets the stroke cap style.
- `stroke_color` — The #stroke_color method returns the stroke color, or nil if the Layout::Style does not have a value for that setting.
- `stroke_color=` — The #stroke_color= method sets the stroke color.
- `stroke_join_style` — The #stroke_join_style method returns the stroke join style, or nil if the Layout::Style does not have a value for that setting.
- `stroke_join_style=` — The #stroke_join_style= method sets the stroke join style.
- `stroke_pattern` — The #stroke_pattern method returns the stroke pattern, or nil if the Layout::Style does not have a value for that setting.
- `stroke_pattern=` — The #stroke_pattern= method sets the stroke pattern.
- `stroke_pattern_scale` — The #stroke_pattern_scale method returns the stroke pattern scale, or nil if the Layout::Style does not have a value for that setting.
- `stroke_pattern_scale=` — The #stroke_pattern_scale= method sets the stroke pattern scale.
- `stroke_width` — The #stroke_width method returns the stroke width, or nil if the Layout::Style does not have a value for that setting.
- `stroke_width=` — The #stroke_width= method sets the stroke width.
- `stroked` — The #stroked method returns whether the Layout::Style has a stroke, or nil if the Layout::Style does not have a value for that setting.
- `stroked=` — The #stroked= method sets whether the Layout::Style has a stroke.
- `suppress_dimension_units` — The #suppress_dimension_units method returns whether the units for dimensions are suppressed, or nil if the Layout::Style does not have a value for that setting.
- `suppress_dimension_units=` — The #suppress_dimension_units= method sets whether the units for dimensions are suppressed.
- `text_alignment` — The #text_alignment method returns the text alignment, or nil if the Layout::Style does not have a value for that setting.
- `text_alignment=` — The #text_alignment= method sets the text alignment.
- `text_anchor` — The #text_anchor method returns the text anchor type, or nil if the Layout::Style does not have a value for that setting.
- `text_anchor=` — The #text_anchor= method sets the text anchor type.
- `text_bold` — The #text_bold method returns whether text is bold, or nil if the Layout::Style does not have a value for that setting.
- `text_bold=` — The #text_bold= method sets whether text is bold.
- `text_color` — The #text_color method returns the text color, or nil if the Layout::Style does not have a value for that setting.
- `text_color=` — The #text_color= method sets the text color.
- `text_elevation` — The #text_elevation method returns the text elevation, or nil if the Layout::Style does not have a value for that setting.
- `text_elevation=` — The #text_elevation= method sets the text elevation.
- `text_italic` — The #text_italic method returns whether text is italic, or nil if the Layout::Style does not have a value for that setting.
- `text_italic=` — The #text_italic= method sets whether text is italic.
- `text_strikethrough` — The #text_strikethrough method returns the text strike through type, or nil if the Layout::Style does not have a value for that setting.
- `text_strikethrough=` — The #text_strikethrough= method sets the text strike through type.
- `text_underline` — The #text_underline method returns the text underline type, or nil if the Layout::Style does not have a value for that setting.
- `text_underline=` — The #text_underline= method sets the text underline type.
