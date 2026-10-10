//! Small filesystem helpers shared across the crate.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::error::AwareError;

/// Whether `metadata` describes a Windows reparse point — the NTFS indirection
/// that backs junctions, mount points and symlink surrogates.
///
/// `FileType::is_symlink()` alone does not answer this: it reports the *name*
/// symlinks Rust models, while a junction (`FILE_ATTRIBUTE_REPARSE_POINT` with
/// no symlink file type) crosses a directory boundary just as effectively and
/// would otherwise pass a `!is_symlink()` guard. Every caller that refuses to
/// follow an indirection therefore needs both checks, and `false` on non-Windows
/// keeps that pair spelled the same way on every platform rather than making
/// callers `cfg` around it.
///
/// Previously typed out once in `install::integrity` and once in
/// `runtime::google_mail`, already drifted: the copies tested the same bit but
/// one spelled it `windows_sys::…::FILE_ATTRIBUTE_REPARSE_POINT` and the other
/// the bare literal `0x400`, so the two guards read as unrelated rules that a
/// grep for the constant found only half of.
#[cfg(windows)]
pub(crate) fn is_reparse_point(metadata: &std::fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    metadata.file_attributes()
        & windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_REPARSE_POINT
        != 0
}

/// Non-Windows filesystems have no reparse points; `is_symlink()` is the whole
/// story there, and the callers' paired check collapses to it.
#[cfg(not(windows))]
pub(crate) fn is_reparse_point(_metadata: &std::fs::Metadata) -> bool {
    false
}

/// Whether `metadata` describes a plain file — a regular file reached without
/// crossing any name indirection.
///
/// The pair `is_file() && !is_symlink() && !is_reparse_point()` was written out
/// at six call sites across `provider_store`, `install::integrity` and
/// `runtime::google_mail`, in three different orders and two different lengths:
/// `runtime::google_mail` omitted the `is_symlink()` clause, which is harmless
/// only because [`std::fs::symlink_metadata`] reports a symlink as neither file
/// nor directory — so the omission is invisible until someone passes
/// [`std::fs::metadata`], which follows the link and answers `is_file()` for its
/// target. Spelling the rule once removes the chance to write five sixths of it.
pub(crate) fn is_plain_file(metadata: &std::fs::Metadata) -> bool {
    metadata.is_file() && !metadata.file_type().is_symlink() && !is_reparse_point(metadata)
}

/// Whether `metadata` describes a plain directory — see [`is_plain_file`].
///
/// A Windows junction is the case that makes the reparse check load-bearing
/// here rather than redundant: it answers `is_dir()` *and* carries the reparse
/// bit, so a guard that tested only `is_dir() && !is_symlink()` would walk
/// straight through it.
pub(crate) fn is_plain_dir(metadata: &std::fs::Metadata) -> bool {
    metadata.is_dir() && !metadata.file_type().is_symlink() && !is_reparse_point(metadata)
}

/// Every plain file beneath `root`, keyed by its `/`-separated path relative to
/// `root` — refusing, rather than skipping, any entry that is not a plain file
/// or a plain directory.
///
/// `subject` names the thing being walked ("agent bundle", "provider package")
/// and appears in each refusal, because these walks are security guards whose
/// message is the only thing a user gets to act on.
///
/// `install::integrity::collect` and `provider_store::walk_regular_files` were
/// this function twice, and had already drifted in a way that matters:
///
/// * `install::integrity` rejected a non-UTF-8 component outright, while
///   `provider_store` ran the relative path through `to_string_lossy()`. A
///   filename that is not valid UTF-8 therefore became a string full of U+FFFD
///   and was *compared* against the package manifest's allowlist instead of
///   refused — so the closed-allowlist check it feeds silently compared the
///   wrong name. The rejecting behaviour is the one kept.
/// * `provider_store` spelled the separator fix as `replace('\\', "/")` on the
///   whole rendered path, which also rewrites a backslash that is part of a
///   filename on Unix. Joining the components, as `install::integrity` did, has
///   no such reach.
///
/// The ordering that a `BTreeMap` key gives is byte-wise over the relative path,
/// which is what `install::integrity`'s digest already re-sorted to, so a tree's
/// digest is unchanged by moving to this walk.
///
/// Callers that need a subset filter it out of the returned map; the walk itself
/// grows no exclusion parameter for one caller's benefit.
pub(crate) fn plain_files_under(
    root: &Path,
    subject: &str,
) -> Result<BTreeMap<String, PathBuf>, AwareError> {
    Ok(plain_files_with_metadata_under(root, subject)?
        .into_iter()
        .map(|(relative, (path, _))| (relative, path))
        .collect())
}

