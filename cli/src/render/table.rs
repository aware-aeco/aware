#![allow(dead_code)]
//! Minimal column-aligned text table. Pads each column to the max width of
//! its values plus 2 spaces. No external dep needed for v0.1.

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
        let mut widths: Vec<usize> = self.headers.iter().map(|h| h.chars().count()).collect();
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                if i < widths.len() {
                    widths[i] = widths[i].max(cell.chars().count());
                }
            }
        }

        let mut out = String::new();
        let format_row = |row: &[String]| -> String {
            let mut line = String::new();
            for (i, cell) in row.iter().enumerate() {
                let w = widths[i];
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
}
