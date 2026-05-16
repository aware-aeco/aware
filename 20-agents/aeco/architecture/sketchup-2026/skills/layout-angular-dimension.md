---
name: yard-layout-angular-dimension
description: Layout::AngularDimension API reference (YARD)
---

# Layout::AngularDimension API reference

References an angular dimension entity. An AngularDimension is composed of the following sub-entities:

## Methods

- `initialize` — The #initialize method creates a new disconnected Layout::AngularDimension.
- `angle` — The #angle method returns the Layout::AngularDimension's angle. The angle is represented in radians.
- `arc_center_point` — The #arc_center_point method returns the paper space location for the dimension arc center point.
- `custom_text=` — The #custom_text= sets whether or not the Layout::AngularDimension uses custom text. When true, the text will display a custom string that doesn't change. When false, the text will display the length measurement and wil update automatically.
- `custom_text?` — The #custom_text? method returns whether the Layout::AngularDimension uses custom text. When true, the text will display a custom string that doesn't change. When false, the text will display the length measurement and will update automatically.
- `end_connection_point` — The #end_connection_point method returns the paper space location for the second connection.
- `end_connection_point=` — The #end_connection_point= method sets the paper space location for the second connection.
- `end_extent_point` — The #end_extent_point method returns the paper space location for the end of the dimension line.
- `end_extent_point=` — The #end_extent_point= method sets the paper space location for the end of the dimension line.
- `end_offset_length=` — The #end_offset_length= method sets the length of the offset from the second connection point to the start of the second extension line. The connection and extent points will not move.
- `end_offset_point` — The #end_offset_point method returns the paper space location for the end of the first extension line. The first extension line runs from this offset point to the end extent point.
- `entities` — The #entities method returns the Entities that represent the Layout::AngularDimension in its exploded form. Depending on the appearance of the Layout::AngularDimension being exploded, this may return anywhere from four to six Entitys: start extension line, end extension line, one or two dimension lines, dimension text, and optionally the leader line.
- `leader_line_type` — The #leader_line_type method returns the type of leader line the Layout::AngularDimension is using.
- `leader_line_type=` — The #leader_line_type= method sets the type of leader line the Layout::AngularDimension is using.
- `radius` — The #radius method returns the Layout::AngularDimension's radius. This is the distance from the arc center point to the dimension line.
- `radius=` — The #radius= method sets the the Layout::AngularDimension's radius. This is the distance from the arc center point to the dimension line.
- `start_connection_point` — The #start_connection_point method returns the paper space location for the first connection.
- `start_connection_point=` — The #start_connection_point= method sets the paper space location for the first connection.
- `start_extent_point` — The #start_extent_point method returns the paper space location for the start of the dimension line.
- `start_extent_point=` — The #start_extent_point= method sets the paper space location for the start of the dimension line.
- `start_offset_length=` — The #start_offset_length= method sets the length of the offset from the first connection point to the start of the first extension line. The connection and extent points will not move.
- `start_offset_point` — The #start_offset_point method returns the paper space location for the start of the first extension line. The first extension line runs from this offset point to the start extent point.
- `text` — With the addition of auto-text in dimensions for LayOut 2019.2, the copy of the dimension text incorrectly provided the plain text when requesting the display text. This has been fixed for LayOut 2020.1.
- `text=` — The #text= method sets the Layout::AngularDimension's FormattedText.
