//! Install an agent or app from a local path. Validates first.

use std::collections::BTreeMap;
use std::path::Path;

use crate::error::AwareError;
use crate::manifest::App;
use crate::manifest::loader::{load_agent, load_app};
use crate::paths::Paths;
use crate::validate::{Severity, has_errors, validate_agent_on_disk, validate_app};

/// Install an agent folder. `src` must contain `manifest.yaml`.
/// Destination is `<paths.agents_dir>/<agent-id>/`. Existing agents are
/// rejected (use `aware agent update` to refresh).
///
/// `source` is recorded beside the agent (#370). This function is the funnel for BOTH install
/// routes — the registry path stages into a scratch dir and hands it here — so it cannot tell
/// from `src` alone whether it is looking at a user's folder or a downloaded payload. Only the
/// caller knows, so only the caller can say.
pub fn install_agent_from_path(
    src: &Path,
    paths: &Paths,
    source: &crate::install::provenance::InstallSource,
) -> Result<String, AwareError> {
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
    // AFTER the copy: if `src` is itself an installed agent directory it carries a marker of its
    // own, and that one describes where IT came from, not where this copy did.
    crate::install::provenance::write(&dst, source);
    Ok(agent.agent)
}

/// Install an app folder. `src` must contain a `.flo` or `.app` file.
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

    // An `exposes-as-agent` app registers a synthesized agent under
    // `agents/<app>/`. Refuse up front (before copying anything) if a real,
    // non-app-backed agent already squats that name, so we never leave a
    // half-installed app behind.
    if app.exposes_as_agent {
        let agent_dst = paths.agents_dir().join(&app.app);
        if agent_dst.exists() && !is_app_backed_agent(&agent_dst, &app.app) {
            return Err(AwareError::Conflict(format!(
                "cannot expose app {0} as an agent: an agent named {0} is already installed",
                app.app
            )));
        }
    }

    std::fs::create_dir_all(paths.apps_dir())?;
    copy_dir_recursive(src, &dst)?;

    if app.exposes_as_agent {
        write_synthesized_agent(&app, paths)?;
    }

    Ok(app.app)
}

/// Write the synthesized callable agent manifest for an `exposes-as-agent` app
/// to `<agents_dir>/<app>/manifest.yaml`, so the app resolves and dispatches as
/// an agent (`agent: <app>, command: <cmd>`). See [`crate::manifest::expose`].
/// `pub(crate)` so `rename`/`duplicate` can regenerate it for the new id.
pub(crate) fn write_synthesized_agent(app: &App, paths: &Paths) -> Result<(), AwareError> {
    let yaml = crate::manifest::expose::synthesize_agent_manifest(app)?;
    let agent_dir = paths.agents_dir().join(&app.app);
    std::fs::create_dir_all(&agent_dir)?;
    std::fs::write(agent_dir.join("manifest.yaml"), yaml)?;
    Ok(())
}

/// Resolve an app's `requires` to installed agent versions and write the
/// install-time `lockfile.yaml` (legacy agent-version pins) into `app_dir`.
/// Best-effort resolution — an agent that isn't installed is simply omitted,
/// matching `aware app install` (it does not require every `requires` to be
/// present). Shared by `install` and `rename`/`duplicate` so a renamed app's
/// on-disk shape is identical to a freshly installed one.
pub(crate) fn write_app_lockfile(
    app: &App,
    app_dir: &Path,
    paths: &Paths,
) -> Result<(), AwareError> {
    let mut resolved = BTreeMap::new();
    for req in &app.requires {
        // Through `split_requires_entry`, not by hand: the id becomes a
        // directory name here, so a padded entry (`"probe-agent @1.3.x"`) would
        // look for `agents/probe-agent /` and miss — silently, because this
        // resolution is best-effort. The pin would then be accepted and enforced
        // everywhere else while the agent went missing from `lockfile.yaml`.
        let (id, _) = crate::manifest::app::split_requires_entry(req);
        // Fenced with every other agent-id join (#365): this id comes from the
        // app file's `requires:` too, and best-effort resolution would otherwise
        // read a manifest from outside `agents/` and lock the version it found.
        if let Ok(m) = crate::manifest::loader::load_agent_by_id(&paths.agents_dir(), id) {
            resolved.insert(id.to_string(), m.version);
        }
    }
    crate::lockfile::write(
        &app.app,
        &app.version,
        resolved,
        &app_dir.join("lockfile.yaml"),
    )
}

