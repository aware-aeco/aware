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
//! append-always copy did not have. The unified rule keys instead on the
//! extension being one Windows can actually *spawn*.
//!
//! The rule is about spawnability throughout, which is why there is no verbatim
//! fallback on Windows. Returning a name no `CreateProcess` can launch is not a
//! better answer than `None` — it is the same failure, moved somewhere the
//! caller cannot diagnose it.

use std::path::{Path, PathBuf};

/// The extensions `Command::new` can actually launch on Windows.
const WINDOWS_EXEC_EXTS: [&str; 4] = ["exe", "cmd", "bat", "com"];

/// The suffixes a bare name is searched under on Windows, in order. Named so the
/// tests exercise the list [`find_on_path`] really uses rather than a copy of it.
const WINDOWS_SEARCH_SUFFIXES: &[&str] = &[".exe", ".cmd", ".bat", ".com"];

/// The single "as written" suffix, used on Unix and for a name that already
/// carries a spawnable extension.
const VERBATIM: &[&str] = &[""];

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
    // On Unix, and for a name that already carries a spawnable extension (e.g.
    // "codex.cmd"), look the name up verbatim. Otherwise search only the executable
    // suffixes — which resolves a dotted bare name like "blender-4.2" to
    // "blender-4.2.exe". There is deliberately no verbatim fallback on Windows: the
    // search is directory-outer, so an extensionless shim in an early `PATH` entry
    // would win over the spawnable `.cmd` in a later one, and hand the caller a file
    // `CreateProcess` cannot launch.
    let exts = if !cfg!(windows) || has_executable_extension(name) {
        VERBATIM
    } else {
        WINDOWS_SEARCH_SUFFIXES
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

    /// A dotted bare name resolves to its `.exe` under the Windows suffix list.
    #[test]
    fn a_dotted_bare_name_resolves_through_the_windows_suffixes() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("blender-4.2.exe"), "").unwrap();
        let dirs = vec![tmp.path().to_path_buf()];

        assert_eq!(
            find_in_dirs("blender-4.2", &dirs, WINDOWS_SEARCH_SUFFIXES),
            Some(tmp.path().join("blender-4.2.exe"))
        );
    }

    /// The second thing Codex caught on #399. The search is directory-outer, so a
    /// verbatim `""` in the Windows suffix list would let an extensionless shim in
    /// an early `PATH` entry beat the spawnable `.cmd` in a later one — and hand
    /// the caller a file `CreateProcess` cannot launch. The Windows list carries no
    /// `""` for exactly that reason.
    #[test]
    fn an_extensionless_early_shim_does_not_shadow_a_later_spawnable_one() {
        let tmp = tempfile::tempdir().unwrap();
        let (first, second) = (tmp.path().join("a"), tmp.path().join("b"));
        std::fs::create_dir_all(&first).unwrap();
        std::fs::create_dir_all(&second).unwrap();
        std::fs::write(first.join("codex"), "").unwrap();
        std::fs::write(second.join("codex.cmd"), "").unwrap();

        let dirs = vec![first, second.clone()];
        assert_eq!(
            find_in_dirs("codex", &dirs, WINDOWS_SEARCH_SUFFIXES),
            Some(second.join("codex.cmd"))
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
