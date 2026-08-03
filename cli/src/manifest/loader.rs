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
    let agents_dir = paths.agents_dir();
    if !agents_dir.exists() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&agents_dir)? {
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

/// True when `id` is a plain path segment safe to join onto a directory: non-empty,
/// not `.`/`..`, and free of path separators or NUL. Used to fence an installed
/// app id (always a plain slug) before it is joined onto apps_dir. The stricter
/// charset/reserved-name check for a *new* id lives in `install::rename`.
pub(crate) fn is_safe_segment(id: &str) -> bool {
    !id.is_empty() && id != "." && id != ".." && !id.contains(['/', '\\', '\0'])
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

pub fn load_agent(manifest_path: &Path) -> Result<Agent, AwareError> {
    let text = std::fs::read_to_string(manifest_path)?;
    let parsed: Agent = serde_yaml::from_str(&text)
        .map_err(|e| AwareError::Validation(format!("{}: {e}", manifest_path.display())))?;
    Ok(parsed)
}

pub fn load_app(manifest_path: &Path) -> Result<App, AwareError> {
    let text = std::fs::read_to_string(manifest_path)?;
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
}