/// [`plain_files_under`], also returning each file's metadata from the same
/// stat the walk already made — so a caller that wants size and mtime does not
/// stat every file a second time.
pub(crate) fn plain_files_with_metadata_under(
    root: &Path,
    subject: &str,
) -> Result<BTreeMap<String, (PathBuf, std::fs::Metadata)>, AwareError> {
    let mut files = BTreeMap::new();
    collect_plain_files(root, root, subject, &mut files)?;
    Ok(files)
}

fn collect_plain_files(
    root: &Path,
    dir: &Path,
    subject: &str,
    out: &mut BTreeMap<String, (PathBuf, std::fs::Metadata)>,
) -> Result<(), AwareError> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        let metadata = std::fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() || is_reparse_point(&metadata) {
            return Err(AwareError::Validation(format!(
                "{subject} contains symlink/reparse indirection: {}",
                path.display()
            )));
        }
        if is_plain_dir(&metadata) {
            collect_plain_files(root, &path, subject, out)?;
        } else if is_plain_file(&metadata) {
            let relative = path.strip_prefix(root).map_err(|_| {
                AwareError::Validation(format!(
                    "{subject} path escaped its root: {}",
                    path.display()
                ))
            })?;
            let normalized = relative
                .components()
                .map(|component| component.as_os_str().to_str())
                .collect::<Option<Vec<_>>>()
                .ok_or_else(|| {
                    AwareError::Validation(format!(
                        "{subject} path is not UTF-8: {}",
                        path.display()
                    ))
                })?
                .join("/");
            out.insert(normalized, (path, metadata));
        } else {
            return Err(AwareError::Validation(format!(
                "{subject} contains a non-regular entry: {}",
                path.display()
            )));
        }
    }
    Ok(())
}

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

/// `path` in the Win32 verbatim form (`\\?\C:\…`, `\\?\UNC\server\share\…`)
/// that raw `*W` file APIs need to reach past `MAX_PATH`.
///
/// `std::fs` adds this prefix itself before every call, so only code that hands
/// a path straight to a Win32 function needs it. The binary carries no
/// `longPathAware` manifest, so without the prefix such a call is capped at 260
/// UTF-16 units whatever `LongPathsEnabled` says — a deep `AWARE_HOME` then fails
/// there with `os error 3` while every `std::fs` call around it succeeds (#593).
///
/// A verbatim path is passed to the filesystem unparsed, so the input is first
/// made absolute and normalised (`/` to `\`, `.` and `..` resolved) by
/// [`std::path::absolute`]; a path that is already verbatim or a device path is
/// returned as it is.
#[cfg(windows)]
pub(crate) fn win32_verbatim(path: &Path) -> std::io::Result<PathBuf> {
    use std::ffi::OsString;
    use std::os::windows::ffi::{OsStrExt, OsStringExt};
    use std::path::{Component, Prefix};

    let absolute = std::path::absolute(path)?;
    let Some(Component::Prefix(prefix)) = absolute.components().next() else {
        return Ok(absolute);
    };
    // `C:\x` becomes `\\?\C:\x`; `\\server\share\x` becomes `\\?\UNC\server\share\x`,
    // i.e. the UNC form drops the first of its two leading separators.
    let (root, skip) = match prefix.kind() {
        Prefix::Disk(_) => (r"\\?\", 0),
        Prefix::UNC(..) => (r"\\?\UNC", 1),
        Prefix::Verbatim(_)
        | Prefix::VerbatimUNC(..)
        | Prefix::VerbatimDisk(_)
        | Prefix::DeviceNS(_) => return Ok(absolute),
    };
    let mut verbatim: Vec<u16> = root.encode_utf16().collect();
    verbatim.extend(absolute.as_os_str().encode_wide().skip(skip));
    Ok(PathBuf::from(OsString::from_wide(&verbatim)))
}

/// What [`replace_file`] achieved once it returned `Ok`: the new file IS in
/// place in every case — the only question left is whether that is durable.
#[derive(Debug)]
pub(crate) enum Replaced {
    /// In place and on disk.
    Durable,
    /// In place — readers already see the new bytes — but making the rename
    /// durable failed, so a power loss could still bring the old file back.
    /// Never "the write failed": the caller must not tell anyone the old file
    /// is untouched.
    NotDurable(std::io::Error),
}

/// The directory holding `path`, as something that can be opened: a bare file
/// name (`myapp.lock`, whose `parent()` is `""`) lives in the current
/// directory, never in the empty path.
pub(crate) fn containing_dir(path: &Path) -> &Path {
    match path.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent,
        _ => Path::new("."),
    }
}

