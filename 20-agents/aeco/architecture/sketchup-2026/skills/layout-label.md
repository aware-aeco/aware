---
name: yard-layout-label
description: Layout::Label API reference (YARD)
---

# Layout::Label API reference

This is an interface to a label entity. A Label consists of a FormattedText and the label leader Path. A Label may be connected to another Entity via a ConnectionPoint.

## Methods

- `initialize` — The #initialize method creates a new disconnected Layout::Label.
- `connect` — The #connect method connects the Layout::Label to the given ConnectionPoint. The leader line will be adjusted to point at the ConnectionPoint. The Layout::Label must be in the same Document as the ConnectionPoint. If both the Layout::Label and the ConnectionPoint's Entity are on non-shared Layout::Layers, they must be on the same Page.
- `connection_type` — The #connection_type method returns the type of the text connection for the Layout::Label.
- `connection_type=` — The #connection_type= method sets the type of the text connection for the Layout::Label.
- `disconnect` — The #disconnect method disconnects the Layout::Label from its ConnectionPoint. The leader line will not be adjusted by disconnecting from a ConnectionPoint.
- `entities` — The #entities method returns the Entities that represent the Layout::Label in its exploded form.
- `leader_line` — The #leader_line method returns a copy of the leader line.
- `leader_line=` — The #leader_line= method sets the leader line.
- `leader_line_type` — The #leader_line_type method returns the type of the leader line for the Layout::Label.
- `leader_line_type=` — The #leader_line_type= method sets the type of the leader line for the Layout::Label.
- `text` — The #text method returns a copy of the FormattedText of the Layout::Label.
- `text=` — The #text= method sets the FormattedText of the Layout::Label.
