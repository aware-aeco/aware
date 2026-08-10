//! Resolving a bare command name to a spawnable executable on `PATH`.
//!
//! Two callers need this — `runtime::invoker` (to find `claude`/`codex`, and to
//! probe whether a vision provider is installed) and `render::blender` (to find
//! `blender`, or whatever `AWARE_BLENDER` names). Each had written its own
//! `find_on_path`, and the two had drifted in exactly the way that only bites on
//! Windows:
//!
//! * `invoker`'s tried `.exe`/`.cmd`/`.bat`/`.com`; `blender`'s omitted `.com`.
//! * `invoker`'s looked a name that *already* carried an extension up verbatim;
//!   `blender`'s appended a second one regardless, so an `AWARE_BLENDER=blender.exe`
//!   override went looking for `blender.exe.exe` and reported Blender missing.
//!
//! Neither copy was wholly right. `invoker`'s verbatim rule keyed on *any*
//! extension, and `Path::extension` splits at the last `.`, so the documented
//! `AWARE_BLENDER=blender-4.2` form parses as extension `"2"` and would have
//! stopped resolving to `blender-4.2.exe` — a regression `blender`'s
//! append-always copy did not have. The unified rule keys on the extension being
//! one Windows can actually spawn, and still falls back to the verbatim name, so
//! every lookup either copy resolved still resolves.

use std::path::{Path, PathBuf};

/// The extensions `Command::new` can actually launch on Windows.
const WINDOWS_EXEC_EXTS: [&str; 4] = ["exe", "cmd", "bat", "com"];

/// Whether `name` already ends in an extension Windows would spawn directly, and
/// so must be looked up verbatim rather than extended again.
///
/// This asks whether the extension is *executable*, not merely whether one is
/// present. `Path::extension` splits at the last `.`, so the documented
/// `AWARE_BLENDER=blender-4.2` override form has extension `"2"` — treating any
/// extension as final would search only for `blender-4.2` and miss the
/// `blender-4.2.exe` on `PATH`.
fn has_executable_extension(name: &str) -> bool {
    Path::new(name)
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| {
            WINDOWS_EXEC_EXTS
                .iter()
                .any(|known| ext.eq_ignore_ascii_case(known))
        })
}

/// Resolve `name` to a directly-spawnable executable on `PATH`.
///
/// On Windows this appends the `PATHEXT` variants `Command::new` can actually
/// launch (`.exe`/`.cmd`/`.bat`/`.com`) — npm ships `codex` only as `.cmd`/`.ps1`
/// shims, and the bare or `.ps1` forms are not spawnable by `CreateProcess`. On
/// Unix the bare name resolves.
pub fn find_on_path(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    let dirs: Vec<PathBuf> = std::env::split_paths(&path).collect();
    // If the caller already supplied a spawnable extension (e.g. "codex.cmd"), don't
    // append a second one — look the name up verbatim. Otherwise try the executable
    // suffixes first and fall back to the verbatim name, which keeps a dotted bare
    // name like "blender-4.2" resolving to "blender-4.2.exe".
    let exts: &[&str] = match () {
        _ if !cfg!(windows) || has_executable_extension(name) => &[""],
        _ => &[".exe", ".cmd", ".bat", ".com", ""],
    };
    find_in_dirs(name, &dirs, exts)
}

/// The lookup itself, with the search path and extensions passed in so it is
/// testable without touching the process environment.
pub fn find_in_dirs(name: &str, dirs: &[PathBuf], exts: &[&str]) -> Option<PathBuf> {
    for dir in dirs {
        for ext in exts {
            let cand = dir.join(format!("{name}{ext}"));
            if cand.is_file() {
                return Some(cand);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Moved here with the function, from `runtime::invoker`.
    #[test]
    fn find_in_dirs_resolves_by_extension_and_misses_cleanly() {
        let tmp = tempfile::tempdir().unwrap();
        let exts: &[&str] = if cfg!(windows) {
            &[".exe", ".cmd", ".bat", ".com"]
        } else {
            &[""]
        };
        let fname = if cfg!(windows) { "toolx.cmd" } else { "toolx" };
        std::fs::write(tmp.path().join(fname), b"shim").unwrap();
        let dirs = vec![tmp.path().to_path_buf()];
        assert!(find_in_dirs("toolx", &dirs, exts).is_some());
        assert!(find_in_dirs("absent", &dirs, exts).is_none());
    }

    #[test]
    fn find_in_dirs_takes_the_first_directory_that_has_it() {
        let tmp = tempfile::tempdir().unwrap();
        let (first, second) = (tmp.path().join("a"), tmp.path().join("b"));
        std::fs::create_dir_all(&first).unwrap();
        std::fs::create_dir_all(&second).unwrap();
        std::fs::write(first.join("tool"), "").unwrap();
        std::fs::write(second.join("tool"), "").unwrap();

        let dirs = vec![first.clone(), second];
        assert_eq!(find_in_dirs("tool", &dirs, &[""]), Some(first.join("tool")));
        assert_eq!(find_in_dirs("absent", &dirs, &[""]), None);
    }

    #[test]
    fn find_in_dirs_tries_each_extension_in_order() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("tool.com"), "").unwrap();
        let dirs = vec![tmp.path().to_path_buf()];

        // `.com` is the extension `render::blender`'s copy had lost.
        assert_eq!(
            find_in_dirs("tool", &dirs, &[".exe", ".cmd", ".bat", ".com"]),
            Some(tmp.path().join("tool.com"))
        );
        assert_eq!(find_in_dirs("tool", &dirs, &[".exe", ".cmd"]), None);
    }

    #[test]
    fn a_name_that_already_has_an_extension_is_looked_up_verbatim() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("blender.exe"), "").unwrap();

        // The bug in `render::blender`'s copy: it would have searched for
        // `blender.exe.exe` and found nothing.
        assert_eq!(
            find_in_dirs("blender.exe", &[tmp.path().to_path_buf()], &[""]),
            Some(tmp.path().join("blender.exe"))
        );
        assert_eq!(
            find_in_dirs("blender.exe", &[tmp.path().to_path_buf()], &[".exe"]),
            None
        );
    }

    /// The case Codex caught on #399: `Path::extension` splits at the last `.`,
    /// so a dotted bare command name is *not* an already-extended name.
    #[test]
    fn a_dotted_bare_name_is_not_treated_as_already_extended() {
        assert!(!has_executable_extension("blender-4.2"));
        assert!(!has_executable_extension("blender"));
        assert!(!has_executable_extension("codex.ps1"));
        assert!(has_executable_extension("codex.cmd"));
        assert!(has_executable_extension("blender.exe"));
        // Windows paths are case-insensitive about this.
        assert!(has_executable_extension("Blender.EXE"));
    }

    /// The Windows search order: executable suffixes first, verbatim last, so a
    /// dotted bare name still resolves to its `.exe` while nothing either former
    /// copy could find becomes unfindable.
    #[test]
    fn suffixes_are_tried_before_the_verbatim_name() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("blender-4.2.exe"), "").unwrap();
        std::fs::write(tmp.path().join("blender-4.2"), "").unwrap();
        let dirs = vec![tmp.path().to_path_buf()];

        assert_eq!(
            find_in_dirs("blender-4.2", &dirs, &[".exe", ".cmd", ".bat", ".com", ""]),
            Some(tmp.path().join("blender-4.2.exe"))
        );
    }

    #[test]
    fn a_directory_is_not_an_executable() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir(tmp.path().join("tool")).unwrap();
        assert_eq!(
            find_in_dirs("tool", &[tmp.path().to_path_buf()], &[""]),
            None
        );
    }
}
