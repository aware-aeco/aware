---
name: yard-layout-table
description: Layout::Table API reference (YARD)
---

# Layout::Table API reference

A Table is a series of rows and columns that holds data.

## Methods

- `initialize` — The #initialize method creates a Layout::Table with a specified size, and a specified number of rows and columns.
- `[]` — The #[] method returns the Layout::TableCell at the specified row and column.
- `dimensions` — The #dimensions method returns the number of rows and columns in a Layout::Table.
- `each` — The #each method iterates in column major order through all of the cells in the Layout::Table.
- `entities` — The #entities method creates and returns the Entities that represent the Layout::Table in its exploded form.
- `get_column` — The #get_column method returns the Layout::TableColumn at the specified index.
- `get_row` — The #get_row method returns the Layout::TableRow at the specified index.
- `insert_column` — The #insert_column method inserts a new column at the specified index.
- `insert_row` — The #insert_row method inserts a new row at the specified index.
- `merge` — The #merge method merges a range of cells within a Layout::Table. Only cells which are not already merged can be merged.
- `remove_column` — The #remove_column method removes the column at the specified index.
- `remove_row` — The #remove_row method removes the row at the specified index.
