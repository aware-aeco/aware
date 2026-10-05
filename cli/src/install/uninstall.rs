//! Uninstall — remove an agent or app folder.

use crate::agent_store::RefGuard;
use crate::error::AwareError;
use crate::paths::Paths;

/// Remove `agents/<id>/`. The immutable agent store (`agent-store/<id>/`) is
/// deliberately left alone (#626): its packages are unreachable once the
/// working copy is gone (a run then refuses with the missing-agent error), and
/// removing them is GC's job (#629), which needs run leases (#627).
///
/// #627: the working copy is moved aside whole by one journaled swap under the
/// agent's swap lock, then deleted — a reader never sees a partially deleted
/// agent, and an interrupted uninstall is finished or rolled back by the next
/// command that touches the agent.
pub fn uninstall_agent(id: &str, paths: &Paths, guard: &RefGuard) -> Result<(), AwareError> {
    // Fenced like every other agent-id join (#365): `id` is typed by a person
    // and must never name a directory outside `agents/` — nor the swap area.
    if !crate::manifest::loader::is_safe_segment(id) || crate::install::swap::is_swap_area(id) {
        return Err(AwareError::NotFound(format!("agent {id} is not installed")));
    }
    let txn = crate::install::swap::begin(paths, guard, &[id], None)?;
    let dir = paths.agents_dir().join(id);
    match crate::agent_store::probe(&dir)? {
        None => return Err(AwareError::NotFound(format!("agent {id} is not installed"))),
        // Only an installed agent is a directory; a stray file (or a link) at
        // that name is not ours to delete, so leave it and say what is there.
        Some(meta) if !meta.is_dir() => {
            return Err(AwareError::Validation(format!(
                "agents/{id} is not an installed agent (it is not a folder), so nothing was removed; move or delete it by hand if it should not be there"
            )));
        }
        Some(_) => {}
    }
    txn.execute(
        crate::install::swap::Op::Uninstall,
        None,
        None,
        vec![crate::install::swap::Outgoing {
            id: id.to_string(),
            digest: crate::install::integrity::tree_digest(&dir).ok(),
        }],
    )
}

pub fn uninstall_app(id: &str, paths: &Paths, guard: &RefGuard) -> Result<(), AwareError> {
    let dir = paths.apps_dir().join(id);
    if !dir.exists() {
        return Err(AwareError::NotFound(format!("app {id} is not installed")));
    }
    // #627-b: the app's approval needed its pinned packages until now (a
    // stamp is availability only; it never fails the uninstall).
    if let Some(source) = crate::manifest::loader::find_app_manifest(&dir)
        && let Ok((app, _)) = crate::app_lock::read_app_source(&source)
    {
        crate::agent_store::stamps::stamp_lock_file(
            paths,
            &crate::fs::containing_dir(&source).join(format!("{}.lock", app.app)),
        );
    }
    // Remove the synthesized agent an `exposes-as-agent` install registered —
    // but only if it is still app-backed by THIS app (never a real agent that
    // happens to share the name).
    crate::install::local::remove_synthesized_agent(id, paths, guard)?;
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
        uninstall_agent("tekla", &paths, &crate::agent_store::open(&paths).unwrap()).unwrap();
        assert!(!dir.exists());
    }

    /// A stray FILE at `agents/<id>` is not an installed agent: uninstall must
    /// leave it untouched (the pre-swap `remove_dir_all` failed on it; the swap
    /// would otherwise move it aside and delete it).
    #[test]
    fn uninstall_never_deletes_a_file_standing_where_an_agent_would_be() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: tmp.path().to_path_buf(),
        };
        std::fs::create_dir_all(paths.agents_dir()).unwrap();
        let file = paths.agents_dir().join("tekla");
        std::fs::write(&file, "not an agent").unwrap();
        let err = uninstall_agent("tekla", &paths, &crate::agent_store::open(&paths).unwrap())
            .unwrap_err();
        assert!(err.to_string().contains("not a folder"), "{err}");
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "not an agent");
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
        let package =
            crate::agent_store::snapshot(&paths, &dir, &crate::agent_store::open(&paths).unwrap())
                .unwrap();
        uninstall_agent("tekla", &paths, &crate::agent_store::open(&paths).unwrap()).unwrap();
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
            let err = uninstall_agent(id, &paths, &crate::agent_store::open(&paths).unwrap())
                .unwrap_err();
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
        let err = uninstall_agent("nope", &paths, &crate::agent_store::open(&paths).unwrap())
            .unwrap_err();
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
        uninstall_app(
            "welded-to-tc",
            &paths,
            &crate::agent_store::open(&paths).unwrap(),
        )
        .unwrap();
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

        uninstall_app("inner", &paths, &crate::agent_store::open(&paths).unwrap()).unwrap();
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

        uninstall_app("inner", &paths, &crate::agent_store::open(&paths).unwrap()).unwrap();
        assert!(!app_dir.exists());
        assert!(
            agent_dir.exists(),
            "a real same-named agent must be preserved"
        );
    }
}
