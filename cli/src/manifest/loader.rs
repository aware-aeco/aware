//! Discovery and loading of installed agents and apps from `~/.aware/`.
//!
//! `discover_agents` / `load_agent` are consumed by Task 9 (agent list).
//! `discover_apps` / `load_app` / `find_app_manifest` are consumed by
//! Tasks 12 (app list/describe) and 13 (app run).

use std::path::{Path, PathBuf};

use crate::error::AwareError;
use crate::manifest::{Agent, App};
use crate::paths::Paths;

/// A discovered agent on disk, with its source path retained for `describe`/`skill` commands.
#[derive(Debug)]
pub struct DiscoveredAgent {
    pub manifest: Agent,
    pub root: PathBuf,
}

/// A discovered app on disk, with its source path retained.
#[derive(Debug)]
pub struct DiscoveredApp {
    pub manifest: App,
    pub root: PathBuf,
    /// Path to the app file this was loaded from. Retained but not read —
    /// callers that need it derive it from `root`. Kept so a discovered app
    /// can always say where it came from without re-deriving the convention.
    #[allow(dead_code)]
    pub manifest_path: PathBuf,
}

/// Walk `<aware_home>/agents/` one level deep. Each subdir containing a
/// `manifest.yaml` is an installed agent. Returns discovered agents sorted
/// by id. Missing `agents/` directory returns an empty Vec (not an error).
pub fn discover_agents(paths: &Paths) -> Result<Vec<DiscoveredAgent>, AwareError> {
    discover_agents_in(&paths.agents_dir())
}

/// [`discover_agents`] against an agents directory directly, for callers that
/// hold one without a [`Paths`] — the runtime invoker, which is constructed
/// with `agents_dir` alone. Same walk, so a catalogue read at dispatch and one
/// read at pre-flight can't drift apart.
pub fn discover_agents_in(agents_dir: &Path) -> Result<Vec<DiscoveredAgent>, AwareError> {
    if !agents_dir.exists() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for entry in std::fs::read_dir(agents_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let root = entry.path();
        let manifest_path = root.join("manifest.yaml");
        if !manifest_path.is_file() {
            continue;
        }
        let manifest = load_agent(&manifest_path)?;
        out.push(DiscoveredAgent { manifest, root });
    }
    out.sort_by(|a, b| a.manifest.agent.cmp(&b.manifest.agent));
    Ok(out)
}

/// Walk `<aware_home>/apps/` one level deep. Each subdir containing a
/// `.flo` or `.app` file is an installed app.
pub fn discover_apps(paths: &Paths) -> Result<Vec<DiscoveredApp>, AwareError> {
    let apps_dir = paths.apps_dir();
    if !apps_dir.exists() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&apps_dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let root = entry.path();
        let manifest_path = match find_app_manifest(&root) {
            Some(p) => p,
            None => continue,
        };
        let manifest = load_app(&manifest_path)?;
        out.push(DiscoveredApp {
            manifest,
            root,
            manifest_path,
        });
    }
    out.sort_by(|a, b| a.manifest.app.cmp(&b.manifest.app));
    Ok(out)
}

/// Resolve an installed app by id to its directory, for the by-id verbs
/// (`run`, `show`, `explain`, `export`). Resolution order — and the fix for the
/// #226 footgun where verbs disagreed on what "id" meant:
///
/// 1. A directory whose NAME is `id` (`apps/<id>/`) — the fast, canonical path
///    (dir name == `app:` field by convention), unchanged from before.
/// 2. Failing that, the app whose manifest `app:` field equals `id` — so an app
///    whose directory was renamed out from under its field stays addressable.
///    Resolving this way means dir-name and field disagree, so it warns (to
///    stderr, leaving `--json` stdout clean); `aware app rename` re-syncs them.
///
/// `NotFound` when neither matches. This is what makes `run`/`show`/`explain`/
/// `export` resolve an app consistently, instead of some keying on the directory
/// and others on the `app:` field.
pub fn resolve_app_dir(paths: &Paths, id: &str) -> Result<PathBuf, AwareError> {
    // An installed app id is always a plain directory segment; reject anything
    // with a path separator / `..` BEFORE joining it onto apps_dir, so a crafted
    // id can never resolve to a directory outside apps/ (defense-in-depth).
    if !is_safe_segment(id) {
        return Err(AwareError::NotFound(format!("app: {id}")));
    }
    let direct = paths.apps_dir().join(id);
    if direct.is_dir() {
        return Ok(direct);
    }
    if let Some(d) = discover_apps(paths)?
        .into_iter()
        .find(|d| d.manifest.app == id)
    {
        let dir = d
            .root
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("?")
            .to_string();
        eprintln!(
            "warning: app {id:?} lives in directory {dir:?} — its directory name and `app:` field disagree; re-sync with `aware app rename {dir} {id}`"
        );
        return Ok(d.root);
    }
    Err(AwareError::NotFound(format!("app: {id}")))
}

