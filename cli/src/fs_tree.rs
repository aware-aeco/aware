//! Recursive directory copy.
//!
//! Three places wanted "copy this tree there": `install::local` (installing an
//! agent or app from a path, and via `install::rename` / `install::registry`),
//! `commands::voice` (installing a voice pack), and a `plugins::claude_code`
//! test fixture. Each had typed its own.
//!
//! They agreed on the walk and differed at three edges: `voice`'s returned
//! [`AwareError`] instead of [`std::io::Result`], left creating the destination
//! root to its caller, and classified entries with `Path::is_dir` (which follows
//! symlinks) where `install`'s used `DirEntry::file_type` (which does not).
//!
//! The first two are not behaviour any caller needs kept, so the shared version
//! creates the destination (the `voice` caller already did, and `create_dir_all`
//! on an existing directory is a no-op) and reports [`std::io::Error`], which
//! `voice` maps at its one call site.
//!
//! The third one matters. Taking `install`'s rule would have made a voice pack
//! containing a symlink to a directory stop installing: the entry would take the
//! file branch and `std::fs::copy` would fail on a directory source. So the
//! `voice` rule is what survives — links are followed and their contents copied
//! — which is also what `install` would have wanted, since its rule turned such
//! a tree into an error rather than a copy. Following links means cycles are
//! reachable, so `visit` refuses one rather than recursing until the stack dies.

use std::io;
use std::path::{Path, PathBuf};

/// Copy `src` and everything under it into `dst`, creating `dst` and any
/// intermediate directories.
///
/// Symlinks are resolved, not reproduced: a link to a directory is walked into
/// and a link to a file has its contents copied. A symlink cycle is reported as
/// an error rather than followed forever.
///
/// Existing files at the destination are overwritten; existing directories are
/// merged into rather than replaced.
pub fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    visit(src, dst, &mut Vec::new())
}

/// `ancestors` holds the canonical path of every directory currently open on the
/// recursion stack, so a link back into one of them is caught on entry.
fn visit(src: &Path, dst: &Path, ancestors: &mut Vec<PathBuf>) -> io::Result<()> {
    let canonical = src.canonicalize()?;
    if ancestors.contains(&canonical) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("symlink cycle at {}", src.display()),
        ));
    }
    ancestors.push(canonical);

    std::fs::create_dir_all(dst)?;
    let result = (|| {
        for entry in std::fs::read_dir(src)?.flatten() {
            let from = entry.path();
            let to = dst.join(entry.file_name());
            // `is_dir` follows symlinks; `DirEntry::file_type` would not, and would
            // send a linked directory down the file branch to fail in `fs::copy`.
            if from.is_dir() {
                visit(&from, &to, ancestors)?;
            } else {
                std::fs::copy(&from, &to)?;
            }
        }
        Ok(())
    })();

    ancestors.pop();
    result
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

    /// The case Codex caught on #399: `commands::voice`'s former copier followed
    /// directory symlinks, so a voice pack containing one installed cleanly.
    /// Classifying with `DirEntry::file_type` instead would have sent it down the
    /// file branch and failed the install in `fs::copy`.
    #[cfg(unix)]
    #[test]
    fn a_symlinked_directory_is_followed_and_its_contents_copied() {
        let tmp = tempfile::tempdir().unwrap();
        let real = tmp.path().join("real");
        std::fs::create_dir_all(&real).unwrap();
        std::fs::write(real.join("inside.txt"), b"inside").unwrap();

        let src = tmp.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::os::unix::fs::symlink(&real, src.join("linked")).unwrap();

        let dst = tmp.path().join("dst");
        copy_dir_recursive(&src, &dst).unwrap();

        assert_eq!(
            std::fs::read(dst.join("linked/inside.txt")).unwrap(),
            b"inside"
        );
        // Resolved, not reproduced — the destination holds a real directory.
        assert!(!dst.join("linked").is_symlink());
    }

    #[cfg(unix)]
    #[test]
    fn a_symlinked_file_has_its_contents_copied() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("target.txt"), b"target").unwrap();

        let src = tmp.path().join("src");
        std::fs::create_dir_all(&src).unwrap();
        std::os::unix::fs::symlink(tmp.path().join("target.txt"), src.join("link.txt")).unwrap();

        let dst = tmp.path().join("dst");
        copy_dir_recursive(&src, &dst).unwrap();

        assert_eq!(std::fs::read(dst.join("link.txt")).unwrap(), b"target");
    }

    /// Following links makes cycles reachable, so they must come back as an error
    /// rather than recursing until the stack dies.
    #[cfg(unix)]
    #[test]
    fn a_symlink_cycle_is_an_error_not_a_stack_overflow() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("src");
        std::fs::create_dir_all(src.join("nested")).unwrap();
        std::os::unix::fs::symlink(&src, src.join("nested/loop")).unwrap();

        let error = copy_dir_recursive(&src, &tmp.path().join("dst"))
            .expect_err("a symlink cycle must not be followed forever");
        assert!(
            error.to_string().contains("symlink cycle"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn a_missing_source_is_an_error_not_a_silent_empty_copy() {
        let tmp = tempfile::tempdir().unwrap();
        let error = copy_dir_recursive(&tmp.path().join("absent"), &tmp.path().join("dst"))
            .expect_err("a missing source must not report success");
        assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
    }
}
