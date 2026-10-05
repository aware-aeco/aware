//! Install an agent or app from a local path. Validates first.

use std::collections::BTreeMap;
use std::path::Path;

use crate::agent_store::RefGuard;
use crate::error::AwareError;
use crate::install::swap;
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
    guard: &RefGuard,
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
    // Staged, then promoted by one journaled swap (#626, #627): the staged tree
    // — receipt included — is snapshotted into the immutable store BEFORE
    // promotion, so a snapshot failure refuses the install with nothing
    // installed. Staging sits in the swap area under `agents/` (same volume,
    // a fresh directory per install), like the registry install's, and is
    // removed on any failure.
    let staged = swap::Staged::new(paths)?;
    let staging = staged.incoming();
    copy_dir_recursive(src, &staging)?;
    // AFTER the copy: if `src` is itself an installed agent directory it carries a marker
    // of its own, and that one describes where IT came from, not where this copy did. A
    // stale store record copied along is not this copy's either.
    //
    // Cleared FIRST, because the write is best-effort. If it failed, an inherited
    // `source: registry` marker would survive and say the opposite of the truth about a
    // LOCAL install — a silent failure in the destructive direction. Absent degrades to
    // "unknown", which the guard judges conservatively; wrong does not.
    let _ = std::fs::remove_file(staging.join(crate::install::provenance::FILE));
    let _ = std::fs::remove_file(staging.join(crate::agent_store::PACKAGE_FILE));
    crate::install::provenance::write(&staging, source);
    let package = crate::agent_store::snapshot(paths, &staging, guard)?;
    let txn = swap::begin(paths, guard, &[agent.agent.as_str()], Some(staged))?;
    // Re-checked under the lock: a concurrent install of the same id may have
    // finished while this one staged.
    if crate::agent_store::probe(&dst)?.is_some() {
        return Err(AwareError::Conflict(format!(
            "agent {} already installed; use `aware agent update {}` to refresh",
            agent.agent, agent.agent
        )));
    }
    txn.execute(
        swap::Op::Install,
        Some(&agent.agent),
        Some(package.digest),
        Vec::new(),
    )?;
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
pub fn install_app_from_path(
    src: &Path,
    paths: &Paths,
    guard: &RefGuard,
) -> Result<App, AwareError> {
    let manifest_path = crate::manifest::loader::require_single_app_manifest(src)?;

    let app = load_app(&manifest_path)?;
    let issues = validate_app(&app);
    if let Some(summary) = error_summary(&issues) {
        return Err(AwareError::Validation(summary));
    }

    // An `exposes-as-agent` app registers a synthesized agent under
    // `agents/<app>/`. Refuse up front (before claiming or copying anything) if a
    // real, non-app-backed agent already squats that name, so we never leave a
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

    // Claimed, not checked (#516). `dst.exists()` used to guard this, and a check
    // is not a reservation: two installs of one id could both pass it and both
    // copy into `dst` — `copy_dir_recursive` creates with `create_dir_all`, which
    // succeeds on an existing directory — leaving one directory holding two apps,
    // each reported installed. `create_dir` fails with `AlreadyExists` instead, so
    // exactly one racer wins the name and every other one copies nothing.
    let dst = paths.apps_dir().join(&app.app);
    std::fs::create_dir_all(paths.apps_dir())?;
    match std::fs::create_dir(&dst) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            return Err(AwareError::Conflict(format!(
                "app {} already installed",
                app.app
            )));
        }
        Err(e) => return Err(e.into()),
    }
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
    // It still deletes NOTHING, even though `dst` is now claimed atomically.
    // `create_dir` guarantees exclusive CREATION, not continued ownership of the
    // name: an `aware app uninstall` can remove `dst` mid-install and a second
    // install can then claim and fill it, so a `remove_dir_all(&dst)` here could
    // destroy that install's files to tidy up after this one. Safe rollback needs
    // install and uninstall to share a per-app lock. Naming the directory and
    // leaving it costs an operator one `aware app uninstall`.
    let resolved = crate::manifest::loader::find_app_manifest(&dst);
    if resolved.as_deref() != Some(installed_manifest.as_path()) {
        return Err(AwareError::Internal(format!(
            "installed {} from {}, but discovery in {} resolves to {} — the install-time and \
             run-time manifests disagree, so {} is NOT safe to run; inspect it and remove it \
             with `aware app uninstall {}` (left in place: removing it is only safe for an \
             install that still owns the directory)",
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
        write_synthesized_agent(&app, paths, guard)?;
    }

    Ok(app)
}

/// Write the synthesized callable agent manifest for an `exposes-as-agent` app
/// to `<agents_dir>/<app>/manifest.yaml`, so the app resolves and dispatches as
/// an agent (`agent: <app>, command: <cmd>`). See [`crate::manifest::expose`].
/// `pub(crate)` so `rename`/`duplicate` can regenerate it for the new id.
///
/// #627: written like any other agent — staged, then swapped in by one
/// journaled transaction under the agent's swap lock — so a reader never sees
/// a half-written synthesized agent. A previous copy is replaced only while it
/// is still app-backed by THIS app; a real agent of that name is refused.
pub(crate) fn write_synthesized_agent(
    app: &App,
    paths: &Paths,
    guard: &RefGuard,
) -> Result<(), AwareError> {
    let yaml = crate::manifest::expose::synthesize_agent_manifest(app)?;
    let staged = swap::Staged::new(paths)?;
    let incoming = staged.incoming();
    std::fs::create_dir_all(&incoming)?;
    std::fs::write(incoming.join("manifest.yaml"), yaml)?;
    let digest = crate::install::integrity::tree_digest(&incoming)?;
    let txn = swap::begin(paths, guard, &[app.app.as_str()], Some(staged))?;
    let agent_dir = paths.agents_dir().join(&app.app);
    let (op, outgoing) = if crate::agent_store::probe(&agent_dir)?.is_some() {
        if !is_app_backed_agent(&agent_dir, &app.app) {
            return Err(AwareError::Conflict(format!(
                "cannot expose app {0} as an agent: an agent named {0} is already installed",
                app.app
            )));
        }
        (
            swap::Op::Replace,
            vec![swap::Outgoing {
                id: app.app.clone(),
                digest: crate::install::integrity::tree_digest(&agent_dir).ok(),
            }],
        )
    } else {
        (swap::Op::Install, Vec::new())
    };
    txn.execute(op, Some(&app.app), Some(digest), outgoing)
}

/// Remove the synthesized agent at `agents/<id>/` — only while it is still
/// app-backed by the app `id` (never a real agent that happens to share the
/// name) — by one journaled swap under its swap lock (#627). `Ok(false)` when
/// there was nothing of that app's to remove.
pub(crate) fn remove_synthesized_agent(
    id: &str,
    paths: &Paths,
    guard: &RefGuard,
) -> Result<bool, AwareError> {
    if !crate::manifest::loader::is_safe_segment(id) || swap::is_swap_area(id) {
        return Ok(false);
    }
    let txn = swap::begin(paths, guard, &[id], None)?;
    let agent_dir = paths.agents_dir().join(id);
    if crate::agent_store::probe(&agent_dir)?.is_none() || !is_app_backed_agent(&agent_dir, id) {
        return Ok(false);
    }
    txn.execute(
        swap::Op::Uninstall,
        None,
        None,
        vec![swap::Outgoing {
            id: id.to_string(),
            digest: crate::install::integrity::tree_digest(&agent_dir).ok(),
        }],
    )?;
    Ok(true)
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
            &crate::agent_store::open(&paths).unwrap(),
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
            &crate::agent_store::open(&paths).unwrap(),
        )
        .unwrap();
        let err = install_agent_from_path(
            &src,
            &paths,
            &crate::install::provenance::InstallSource::Local {
                path: "fixture".into(),
            },
            &crate::agent_store::open(&paths).unwrap(),
        )
        .unwrap_err();
        assert!(matches!(err, AwareError::Conflict(_)));
    }

    #[test]
    fn local_install_rejects_agent_requiring_a_newer_cli_before_copy() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: tmp.path().join("aware"),
        };
        let src = tmp.path().join("future-agent");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(
            src.join("manifest.yaml"),
            "agent: future-agent\nversion: 1.0.0\ndescription: future\nstateful: false\n\
             status: requires-runtime\nminimum-cli-version: 999.0.0\nlicense: MIT\n\
             transport: { builtin: {} }\ncommands: { run: { lifecycle: single, description: x } }\n",
        )
        .unwrap();

        let err = install_agent_from_path(
            &src,
            &paths,
            &crate::install::provenance::InstallSource::Local {
                path: "fixture".into(),
            },
            &crate::agent_store::open(&paths).unwrap(),
        )
        .unwrap_err();
        assert!(err.to_string().contains("E_AGENT_RUNTIME_TOO_OLD"), "{err}");
        assert!(!paths.agents_dir().join("future-agent").exists());
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

        let installed =
            install_app_from_path(&app_src, &paths, &crate::agent_store::open(&paths).unwrap())
                .unwrap();
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

        let installed =
            install_app_from_path(&app_src, &paths, &crate::agent_store::open(&paths).unwrap())
                .unwrap();
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

        let err =
            install_app_from_path(&app_src, &paths, &crate::agent_store::open(&paths).unwrap())
                .unwrap_err();
        assert!(matches!(err, AwareError::Conflict(_)), "got: {err:?}");
        // The app must NOT have been partially installed.
        assert!(!paths.apps_dir().join("inner").exists());
    }

    /// Codex round 4 (#627-a): an `exposes-as-agent` app whose id is the swap
    /// area's name (in any case) would need `agents/.aware-swap/` as its
    /// synthesized agent. It is refused at validation, naming the reserved
    /// name, BEFORE `apps/<id>/` is claimed — never a half-installed app.
    #[test]
    fn an_exposed_app_named_like_the_swap_area_is_refused_before_anything_is_claimed() {
        for id in [".aware-swap", ".AWARE-Swap"] {
            let tmp = tempfile::tempdir().unwrap();
            let paths = Paths {
                aware_home: tmp.path().to_path_buf(),
            };
            let app_src = tmp.path().join("src/x");
            std::fs::create_dir_all(&app_src).unwrap();
            std::fs::write(
                app_src.join("x.flo"),
                format!(
                    "app: {id}\nversion: 0.1.0\ndescription: x\nexposes-as-agent: true\n\
                     exposed-commands: {{ run: {{ lifecycle: single }} }}\n\
                     nodes: [{{ id: n, inline: {{ kind: predicate, description: p, code: 'true' }} }}]\nrequires: []\n"
                ),
            )
            .unwrap();
            let err =
                install_app_from_path(&app_src, &paths, &crate::agent_store::open(&paths).unwrap())
                    .unwrap_err();
            assert!(matches!(err, AwareError::Validation(_)), "{id}: {err:?}");
            assert!(err.to_string().contains("reserved"), "{id}: {err}");
            assert!(
                !paths.apps_dir().join(id).exists(),
                "{id}: nothing may be claimed under apps/"
            );
        }
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

        let err =
            install_app_from_path(&app_src, &paths, &crate::agent_store::open(&paths).unwrap())
                .unwrap_err();
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

        let installed =
            install_app_from_path(&app_src, &paths, &crate::agent_store::open(&paths).unwrap())
                .unwrap();
        assert_eq!(installed.app, "alpha");

        let app_dir = paths.apps_dir().join("alpha");
        let discovered = crate::manifest::loader::find_app_manifest(&app_dir)
            .expect("installed app must be discoverable");
        assert_eq!(discovered, app_dir.join("bundle.flo"));
        assert_eq!(load_app(&discovered).unwrap().app, "alpha");
    }

    /// #516: concurrent installs of one app id must not merge into one directory.
    /// `dst.exists()` was a check, not a reservation — every racer could pass it
    /// and `copy_dir_recursive` then merged them all into `apps/<id>/`. The winner
    /// must be decided by one atomic operation: exactly one install succeeds, every
    /// other one gets `Conflict`, and the directory holds the winner's files only.
    #[test]
    fn concurrent_installs_of_one_id_claim_the_destination_exactly_once() {
        const RACERS: usize = 8;
        for round in 0..20 {
            let tmp = tempfile::tempdir().unwrap();
            let paths = Paths {
                aware_home: tmp.path().to_path_buf(),
            };
            let sources: Vec<_> = (0..RACERS)
                .map(|i| {
                    let src = tmp.path().join(format!("src/racer-{i}"));
                    write_fixture_app(&src.join(format!("racer-{i}.flo")), "same-id", "racer");
                    std::fs::write(src.join(format!("owned-by-{i}.txt")), "x").unwrap();
                    src
                })
                .collect();

            let barrier = std::sync::Barrier::new(RACERS);
            let results: Vec<_> = std::thread::scope(|scope| {
                let handles: Vec<_> = sources
                    .iter()
                    .map(|src| {
                        let (barrier, paths) = (&barrier, &paths);
                        scope.spawn(move || {
                            barrier.wait();
                            install_app_from_path(
                                src,
                                paths,
                                &crate::agent_store::open(paths).unwrap(),
                            )
                        })
                    })
                    .collect();
                handles.into_iter().map(|h| h.join().unwrap()).collect()
            });

            let winners: Vec<usize> = results
                .iter()
                .enumerate()
                .filter_map(|(i, r)| r.is_ok().then_some(i))
                .collect();
            assert_eq!(winners.len(), 1, "round {round}: winners {winners:?}");
            for (i, result) in results.iter().enumerate() {
                if let Err(err) = result {
                    assert!(
                        matches!(err, AwareError::Conflict(_)),
                        "round {round}: racer {i} lost with {err:?}, expected Conflict"
                    );
                }
            }
            let winner = winners[0];
            let mut entries: Vec<String> = std::fs::read_dir(paths.apps_dir().join("same-id"))
                .unwrap()
                .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
                .collect();
            entries.sort();
            assert_eq!(
                entries,
                vec![
                    format!("owned-by-{winner}.txt"),
                    format!("racer-{winner}.flo")
                ],
                "round {round}: the installed directory must hold the winner's files only"
            );
        }
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

    #[test]
    fn a_local_install_snapshots_before_promotion_and_a_failure_installs_nothing() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("src-agent");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(
            src.join("manifest.yaml"),
            "agent: loc\nversion: 1.0.0\ndescription: x\nstateful: false\nlicense: MIT\n\
             transport:\n  cli:\n    binary: aware-loc\ncommands:\n  go:\n    lifecycle: single\n    description: x\n",
        )
        .unwrap();
        let paths = Paths {
            aware_home: tmp.path().join("aware"),
        };
        let source = crate::install::provenance::InstallSource::Local {
            path: src.display().to_string(),
        };

        crate::agent_store::inject_fault(0, crate::agent_store::FaultStep::Rename);
        let refused = install_agent_from_path(
            &src,
            &paths,
            &source,
            &crate::agent_store::open(&paths).unwrap(),
        );
        crate::agent_store::clear_fault();
        assert!(refused.is_err());
        assert!(
            !paths.agents_dir().join("loc").exists(),
            "nothing installed"
        );
        assert!(
            crate::install::swap::leftover_txn_dirs(&paths).is_empty(),
            "staging cleaned"
        );

        install_agent_from_path(
            &src,
            &paths,
            &source,
            &crate::agent_store::open(&paths).unwrap(),
        )
        .unwrap();
        let installed = paths.agents_dir().join("loc");
        let digest = crate::install::integrity::tree_digest(&installed).unwrap();
        let key = crate::agent_store::receipt_key(&installed).unwrap();
        assert_ne!(
            key,
            crate::agent_store::NO_RECEIPT,
            "the local receipt is part of it"
        );
        let package = crate::agent_store::digest_container(&paths, "loc", &digest)
            .unwrap()
            .join(&key);
        assert!(crate::agent_store::verify_package(&package, "loc", &digest, &key).is_ok());
    }
}