/// True when `id` is a plain path segment: `dir.join(id)` then names a direct
/// child of `dir` **lexically**. Used to fence an installed app or agent id
/// (always a plain slug) that reaches us from a file, before it is joined onto
/// `apps_dir`/`agents_dir`. The stricter charset/reserved-name check for a *new*
/// id lives in `install::rename`.
///
/// Asked of the platform rather than hand-rolled, because a hand-rolled version
/// got it wrong: rejecting `.`, `..` and the separators still let a Windows
/// **drive-relative** id like `C:evil` through, and `Path::join` discards the
/// whole base when the appended path carries a prefix — so `agents/` + `C:evil`
/// is `C:evil\manifest.yaml`, not under `agents/` at all (#349 review). Demanding
/// that `id` parse as exactly ONE `Component::Normal` covers every escape shape
/// the running platform can spell, in one question: prefixes (`C:evil`,
/// `\\host\share`), roots, `.`/`..`, and embedded separators.
///
/// Two limits, stated because the guard is easy to over-read:
///
/// - **Lexical, not physical.** A permitted id that names a *symlink* can still
///   resolve outside `dir`. Containment against symlinks is a different check
///   (canonicalise and compare), not this one.
/// - **Per platform, by design.** `\` is a separator on Windows and an ordinary
///   filename character on POSIX, so `a\b` is rejected on one and accepted on the
///   other. That is correct: the question is what *this* platform's `join` does.
///
/// It does NOT reject Windows reserved device names (`CON`, `NUL`, `COM1`) — they
/// name no directory but they do not escape either, so opening one simply fails
/// where it is used, which is the honest outcome for an id nothing installed.
pub(crate) fn is_safe_segment(id: &str) -> bool {
    if id.is_empty() || id.contains('\0') {
        return false;
    }
    let mut components = Path::new(id).components();
    let Some(std::path::Component::Normal(first)) = components.next() else {
        return false;
    };
    // The component must be the WHOLE id, not merely what it normalises to.
    // `components()` quietly drops a trailing separator and a trailing `/.`, so
    // `my-app/` and `my-app/.` both read as one `Normal` — and the lockfile write
    // would then land at `<source-dir>/my-app/.lock`, inside a subdirectory
    // rather than beside the source. No escape, but not what the author asked
    // for either, and an id is a name rather than a path however short.
    components.next().is_none() && first == id
}

pub(crate) fn find_app_manifest(root: &Path) -> Option<PathBuf> {
    // Preferred: <root>/<dir-name>.flo, then any *.flo, then any *.app.
    let dir_name = root.file_name()?.to_string_lossy().to_string();
    let canonical = root.join(format!("{dir_name}.flo"));
    if canonical.is_file() {
        return Some(canonical);
    }
    for ext in ["flo", "app"] {
        for entry in std::fs::read_dir(root).ok()?.flatten() {
            let p = entry.path();
            if p.extension().is_some_and(|e| e == ext) {
                return Some(p);
            }
        }
    }
    None
}

/// Read the manifest text, naming the file in BOTH failure modes.
///
/// The YAML branch below always named it; the read did not, so an unreadable
/// manifest surfaced as a bare `io: Access is denied. (os error 5)` naming
/// neither the file nor the agent — the operator could not tell which of dozens
/// of installed agents to look at. `AwareError::Io` carries no path, so the path
/// goes into the message and the error KIND is preserved, leaving the class and
/// exit code unchanged.
fn read_manifest(manifest_path: &Path) -> Result<String, AwareError> {
    std::fs::read_to_string(manifest_path).map_err(|e| {
        std::io::Error::new(e.kind(), format!("{}: {e}", manifest_path.display())).into()
    })
}

