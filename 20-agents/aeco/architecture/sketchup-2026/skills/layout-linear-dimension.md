---
name: yard-layout-linear-dimension
description: Layout::LinearDimension API reference (YARD)
---

# Layout::LinearDimension API reference

References a linear dimension entity. A LinearDimension is composed of the following sub-entities:

## Methods

- `initialize` — The #initialize method creates a new disconnected Layout::LinearDimension.
- `auto_scale=` — The #auto_scale= method sets whether the scale for the Layout::LinearDimension is set automatically.
- `auto_scale?` — The #auto_scale? method returns whether the scale for the Layout::LinearDimension is set automatically.
- `connect` — The #connect method connects the Layout::LinearDimension to one or two Entitys using the provided ConnectionPoints. The Layout::LinearDimension must be in the same Document as the Entitys, and on the same Page if the Entitys are on non-shared Layout::Layers.
- `custom_text=` — The #custom_text= method sets whether the Layout::LinearDimension uses custom text. When true, the Layout::LinearDimension will display a custom string that doesn't change. When false, it will display the length measurement and will update automatically.
- `custom_text?` — The #custom_text? method returns whether the Layout::LinearDimension uses custom text. When true, the Layout::LinearDimension will display a custom string that doesn't change. When false, it will display the length measurement and will update automatically.
- `disconnect` — The #disconnect method disconnects the Layout::LinearDimension from its ConnectionPoints. The dimension will not be adjusted by disconnecting from its ConnectionPoints.
- `end_connection_point` — The #end_connection_point method returns the paper space location for the second connection.
- `end_connection_point=` — The #end_connection_point= method sets the paper space location for the second connection.
- `end_extent_point` — The #end_extent_point method returns the paper space location for the end of the dimension line.
- `end_extent_point=` — The #end_extent_point= method sets the paper space location for the end of the dimension line.
- `end_offset_length=` — The #end_offset_length= method sets the length of the offset from the second ConnectionPoint to the start of the second extension line. The ConnectionPoint and extent point will not move.
- `end_offset_point` — The #end_offset_point method returns the paper space location for the end of the first extension line. The first extension line runs from this offset point to the end extent point.
- `entities` — The #entities method returns the Entities that represent the Layout::LinearDimension in its exploded form. Depending on the appearance of the Layout::LinearDimension, this may return anywhere from four to six Entitys: start extension line, end extension line, one or two dimension lines, dimension text, and optionally the leader line.
- `leader_line_type` — The #leader_line_type method returns the type of leader line the Layout::LinearDimension is using.
- `leader_line_type=` — The #leader_line_type= method sets the type of leader line the Layout::LinearDimension is using.
- `leader_line_visible?` — The #leader_line_visible? method returns whether the leader line is currently visible.
- `scale` — The #scale method returns the scale being used for the Layout::LinearDimension.
- `scale=` — The #scale= method sets the scale being used for the Layout::LinearDimension.
- `start_connection_point` — The #start_connection_point method returns the paper space location for the first connection.
- `start_connection_point=` — The #start_connection_point= method sets the paper space location for the first connection.
- `start_extent_point` — The #start_extent_point method returns the paper space location for the start of the dimension line.
- `start_extent_point=` — The #start_extent_point= method sets the paper space location for the start of the dimension line.
- `start_offset_length=` — The #start_offset_length= method sets the length of the offset from the first ConnectionPoint to the start of the first extension line. The ConnectionPoint and extent point will not move.
- `start_offset_point` — The #start_offset_point method returns the paper space location for the start of the first extension line. The first extension line runs from this offset point to the start extent point.
- `text` — With the addition of auto-text in dimensions for LayOut 2019.2, the copy of the dimension text incorrectly provided the plain text when requesting the display text. This has been fixed for LayOut 2020.1.
- `text=` — The #text= method sets the Layout::LinearDimension's FormattedText.
