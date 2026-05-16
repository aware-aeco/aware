---
name: yard-layout-formatted-text
description: Layout::FormattedText API reference (YARD)
---

# Layout::FormattedText API reference

A formatted text entity.

## Methods

- `initialize` — The #initialize method creates a new Layout::FormattedText.
- `new_from_file` — The new_from_file method creates a new Layout::FormattedText from a text file.
- `append_plain_text` — This method does not support more than two different style runs in a single text string.
- `apply_style` — The #apply_style method sets the Style for the text starting at the given character index, and running for the given number of characters.
- `display_text` — Passing an invalid Page will prevent an auto-text tag from being substituted with its display representation.
- `grow_mode` — The #grow_mode method returns the mode for how the Layout::FormattedText sizes itself.
- `grow_mode=` — The #grow_mode= method sets the mode for how the Layout::FormattedText sizes itself.
- `plain_text` — The #plain_text method returns the plain text representation of the Layout::FormattedText.
- `plain_text=` — The #plain_text= method sets the plain text representation of the Layout::FormattedText.
- `rtf` — Passing an invalid Page will prevent an auto-text tag from being substituted with its display representation.
- `rtf=` — The #rtf= method sets the raw RTF representation of the Layout::FormattedText.
- `style` — The #style method returns a Style for the text starting at the given character index, and running for the given length.
