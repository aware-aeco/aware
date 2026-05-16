# layout-table-cell-span

Lifecycle: single

The #span method returns the row and column span of a Layout::TableCell. If the values returned are both 1, then it is a normal, non-merged cell. If either of the values are greater than 1, then it is a merged cell. If the values are both 0, then it is an unused cell that resides within the inner portion of another merged cell.