/// Atomically replace `destination` with the fully written, fsynced file
/// `source` in the SAME directory: a reader sees the old bytes or the new ones,
/// never a mix, and a crash leaves one of the two.
///
/// `Err` means the replace did NOT happen and `destination` is untouched.
/// `Ok` means it did; [`Replaced::NotDurable`] reports a failure of the
/// durability step that follows (Unix: an fsync of the directory; Windows:
/// nothing — `MOVEFILE_WRITE_THROUGH` returns only once the move is on disk).
pub(crate) fn replace_file(source: &Path, destination: &Path) -> std::io::Result<Replaced> {
    rename_replacing(source, destination)?;
    Ok(match sync_after_replace(destination) {
        Ok(()) => Replaced::Durable,
        Err(error) => Replaced::NotDurable(error),
    })
}

#[cfg(test)]
thread_local! {
    static FAIL_POST_REPLACE_SYNC: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Make the next post-replace durability step on this thread fail.
#[cfg(test)]
pub(crate) fn inject_post_replace_sync_failure() {
    FAIL_POST_REPLACE_SYNC.with(|f| f.set(true));
}

fn sync_after_replace(destination: &Path) -> std::io::Result<()> {
    #[cfg(test)]
    if FAIL_POST_REPLACE_SYNC.with(|f| f.replace(false)) {
        return Err(std::io::Error::other("injected post-replace sync failure"));
    }
    #[cfg(unix)]
    {
        let dir = containing_dir(destination);
        std::fs::File::open(dir)
            .and_then(|handle| handle.sync_all())
            .map_err(|e| std::io::Error::new(e.kind(), format!("fsync {}: {e}", dir.display())))
    }
    #[cfg(not(unix))]
    {
        let _ = destination;
        Ok(())
    }
}

#[cfg(not(windows))]
fn rename_replacing(source: &Path, destination: &Path) -> std::io::Result<()> {
    std::fs::rename(source, destination)
}

#[cfg(windows)]
fn rename_replacing(source: &Path, destination: &Path) -> std::io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{
        MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW,
    };
    // `std::fs` adds the verbatim prefix itself; this raw call must, or a deep
    // path fails here with `os error 3` past MAX_PATH (#593).
    let source = win32_verbatim(source)?;
    let destination = win32_verbatim(destination)?;
    let source_wide = source
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let destination_wide = destination
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    // A reader holding `destination` open (`aware app run` reading the lock
    // it is replacing) fails the replace transiently: retried, bounded.
    retry_transient(|| {
        // SAFETY: both pointers name live, NUL-terminated UTF-16 buffers for the duration of the call.
        let moved = unsafe {
            MoveFileExW(
                source_wide.as_ptr(),
                destination_wide.as_ptr(),
                MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
            )
        };
        if moved == 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    })
}

/// The filesystem identity of the entry at `path` (the entry itself, not a
/// link target): Unix `st_dev`/`st_ino`, Windows volume serial number + file
/// index. Two names have the same identity exactly when the filesystem
/// resolves both to one entry — e.g. `agents/Alpha` and `agents/alpha` on a
/// case-insensitive volume, and never on a case-sensitive one.
#[cfg(unix)]
pub(crate) fn entry_identity(path: &Path) -> std::io::Result<(u64, u64, u64)> {
    use std::os::unix::fs::MetadataExt;
    let meta = std::fs::symlink_metadata(path)?;
    Ok((meta.dev(), meta.ino(), 0))
}

#[cfg(windows)]
pub(crate) fn entry_identity(path: &Path) -> std::io::Result<(u64, u64, u64)> {
    use std::os::windows::fs::OpenOptionsExt;
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Storage::FileSystem::{
        BY_HANDLE_FILE_INFORMATION, FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT,
        GetFileInformationByHandle,
    };
    // No access rights beyond attributes: a directory opens with backup
    // semantics, the entry itself (not a junction's target) with
    // OPEN_REPARSE_POINT, and other handles are not disturbed.
    let file = retry_transient(|| {
        std::fs::OpenOptions::new()
            .access_mode(0)
            .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT)
            .open(path)
    })?;
    let mut info = std::mem::MaybeUninit::<BY_HANDLE_FILE_INFORMATION>::uninit();
    // SAFETY: the raw handle belongs to the open File and stays live through
    // the call; the OS fills the output structure only on success.
    let ok = unsafe { GetFileInformationByHandle(file.as_raw_handle(), info.as_mut_ptr()) };
    if ok == 0 {
        return Err(std::io::Error::last_os_error());
    }
    // SAFETY: GetFileInformationByHandle returned success and initialized info.
    let info = unsafe { info.assume_init() };
    Ok((
        u64::from(info.dwVolumeSerialNumber),
        u64::from(info.nFileIndexHigh),
        u64::from(info.nFileIndexLow),
    ))
}

