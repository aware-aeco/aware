//! Small filesystem helpers shared across the crate.

use std::path::Path;

/// Recursively copy every file under `src` into `dst`, creating `dst` and any
/// missing subdirectories along the way. Delegates to [`std::fs::copy`] for
/// each regular file (so symlinks are read through, not preserved), and skips
/// entries whose `read_dir` metadata fails via `.flatten()`.
///
/// Non-atomic on failure: a mid-copy error leaves whatever was already written
/// in place. Callers that need permission bits, xattrs, or symlinks preserved
/// verbatim should reach for the platform tool instead.
///
/// Previously reinvented in `install::local`, `commands::voice`, and
/// `plugins::claude_code`'s tests — three walks with subtle behaviour drift
/// (one skipped the initial `create_dir_all(dst)` and relied on the caller
/// having created it, one returned `AwareError`, one returned `io::Result`).
/// Consolidated here so the next symlink / permissions / atomicity question
/// has a single place to change. Callers wanting a non-`io::Error` result
/// wrap this call at the boundary — cheaper than four almost-identical
/// implementations diverging further.
///
/// The integration-test helper in `tests/common` cannot use this (it lives
/// in a separate crate root that cannot see internal modules) and stays a
/// deliberate shadow; that copy is small enough to leave alone until the CLI
/// grows a `lib` target worth having.
pub(crate) fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
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
    fn copies_a_nested_tree_and_creates_the_destination() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("src");
        let dst = tmp.path().join("dst");
        std::fs::create_dir_all(src.join("a/b")).unwrap();
        std::fs::write(src.join("top.txt"), b"top").unwrap();
        std::fs::write(src.join("a/mid.txt"), b"mid").unwrap();
        std::fs::write(src.join("a/b/leaf.txt"), b"leaf").unwrap();

        // `dst` does not exist — the helper must create it (a caller had a
        // bug because of this).
        copy_dir_recursive(&src, &dst).unwrap();

        assert_eq!(std::fs::read(dst.join("top.txt")).unwrap(), b"top");
        assert_eq!(std::fs::read(dst.join("a/mid.txt")).unwrap(), b"mid");
        assert_eq!(std::fs::read(dst.join("a/b/leaf.txt")).unwrap(), b"leaf");
    }

    #[test]
    fn copying_into_an_existing_destination_is_idempotent() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("src");
        let dst = tmp.path().join("dst");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::create_dir_all(&dst).unwrap();
        std::fs::write(src.join("f"), b"one").unwrap();

        copy_dir_recursive(&src, &dst).unwrap();
        copy_dir_recursive(&src, &dst).unwrap();

        assert_eq!(std::fs::read(dst.join("f")).unwrap(), b"one");
    }
}
