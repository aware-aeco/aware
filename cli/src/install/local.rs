//! Install an agent or app from a local path. Validates first.

use std::collections::BTreeMap;
use std::path::Path;

use crate::error::AwareError;
use crate::manifest::App;
use crate::manifest::loader::{load_agent, load_app};
use crate::paths::Paths;
use crate::validate::{error_summary, validate_agent_on_disk, validate_app};

// `copy_dir_recursive` used to live in this module. It moved to `crate::fs`
// when three modules turned out to be running near-identical walks (one with a
// subtle missing `create_dir_all(dst)`). Re-exported here so `install::rename`
// and `install::registry` — the in-tree callers that already imported it via
// `super::local::copy_dir_recursive` — don't have to update their imports.
pub(crate) use crate::fs::copy_dir_recursive;

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
    if let Some(summary) = error_summary(&issues) {
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
    //
    // Cleared FIRST, because the write is best-effort. If it failed, an inherited
    // `source: registry` marker would survive and say the opposite of the truth about a
    // LOCAL install — a silent failure in the destructive direction. Absent degrades to
    // "unknown", which the guard judges conservatively; wrong does not.
    let _ = std::fs::remove_file(dst.join(crate::install::provenance::FILE));
    crate::install::provenance::write(&dst, source);
    Ok(agent.agent)
}

/// Install an app folder. `src` must contain exactly one `.flo` or `.app` file
/// — see [`crate::manifest::loader::require_single_app_manifest`] for why more
/// than one is refused rather than resolved.
///
/// Returns the manifest it validated (its `app:` field is the installed id, and
/// the directory name under `apps/`), rather than the id alone. The caller needs
/// the manifest to write `lockfile.yaml`, and re-reading it from the installed
/// directory meant a second, differently-ordered selector: on a directory
/// holding two manifests the lock could describe one app while every later verb
/// loaded the other (#502).
pub fn install_app_from_path(src: &Path, paths: &Paths) -> Result<App, AwareError> {
    let manifest_path = crate::manifest::loader::require_single_app_manifest(src)?;

    let app = load_app(&manifest_path)?;
    let issues = validate_app(&app);
    if let Some(summary) = error_summary(&issues) {
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

    // The copy preserves file names, and the source held exactly one manifest,
    // so this is the same file `app` was parsed from — under its new root.
    let installed_manifest = dst.join(
        manifest_path
            .file_name()
            .ok_or_else(|| AwareError::Internal("app manifest has no file name".into()))?,
    );

    // The postcondition #502 is about, checked rather than assumed: the file
    // every later verb will load must be the file install just validated. It
    // holds by construction — one manifest in, one manifest copied, and the
    // canonical selector can only return that one — so reaching the refusal
    // means a selector has drifted apart from install again. Checked BEFORE the
    // synthesized agent is written, so a refused install registers no agent.
    //
    // It deletes NOTHING, deliberately. Being unreachable for a lone install is
    // exactly what makes cleanup dangerous here: the way to reach it is a SECOND
    // `app install` of the same id running concurrently. `dst.exists()` above is
    // a check, not a reservation, so both can pass it and both can copy into
    // `dst`; the merged directory then holds two manifests and whichever process
    // loses the selection arrives here — with the other process's files, possibly
    // already reported to its user as installed. A `remove_dir_all(&dst)` on that
    // path destroys a successful install to tidy up after a failed one. Naming
    // the directory and leaving it costs an operator one `aware app uninstall`;
    // the alternative costs them someone else's app. (The `exists()` race itself
    // predates this check and is not this change's to fix — see #516.)
    let resolved = crate::manifest::loader::find_app_manifest(&dst);
    if resolved.as_deref() != Some(installed_manifest.as_path()) {
        return Err(AwareError::Internal(format!(
            "installed {} from {}, but discovery in {} resolves to {} — the install-time and \
             run-time manifests disagree, so {} is NOT safe to run; inspect it and remove it \
             with `aware app uninstall {}` (left in place: a concurrent install of the same id \
             may own these files)",
            app.app,
            manifest_path.display(),
            dst.display(),
            resolved
                .as_deref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "nothing".into()),
            app.app,
            app.app,
        )));
    }

    if app.exposes_as_agent {
        write_synthesized_agent(&app, paths)?;
    }

    Ok(app)
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
        assert_eq!(installed.app, "welded-to-tc");
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
        assert_eq!(installed.app, "inner");
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

    /// The #502 repro. `bundle/` holds `bundle.flo` (`app: alpha`) beside
    /// `alpha.flo` (`app: decoy`). Install used to take one of them by
    /// filesystem order and copy the whole folder, after which discovery — which
    /// prefers the manifest named after the now-`alpha` directory — loaded the
    /// OTHER one: the lock said `alpha`, `app list` and `app show` said `decoy`.
    ///
    /// The folder is refused, and nothing is copied: a half-installed app whose
    /// identity is already in dispute is worse than no app.
    #[test]
    fn install_refuses_a_source_folder_holding_two_manifests() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: tmp.path().to_path_buf(),
        };
        let app_src = tmp.path().join("src/bundle");
        write_fixture_app(&app_src.join("bundle.flo"), "alpha", "selected manifest");
        write_fixture_app(
            &app_src.join("alpha.flo"),
            "decoy",
            "sibling decoy manifest",
        );

        let err = install_app_from_path(&app_src, &paths).unwrap_err();
        let AwareError::Validation(msg) = &err else {
            panic!("expected a validation error, got: {err:?}");
        };
        assert!(
            msg.contains("alpha.flo") && msg.contains("bundle.flo"),
            "{msg}"
        );
        assert!(
            !paths.apps_dir().join("alpha").exists(),
            "an ambiguous folder must not be copied"
        );
        assert!(!paths.apps_dir().join("decoy").exists());
    }

    /// The invariant the refusal buys, on the shape that made #502 reachable: a
    /// manifest whose file name is NOT the app id, so the installed directory is
    /// renamed out from under it. What install validated, what it reports, and
    /// what discovery later loads must all be the one file.
    #[test]
    fn the_installed_manifest_is_the_one_discovery_resolves() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: tmp.path().to_path_buf(),
        };
        let app_src = tmp.path().join("src/bundle");
        write_fixture_app(&app_src.join("bundle.flo"), "alpha", "selected manifest");

        let installed = install_app_from_path(&app_src, &paths).unwrap();
        assert_eq!(installed.app, "alpha");

        let app_dir = paths.apps_dir().join("alpha");
        let discovered = crate::manifest::loader::find_app_manifest(&app_dir)
            .expect("installed app must be discoverable");
        assert_eq!(discovered, app_dir.join("bundle.flo"));
        assert_eq!(load_app(&discovered).unwrap().app, "alpha");
    }

    fn write_fixture_app(path: &Path, id: &str, description: &str) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(
            path,
            format!(
                "app: {id}\nversion: 0.1.0\ndescription: {description}\n\
                 nodes:\n  - id: gate\n    inline:\n      kind: predicate\n\
                 \x20     description: always pass\n      code: 'true'\nrequires: []\n"
            ),
        )
        .unwrap();
    }
}
