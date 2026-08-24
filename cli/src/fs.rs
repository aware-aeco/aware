//! Small filesystem helpers shared across the crate.

use std::path::{Path, PathBuf};

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
/// directories behave consistently.
///
/// Following directory symlinks makes a cycle reachable (a link pointing back
/// into its own ancestry), which would otherwise recurse until the stack
/// overflowed — an abort, with nothing a user could act on. Guarded by
/// canonicalizing each directory and refusing to descend into one already on
/// the active ancestry path. Depth itself is *not* capped: an ordinary deep
/// tree copies fine, which is what the implementations this replaces did.
///
/// A second guard covers the case the ancestry check structurally cannot: a
/// followed symlink whose target is `dst` — or anything beneath it — such as an
/// installed app being renamed via `link -> ../new-name`. That is not a cycle in
/// the source; it is the destination copied into itself, growing `dst/link/link/…`
/// without bound because each level is a *new* canonical path the ancestry set
/// never matches. `dst` is canonicalized once at entry and any directory that
/// resolves into it is refused before it is descended into.
///
/// Non-atomic on failure: a mid-copy error leaves whatever was already written
/// in place. That is pre-existing for every IO error this can hit (a full disk,
/// a permission denial) and is not specific to the cycle case; callers that
/// need an all-or-nothing install must stage and swap. Callers that need
/// permission bits, xattrs, or symlinks preserved verbatim should reach for the
/// platform tool instead.
///
/// Previously reinvented in `install::local`, `commands::voice`, and
/// `plugins::claude_code`'s tests — three walks with subtle behaviour drift.
/// One skipped the initial `create_dir_all(dst)` and relied on the caller
/// having created it; one returned `AwareError` and the others `io::Result`;
/// and two used `DirEntry::file_type()` where `commands::voice` used
/// `Path::is_dir()`, which is exactly the symlink difference above. The
/// `is_dir()` behaviour is the one kept, so no caller loses anything it had —
/// and `commands::voice`, the one caller that already followed directory
/// links, gains a cycle guard it never had. Consolidated here so the next
/// symlink / permissions / atomicity question has a single place to change;
/// callers wanting a non-`io::Error` result wrap this at the boundary.
///
/// The integration-test helper in `tests/common` cannot use this (it lives in a
/// separate crate root that cannot see internal modules) and stays a deliberate
/// shadow; that copy is small enough to leave alone until the CLI grows a `lib`
/// target worth having.
pub(crate) fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    // Create `dst` up front so it can be canonicalized: the walk must refuse to
    // descend into the destination tree, and comparing canonical paths needs the
    // real directory to exist first. `copy_dir_tracked` also calls
    // `create_dir_all(dst)` per level; re-creating this one is idempotent.
    std::fs::create_dir_all(dst)?;
    let dst_root = std::fs::canonicalize(dst)?;
    let mut ancestry = Vec::new();
    copy_dir_tracked(src, dst, &dst_root, &mut ancestry)
}

