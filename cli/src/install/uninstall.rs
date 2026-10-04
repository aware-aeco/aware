//! Uninstall — remove an agent or app folder.

use crate::error::AwareError;
use crate::paths::Paths;

/// Remove `agents/<id>/`. The immutable agent store (`agent-store/<id>/`) is
/// deliberately left alone (#626): its packages are unreachable once the
/// working copy is gone (a run then refuses with the missing-agent error), and
/// removing them is GC's job (#629), which needs run leases (#627).
pub fn uninstall_agent(id: &str, paths: &Paths) -> Result<(), AwareError> {
    // Fenced like every other agent-id join (#365): `id` is typed by a person
    // and must never name a directory outside `agents/` for `remove_dir_all`.
    if !crate::manifest::loader::is_safe_segment(id) {
        return Err(AwareError::NotFound(format!("agent {id} is not installed")));
    }
    let dir = paths.agents_dir().join(id);
    if !dir.exists() {
        return Err(AwareError::NotFound(format!("agent {id} is not installed")));
    }
    std::fs::remove_dir_all(&dir)?;
    Ok(())
}

pub fn uninstall_app(id: &str, paths: &Paths) -> Result<(), AwareError> {
    let dir = paths.apps_dir().join(id);
    if !dir.exists() {
        return Err(AwareError::NotFound(format!("app {id} is not installed")));
    }
    // Remove the synthesized agent an `exposes-as-agent` install registered —
    // but only if it is still app-backed by THIS app (never a real agent that
    // happens to share the name).
    let agent_dir = paths.agents_dir().join(id);
    if agent_dir.exists() && crate::install::local::is_app_backed_agent(&agent_dir, id) {
        std::fs::remove_dir_all(&agent_dir)?;
    }
    std::fs::remove_dir_all(&dir)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uninstalls_existing_agent() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: tmp.path().to_path_buf(),
        };
        let dir = paths.agents_dir().join("tekla");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("manifest.yaml"), "agent: tekla\n").unwrap();
        uninstall_agent("tekla", &paths).unwrap();
        assert!(!dir.exists());
    }

    #[test]
    fn uninstall_leaves_the_agent_store_alone() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: tmp.path().to_path_buf(),
        };
        let dir = paths.agents_dir().join("tekla");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("manifest.yaml"),
            "agent: tekla\nversion: 1.0.0\ndescription: x\nstateful: false\nlicense: MIT\n\
             transport:\n  cli:\n    binary: aware-tekla\ncommands: {}\n",
        )
        .unwrap();
        let package = crate::agent_store::snapshot(&paths, &dir).unwrap();
        uninstall_agent("tekla", &paths).unwrap();
        assert!(!dir.exists());
        assert!(
            package.root.join("manifest.yaml").is_file(),
            "uninstall must not touch the store"
        );
    }

    #[test]
    fn a_path_shaped_id_is_refused_before_any_removal() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: tmp.path().join("home"),
        };
        // A directory OUTSIDE agents/ that `agents/../victim` would name.
        let victim = paths.aware_home.join("victim");
        std::fs::create_dir_all(&victim).unwrap();
        std::fs::create_dir_all(paths.agents_dir()).unwrap();
        for id in ["../victim", "..", "a/b", ""] {
            let err = uninstall_agent(id, &paths).unwrap_err();
            assert!(matches!(err, AwareError::NotFound(_)), "{id:?}: {err:?}");
        }
        assert!(
            victim.is_dir(),
            "the fence must refuse before remove_dir_all"
        );
    }

    #[test]
    fn missing_agent_returns_not_found() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: tmp.path().to_path_buf(),
        };
        let err = uninstall_agent("nope", &paths).unwrap_err();
        assert!(matches!(err, AwareError::NotFound(_)));
    }

    #[test]
    fn uninstalls_existing_app() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: tmp.path().to_path_buf(),
        };
        let dir = paths.apps_dir().join("welded-to-tc");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("welded-to-tc.app"), "app: welded-to-tc\n").unwrap();
        uninstall_app("welded-to-tc", &paths).unwrap();
        assert!(!dir.exists());
    }

    #[test]
    fn uninstall_app_removes_its_synthesized_agent() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: tmp.path().to_path_buf(),
        };
        let app_dir = paths.apps_dir().join("inner");
        std::fs::create_dir_all(&app_dir).unwrap();
        std::fs::write(app_dir.join("inner.flo"), "app: inner\n").unwrap();
        // An app-backed synth agent (declares an `app` transport).
        let agent_dir = paths.agents_dir().join("inner");
        std::fs::create_dir_all(&agent_dir).unwrap();
        std::fs::write(
            agent_dir.join("manifest.yaml"),
            "agent: inner\nversion: 1.0\ndescription: x\nstateful: false\nlicense: app-exposed\n\
             transport:\n  app:\n    backed-by: inner\ncommands: { run: { lifecycle: single, description: x } }\n",
        )
        .unwrap();

        uninstall_app("inner", &paths).unwrap();
        assert!(!app_dir.exists(), "app dir must be removed");
        assert!(!agent_dir.exists(), "synth agent dir must be removed");
    }

    #[test]
    fn uninstall_app_keeps_a_real_agent_of_the_same_name() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: tmp.path().to_path_buf(),
        };
        let app_dir = paths.apps_dir().join("inner");
        std::fs::create_dir_all(&app_dir).unwrap();
        std::fs::write(app_dir.join("inner.flo"), "app: inner\n").unwrap();
        // A REAL (cli) agent that merely shares the name must NOT be removed.
        let agent_dir = paths.agents_dir().join("inner");
        std::fs::create_dir_all(&agent_dir).unwrap();
        std::fs::write(
            agent_dir.join("manifest.yaml"),
            "agent: inner\nversion: 1.0\ndescription: x\nstateful: false\nlicense: MIT\n\
             transport: { cli: { binary: aware-inner } }\ncommands: { go: { lifecycle: single, description: x } }\n",
        )
        .unwrap();

        uninstall_app("inner", &paths).unwrap();
        assert!(!app_dir.exists());
        assert!(
            agent_dir.exists(),
            "a real same-named agent must be preserved"
        );
    }
}
