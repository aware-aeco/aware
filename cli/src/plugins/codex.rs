//! Codex plugin generator — scaffold only.
//!
//! The Codex plugin format isn't settled. This generator writes a TODO marker
//! so users know the directory exists for the eventual integration.

#![allow(dead_code)]

use std::path::Path;

use crate::error::AwareError;
use crate::manifest::loader::DiscoveredAgent;

pub fn generate(_agents: &[DiscoveredAgent], plugin_root: &Path) -> Result<usize, AwareError> {
    let dir = plugin_root.join("aware-aeco");
    if dir.exists() {
        std::fs::remove_dir_all(&dir)?;
    }
    std::fs::create_dir_all(&dir)?;
    let readme = "# AWARE Plugin for Codex\n\n\
                  The Codex plugin format is not yet settled. This directory is a placeholder.\n\n\
                  Open an issue at https://github.com/aware-aeco/aware to contribute the format.\n";
    std::fs::write(dir.join("README.md"), readme)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_readme_marker() {
        let tmp = tempfile::tempdir().unwrap();
        generate(&[], tmp.path()).unwrap();
        assert!(tmp.path().join("aware-aeco/README.md").is_file());
    }

    #[test]
    fn idempotent() {
        let tmp = tempfile::tempdir().unwrap();
        generate(&[], tmp.path()).unwrap();
        let first = std::fs::read(tmp.path().join("aware-aeco/README.md")).unwrap();
        generate(&[], tmp.path()).unwrap();
        let second = std::fs::read(tmp.path().join("aware-aeco/README.md")).unwrap();
        assert_eq!(first, second);
    }
}