/// True when `<agent_dir>/manifest.yaml` is a synthesized agent backed by the
/// app `app_id` (declares an `app` transport pointing back at it). Used to tell
/// a regenerable synth manifest apart from a real installed agent of the same
/// name.
pub(crate) fn is_app_backed_agent(agent_dir: &Path, app_id: &str) -> bool {
    let manifest = agent_dir.join("manifest.yaml");
    load_agent(&manifest)
        .ok()
        .and_then(|a| a.transport.app)
        .is_some_and(|t| t.backed_by == app_id)
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
        let installed = install_agent_from_path(
            &src,
            &paths,
            &crate::install::provenance::InstallSource::Local {
                path: "fixture".into(),
            },
        )
        .unwrap();
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
        install_agent_from_path(
            &src,
            &paths,
            &crate::install::provenance::InstallSource::Local {
                path: "fixture".into(),
            },
        )
        .unwrap();
        let err = install_agent_from_path(
            &src,
            &paths,
            &crate::install::provenance::InstallSource::Local {
                path: "fixture".into(),
            },
        )
        .unwrap_err();
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
            .join("30-apps/_examples/welded-to-tc.app");
        std::fs::copy(&flo, app_src.join("welded-to-tc.app")).unwrap();

        let installed = install_app_from_path(&app_src, &paths).unwrap();
        assert_eq!(installed, "welded-to-tc");
        assert!(
            tmp.path()
                .join("apps/welded-to-tc/welded-to-tc.app")
                .is_file()
        );
    }

    #[test]
    fn installing_exposes_as_agent_app_registers_a_synth_agent() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: tmp.path().to_path_buf(),
        };
        let app_src = tmp.path().join("src/inner");
        std::fs::create_dir_all(&app_src).unwrap();
        std::fs::write(
            app_src.join("inner.flo"),
            r#"app: inner
version: 0.2.0
description: an exposed inner app
exposes-as-agent: true
exposed-commands:
  run:
    lifecycle: single
    inputs:
      phase:
        type: string
nodes:
  - id: gate
    inline:
      kind: predicate
      description: always pass
      code: 'true'
requires: []
"#,
        )
        .unwrap();

        let installed = install_app_from_path(&app_src, &paths).unwrap();
        assert_eq!(installed, "inner");
        // The synthesized agent manifest was registered and is app-backed.
        let agent_manifest = tmp.path().join("agents/inner/manifest.yaml");
        assert!(agent_manifest.is_file(), "synth agent manifest not written");
        let agent = load_agent(&agent_manifest).unwrap();
        assert_eq!(agent.agent, "inner");
        assert_eq!(
            agent.transport.app.unwrap().backed_by,
            "inner",
            "synth agent must declare an app transport backed by the app"
        );
    }

    #[test]
    fn install_refuses_to_clobber_a_real_agent_of_the_same_name() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: tmp.path().to_path_buf(),
        };
        // A real (cli) agent named `inner` already installed.
        let agent_dir = paths.agents_dir().join("inner");
        std::fs::create_dir_all(&agent_dir).unwrap();
        std::fs::write(
            agent_dir.join("manifest.yaml"),
            "agent: inner\nversion: 1.0\ndescription: real\nstateful: false\nlicense: MIT\n\
             transport: { cli: { binary: aware-inner } }\ncommands: { go: { lifecycle: single, description: x } }\n",
        )
        .unwrap();

        let app_src = tmp.path().join("src/inner");
        std::fs::create_dir_all(&app_src).unwrap();
        std::fs::write(
            app_src.join("inner.flo"),
            "app: inner\nversion: 0.1.0\ndescription: x\nexposes-as-agent: true\n\
             exposed-commands: { run: { lifecycle: single } }\nnodes: [{ id: n, inline: { kind: predicate, description: p, code: 'true' } }]\nrequires: []\n",
        )
        .unwrap();

        let err = install_app_from_path(&app_src, &paths).unwrap_err();
        assert!(matches!(err, AwareError::Conflict(_)), "got: {err:?}");
        // The app must NOT have been partially installed.
        assert!(!paths.apps_dir().join("inner").exists());
    }
}
