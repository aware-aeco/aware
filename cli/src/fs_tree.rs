//! Recursive directory copy.
//!
//! Three places wanted "copy this tree there": `install::local` (installing an
//! agent or app from a path, and via `install::rename` / `install::registry`),
//! `commands::voice` (installing a voice pack), and a `plugins::claude_code`
//! test fixture. Each had typed its own.
//!
//! They agreed on the walk and differed only at the edges — `voice`'s returned
//! [`AwareError`] instead of [`std::io::Result`], and left creating the
//! destination root to its caller. Neither difference is a behaviour a caller
//! needs to keep, so this is one function: it creates the destination (the
//! `voice` caller already did, and `create_dir_all` on an existing directory is
//! a no-op) and reports [`std::io::Error`], which `voice` maps at its one call
//! site.

use std::path::Path;

/// Copy `src` and everything under it into `dst`, creating `dst` and any
/// intermediate directories.
///
/// Existing files at the destination are overwritten; existing directories are
/// merged into rather than replaced.
pub fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)?.flatten() {
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_recursive(&from, &to)?;
        } else {
            std::fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_nested_files_and_creates_the_destination_root() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("src");
        std::fs::create_dir_all(src.join("nested/deeper")).unwrap();
        std::fs::write(src.join("top.txt"), b"top").unwrap();
        std::fs::write(src.join("nested/deeper/leaf.txt"), b"leaf").unwrap();

        // `dst` does not exist yet — the copy creates it, which is the half
        // `commands::voice`'s former copy left to its caller.
        let dst = tmp.path().join("out/dst");
        copy_dir_recursive(&src, &dst).unwrap();

        assert_eq!(std::fs::read(dst.join("top.txt")).unwrap(), b"top");
        assert_eq!(
            std::fs::read(dst.join("nested/deeper/leaf.txt")).unwrap(),
            b"leaf"
        );
    }

    #[test]
    fn merges_into_an_existing_destination_and_overwrites_collisions() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("shared.txt"), b"new").unwrap();

        let dst = tmp.path().join("dst");
        std::fs::create_dir_all(&dst).unwrap();
        std::fs::write(dst.join("shared.txt"), b"old").unwrap();
        std::fs::write(dst.join("kept.txt"), b"kept").unwrap();

        copy_dir_recursive(&src, &dst).unwrap();

        assert_eq!(std::fs::read(dst.join("shared.txt")).unwrap(), b"new");
        assert_eq!(std::fs::read(dst.join("kept.txt")).unwrap(), b"kept");
    }

    #[test]
    fn a_missing_source_is_an_error_not_a_silent_empty_copy() {
        let tmp = tempfile::tempdir().unwrap();
        let error = copy_dir_recursive(&tmp.path().join("absent"), &tmp.path().join("dst"))
            .expect_err("a missing source must not report success");
        assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
    }
}
