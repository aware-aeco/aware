---
name: yard-layout-table-cell
description: Layout::TableCell API reference (YARD)
---

# Layout::TableCell API reference

A TableCell is a single cell from a table that contains data.

## Methods

- `data` — The #data method creates a copy of the FormattedText for the Layout::TableCell.
- `data=` — At the moment, this only works for Entitys that are FormattedTexts. However, future versions of LayOut may support other types of Entitys for Layout::TableCells, so this method cannot be assumed to always fail with inputs of other Entity types.
- `rotation` — The #rotation method returns the rotation of a Layout::TableCell.
- `rotation=` — The #rotation= method sets the rotation of a Layout::TableCell.
- `span` — The #span method returns the row and column span of a Layout::TableCell. If the values returned are both 1, then it is a normal, non-merged cell. If either of the values are greater than 1, then it is a merged cell. If the values are both 0, then it is an unused cell that resides within the inner portion of another merged cell.
