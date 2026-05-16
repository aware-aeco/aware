//! Install an agent or app from a local path. Validates first.

use std::path::Path;

use crate::error::AwareError;
use crate::manifest::loader::{load_agent, load_app};
use crate::paths::Paths;
use crate::validate::{Severity, has_errors, validate_agent_on_disk, validate_app};

/// Install an agent folder. `src` must contain `manifest.yaml`.
/// Destination is `<paths.agents_dir>/<agent-id>/`. Existing agents are
/// rejected (use `aware agent update` to refresh).
pub fn install_agent_from_path(src: &Path, paths: &Paths) -> Result<String, AwareError> {
    let manifest_path = src.join("manifest.yaml");
    if !manifest_path.is_file() {
        return Err(AwareError::Validation(format!(
            "no manifest.yaml in {}",
            src.display()
        )));
    }
    let agent = load_agent(&manifest_path)?;
    let issues = validate_agent_on_disk(&agent, src);
    if has_errors(&issues) {
        let summary = issues
            .iter()
            .filter(|i| i.severity == Severity::Error)
            .map(|i| format!("[{}] {}", i.code, i.message))
            .collect::<Vec<_>>()
            .join("; ");
        return Err(AwareError::Validation(summary));
    }

    let dst = paths.agents_dir().join(&agent.agent);
    if dst.exists() {
        return Err(AwareError::Conflict(format!(
            "agent {} already installed; use `aware agent update {}` to refresh",
            agent.agent, agent.agent
        )));
    }
    std::fs::create_dir_all(paths.agents_dir())?;
    copy_dir_recursive(src, &dst)?;
    Ok(agent.agent)
}

/// Install an app folder. `src` must contain a `.flo` or `.app` file.
#[allow(dead_code)] // consumed by v0.2 `aware app install` (Task 13+)
pub fn install_app_from_path(src: &Path, paths: &Paths) -> Result<String, AwareError> {
    let manifest_path = std::fs::read_dir(src)?
        .flatten()
        .map(|e| e.path())
        .find(|p| {
            matches!(
                p.extension().and_then(|e| e.to_str()),
                Some("flo") | Some("app")
            )
        })
        .ok_or_else(|| {
            AwareError::Validation(format!("no .flo or .app file in {}", src.display()))
        })?;

    let app = load_app(&manifest_path)?;
    let issues = validate_app(&app);
    if has_errors(&issues) {
        let summary = issues
            .iter()
            .filter(|i| i.severity == Severity::Error)
            .map(|i| format!("[{}] {}", i.code, i.message))
            .collect::<Vec<_>>()
            .join("; ");
        return Err(AwareError::Validation(summary));
    }

    let dst = paths.apps_dir().join(&app.app);
    if dst.exists() {
        return Err(AwareError::Conflict(format!(
            "app {} already installed",
            app.app
        )));
    }
    std::fs::create_dir_all(paths.apps_dir())?;
    copy_dir_recursive(src, &dst)?;
    Ok(app.app)
}

pub(crate) fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)?.flatten() {
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_recursive(&from, &to)?;
        } else {
            std::fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_agent_path(rel: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join(rel)
    }

    #[test]
    fn installs_tekla_from_repo() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: tmp.path().to_path_buf(),
        };
        let src = repo_agent_path("20-agents/aeco/engineering/tekla");
        let installed = install_agent_from_path(&src, &paths).unwrap();
        assert_eq!(installed, "tekla");
        assert!(tmp.path().join("agents/tekla/manifest.yaml").is_file());
        assert!(
            tmp.path()
                .join("agents/tekla/skills/drawing-identity.md")
                .is_file()
        );
    }

    #[test]
    fn rejects_install_when_already_present() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: tmp.path().to_path_buf(),
        };
        let src = repo_agent_path("20-agents/aeco/engineering/tekla");
        install_agent_from_path(&src, &paths).unwrap();
        let err = install_agent_from_path(&src, &paths).unwrap_err();
        assert!(matches!(err, AwareError::Conflict(_)));
    }

    #[test]
    fn installs_app_from_path() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: tmp.path().to_path_buf(),
        };
        let app_src = tmp.path().join("src/welded-to-tc");
        std::fs::create_dir_all(&app_src).unwrap();
        let flo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("30-apps/_examples/welded-to-tc.flo");
        std::fs::copy(&flo, app_src.join("welded-to-tc.flo")).unwrap();

        let installed = install_app_from_path(&app_src, &paths).unwrap();
        assert_eq!(installed, "welded-to-tc");
        assert!(
            tmp.path()
                .join("apps/welded-to-tc/welded-to-tc.flo")
                .is_file()
        );
    }
}
