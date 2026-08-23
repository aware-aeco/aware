//! Subcommand modules.
//!
//! Each group (`agent`, `app`, `connect`, `skill`, `build`, `doctor`) lives
//! in its own module. The fresh-session implementer fills these in per
//! the phases in `10-core/cli-roadmap.md`.

pub mod agent;
pub mod app;
pub mod build;
pub mod connect;
pub mod coverage;
pub mod diagram;
pub mod doctor;
pub mod key;
pub mod model_reader_host;
pub mod plugins;
pub mod receipt_cli;
pub mod report;
pub mod search;
pub mod sidecar;
pub mod skill;
pub mod tree;
pub mod voice;

/// Extract the group bucket for a command from its `description`.
///
/// Heuristic: if the description starts with `Word.AnotherWord` (no spaces
/// between Word and `.`), that's the owning class — return the leading
/// `Word`. Otherwise return `Top-level`.
///
/// `commands::tree` and `commands::report` each carried a byte-identical
/// private copy, with a `TODO` on one of them asking for exactly this hoist.
/// The two are the *same* grouping — `aware tree` and `aware report substrate`
/// render one substrate, and a reader who saw a command under `Viewer` in the
/// tree and under `Top-level` in the report would be looking at a bug, not at
/// two features. One copy is what keeps them from drifting into that.
pub(crate) fn extract_group(description: &str) -> String {
    let trimmed = description.trim_start();
    let head: String = trimmed
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '$')
        .collect();
    if head.is_empty() {
        return "Top-level".into();
    }
    if let Some(rest) = trimmed.strip_prefix(&head)
        && rest.starts_with('.')
    {
        return head;
    }
    "Top-level".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_group_picks_leading_class_from_type_method() {
        assert_eq!(extract_group("Viewer.LoadModel"), "Viewer");
        assert_eq!(extract_group("Camera.GetFlight()"), "Camera");
        assert_eq!(extract_group("IFC4.Wall"), "IFC4");
    }

    #[test]
    fn extract_group_falls_back_to_top_level_for_prose() {
        assert_eq!(
            extract_group("Subscribe to ModelObjectChanged events on the active Tekla model."),
            "Top-level"
        );
        assert_eq!(
            extract_group("Adds the given assemblable instance to the assembly."),
            "Top-level"
        );
    }

    #[test]
    fn extract_group_handles_leading_whitespace() {
        assert_eq!(extract_group("  Viewer.LoadModel"), "Viewer");
    }

    #[test]
    fn extract_group_treats_word_then_space_as_prose() {
        // "Foo bar" is prose; "Foo.bar" is Type.Method
        assert_eq!(
            extract_group("Returns true if a property is overridden"),
            "Top-level"
        );
    }
}
