---
name: yard-layout-group
description: Layout::Group API reference (YARD)
---

# Layout::Group API reference

A group is a special type of Entity that does not belong to a Layer and contains other Entitys as children. A Group's children may include other Groups, allowing for a hierarchical tree structure of Entitys. A Group must contain at least one child and will be automatically collapsed if an operation is performed that results in the Group being empty.

## Methods

- `initialize` — The #initialize method creates a new Layout::Group.
- `entities` — The #entities method returns the Entities that belong to the Layout::Group.
- `remove_scale_factor` — The #remove_scale_factor method removes the scale factor from the Layout::Group.
- `scale_factor` — The #scale_factor method returns the the scale factor associated with the Layout::Group.
- `scale_precision` — The #scale_precision method returns the precision used for the scale of the Layout::Group.
- `scale_precision=` — LayOut only allows for a finite set of precision values for each units setting, so it will set the precision to the closest valid setting for the specified units. See the “Units” section of LayOut's “Document Setup” dialog for a reference of the available precisions for each units setting.
- `scale_units` — The #scale_units method returns the units format used in the scale for the Layout::Group.
- `scale_units=` — The #scale_units= method sets the the units format for the scale of the Layout::Group.
- `set_scale_factor` — The #set_scale_factor method sets the the scale factor for the Layout::Group.
- `ungroup` — The #ungroup method removes all Entitys from the Layout::Group and deletes the Layout::Group.