/// Load the manifest of an installed agent BY ID — the only sanctioned way to
/// turn an agent id into a manifest.
///
/// The id reaches most callers from a file (a node's `agent:`, a manifest's
/// `backed-by:`) and is joined onto `agents/` to find the manifest, so every one
/// of those joins needs the [`is_safe_segment`] fence or a path-shaped id reads a
/// `manifest.yaml` from anywhere on disk. There were seventeen such joins and the
/// fence was on none of them (#365), which is the argument for one function
/// rather than seventeen guards: a guard you have to remember is a guard you will
/// forget. `cli/tests/agent_id_joins_are_fenced.rs` fails the build if a new raw
/// join of that shape appears — it is a text scan, so it catches the shape
/// rather than the intent, and a caller determined to build the path another way
/// (`PathBuf::push`, `format!`) is beyond it.
///
/// "Not a plain segment" and "not installed" are the same answer to a caller, so
/// they share `NotFound` — no caller has to learn a new error.
pub fn load_agent_by_id(agents_dir: &Path, id: &str) -> Result<Agent, AwareError> {
    load_agent(&agent_manifest_path(agents_dir, id)?)
}

/// The fenced path of an installed agent's manifest, for the callers that need
/// the path rather than the parsed manifest (an existence check, an error
/// message). Same fence, same `NotFound`; splitting it out keeps those callers
/// from hand-rolling the join and losing the guard.
pub fn agent_manifest_path(agents_dir: &Path, id: &str) -> Result<PathBuf, AwareError> {
    if !is_safe_segment(id) {
        return Err(AwareError::NotFound(format!("agent {id} is not installed")));
    }
    Ok(agents_dir.join(id).join("manifest.yaml"))
}

pub fn load_agent(manifest_path: &Path) -> Result<Agent, AwareError> {
    let text = read_manifest(manifest_path)?;
    let parsed: Agent = serde_yaml::from_str(&text)
        .map_err(|e| AwareError::Validation(format!("{}: {e}", manifest_path.display())))?;
    Ok(parsed)
}