/// `ancestry` holds the canonical path of every directory currently open on the
/// recursion stack — the walk's active path, not every directory it has seen.
/// A sibling subtree reached twice by two different links is fine and copies
/// twice; only re-entering a directory that is still open above us is a cycle.
fn copy_dir_tracked(
    src: &Path,
    dst: &Path,
    dst_root: &Path,
    ancestry: &mut Vec<PathBuf>,
) -> std::io::Result<()> {
    // `canonicalize` resolves every link in the path, so two names for the same
    // directory compare equal — which is the whole point of the check.
    let identity = std::fs::canonicalize(src)?;

    // Containment check on the destination, orthogonal to the ancestry check on
    // the source below. A followed symlink resolving to `dst` — or anything
    // inside it — must not be descended into: copying the destination into
    // itself grows without bound, since every `dst/link/link/…` level is a fresh
    // canonical path the ancestry set never matches. Checked before this level's
    // `create_dir_all`, which is what would make such a link resolvable.
    if identity == *dst_root || identity.starts_with(dst_root) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "copy_dir_recursive: refusing to copy the destination into itself — \
                 {} resolves to {}, which is inside the copy target {}",
                src.display(),
                identity.display(),
                dst_root.display()
            ),
        ));
    }

    if ancestry.contains(&identity) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "copy_dir_recursive: directory symlink cycle at {} — \
                 it resolves to {}, which the copy is already inside",
                src.display(),
                identity.display()
            ),
        ));
    }
    ancestry.push(identity);

    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)?.flatten() {
        let from = entry.path();
        let to = dst.join(entry.file_name());
        // `Path::is_dir()` and not `entry.file_type()?.is_dir()`: the former
        // resolves a symlink to its target, so a symlinked directory is walked
        // rather than handed to `fs::copy` (which would fail on a directory).
        if from.is_dir() {
            copy_dir_tracked(&from, &to, dst_root, ancestry)?;
        } else {
            std::fs::copy(&from, &to)?;
        }
    }

    // Only on the success path; an error unwinds the whole call chain, so the
    // vector is dropped rather than reused.
    ancestry.pop();
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

    /// An ordinary acyclic tree copies at any depth. The cycle guard must key
    /// on directory identity, not on a depth counter: all three
    /// implementations this replaces copied arbitrarily deep trees, and a
    /// structural depth limit would reject a valid agent / app / voice pack
    /// (leaving a partial install behind, since the copy is not atomic).
    #[test]
    fn a_deep_acyclic_tree_copies_rather_than_tripping_the_cycle_guard() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("src");
        let dst = tmp.path().join("dst");

        // Comfortably past any depth cap a guard might have been tempted to use.
        let mut deep = src.clone();
        for i in 0..100 {
            deep = deep.join(format!("d{i}"));
        }
        std::fs::create_dir_all(&deep).unwrap();
        std::fs::write(deep.join("bottom.txt"), b"bottom").unwrap();

        copy_dir_recursive(&src, &dst).unwrap();

        let mut copied = dst.clone();
        for i in 0..100 {
            copied = copied.join(format!("d{i}"));
        }
        assert_eq!(
            std::fs::read(copied.join("bottom.txt")).unwrap(),
            b"bottom",
            "a 100-level acyclic tree must copy through"
        );
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

    /// Following directory symlinks makes an ancestry cycle reachable. It must
    /// become an error rather than a stack overflow.
    #[cfg(unix)]
    #[test]
    fn a_symlink_cycle_errors_instead_of_overflowing_the_stack() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("src");
        let dst = tmp.path().join("dst");
        std::fs::create_dir_all(src.join("sub")).unwrap();
        std::fs::write(src.join("sub/f.txt"), b"f").unwrap();
        // `src/sub/loop` -> `src`, so the walk can descend forever.
        std::os::unix::fs::symlink(&src, src.join("sub/loop")).unwrap();

        let err = copy_dir_recursive(&src, &dst)
            .expect_err("a symlink cycle must be reported, not recursed forever");
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
        assert!(
            err.to_string().contains("cycle"),
            "the error should name the cycle, got: {err}"
        );
    }

    /// A followed symlink in `src` that resolves to `dst` (or beneath it) must
    /// be refused, not copied into the destination endlessly. This is the
    /// rename/install shape `link -> ../new-name` where `new-name` is the copy
    /// target. The ancestry check structurally cannot catch it — every
    /// `dst/link/link/…` level is a fresh canonical path — so the destination
    /// containment guard is what stops the runaway growth (previously an
    /// `ENAMETOOLONG` abort leaving a partial install).
    #[cfg(unix)]
    #[test]
    fn a_source_link_into_the_destination_is_refused_not_copied_forever() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("src");
        let dst = tmp.path().join("dst");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("a.txt"), b"a").unwrap();
        // A link in SRC resolving to DST. `dst` need not exist yet — the copy
        // creates it, at which point the link resolves and the walk would
        // otherwise begin copying `dst` into `dst/link`.
        std::os::unix::fs::symlink(&dst, src.join("link")).unwrap();

        let err = copy_dir_recursive(&src, &dst)
            .expect_err("a source link resolving to the destination must be refused");
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
        assert!(
            err.to_string().contains("into itself"),
            "the error should explain the destination self-copy, got: {err}"
        );
    }

    /// The guard tracks the ACTIVE ancestry, not every directory seen. Two
    /// links to the same subtree from different branches are not a cycle and
    /// must both copy — tracking "visited" instead would wrongly reject this.
    #[cfg(unix)]
    #[test]
    fn the_same_directory_reached_twice_from_different_branches_is_not_a_cycle() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("src");
        let dst = tmp.path().join("dst");
        let shared = tmp.path().join("shared");
        std::fs::create_dir_all(src.join("one")).unwrap();
        std::fs::create_dir_all(src.join("two")).unwrap();
        std::fs::create_dir_all(&shared).unwrap();
        std::fs::write(shared.join("s.txt"), b"shared").unwrap();
        std::os::unix::fs::symlink(&shared, src.join("one/link")).unwrap();
        std::os::unix::fs::symlink(&shared, src.join("two/link")).unwrap();

        copy_dir_recursive(&src, &dst).unwrap();

        assert_eq!(
            std::fs::read(dst.join("one/link/s.txt")).unwrap(),
            b"shared"
        );
        assert_eq!(
            std::fs::read(dst.join("two/link/s.txt")).unwrap(),
            b"shared"
        );
    }
}
