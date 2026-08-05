//! Minimal column-aligned text table. Pads each column to the max width of
//! its values plus 2 spaces. No external dep needed for v0.1.

// CLAUDE.md §Code style: "Errors as data, not exceptions." A panicking index is
// an exception in all but name, and `render` shipped one — see the comment on
// the width loop below.
//
// `clippy::indexing_slicing` is a `restriction` lint, so `cargo clippy -D
// warnings` does not enable it and nothing caught that. It cannot simply be
// denied crate-wide: it also fires on `serde_json::Value` indexing, which
// yields `Value::Null` for a missing key rather than panicking, and this crate
// has ~100 such sites — denying it wholesale would mean ~100 `#[allow]`s, which
// is re-opening a gate rather than passing it. Denying it *here* costs nothing:
// this module's whole job is formatting, no index in it is load-bearing, and
// after the fix below it has none. `not(test)` mirrors `src/main.rs` — CLAUDE.md
// permits panicking constructs in test code.
#![cfg_attr(not(test), deny(clippy::indexing_slicing))]

pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(headers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            headers: headers.into_iter().map(Into::into).collect(),
            rows: Vec::new(),
        }
    }

    pub fn row(&mut self, row: impl IntoIterator<Item = impl Into<String>>) -> &mut Self {
        self.rows.push(row.into_iter().map(Into::into).collect());
        self
    }

    pub fn render(&self) -> String {
        // Sized to the widest *row*, not to the header row. `Table::row` accepts
        // any number of cells and never checks it against the header arity, so a
        // row with more cells than there are headers is in-contract — and the
        // extra columns still need a width. Sizing from the headers alone and
        // then dropping the overflow (`if i < widths.len()`) is what left
        // `format_row` indexing past the end of `widths`: it iterates the *row*,
        // so it asked for widths that were deliberately never recorded, and a
        // one-header table with a two-cell row aborted the process.
        //
        // `get_mut`/`push` rather than an index: `enumerate` yields `i` in
        // lockstep with `widths.len()`, so a miss means exactly "this column is
        // new", and the growth stays panic-free by construction.
        let mut widths: Vec<usize> = Vec::new();
        for row in std::iter::once(&self.headers).chain(self.rows.iter()) {
            for (i, cell) in row.iter().enumerate() {
                let width = cell.chars().count();
                match widths.get_mut(i) {
                    Some(slot) => *slot = (*slot).max(width),
                    None => widths.push(width),
                }
            }
        }

        let mut out = String::new();
        let format_row = |row: &[String]| -> String {
            let mut line = String::new();
            for (i, cell) in row.iter().enumerate() {
                // `widths` now covers every column of every row, so the fallback
                // is unreachable — it is here so this stays total rather than
                // trading the old panic for a differently-spelled one.
                let w = widths.get(i).copied().unwrap_or(0);
                let pad = w.saturating_sub(cell.chars().count());
                line.push_str(cell);
                line.push_str(&" ".repeat(pad));
                if i + 1 < row.len() {
                    line.push_str("  ");
                }
            }
            line.trim_end().to_string()
        };

        out.push_str(&format_row(&self.headers));
        out.push('\n');
        for row in &self.rows {
            out.push_str(&format_row(row));
            out.push('\n');
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_aligned_table() {
        let mut t = Table::new(["ID", "VERSION"]);
        t.row(["tekla", "2025.0.1"]).row(["html-report", "1.0.0"]);
        let s = t.render();
        let lines: Vec<_> = s.lines().collect();
        assert_eq!(lines[0], "ID           VERSION");
        assert_eq!(lines[1], "tekla        2025.0.1");
        assert_eq!(lines[2], "html-report  1.0.0");
    }

    #[test]
    fn empty_table_renders_just_header() {
        let t = Table::new(["A", "B"]);
        let s = t.render();
        assert_eq!(s, "A  B\n");
    }

    /// The regression. Before the fix `widths` was sized from the headers, so
    /// `format_row` — which iterates the *row* — indexed `widths[1]` on a
    /// one-column `widths` and aborted the process.
    ///
    /// Asserting the rendered text, not merely that it returned: a `render` that
    /// silently dropped the overflow cell would also "not panic", and losing a
    /// column of output is the same defect wearing a quieter coat.
    #[test]
    fn a_row_wider_than_the_header_renders_instead_of_panicking() {
        let mut t = Table::new(["ID"]);
        t.row(["tekla", "2025.0.1"]).row(["html-report", "1.0.0"]);
        let lines: Vec<String> = t.render().lines().map(str::to_string).collect();
        assert_eq!(
            lines,
            vec!["ID", "tekla        2025.0.1", "html-report  1.0.0",],
            "the overflow column must be rendered and aligned, not dropped"
        );
    }

    /// The mirror case: fewer cells than headers. This never panicked, but the
    /// widths rewrite touches both paths, so it is pinned too.
    #[test]
    fn a_row_narrower_than_the_header_keeps_the_remaining_columns_aligned() {
        let mut t = Table::new(["ID", "VERSION", "KIND"]);
        t.row(["tekla"]).row(["html-report", "1.0.0", "native"]);
        let lines: Vec<String> = t.render().lines().map(str::to_string).collect();
        assert_eq!(
            lines,
            vec![
                "ID           VERSION  KIND",
                "tekla",
                "html-report  1.0.0    native",
            ]
        );
    }
}
