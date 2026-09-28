//! The shared body of a host-plugin generator whose format is not settled yet.
//!
//! `codex` and `opencode` were two files that differed in exactly three tokens:
//! the host's display name in the module doc, in the README heading and in the
//! README's first sentence. Everything else — the `aware-aeco` directory name,
//! the remove-then-recreate for idempotence, the issue link, the `Ok(0)` count,
//! and a pair of tests asserting the marker exists and is stable — was
//! duplicated verbatim, including the tests.
//!
//! The host name is the only thing that varies, so it is the only thing that is
//! a parameter. Substituting it into a template is what a substrate does with a
//! host: the placeholder body belongs to AWARE, and `Codex` / `OpenCode` are
//! data passed in by the caller that already knows which target directory it is
//! writing to. A second scaffolded host is now a call, not a file; and when one
//! of these formats does settle, its real generator replaces one call site
//! without leaving the other reading like an abandoned copy of it.

use std::path::Path;

use crate::error::AwareError;

/// Write the placeholder marker for `host` under `plugin_root`, returning the
/// number of agents exposed — always zero, because nothing is generated yet.
///
/// Idempotent: the `aware-aeco` directory is removed and rewritten, so a repeat
/// run leaves byte-identical content rather than merging into a stale tree.
pub fn generate(host: &str, plugin_root: &Path) -> Result<usize, AwareError> {
    let dir = plugin_root.join("aware-aeco");
    if dir.exists() {
        std::fs::remove_dir_all(&dir)?;
    }
    std::fs::create_dir_all(&dir)?;
    let readme = format!(
        "# AWARE Plugin for {host}\n\n\
         The {host} plugin format is not yet settled. This directory is a placeholder.\n\n\
         Open an issue at https://github.com/aware-aeco/aware to contribute the format.\n"
    );
    std::fs::write(dir.join("README.md"), readme)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Both hosts in one test: the point of the shared body is that neither can
    /// drift, and asserting on one alone would not notice if the other stopped
    /// going through here.
    #[test]
    fn writes_a_readme_marker_naming_the_host() {
        for host in ["Codex", "OpenCode"] {
            let tmp = tempfile::tempdir().unwrap();
            generate(host, tmp.path()).unwrap();
            let readme = std::fs::read_to_string(tmp.path().join("aware-aeco/README.md")).unwrap();
            assert!(
                readme.starts_with(&format!("# AWARE Plugin for {host}\n")),
                "the marker must name the host it was generated for: {readme}"
            );
            assert!(
                readme.contains(&format!("The {host} plugin format is not yet settled.")),
                "the body must name the host too, not just the heading: {readme}"
            );
        }
    }

    #[test]
    fn is_idempotent() {
        let tmp = tempfile::tempdir().unwrap();
        generate("Codex", tmp.path()).unwrap();
        let first = std::fs::read(tmp.path().join("aware-aeco/README.md")).unwrap();
        generate("Codex", tmp.path()).unwrap();
        let second = std::fs::read(tmp.path().join("aware-aeco/README.md")).unwrap();
        assert_eq!(first, second);
    }

    /// A rerun for a *different* host must not leave the previous host's marker
    /// behind. The remove-then-recreate is what guarantees it; without that
    /// step `create_dir_all` would succeed and `write` would overwrite only the
    /// one file it names.
    #[test]
    fn rewriting_for_another_host_replaces_the_marker() {
        let tmp = tempfile::tempdir().unwrap();
        generate("Codex", tmp.path()).unwrap();
        generate("OpenCode", tmp.path()).unwrap();
        let readme = std::fs::read_to_string(tmp.path().join("aware-aeco/README.md")).unwrap();
        assert!(readme.contains("OpenCode"), "{readme}");
        assert!(!readme.contains("Codex"), "{readme}");
    }
}