pub fn load_app(manifest_path: &Path) -> Result<App, AwareError> {
    let text = read_manifest(manifest_path)?;
    let parsed: App = serde_yaml::from_str(&text)
        .map_err(|e| AwareError::Validation(format!("{}: {e}", manifest_path.display())))?;
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixtures_paths() -> Paths {
        let tmp = tempfile::tempdir().unwrap();
        let aware = tmp.path().join("aware");
        std::fs::create_dir_all(aware.join("agents/tekla")).unwrap();
        std::fs::create_dir_all(aware.join("apps/welded-to-tc")).unwrap();

        let repo_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let src_manifest = repo_root.join("20-agents/aeco/engineering/tekla/manifest.yaml");
        std::fs::copy(&src_manifest, aware.join("agents/tekla/manifest.yaml")).unwrap();

        let src_flo = repo_root.join("30-apps/_examples/welded-to-tc.app");
        std::fs::copy(&src_flo, aware.join("apps/welded-to-tc/welded-to-tc.app")).unwrap();

        let aware = aware.clone();
        // Intentional leak: keeps the TempDir alive for the test process.
        // The OS cleans up at process exit. A Drop-based approach would
        // destroy the directory before the assertions run.
        std::mem::forget(tmp);
        Paths { aware_home: aware }
    }

    #[test]
    fn discovers_agents() {
        let paths = fixtures_paths();
        let agents = discover_agents(&paths).unwrap();
        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0].manifest.agent, "tekla");
    }

    #[test]
    fn discovers_apps() {
        let paths = fixtures_paths();
        let apps = discover_apps(&paths).unwrap();
        assert_eq!(apps.len(), 1);
        assert_eq!(apps[0].manifest.app, "welded-to-tc");
    }

    #[test]
    fn missing_agents_dir_returns_empty() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: tmp.path().join("nope"),
        };
        let agents = discover_agents(&paths).unwrap();
        assert!(agents.is_empty());
    }

    #[test]
    fn a_safe_segment_is_one_that_cannot_leave_its_directory() {
        for ok in [
            "probe-agent",
            "allplan-2024.0",
            "with space",
            "..evil",
            "CON",
        ] {
            assert!(is_safe_segment(ok), "{ok:?} names a plain segment");
        }
        // Rejected everywhere: `/` is a separator on both platforms.
        for bad in [
            "",
            ".",
            "..",
            "a/b",
            "/abs",
            "\0",
            "sub/../..",
            // A trailing separator normalises AWAY under `components()`, so
            // these read as one `Normal` and were accepted until the id was
            // compared against the component itself.
            "my-app/",
            "my-app/.",
        ] {
            assert!(!is_safe_segment(bad), "{bad:?} is not a plain segment");
        }
        // Windows-only, and the cfg is the point rather than a convenience: `\`
        // separates there and is an ordinary filename character on POSIX, so
        // `a\b` really IS a plain segment on Linux. A guard that answered the
        // same on both would be wrong on one of them.
        //
        // `C:evil` is the escape the first, hand-rolled guard missed: it carries
        // no separator, so a `.contains(['/', '\\'])` test passed it — and
        // `Path::join` then throws the base away.
        #[cfg(windows)]
        {
            for bad in [
                "a\\b",
                "\\abs",
                "C:evil",
                "C:",
                "c:x",
                "C:..",
                "C:\\evil",
                "\\\\host\\share",
            ] {
                assert!(
                    !is_safe_segment(bad),
                    "{bad:?} is not a plain segment on Windows"
                );
            }
            // Of those, the ones that genuinely leave `agents/` — asserted, so
            // the case cannot go vacuous if `join`'s behaviour ever changes.
            // (`a\b` is excluded on purpose: it stays inside, it simply is not
            // one segment.)
            for escapes in ["\\abs", "C:evil", "C:", "c:x", "C:..", "C:\\evil"] {
                assert!(
                    !std::path::Path::new("agents")
                        .join(escapes)
                        .starts_with("agents"),
                    "{escapes:?} was expected to escape"
                );
            }
        }
        // The mirror image, so the POSIX arm is asserted rather than merely
        // skipped: there, a backslash names a file and nothing escapes.
        #[cfg(not(windows))]
        for ok in ["a\\b", "C:evil", "C:"] {
            assert!(
                is_safe_segment(ok),
                "{ok:?} is an ordinary POSIX filename, not a traversal"
            );
        }
    }

    /// A syntactically complete agent manifest under a chosen `agent:` id.
    /// Hand-written rather than copied from `20-agents/`, because the tests
    /// below need the id and the directory name to DISAGREE, which no shipped
    /// manifest does.
    fn write_agent(dir: &Path, id: &str) {
        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(
            dir.join("manifest.yaml"),
            format!(
                "agent: {id}\nversion: 1.0.0\ndescription: a loader fixture\nstateful: false\n\
                 license: MIT\ntransport:\n  cli:\n    binary: aware-probe\n\
                 commands:\n  probe:\n    lifecycle: single\n    description: x\n    mode: read\n"
            ),
        )
        .unwrap();
    }

    /// A syntactically complete app source under a chosen `app:` id.
    fn write_app(path: &Path, id: &str) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(
            path,
            format!(
                "app: {id}\nversion: 0.0.1\ndescription: a loader fixture\n\
                 nodes: []\nconnections: []\nrequires: []\n"
            ),
        )
        .unwrap();
    }

    /// Discovery is ordered by the `agent:` FIELD, not by directory name. The
    /// two are the same for every real install, so the distinction only shows up
    /// on a desynced tree — which is exactly where `agent list` output must stay
    /// stable. Directory names here sort opposite to the fields they contain, so
    /// sorting on `root` would reverse the result rather than merely permute it.
    #[test]
    fn discovery_orders_agents_by_their_declared_id_not_their_directory() {
        let tmp = tempfile::tempdir().unwrap();
        let agents_dir = tmp.path().join("agents");
        write_agent(&agents_dir.join("zzz-directory"), "alpha-agent");
        write_agent(&agents_dir.join("aaa-directory"), "beta-agent");

        let found = discover_agents_in(&agents_dir).unwrap();
        let ids: Vec<&str> = found.iter().map(|d| d.manifest.agent.as_str()).collect();
        assert_eq!(ids, ["alpha-agent", "beta-agent"]);
    }

    /// Same question for apps, whose `app:` field desyncs from its directory
    /// often enough to have its own re-sync command (`aware app rename`).
    #[test]
    fn discovery_orders_apps_by_their_declared_id_not_their_directory() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: tmp.path().to_path_buf(),
        };
        write_app(&paths.apps_dir().join("zzz-directory/src.flo"), "alpha-app");
        write_app(&paths.apps_dir().join("aaa-directory/src.flo"), "beta-app");

        let found = discover_apps(&paths).unwrap();
        let ids: Vec<&str> = found.iter().map(|d| d.manifest.app.as_str()).collect();
        assert_eq!(ids, ["alpha-app", "beta-app"]);
    }

    /// A subdirectory of `agents/` with no `manifest.yaml` is not an agent — it
    /// is skipped, not read. Anything left beside an install (an unpacked
    /// tarball's staging dir, a `.git` checkout, a half-removed agent) lands
    /// here, and treating one as an agent turns every subsequent command into a
    /// hard parse error over a directory nobody asked about.
    #[test]
    fn discovery_skips_a_directory_that_holds_no_manifest() {
        let tmp = tempfile::tempdir().unwrap();
        let agents_dir = tmp.path().join("agents");
        write_agent(&agents_dir.join("real-agent"), "real-agent");
        std::fs::create_dir_all(agents_dir.join("not-an-agent/nested")).unwrap();
        std::fs::write(agents_dir.join("not-an-agent/README.md"), "hi").unwrap();

        let found = discover_agents_in(&agents_dir).unwrap();
        let ids: Vec<&str> = found.iter().map(|d| d.manifest.agent.as_str()).collect();
        assert_eq!(ids, ["real-agent"]);
    }

    /// `.flo` outranks `.app` when a directory holds both — the outer loop over
    /// extensions is what decides it, so the result does not depend on
    /// `read_dir` order. Names are chosen so an alphabetical tie-break would
    /// pick the `.app`, and so would swapping the two extensions.
    ///
    /// Not asserted here: which of two `.flo` files wins when neither is named
    /// after the directory. `find_app_manifest` returns the first `read_dir`
    /// yields, and that order is filesystem-defined, so there is no answer a
    /// test could pin without asserting on the filesystem instead of on us.
    #[test]
    fn app_source_lookup_prefers_flo_over_app() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path().join("demo");
        write_app(&root.join("aaa.app"), "demo");
        write_app(&root.join("zzz.flo"), "demo");

        let found = find_app_manifest(&root).unwrap();
        assert_eq!(found.file_name().unwrap(), "zzz.flo");
    }

    /// With no `.flo` present, a `.app` is still a source — the older extension
    /// stays installable rather than reading as "this directory holds no app".
    #[test]
    fn app_source_lookup_accepts_a_lone_app_extension() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path().join("demo");
        write_app(&root.join("legacy.app"), "demo");

        let found = find_app_manifest(&root).unwrap();
        assert_eq!(found.file_name().unwrap(), "legacy.app");
    }

    /// A directory with no source, and a directory that does not exist at all,
    /// are both `None` rather than a panic: `find_app_manifest` flattens its
    /// `read_dir` error, and several callers reach it with a path they have not
    /// checked (`app show` on an id resolved from a stale lockfile).
    #[test]
    fn app_source_lookup_is_none_for_a_sourceless_or_absent_directory() {
        let tmp = tempfile::tempdir().unwrap();
        let empty = tmp.path().join("empty");
        std::fs::create_dir_all(&empty).unwrap();
        std::fs::write(empty.join("notes.txt"), "not an app").unwrap();
        assert!(find_app_manifest(&empty).is_none());
        assert!(find_app_manifest(&tmp.path().join("does-not-exist")).is_none());
    }

    /// Resolution order 1 before 2: the DIRECTORY name wins over an `app:` field
    /// match. On a tree where both exist and cross over — `apps/one/` declaring
    /// `two` and `apps/two/` declaring `one` — the two orders return different
    /// directories, so the fallback cannot quietly become the primary.
    #[test]
    fn app_resolution_prefers_the_directory_name_over_a_field_match() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: tmp.path().to_path_buf(),
        };
        write_app(&paths.apps_dir().join("one/src.flo"), "two");
        write_app(&paths.apps_dir().join("two/src.flo"), "one");

        assert_eq!(
            resolve_app_dir(&paths, "one").unwrap(),
            paths.apps_dir().join("one")
        );
    }

    /// The `is_safe_segment` fence in `resolve_app_dir` has to be checked
    /// against a target that really is there: `apps/../escapee` names a real
    /// directory, so `direct.is_dir()` is true and the fence is the only thing
    /// standing between a crafted id and a directory outside `apps/`.
    #[test]
    fn app_resolution_refuses_an_id_that_points_outside_the_apps_directory() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: tmp.path().to_path_buf(),
        };
        write_app(&paths.apps_dir().join("innocent/src.flo"), "innocent");
        write_app(&paths.aware_home.join("escapee").join("src.flo"), "escapee");
        assert!(paths.apps_dir().join("../escapee").is_dir());

        let err = resolve_app_dir(&paths, "../escapee").unwrap_err();
        assert!(
            matches!(err, AwareError::NotFound(_)),
            "expected NotFound, got {err:?}"
        );
    }

    /// The same fence one layer down, where #365 found seventeen unguarded
    /// joins. The manifest the id points at is a VALID agent manifest, so
    /// nothing downstream would object: the id has to be refused here or it is
    /// not refused at all.
    #[test]
    fn agent_lookup_refuses_an_id_that_reads_a_manifest_outside_the_agents_directory() {
        let tmp = tempfile::tempdir().unwrap();
        let agents_dir = tmp.path().join("agents");
        std::fs::create_dir_all(&agents_dir).unwrap();
        write_agent(&tmp.path().join("outside"), "smuggled");
        assert!(agents_dir.join("../outside/manifest.yaml").is_file());

        for id in ["../outside", "./outside"] {
            let err = load_agent_by_id(&agents_dir, id).unwrap_err();
            assert!(
                matches!(err, AwareError::NotFound(_)),
                "{id:?}: expected NotFound, got {err:?}"
            );
            let err = agent_manifest_path(&agents_dir, id).unwrap_err();
            assert!(
                matches!(err, AwareError::NotFound(_)),
                "{id:?}: expected NotFound, got {err:?}"
            );
        }

        // Non-vacuous: the same call on a plain id resolves and loads, so the
        // rejections above are the fence talking and not a broken fixture.
        write_agent(&agents_dir.join("resident"), "resident");
        assert_eq!(
            agent_manifest_path(&agents_dir, "resident").unwrap(),
            agents_dir.join("resident").join("manifest.yaml")
        );
        assert_eq!(
            load_agent_by_id(&agents_dir, "resident").unwrap().agent,
            "resident"
        );
    }

    /// An unreadable manifest names the FILE. `AwareError::Io` carries no path,
    /// so without this the operator gets a bare `No such file or directory` and
    /// no way to tell which of dozens of installed agents produced it.
    #[test]
    fn an_unreadable_manifest_names_the_file_it_failed_to_read() {
        let tmp = tempfile::tempdir().unwrap();
        let missing = tmp.path().join("ghost-agent").join("manifest.yaml");

        let err = load_agent(&missing).unwrap_err();
        let text = err.to_string();
        assert!(
            text.contains(&missing.display().to_string()),
            "error did not name the manifest: {text}"
        );
        assert!(
            matches!(err, AwareError::Io(_)),
            "the io KIND must survive the rewrap, got {err:?}"
        );
    }

    /// A malformed manifest names the file too, and lands as `Validation` (exit
    /// class 3) rather than as an io error — the author has a typo to fix, not
    /// a disk to check.
    #[test]
    fn a_malformed_manifest_names_the_file_and_reads_as_a_validation_error() {
        let tmp = tempfile::tempdir().unwrap();
        let agent_path = tmp.path().join("broken-agent.yaml");
        std::fs::write(&agent_path, "agent: broken\nversion: [unclosed\n").unwrap();
        let app_path = tmp.path().join("broken-app.flo");
        std::fs::write(&app_path, "app: broken\nnodes: {not: a list}\n").unwrap();

        for (path, err) in [
            (&agent_path, load_agent(&agent_path).unwrap_err()),
            (&app_path, load_app(&app_path).unwrap_err()),
        ] {
            let text = err.to_string();
            assert!(
                text.contains(&path.display().to_string()),
                "error did not name the manifest: {text}"
            );
            assert!(
                matches!(err, AwareError::Validation(_)),
                "expected Validation, got {err:?}"
            );
        }
    }
}
