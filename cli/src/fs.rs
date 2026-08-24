//! Small filesystem helpers shared across the crate.

use std::path::Path;

/// How deep [`copy_dir_recursive`] will descend before giving up.
///
/// This exists because the walk follows directory symlinks (see below), and a
/// pack containing a link back to one of its own ancestors would otherwise
/// recurse until the stack overflows — an abort, with no message a user could
/// act on. Every tree this helper actually copies (an agent folder, a voice
/// pack, a staged registry payload) is a handful of levels deep, so the cap is
/// far above anything legitimate and turns the pathological case into a plain
/// error naming the path.
const MAX_DEPTH: usize = 64;

/// Recursively copy every file under `src` into `dst`, creating `dst` and any
/// missing subdirectories along the way. Delegates to [`std::fs::copy`] for
/// each regular file, and skips entries whose `read_dir` metadata fails via
/// `.flatten()`.
///
/// **Symlinks are followed, not preserved** — in both directions, deliberately.
/// [`std::fs::copy`] already reads a symlinked *file* through to its target, so
/// treating a symlinked *directory* as an opaque non-directory (which
/// `DirEntry::file_type()` reports it as) is not "preserving" anything: it
/// hands a directory to `fs::copy`, which fails with `InvalidInput` — possibly
/// after the walk has already written part of the destination. This function
/// therefore tests `Path::is_dir()`, which resolves the link, so files and
/// directories behave consistently. Bounded by [`MAX_DEPTH`].
///
/// Non-atomic on failure: a mid-copy error leaves whatever was already written
/// in place. Callers that need permission bits, xattrs, or symlinks preserved
/// verbatim should reach for the platform tool instead.
///
/// Previously reinvented in `install::local`, `commands::voice`, and
/// `plugins::claude_code`'s tests — three walks with subtle behaviour drift.
/// One skipped the initial `create_dir_all(dst)` and relied on the caller
/// having created it; one returned `AwareError` and the others `io::Result`;
/// and two used `DirEntry::file_type()` where `commands::voice` used
/// `Path::is_dir()`, which is exactly the symlink difference above. The
/// `is_dir()` behaviour is the one kept, so no caller loses anything it had.
/// Consolidated here so the next symlink / permissions / atomicity question has
/// a single place to change; callers wanting a non-`io::Error` result wrap this
/// at the boundary.
///
/// The integration-test helper in `tests/common` cannot use this (it lives in a
/// separate crate root that cannot see internal modules) and stays a deliberate
/// shadow; that copy is small enough to leave alone until the CLI grows a `lib`
/// target worth having.
pub(crate) fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    copy_dir_at_depth(src, dst, 0)
}

fn copy_dir_at_depth(src: &Path, dst: &Path, depth: usize) -> std::io::Result<()> {
    if depth > MAX_DEPTH {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "copy_dir_recursive: exceeded {MAX_DEPTH} levels at {} — \
                 the source tree is nested past any legitimate depth, \
                 or a directory symlink points back into its own ancestry",
                src.display()
            ),
        ));
    }
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)?.flatten() {
        let from = entry.path();
        let to = dst.join(entry.file_name());
        // `Path::is_dir()` and not `entry.file_type()?.is_dir()`: the former
        // resolves a symlink to its target, so a symlinked directory is walked
        // rather than handed to `fs::copy` (which would fail on a directory).
        if from.is_dir() {
            copy_dir_at_depth(&from, &to, depth + 1)?;
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

        // `dst` does not exist — the helper must create it (a caller relied on
        // creating it beforehand and would otherwise regress).
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

    /// A symlinked directory is walked through, not handed to `fs::copy`.
    ///
    /// `commands::voice` used `Path::is_dir()` and so handled this; the two
    /// `DirEntry::file_type()` copies did not, and consolidating on the wrong
    /// one would have made `aware voice install` fail with `InvalidInput` on a
    /// pack containing such a link — after partially writing the destination.
    #[cfg(unix)]
    #[test]
    fn a_symlinked_directory_is_followed_and_its_contents_copied() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("src");
        let dst = tmp.path().join("dst");
        let real = tmp.path().join("real");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::create_dir_all(&real).unwrap();
        std::fs::write(real.join("inside.txt"), b"inside").unwrap();
        std::os::unix::fs::symlink(&real, src.join("linked")).unwrap();

        copy_dir_recursive(&src, &dst).unwrap();

        assert_eq!(
            std::fs::read(dst.join("linked/inside.txt")).unwrap(),
            b"inside",
            "a directory symlink must be walked through, not copied as a file"
        );
    }

    /// A symlinked file still copies its target's bytes — `fs::copy` reads
    /// through the link, so files and directories stay consistent.
    #[cfg(unix)]
    #[test]
    fn a_symlinked_file_copies_its_target_contents() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("src");
        let dst = tmp.path().join("dst");
        std::fs::create_dir_all(&src).unwrap();
        let target = tmp.path().join("target.txt");
        std::fs::write(&target, b"payload").unwrap();
        std::os::unix::fs::symlink(&target, src.join("link.txt")).unwrap();

        copy_dir_recursive(&src, &dst).unwrap();

        assert_eq!(std::fs::read(dst.join("link.txt")).unwrap(), b"payload");
    }

    /// Following directory symlinks makes an ancestor loop reachable. The
    /// depth cap must turn that into an error rather than a stack overflow.
    #[cfg(unix)]
    #[test]
    fn a_symlink_loop_errors_instead_of_overflowing_the_stack() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("src");
        let dst = tmp.path().join("dst");
        std::fs::create_dir_all(src.join("sub")).unwrap();
        std::fs::write(src.join("sub/f.txt"), b"f").unwrap();
        // `src/sub/loop` -> `src`, so the walk can descend forever.
        std::os::unix::fs::symlink(&src, src.join("sub/loop")).unwrap();

        let err = copy_dir_recursive(&src, &dst)
            .expect_err("a symlink loop must be reported, not recursed forever");
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
        assert!(
            err.to_string().contains("exceeded"),
            "the error should name the depth cap, got: {err}"
        );
    }
}
