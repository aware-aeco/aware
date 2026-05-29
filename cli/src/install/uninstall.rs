//! Uninstall — remove an agent or app folder.

#![allow(dead_code)]

use crate::error::AwareError;
use crate::paths::Paths;

pub fn uninstall_agent(id: &str, paths: &Paths) -> Result<(), AwareError> {
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
        std::fs::write(dir.join("welded-to-tc.flo"), "app: welded-to-tc\n").unwrap();
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
