---
name: yard-sketchup-text
description: Sketchup::Text API reference (YARD)
---

# Sketchup::Text API reference

The Text class contains method to manipulate a Text entity object.

## Methods

- `arrow_type` — The arrow_type method retrieves the current arrow type used for the leader text.
- `arrow_type=` — The arrow_type= method sets the arrow type used for leader text.
- `attached_to` — The #attached_to method returns an array of the attached InstancePath object and the Geom::Point3d.
- `attached_to=` — The #attached_to= method will attach the Sketchup::Text to another DrawingElement.
- `display_leader=` — The display_leader= method accepts true or false for whether to display the leader.
- `display_leader?` — The display_leader? method returns the status of the leader.
- `has_leader?` — The has_leader method is used to determine if the Text object has a leader.
- `leader_type` — The #leader_type method retrieves the currently set leader type.
- `leader_type=` — ALeaderModel cannot be set. It is only used internally as a default value. Trying to set it will raise a warning.
- `line_weight` — The line_weight method returns a line weight in number of pixels.
- `line_weight=` — The line_weight= method sets the line weight in pixels.
- `point` — The point method is used to get the point associated with the text.
- `point=` — The point= method is used to set the point associated with the text.
- `set_text` — The set_text method is used to set the text within a Text object without recording an Undo operation.
- `text` — The text method is used to retrieve the string version of a Text object.
- `text=` — The text= method is used to set the string version of a Text object.
- `vector` — The vector method is used to get the vector associated with the text.
- `vector=` — The vector= method is used to set the vector associated with the text.
