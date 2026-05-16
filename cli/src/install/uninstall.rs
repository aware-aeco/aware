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
}