/// Run `op`, retrying for up to two seconds while Windows reports the target
/// as transiently held: `ERROR_ACCESS_DENIED` (5, also what a file in a
/// delete-pending state or a directory with an open child returns) or
/// `ERROR_SHARING_VIOLATION` (32). Another process (or thread) reading the
/// file, a concurrent delete finishing, an indexer or a virus scanner all
/// clear within milliseconds; a real permission fault is still returned, just
/// two seconds later. Unix returns the first result.
pub(crate) fn retry_transient<T>(mut op: impl FnMut() -> std::io::Result<T>) -> std::io::Result<T> {
    let mut attempts = 0;
    loop {
        match op() {
            Err(error)
                if cfg!(windows)
                    && matches!(error.raw_os_error(), Some(5 | 32))
                    && attempts < 100 =>
            {
                attempts += 1;
                std::thread::sleep(std::time::Duration::from_millis(20));
            }
            result => return result,
        }
    }
}

/// Rename the directory `source` to `destination`, failing — never replacing —
/// when `destination` already exists. Used to publish a store package (#626)
/// and for every move of an agent swap (#627). Both paths must be on the same
/// volume (callers keep them under one parent tree).
///
/// Windows: `MoveFileExW(MOVEFILE_WRITE_THROUGH)` without
/// `MOVEFILE_REPLACE_EXISTING`, so an existing name is refused by the OS and the
/// move is on disk when the call returns. Unix: `rename(2)` would replace an
/// EMPTY destination directory, so the name is checked first; callers hold the
/// lock that makes that check meaningful (the swap lock), or tolerate the race
/// (two snapshots of identical bytes). Follow with [`sync_dir`] on both parents
/// for durability on Unix.
#[cfg(not(windows))]
pub(crate) fn rename_dir_no_replace(source: &Path, destination: &Path) -> std::io::Result<()> {
    match std::fs::symlink_metadata(destination) {
        Ok(_) => Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("{} already exists", destination.display()),
        )),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            std::fs::rename(source, destination)
        }
        Err(error) => Err(error),
    }
}

#[cfg(windows)]
pub(crate) fn rename_dir_no_replace(source: &Path, destination: &Path) -> std::io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{MOVEFILE_WRITE_THROUGH, MoveFileExW};
    // Same verbatim-prefix requirement as `rename_replacing` (#593).
    let source = win32_verbatim(source)?;
    let destination = win32_verbatim(destination)?;
    let source_wide = source
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let destination_wide = destination
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    // No MOVEFILE_REPLACE_EXISTING: an existing destination must never be replaced.
    // A file open inside `source` (an unlocked reader, a scanner) makes the
    // move fail transiently: retried, bounded.
    retry_transient(|| {
        // SAFETY: both pointers name live, NUL-terminated UTF-16 buffers for the duration of the call.
        let moved = unsafe {
            MoveFileExW(
                source_wide.as_ptr(),
                destination_wide.as_ptr(),
                MOVEFILE_WRITE_THROUGH,
            )
        };
        if moved == 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    })
}

