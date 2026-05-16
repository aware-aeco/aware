---
name: yard-layout-reference-entity
description: Layout::ReferenceEntity API reference (YARD)
---

# Layout::ReferenceEntity API reference

An entity that represents the data inserted from an external file.

## Methods

- `clip_mask` — The #clip_mask method returns the clip mask of the Layout::ReferenceEntity, or nil if it does not have a clip mask.
- `clip_mask=` — The #clip_mask= method sets the clip mask of the Layout::ReferenceEntity. clip_mask can be a Layout::Rectangle, Ellipse, or Path, or nil, and it must not currently exist in a Document, or Group.
- `entities` — The #entities method returns the Group that represents the Layout::ReferenceEntity in its exploded form.