/// Make a directory's entries durable (Unix: fsync the directory). A no-op on
/// Windows, where directory handles cannot be flushed portably and the
/// write-through moves above carry durability instead.
pub(crate) fn sync_dir(dir: &Path) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        std::fs::File::open(dir)
            .and_then(|handle| handle.sync_all())
            .map_err(|e| std::io::Error::new(e.kind(), format!("fsync {}: {e}", dir.display())))
    }
    #[cfg(not(unix))]
    {
        let _ = dir;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rename_dir_no_replace_moves_and_never_replaces() {
        let tmp = tempfile::tempdir().unwrap();
        let a = tmp.path().join("a");
        let b = tmp.path().join("b");
        std::fs::create_dir(&a).unwrap();
        std::fs::write(a.join("f"), "one").unwrap();
        rename_dir_no_replace(&a, &b).unwrap();
        assert!(!a.exists());
        assert_eq!(std::fs::read_to_string(b.join("f")).unwrap(), "one");
        // An existing destination — even an EMPTY one, which rename(2) would
        // replace — is refused and both sides are left as they were.
        std::fs::create_dir(&a).unwrap();
        std::fs::write(a.join("f"), "two").unwrap();
        assert!(rename_dir_no_replace(&a, &b).is_err());
        let empty = tmp.path().join("empty");
        std::fs::create_dir(&empty).unwrap();
        assert!(rename_dir_no_replace(&a, &empty).is_err());
        assert_eq!(std::fs::read_to_string(a.join("f")).unwrap(), "two");
        assert_eq!(std::fs::read_to_string(b.join("f")).unwrap(), "one");
    }

    /// Review #628-3: a bare file name has parent `""`; the directory to sync
    /// is the current one, never the empty path.
    #[test]
    fn the_containing_directory_of_a_bare_file_name_is_the_current_one() {
        assert_eq!(containing_dir(Path::new("myapp.lock")), Path::new("."));
        assert_eq!(containing_dir(Path::new("")), Path::new("."));
        assert_eq!(
            containing_dir(Path::new("apps/x/x.lock")),
            Path::new("apps/x")
        );
    }

    #[test]
    fn a_failed_post_replace_sync_still_reports_the_file_as_replaced() {
        let tmp = tempfile::tempdir().unwrap();
        let (staged, live) = (tmp.path().join(".staged"), tmp.path().join("live"));
        std::fs::write(&live, b"old").unwrap();
        std::fs::write(&staged, b"new").unwrap();
        inject_post_replace_sync_failure();
        match replace_file(&staged, &live).unwrap() {
            Replaced::NotDurable(error) => {
                assert!(error.to_string().contains("injected"), "{error}")
            }
            Replaced::Durable => panic!("the injected sync failure was swallowed"),
        }
        assert_eq!(std::fs::read(&live).unwrap(), b"new");
    }

    #[test]
    fn replace_file_swaps_in_the_new_bytes_and_consumes_the_source() {
        let tmp = tempfile::tempdir().unwrap();
        let (staged, live) = (tmp.path().join(".staged"), tmp.path().join("live"));
        std::fs::write(&live, b"old").unwrap();
        std::fs::write(&staged, b"new").unwrap();
        assert!(matches!(
            replace_file(&staged, &live).unwrap(),
            Replaced::Durable
        ));
        assert_eq!(std::fs::read(&live).unwrap(), b"new");
        assert!(!staged.exists());
        // A missing destination is simply created.
        let fresh = tmp.path().join("fresh");
        std::fs::write(&staged, b"first").unwrap();
        replace_file(&staged, &fresh).unwrap();
        assert_eq!(std::fs::read(&fresh).unwrap(), b"first");
    }

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

    /// The walk's whole output contract in one assertion: it descends, it keys
    /// by the `/`-separated relative path on every platform, and it carries the
    /// real path so a caller can reopen the file without rebuilding it.
    #[test]
    fn the_walk_descends_and_keys_by_slash_separated_relative_paths() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join("bin").join("deep")).unwrap();
        std::fs::write(root.join("top"), b"t").unwrap();
        std::fs::write(root.join("bin").join("run"), b"r").unwrap();
        std::fs::write(root.join("bin").join("deep").join("lib.so"), b"l").unwrap();

        let files = plain_files_under(root, "subject").unwrap();

        assert_eq!(
            files.keys().cloned().collect::<Vec<_>>(),
            vec![
                "bin/deep/lib.so".to_string(),
                "bin/run".to_string(),
                "top".to_string(),
            ],
            "keys must be `/`-separated relative paths in byte order"
        );
        assert_eq!(
            files["bin/deep/lib.so"],
            root.join("bin").join("deep").join("lib.so")
        );
    }

    /// Every refusal names the subject it was given, because the message is the
    /// only thing a user of a security guard gets to act on. A shared walk that
    /// dropped the subject would report "contains symlink/reparse indirection"
    /// with no clue whether an agent bundle or a provider package was at fault.
    #[cfg(unix)]
    #[test]
    fn a_link_anywhere_beneath_the_root_is_refused_and_names_the_subject() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir(root.join("bin")).unwrap();
        std::os::unix::fs::symlink("/etc/passwd", root.join("bin").join("secrets")).unwrap();

        match plain_files_under(root, "provider package") {
            Err(crate::error::AwareError::Validation(message)) => {
                assert!(
                    message.contains("provider package contains symlink/reparse indirection"),
                    "the link guard, not the non-regular fallback, must reject this: {message}"
                );
                assert!(message.contains("secrets"), "{message}");
            }
            other => panic!("expected the link guard to refuse the entry, got {other:?}"),
        }
    }

    /// A FIFO is neither a plain file nor a plain directory, and is refused
    /// rather than skipped: these walks feed closed-allowlist comparisons and a
    /// digest, both of which are wrong if an entry is quietly omitted.
    #[cfg(unix)]
    #[test]
    fn a_non_regular_entry_is_refused_rather_than_skipped() {
        use std::os::unix::fs::FileTypeExt;

        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        let fifo = root.join("pipe");
        let status = std::process::Command::new("mkfifo")
            .arg(&fifo)
            .status()
            .expect("mkfifo");
        assert!(status.success(), "mkfifo failed");
        assert!(
            std::fs::symlink_metadata(&fifo)
                .unwrap()
                .file_type()
                .is_fifo()
        );

        match plain_files_under(root, "agent bundle") {
            Err(crate::error::AwareError::Validation(message)) => assert!(
                message.contains("agent bundle contains a non-regular entry"),
                "{message}"
            ),
            other => panic!("expected the non-regular guard to refuse the FIFO, got {other:?}"),
        }
    }

    /// The drift this walk exists to end: `provider_store` ran the relative path
    /// through `to_string_lossy()`, so a name that is not valid UTF-8 became a
    /// string of U+FFFD and was *compared* against a manifest allowlist instead
    /// of refused. Refusing is the behaviour kept, and this is what holds it.
    #[cfg(unix)]
    #[test]
    fn a_non_utf8_filename_is_refused_rather_than_lossily_renamed() {
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;

        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        // 0x80 is a continuation byte with no lead byte: valid in a POSIX
        // filename, never valid UTF-8.
        let name = OsStr::from_bytes(b"bad\x80name");
        std::fs::write(root.join(name), b"x").unwrap();

        match plain_files_under(root, "provider package") {
            Err(crate::error::AwareError::Validation(message)) => assert!(
                message.contains("provider package path is not UTF-8"),
                "{message}"
            ),
            other => panic!("expected the UTF-8 guard to refuse the entry, got {other:?}"),
        }
    }

    /// `is_plain_dir` must not accept a directory reached through a link. On
    /// Unix `symlink_metadata` already reports the link itself, so the clause
    /// that carries this is `!is_symlink()`; on Windows a junction is what makes
    /// the reparse half load-bearing, which no test here can reach.
    #[cfg(unix)]
    #[test]
    fn a_symlink_is_neither_a_plain_file_nor_a_plain_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("real");
        std::fs::create_dir(&target).unwrap();
        let link = tmp.path().join("link");
        std::os::unix::fs::symlink(&target, &link).unwrap();

        let through_link = std::fs::symlink_metadata(&link).unwrap();
        assert!(!is_plain_dir(&through_link));
        assert!(!is_plain_file(&through_link));

        let direct = std::fs::symlink_metadata(&target).unwrap();
        assert!(is_plain_dir(&direct));
        assert!(!is_plain_file(&direct));
    }

    #[cfg(windows)]
    #[test]
    fn win32_verbatim_prefixes_disk_and_unc_paths_and_normalises_them() {
        let cases = [
            (r"C:\a\b.json", r"\\?\C:\a\b.json"),
            ("C:/a/./x/../b.json", r"\\?\C:\a\b.json"),
            (r"\\server\share\a\b", r"\\?\UNC\server\share\a\b"),
            (r"\\?\C:\already\verbatim", r"\\?\C:\already\verbatim"),
            (r"\\?\UNC\server\share\x", r"\\?\UNC\server\share\x"),
            (r"\\.\pipe\name", r"\\.\pipe\name"),
        ];
        for (input, expected) in cases {
            assert_eq!(
                win32_verbatim(Path::new(input)).unwrap(),
                PathBuf::from(expected),
                "{input}"
            );
        }
        let relative = win32_verbatim(Path::new("rel")).unwrap();
        let expected = std::env::current_dir().unwrap().join("rel");
        assert_eq!(relative, win32_verbatim(&expected).unwrap());
        assert!(relative.as_os_str().to_string_lossy().starts_with(r"\\?\"));
    }
}
