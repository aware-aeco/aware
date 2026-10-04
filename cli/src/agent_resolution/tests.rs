//! Resolver + `aware app check` unit tests (#626, plan tests 3, 4b, 5, 6).

use super::*;
use crate::agent_store::{NO_RECEIPT, PACKAGE_FILE};

struct Home {
    _tmp: tempfile::TempDir,
    paths: Paths,
}

fn home() -> Home {
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths {
        aware_home: tmp.path().to_path_buf(),
    };
    Home { _tmp: tmp, paths }
}

/// Write `agents/<id>/` at `version`. `extra` lands in a skill file so two
/// copies of one version can differ in bytes.
fn write_agent(paths: &Paths, id: &str, version: &str, extra: &str) -> PathBuf {
    let dir = paths.agents_dir().join(id);
    if dir.exists() {
        std::fs::remove_dir_all(&dir).unwrap();
    }
    std::fs::create_dir_all(dir.join("skills")).unwrap();
    std::fs::write(
        dir.join("manifest.yaml"),
        format!(
            "agent: {id}\nversion: {version}\ndescription: x\nstateful: false\nlicense: MIT\n\
             transport:\n  cli:\n    binary: aware-{id}-{version}\n\
             commands:\n  go:\n    lifecycle: single\n    mode: read\n    description: x\n    \
             outputs:\n      type: single\n      schema:\n        v{}: {{ type: string }}\n",
            version.replace('.', "_")
        ),
    )
    .unwrap();
    std::fs::write(
        dir.join("skills").join("s.md"),
        format!("{version} {extra}"),
    )
    .unwrap();
    dir
}

fn app_using(ids: &[&str]) -> App {
    let nodes: String = ids
        .iter()
        .map(|id| format!("  - id: n-{id}\n    agent: {id}\n    command: go\n"))
        .collect();
    serde_yaml::from_str(&format!(
        "app: demo\nversion: 0.1.0\ndescription: x\nnodes:\n{nodes}connections: []\n"
    ))
    .unwrap()
}

fn lock(pins: &[(&str, &str)], digests: &[(&str, &str)], bundle: &[(&str, &str)]) -> LockFile {
    let map = |pairs: &[(&str, &str)]| {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<BTreeMap<_, _>>()
    };
    LockFile {
        source_hash: "sha256:x".into(),
        compiled_at: "t".into(),
        compiler_version: "t".into(),
        app: "demo".into(),
        version: "0.1.0".into(),
        agent_pins: map(pins),
        agent_bundle_pins: map(bundle),
        agent_digests: map(digests),
        nodes: Vec::new(),
        schedule: None,
        engineering: None,
    }
}

fn digest(dir: &Path) -> String {
    crate::install::integrity::tree_digest(dir).unwrap()
}

fn store_root(paths: &Paths) -> PathBuf {
    paths.agent_store_dir()
}

fn version_of(catalogue: &ResolvedCatalogue, id: &str) -> String {
    catalogue.get(id).unwrap().manifest.version.clone()
}

fn official_receipt(dir: &Path, id: &str, version: &str) {
    let d = digest(dir);
    crate::install::provenance::write_required(
        dir,
        &crate::install::provenance::InstallSource::Registry {
            key: id.into(),
            version: version.into(),
            manifest_agent: Some(id.into()),
            manifest_version: Some(version.into()),
            entry_digest: Some(d.clone()),
            installed_digest: Some(d),
            official_source: true,
        },
    )
    .unwrap();
}

// ── plan test 3: the resolver ────────────────────────────────────────────────

#[test]
fn a_digest_lock_resolves_to_the_stored_copy_after_an_update() {
    let h = home();
    let v1 = write_agent(&h.paths, "alpha", "1.0.0", "");
    let d1 = digest(&v1);
    crate::agent_store::snapshot(&h.paths, &v1).unwrap(); // compile's snapshot
    write_agent(&h.paths, "alpha", "1.1.0", ""); // the update

    let resolved = resolve_agents(
        &h.paths,
        &app_using(&["alpha"]),
        &lock(&[("alpha", "1.0.0")], &[("alpha", &d1)], &[]),
        Selection::Default,
    )
    .unwrap();
    let agent = resolved.get("alpha").unwrap();
    assert_eq!(agent.manifest.version, "1.0.0", "the approved bytes run");
    assert!(
        agent.root.starts_with(store_root(&h.paths)),
        "{}",
        agent.root.display()
    );
    assert_eq!(digest(&agent.root), d1);
    let info = resolved.info("alpha").unwrap();
    assert_eq!(info.resolution, Resolution::Stored);
    assert_eq!(info.approval, Approval::Bytes);
}

#[test]
fn a_matching_current_copy_is_snapshotted_and_dispatched_from_the_store() {
    let h = home();
    let v1 = write_agent(&h.paths, "alpha", "1.0.0", "");
    let d1 = digest(&v1);
    assert!(!store_root(&h.paths).exists());

    let resolved = resolve_agents(
        &h.paths,
        &app_using(&["alpha"]),
        &lock(&[("alpha", "1.0.0")], &[("alpha", &d1)], &[]),
        Selection::Default,
    )
    .unwrap();
    let agent = resolved.get("alpha").unwrap();
    assert!(
        agent.root.starts_with(store_root(&h.paths)),
        "even the current version dispatches from an immutable snapshot"
    );
    assert_eq!(
        resolved.info("alpha").unwrap().resolution,
        Resolution::Current
    );
    // Editing the working copy after preflight cannot reach this run's bytes.
    std::fs::write(v1.join("skills").join("s.md"), "edited").unwrap();
    assert_eq!(digest(&agent.root), d1);
}

#[test]
fn a_tampered_stored_package_refuses_and_never_falls_back() {
    let h = home();
    let v1 = write_agent(&h.paths, "alpha", "1.0.0", "");
    let d1 = digest(&v1);
    let package = crate::agent_store::snapshot(&h.paths, &v1).unwrap();
    write_agent(&h.paths, "alpha", "1.1.0", "");
    std::fs::write(package.root.join("skills").join("s.md"), "tampered").unwrap();

    let error = resolve_agents(
        &h.paths,
        &app_using(&["alpha"]),
        &lock(&[("alpha", "1.0.0")], &[("alpha", &d1)], &[]),
        Selection::Default,
    )
    .unwrap_err()
    .to_string();
    assert!(
        error.contains("E_APP_LOCK_AGENT_BUNDLE_PIN_MISMATCH"),
        "{error}"
    );
    assert!(error.contains("does not verify"), "{error}");
}

#[test]
fn a_package_whose_record_or_manifest_identity_disagrees_is_refused() {
    let h = home();
    let v1 = write_agent(&h.paths, "alpha", "1.0.0", "");
    let d1 = digest(&v1);
    let package = crate::agent_store::snapshot(&h.paths, &v1).unwrap();
    write_agent(&h.paths, "alpha", "1.1.0", "");
    let record = package.root.join(PACKAGE_FILE);
    let text = std::fs::read_to_string(&record).unwrap();
    std::fs::write(&record, text.replace("agent: alpha", "agent: beta")).unwrap();

    let error = resolve_agents(
        &h.paths,
        &app_using(&["alpha"]),
        &lock(&[("alpha", "1.0.0")], &[("alpha", &d1)], &[]),
        Selection::Default,
    )
    .unwrap_err()
    .to_string();
    assert!(
        error.contains("E_APP_LOCK_AGENT_BUNDLE_PIN_MISMATCH"),
        "{error}"
    );

    // …and a lock whose version pin disagrees with the package's own manifest
    // refuses too, even though the bytes hash right.
    std::fs::write(&record, text).unwrap();
    let error = resolve_agents(
        &h.paths,
        &app_using(&["alpha"]),
        &lock(&[("alpha", "9.9.9")], &[("alpha", &d1)], &[]),
        Selection::Default,
    )
    .unwrap_err()
    .to_string();
    assert!(
        error.contains("E_APP_LOCK_AGENT_BUNDLE_PIN_MISMATCH"),
        "{error}"
    );
    assert!(error.contains("not the pinned 9.9.9"), "{error}");
}

#[test]
fn a_version_only_lock_never_resolves_to_an_older_package_by_version() {
    let h = home();
    let v1 = write_agent(&h.paths, "alpha", "1.0.0", "");
    crate::agent_store::snapshot(&h.paths, &v1).unwrap(); // 1.0.0 IS in the store
    write_agent(&h.paths, "alpha", "1.1.0", "");

    let error = resolve_agents(
        &h.paths,
        &app_using(&["alpha"]),
        &lock(&[("alpha", "1.0.0")], &[], &[]),
        Selection::Default,
    )
    .unwrap_err()
    .to_string();
    assert!(error.contains("E_APP_LOCK_AGENT_PIN_MISMATCH"), "{error}");
    assert!(error.contains("installed version is 1.1.0"), "{error}");
}

#[test]
fn a_version_only_lock_on_a_matching_copy_runs_a_snapshot_labelled_version_only() {
    let h = home();
    write_agent(&h.paths, "alpha", "1.0.0", "");
    let resolved = resolve_agents(
        &h.paths,
        &app_using(&["alpha"]),
        &lock(&[("alpha", "1.0.0")], &[], &[]),
        Selection::Default,
    )
    .unwrap();
    let agent = resolved.get("alpha").unwrap();
    assert!(agent.root.starts_with(store_root(&h.paths)));
    let info = resolved.info("alpha").unwrap();
    assert_eq!(info.approval, Approval::VersionOnly);
    assert_eq!(info.resolution, Resolution::CurrentVersionOnly);
}

#[test]
fn a_legacy_lock_with_an_official_bundle_pin_resolves_to_the_stored_copy() {
    // Every FloLess-approved Tekla workflow: compiled by AWARE <= 0.148 with only
    // `agent-bundle-pins`. Its first `agent update` must not break it.
    let h = home();
    let v1 = write_agent(&h.paths, "alpha", "1.0.0", "");
    official_receipt(&v1, "alpha", "1.0.0");
    let d1 = digest(&v1);
    // The update snapshots the outgoing copy before replacing it.
    crate::agent_store::snapshot(&h.paths, &v1).unwrap();
    write_agent(&h.paths, "alpha", "1.1.0", "");

    let resolved = resolve_agents(
        &h.paths,
        &app_using(&["alpha"]),
        &lock(&[("alpha", "1.0.0")], &[], &[("alpha", &d1)]),
        Selection::Default,
    )
    .unwrap();
    assert_eq!(version_of(&resolved, "alpha"), "1.0.0");
    assert_eq!(resolved.info("alpha").unwrap().approval, Approval::Bytes);
    assert!(resolved.info("alpha").unwrap().official_claim);
}

#[test]
fn an_uninstalled_agent_is_missing_whatever_the_store_holds() {
    let h = home();
    let v1 = write_agent(&h.paths, "alpha", "1.0.0", "");
    let d1 = digest(&v1);
    crate::agent_store::snapshot(&h.paths, &v1).unwrap();
    std::fs::remove_dir_all(&v1).unwrap();

    let app = app_using(&["alpha"]);
    let resolved = resolve_agents(
        &h.paths,
        &app,
        &lock(&[("alpha", "1.0.0")], &[("alpha", &d1)], &[]),
        Selection::Default,
    )
    .unwrap();
    assert!(resolved.get("alpha").is_none());
    let missing =
        crate::validate::missing_agents(&app, resolved.agents(), crate::validate::Severity::Error);
    assert_eq!(missing.len(), 1, "the run's missing-agent refusal fires");
}

#[test]
fn an_installed_agent_the_lock_never_pinned_is_refused() {
    let h = home();
    write_agent(&h.paths, "alpha", "1.0.0", "");
    let error = resolve_agents(
        &h.paths,
        &app_using(&["alpha"]),
        &lock(&[], &[], &[]),
        Selection::Default,
    )
    .unwrap_err()
    .to_string();
    assert!(error.contains("E_APP_LOCK_AGENT_PIN_MISMATCH"), "{error}");
    assert!(error.contains("no version"), "{error}");
}

#[test]
fn approved_bytes_that_exist_nowhere_refuse_as_pin_not_installed() {
    let h = home();
    let v1 = write_agent(&h.paths, "alpha", "1.0.0", "");
    let d1 = digest(&v1);
    write_agent(&h.paths, "alpha", "1.1.0", ""); // never snapshotted
    let error = resolve_agents(
        &h.paths,
        &app_using(&["alpha"]),
        &lock(&[("alpha", "1.0.0")], &[("alpha", &d1)], &[]),
        Selection::Default,
    )
    .unwrap_err()
    .to_string();
    assert!(error.contains("E_APP_LOCK_AGENT_PIN_MISMATCH"), "{error}");
    assert!(error.contains("no longer installed"), "{error}");
    assert!(error.contains("1.1.0"), "{error}");
}

#[test]
fn inconsistent_or_malformed_lock_digests_refuse_before_anything_resolves() {
    let h = home();
    let v1 = write_agent(&h.paths, "alpha", "1.0.0", "");
    let d1 = digest(&v1);
    let other = format!("sha256:{}", "c".repeat(64));
    let app = app_using(&["alpha"]);
    for bad in [
        lock(
            &[("alpha", "1.0.0")],
            &[("alpha", &d1)],
            &[("alpha", &other)],
        ),
        lock(&[("alpha", "1.0.0")], &[("alpha", "sha256:../../x")], &[]),
        lock(&[("alpha", "1.0.0")], &[], &[("alpha", "md5:abc")]),
        lock(&[], &[("alpha", &d1)], &[]),
    ] {
        let error = resolve_agents(&h.paths, &app, &bad, Selection::Default)
            .unwrap_err()
            .to_string();
        assert!(error.contains("E_APP_LOCK_INVALID"), "{error}");
    }
    assert!(
        !store_root(&h.paths).exists(),
        "an invalid lock must refuse before any snapshot is written"
    );
}

#[test]
fn receipt_choice_prefers_the_current_receipt_then_official_then_local() {
    let h = home();
    let v1 = write_agent(&h.paths, "alpha", "1.0.0", "");
    let d1 = digest(&v1);
    // Same bytes, two provenances: a local install and an official one.
    crate::install::provenance::write_required(
        &v1,
        &crate::install::provenance::InstallSource::Local { path: "x".into() },
    )
    .unwrap();
    let local = crate::agent_store::snapshot(&h.paths, &v1).unwrap();
    official_receipt(&v1, "alpha", "1.0.0");
    let official = crate::agent_store::snapshot(&h.paths, &v1).unwrap();
    assert_ne!(local.root, official.root);
    let app = app_using(&["alpha"]);
    let pinned = lock(&[("alpha", "1.0.0")], &[("alpha", &d1)], &[]);

    // The current copy (official receipt now) hashes to the pin: its own wins.
    let resolved = resolve_agents(&h.paths, &app, &pinned, Selection::Default).unwrap();
    assert_eq!(resolved.get("alpha").unwrap().root, official.root);

    // Back to the local receipt on the working copy: still its own receipt.
    crate::install::provenance::write_required(
        &v1,
        &crate::install::provenance::InstallSource::Local { path: "x".into() },
    )
    .unwrap();
    let resolved = resolve_agents(&h.paths, &app, &pinned, Selection::Default).unwrap();
    assert_eq!(resolved.get("alpha").unwrap().root, local.root);
    assert!(
        resolved.info("alpha").unwrap().official_claim,
        "the strict precheck sees the official candidate"
    );

    // Current no longer matches: the total order picks the official receipt.
    write_agent(&h.paths, "alpha", "1.1.0", "");
    let resolved = resolve_agents(&h.paths, &app, &pinned, Selection::Default).unwrap();
    assert_eq!(resolved.get("alpha").unwrap().root, official.root);
    assert_eq!(
        resolved.info("alpha").unwrap().resolution,
        Resolution::Stored
    );
}

#[test]
fn an_app_backed_agent_carries_its_backing_apps_own_resolution() {
    let h = home();
    let v1 = write_agent(&h.paths, "alpha", "1.0.0", "");
    let d1 = digest(&v1);
    crate::agent_store::snapshot(&h.paths, &v1).unwrap();

    // The backing app, compiled against alpha 1.0.0.
    let backing_dir = h.paths.apps_dir().join("inner");
    std::fs::create_dir_all(&backing_dir).unwrap();
    let source = "app: inner\nversion: 0.1.0\ndescription: x\nexposes-as-agent: true\n\
                  nodes:\n  - id: n\n    agent: alpha\n    command: go\nconnections: []\n";
    std::fs::write(backing_dir.join("inner.flo"), source).unwrap();
    let mut inner_lock = lock(&[("alpha", "1.0.0")], &[("alpha", &d1)], &[]);
    inner_lock.app = "inner".into();
    inner_lock.source_hash = format!(
        "sha256:{:x}",
        <sha2::Sha256 as sha2::Digest>::digest(source.as_bytes())
    );
    crate::app_lock::write_lockfile(&inner_lock, &backing_dir.join("inner.flo")).unwrap();

    // The synthesized wrapper agent.
    let wrapper = h.paths.agents_dir().join("inner");
    std::fs::create_dir_all(&wrapper).unwrap();
    std::fs::write(
        wrapper.join("manifest.yaml"),
        "agent: inner\nversion: 0.1.0\ndescription: x\nstateful: false\nlicense: app-exposed\n\
         transport:\n  app:\n    backed-by: inner\ncommands: { go: { lifecycle: single, description: x } }\n",
    )
    .unwrap();
    let dw = digest(&wrapper);

    // alpha is updated AFTER both apps were compiled.
    write_agent(&h.paths, "alpha", "1.1.0", "");

    let resolved = resolve_agents(
        &h.paths,
        &app_using(&["inner"]),
        &lock(&[("inner", "0.1.0")], &[("inner", &dw)], &[]),
        Selection::Default,
    )
    .unwrap();
    let nested = resolved
        .nested("inner")
        .expect("backing app resolved at preflight");
    assert_eq!(nested.backed_by, "inner");
    assert_eq!(version_of(&nested.catalogue, "alpha"), "1.0.0");
    assert_eq!(nested.app_owned().unwrap().app, "inner");
    let reachable: Vec<String> = resolved
        .reachable(&app_using(&["inner"]))
        .into_iter()
        .map(|r| r.id)
        .collect();
    assert_eq!(reachable, vec!["alpha".to_string()]);
}

#[test]
fn an_app_backed_leaf_inside_a_backing_app_exceeds_the_one_hop_limit() {
    let h = home();
    // inner backs wrapper `inner`; inner itself dispatches app-backed `deeper`.
    let deeper = h.paths.agents_dir().join("deeper");
    std::fs::create_dir_all(&deeper).unwrap();
    std::fs::write(
        deeper.join("manifest.yaml"),
        "agent: deeper\nversion: 0.1.0\ndescription: x\nstateful: false\nlicense: app-exposed\n\
         transport:\n  app:\n    backed-by: deeper\ncommands: { go: { lifecycle: single, description: x } }\n",
    )
    .unwrap();
    let backing_dir = h.paths.apps_dir().join("inner");
    std::fs::create_dir_all(&backing_dir).unwrap();
    let source = "app: inner\nversion: 0.1.0\ndescription: x\nexposes-as-agent: true\n\
                  nodes:\n  - id: n\n    agent: deeper\n    command: go\nconnections: []\n";
    std::fs::write(backing_dir.join("inner.flo"), source).unwrap();
    let mut inner_lock = lock(&[("deeper", "0.1.0")], &[], &[]);
    inner_lock.app = "inner".into();
    inner_lock.source_hash = format!(
        "sha256:{:x}",
        <sha2::Sha256 as sha2::Digest>::digest(source.as_bytes())
    );
    crate::app_lock::write_lockfile(&inner_lock, &backing_dir.join("inner.flo")).unwrap();
    let wrapper = h.paths.agents_dir().join("inner");
    std::fs::create_dir_all(&wrapper).unwrap();
    std::fs::write(
        wrapper.join("manifest.yaml"),
        "agent: inner\nversion: 0.1.0\ndescription: x\nstateful: false\nlicense: app-exposed\n\
         transport:\n  app:\n    backed-by: inner\ncommands: { go: { lifecycle: single, description: x } }\n",
    )
    .unwrap();
    let error = resolve_agents(
        &h.paths,
        &app_using(&["inner"]),
        &lock(&[("inner", "0.1.0")], &[], &[]),
        Selection::Default,
    )
    .unwrap_err()
    .to_string();
    assert!(error.contains("one-hop limit"), "{error}");
}

// ── plan tests 4b + 5: compile pins from the store, for every agent ──────────

fn write_source(dir: &Path, body: &str) -> PathBuf {
    std::fs::create_dir_all(dir).unwrap();
    let source = dir.join("demo.flo");
    std::fs::write(&source, body).unwrap();
    source
}

#[test]
fn compile_writes_agent_digests_for_registry_local_and_app_built_agents() {
    let h = home();
    let reg = write_agent(&h.paths, "reg", "1.0.0", "");
    official_receipt(&reg, "reg", "1.0.0");
    let local = write_agent(&h.paths, "loc", "2.0.0", "");
    crate::install::provenance::write_required(
        &local,
        &crate::install::provenance::InstallSource::Local { path: "x".into() },
    )
    .unwrap();
    // App-built: a synthesized exposes-as-agent wrapper, no receipt at all.
    let built = h.paths.agents_dir().join("built");
    std::fs::create_dir_all(&built).unwrap();
    std::fs::write(
        built.join("manifest.yaml"),
        "agent: built\nversion: 0.3.0\ndescription: x\nstateful: false\nlicense: app-exposed\n\
         transport:\n  app:\n    backed-by: built\ncommands: { go: { lifecycle: single, description: x } }\n",
    )
    .unwrap();

    let source = write_source(
        &h.paths.aware_home.join("src"),
        "app: demo\nversion: 0.1.0\ndescription: x\nnodes:\n\
         \x20 - id: a\n    agent: reg\n    command: go\n\
         \x20 - id: b\n    agent: loc\n    command: go\n\
         \x20 - id: c\n    agent: built\n    command: go\n    mode: read\nconnections: []\n",
    );
    let (_, lock) = crate::app_lock::compile_to_disk_with_lock(&source, &h.paths).unwrap();
    for (id, dir) in [("reg", &reg), ("loc", &local), ("built", &built)] {
        assert_eq!(lock.agent_digests.get(id), Some(&digest(dir)), "{id}");
        let hex = crate::agent_store::digest_hex(&lock.agent_digests[id]).unwrap();
        assert!(
            store_root(&h.paths).join(id).join(hex).is_dir(),
            "{id} was snapshotted at compile"
        );
    }
    assert_eq!(lock.agent_bundle_pins.get("reg"), Some(&digest(&reg)));
    assert!(!lock.agent_bundle_pins.contains_key("loc"));
    assert!(check_lock_consistency(&lock).is_ok());
}

#[test]
fn compile_pins_and_node_details_come_from_one_stored_copy() {
    let h = home();
    let current = write_agent(&h.paths, "alpha", "1.0.0", "");
    let source = write_source(
        &h.paths.aware_home.join("src"),
        "app: demo\nversion: 0.1.0\ndescription: x\nnodes:\n\
         \x20 - id: a\n    agent: alpha\n    command: go\nconnections: []\n",
    );
    // A writer rewrites the working copy to 1.1.0 between the digest and the
    // copy. Whatever compile ends up pinning, the version, the digest and the
    // compiled output schema must all describe ONE stored package.
    let _ = current;
    let paths = h.paths.clone();
    let mut fired = false;
    crate::agent_store::set_before_copy(Box::new(move || {
        if !fired {
            fired = true;
            write_agent(&paths, "alpha", "1.1.0", "mid-compile");
        }
    }));
    let (_, lock) = crate::app_lock::compile_to_disk_with_lock(&source, &h.paths).unwrap();
    crate::agent_store::clear_fault();

    let pinned_digest = lock.agent_digests["alpha"].clone();
    let pinned_version = lock.agent_pins["alpha"].clone();
    let package = crate::agent_store::package_candidates(&h.paths, "alpha", &pinned_digest)
        .unwrap()
        .pop()
        .unwrap()
        .1;
    let stored = crate::manifest::loader::load_agent(&package.join("manifest.yaml")).unwrap();
    assert_eq!(stored.version, pinned_version);
    let schema = serde_yaml::to_string(&lock.nodes[0].output_schema).unwrap();
    let expected_field = format!("v{}", pinned_version.replace('.', "_"));
    assert!(
        schema.contains(&expected_field),
        "node details came from a different copy than the pins: {schema}"
    );
    assert_eq!(
        pinned_version, "1.1.0",
        "the retry compiled the bytes it pinned"
    );
}

// ── plan test 6: `aware app check` reports every resolution ──────────────────

fn compile_app(h: &Home, ids: &[&str]) -> PathBuf {
    let nodes: String = ids
        .iter()
        .map(|id| format!("  - id: n-{id}\n    agent: {id}\n    command: go\n"))
        .collect();
    let source = write_source(
        &h.paths.apps_dir().join("demo"),
        &format!("app: demo\nversion: 0.1.0\ndescription: x\nnodes:\n{nodes}connections: []\n"),
    );
    crate::app_lock::compile_to_disk(&source, &h.paths).unwrap();
    source
}

fn row<'a>(check: &'a AppCheck, id: &str) -> &'a AgentCheck {
    check.agents.iter().find(|r| r.agent == id).unwrap()
}

fn edit_lock(source: &Path, edit: impl FnOnce(&mut LockFile)) {
    let path = source.with_file_name("demo.lock");
    let mut lock: LockFile =
        serde_yaml::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    edit(&mut lock);
    crate::app_lock::write_lockfile(&lock, source).unwrap();
}

fn store_listing(paths: &Paths) -> Vec<PathBuf> {
    let mut out = Vec::new();
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            out.push(entry.path());
            if entry.path().is_dir() {
                walk(&entry.path(), out);
            }
        }
    }
    walk(&paths.agent_store_dir(), &mut out);
    out.sort();
    out
}

#[test]
fn app_check_reports_current_stored_and_writes_nothing() {
    let h = home();
    write_agent(&h.paths, "cur", "1.0.0", "");
    write_agent(&h.paths, "upd", "1.0.0", "");
    let source = compile_app(&h, &["cur", "upd"]);
    write_agent(&h.paths, "upd", "2.0.0", "");
    let before = store_listing(&h.paths);

    let check = check_app(&h.paths, &source).unwrap();
    assert_eq!(check.lock, LockState::Valid);
    assert!(check.source_current);
    assert!(check.approval_current);
    assert_eq!(check.approval_kind, Some(Approval::Bytes));
    assert_eq!(row(&check, "cur").resolution, Resolution::Current);
    let upd = row(&check, "upd");
    assert_eq!(upd.resolution, Resolution::Stored);
    assert_eq!(upd.pinned_version.as_deref(), Some("1.0.0"));
    assert_eq!(upd.installed_version.as_deref(), Some("2.0.0"));
    assert_eq!(
        store_listing(&h.paths),
        before,
        "app check must write nothing"
    );

    let json = serde_json::to_value(&check).unwrap();
    for key in [
        "app",
        "approval-current",
        "approval-kind",
        "source-current",
        "lock",
        "agents",
    ] {
        assert!(json.get(key).is_some(), "missing {key}: {json}");
    }
    assert_eq!(json["agents"][1]["resolution"], "stored");
    assert!(json["agents"][1].get("pinned-digest").is_some());
}

#[test]
fn app_check_reports_every_refusal_as_data() {
    let h = home();
    for id in ["miss", "gone", "bad", "legacy", "fine"] {
        write_agent(&h.paths, id, "1.0.0", "");
    }
    let source = compile_app(&h, &["miss", "gone", "bad", "legacy", "fine"]);
    // missing: uninstalled after compile.
    std::fs::remove_dir_all(h.paths.agents_dir().join("miss")).unwrap();
    // pin-not-installed: updated, and the stored copy removed by hand.
    let gone_digest = {
        let lock: LockFile = serde_yaml::from_str(
            &std::fs::read_to_string(source.with_file_name("demo.lock")).unwrap(),
        )
        .unwrap();
        lock.agent_digests["gone"].clone()
    };
    std::fs::remove_dir_all(
        crate::agent_store::digest_container(&h.paths, "gone", &gone_digest).unwrap(),
    )
    .unwrap();
    write_agent(&h.paths, "gone", "2.0.0", "");
    // digest-mismatch: updated, and the stored copy tampered.
    let bad_digest = {
        let lock: LockFile = serde_yaml::from_str(
            &std::fs::read_to_string(source.with_file_name("demo.lock")).unwrap(),
        )
        .unwrap();
        lock.agent_digests["bad"].clone()
    };
    let bad_pkg = crate::agent_store::digest_container(&h.paths, "bad", &bad_digest)
        .unwrap()
        .join(NO_RECEIPT);
    std::fs::write(bad_pkg.join("skills").join("s.md"), "tampered").unwrap();
    write_agent(&h.paths, "bad", "2.0.0", "");
    // legacy: a lock with no digest for it, then the agent moves on.
    edit_lock(&source, |lock| {
        lock.agent_digests.remove("legacy");
        lock.agent_digests.remove("fine");
    });
    write_agent(&h.paths, "legacy", "2.0.0", "");
    // never-approved: a new node whose agent the lock does not pin.
    write_agent(&h.paths, "newbie", "1.0.0", "");
    let text = std::fs::read_to_string(&source).unwrap();
    let edited = text.replace(
        "connections: []",
        "  - id: n-newbie\n    agent: newbie\n    command: go\nconnections: []",
    );
    std::fs::write(&source, &edited).unwrap();
    edit_lock(&source, |lock| {
        lock.source_hash = format!(
            "sha256:{:x}",
            <sha2::Sha256 as sha2::Digest>::digest(edited.as_bytes())
        );
    });

    let check = check_app(&h.paths, &source).unwrap();
    assert!(check.source_current);
    assert!(!check.approval_current);
    assert_eq!(check.approval_kind, None);
    assert_eq!(row(&check, "miss").resolution, Resolution::Missing);
    assert_eq!(row(&check, "gone").resolution, Resolution::PinNotInstalled);
    assert_eq!(row(&check, "bad").resolution, Resolution::DigestMismatch);
    assert_eq!(
        row(&check, "legacy").resolution,
        Resolution::LegacyPinMismatch
    );
    assert_eq!(
        row(&check, "fine").resolution,
        Resolution::CurrentVersionOnly
    );
    assert_eq!(row(&check, "newbie").resolution, Resolution::NeverApproved);
    for r in &check.agents {
        assert!(!r.detail.is_empty(), "{} has no detail", r.agent);
    }
}

#[test]
fn app_check_reports_a_legacy_lock_that_runs_as_version_only() {
    let h = home();
    write_agent(&h.paths, "alpha", "1.0.0", "");
    let source = compile_app(&h, &["alpha"]);
    edit_lock(&source, |lock| lock.agent_digests.clear());
    let check = check_app(&h.paths, &source).unwrap();
    assert!(check.approval_current);
    assert_eq!(check.approval_kind, Some(Approval::VersionOnly));
}

#[test]
fn app_check_reports_lock_and_source_drift_as_data() {
    let h = home();
    write_agent(&h.paths, "alpha", "1.0.0", "");
    let source = compile_app(&h, &["alpha"]);

    std::fs::write(
        &source,
        std::fs::read_to_string(&source).unwrap() + "# edited\n",
    )
    .unwrap();
    let check = check_app(&h.paths, &source).unwrap();
    assert_eq!(check.lock, LockState::Valid);
    assert!(!check.source_current);
    assert!(!check.approval_current);

    edit_lock(&source, |lock| {
        lock.agent_bundle_pins
            .insert("alpha".into(), format!("sha256:{}", "d".repeat(64)));
    });
    let check = check_app(&h.paths, &source).unwrap();
    assert_eq!(check.lock, LockState::Invalid);
    assert!(!check.approval_current);

    std::fs::remove_file(source.with_file_name("demo.lock")).unwrap();
    let check = check_app(&h.paths, &source).unwrap();
    assert_eq!(check.lock, LockState::Missing);
    assert!(!check.approval_current);

    std::fs::write(source.with_file_name("demo.lock"), "{ not: [valid").unwrap();
    let check = check_app(&h.paths, &source).unwrap();
    assert_eq!(check.lock, LockState::Invalid);
}

#[test]
fn app_check_fails_only_when_it_cannot_run() {
    let h = home();
    let missing = h.paths.apps_dir().join("nope").join("nope.flo");
    assert!(check_app(&h.paths, &missing).is_err());
    let source = write_source(&h.paths.apps_dir().join("demo"), "app: [unparseable");
    assert!(check_app(&h.paths, &source).is_err());
}

// ── review round 1: hashing and snapshot failures are named, not relabelled ──

/// Put a directory link (a junction on Windows) inside `dir`, which makes the
/// tree unhashable: `tree_digest` refuses reparse/symlink indirection.
fn link_inside(dir: &Path, outside: &Path) {
    std::fs::create_dir_all(outside).unwrap();
    let link = dir.join("linked");
    #[cfg(windows)]
    {
        let status = std::process::Command::new("cmd.exe")
            .args(["/C", "mklink", "/J"])
            .arg(&link)
            .arg(outside)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .unwrap();
        assert!(status.success(), "failed to create test junction");
    }
    #[cfg(unix)]
    std::os::unix::fs::symlink(outside, &link).unwrap();
}

#[test]
fn an_unhashable_current_copy_is_named_not_called_changed_bytes() {
    let h = home();
    let v1 = write_agent(&h.paths, "alpha", "1.0.0", "");
    let d1 = digest(&v1);
    crate::agent_store::snapshot(&h.paths, &v1).unwrap();
    link_inside(&v1, &h.paths.aware_home.join("outside"));
    let pinned = lock(&[("alpha", "1.0.0")], &[("alpha", &d1)], &[]);

    // The approved bytes are still stored: they run, and the detail says why
    // the installed copy could not be compared — not that it changed.
    let outcome = assess_agent(&h.paths, "alpha", &pinned, Mode::Check).unwrap();
    assert_eq!(outcome.resolution, Resolution::Stored);
    assert!(
        outcome.detail.contains("cannot be hashed"),
        "{}",
        outcome.detail
    );

    // With no stored copy, the refusal names the hashing failure.
    std::fs::remove_dir_all(h.paths.agent_store_dir()).unwrap();
    let error = resolve_agents(
        &h.paths,
        &app_using(&["alpha"]),
        &pinned,
        Selection::Default,
    )
    .unwrap_err()
    .to_string();
    assert!(error.contains("cannot be hashed"), "{error}");
    assert!(!error.contains("has changed"), "{error}");
}

#[test]
fn a_failed_snapshot_of_the_matching_copy_falls_through_to_a_valid_stored_copy() {
    let h = home();
    let v1 = write_agent(&h.paths, "alpha", "1.0.0", "");
    let d1 = digest(&v1);
    official_receipt(&v1, "alpha", "1.0.0");
    let official = crate::agent_store::snapshot(&h.paths, &v1).unwrap();
    crate::install::provenance::write_required(
        &v1,
        &crate::install::provenance::InstallSource::Local { path: "x".into() },
    )
    .unwrap();
    // The current copy's OWN package exists but is corrupt, so its snapshot
    // refuses — yet an equally valid package of the same bytes is stored.
    let own = crate::agent_store::snapshot(&h.paths, &v1).unwrap();
    std::fs::write(own.root.join("skills").join("s.md"), "tampered").unwrap();
    let pinned = lock(&[("alpha", "1.0.0")], &[("alpha", &d1)], &[]);
    let app = app_using(&["alpha"]);

    let resolved = resolve_agents(&h.paths, &app, &pinned, Selection::Default).unwrap();
    assert_eq!(resolved.get("alpha").unwrap().root, official.root);
    assert_eq!(
        resolved.info("alpha").unwrap().resolution,
        Resolution::Stored
    );
    // …and `app check` agrees with the run.
    let check = assess_agent(&h.paths, "alpha", &pinned, Mode::Check).unwrap();
    assert_eq!(check.resolution, Resolution::Stored);
}

#[test]
fn a_snapshot_failure_with_nothing_stored_keeps_its_own_error_class() {
    let h = home();
    let v1 = write_agent(&h.paths, "alpha", "1.0.0", "");
    let d1 = digest(&v1);
    let pinned = lock(&[("alpha", "1.0.0")], &[("alpha", &d1)], &[]);
    crate::agent_store::inject_fault(0, crate::agent_store::FaultStep::Rename);
    let error = resolve_agents(
        &h.paths,
        &app_using(&["alpha"]),
        &pinned,
        Selection::Default,
    )
    .unwrap_err();
    crate::agent_store::clear_fault();
    assert!(matches!(error, AwareError::Internal(_)), "{error:?}");
    assert!(
        !error.to_string().contains("BUNDLE_PIN_MISMATCH"),
        "a failed snapshot is not a bundle mismatch: {error}"
    );
}

#[test]
fn a_legacy_lock_on_an_unhashable_copy_is_a_data_refusal_in_check_and_in_run() {
    let h = home();
    let v1 = write_agent(&h.paths, "alpha", "1.0.0", "");
    let source = compile_app(&h, &["alpha"]);
    edit_lock(&source, |lock| lock.agent_digests.clear());
    link_inside(&v1, &h.paths.aware_home.join("outside"));

    let check = check_app(&h.paths, &source).expect("check reports data, it does not fail");
    assert!(!check.approval_current);
    assert_eq!(row(&check, "alpha").resolution, Resolution::DigestMismatch);
    assert!(row(&check, "alpha").detail.contains("cannot be hashed"));

    let lock: LockFile =
        serde_yaml::from_str(&std::fs::read_to_string(source.with_file_name("demo.lock")).unwrap())
            .unwrap();
    assert!(resolve_agents(&h.paths, &app_using(&["alpha"]), &lock, Selection::Default).is_err());
}

// ── review round 1, item 4: invalid stored candidates are reported, not dropped ──

#[test]
fn an_invalid_stored_candidate_is_reported_even_when_a_valid_one_serves() {
    let h = home();
    let v1 = write_agent(&h.paths, "alpha", "1.0.0", "");
    let d1 = digest(&v1);
    official_receipt(&v1, "alpha", "1.0.0");
    let official = crate::agent_store::snapshot(&h.paths, &v1).unwrap();
    crate::install::provenance::write_required(
        &v1,
        &crate::install::provenance::InstallSource::Local { path: "x".into() },
    )
    .unwrap();
    let local = crate::agent_store::snapshot(&h.paths, &v1).unwrap();
    std::fs::write(local.root.join("skills").join("s.md"), "tampered").unwrap();
    write_agent(&h.paths, "alpha", "1.1.0", ""); // current no longer matches
    let pinned = lock(&[("alpha", "1.0.0")], &[("alpha", &d1)], &[]);
    let app = app_using(&["alpha"]);

    let resolved = resolve_agents(&h.paths, &app, &pinned, Selection::Default).unwrap();
    assert_eq!(resolved.get("alpha").unwrap().root, official.root);
    let info = resolved.info("alpha").unwrap();
    assert_eq!(
        info.invalid_candidates.len(),
        1,
        "{:?}",
        info.invalid_candidates
    );
    assert!(
        info.invalid_candidates[0]
            .reason
            .contains("changed after the snapshot")
    );
    // …in the run record…
    let record = serde_json::to_value(info).unwrap();
    assert_eq!(
        record["invalid-candidates"].as_array().unwrap().len(),
        1,
        "{record}"
    );
    // …as a warning the run prints…
    let warnings = invalid_candidate_warnings(&resolved);
    assert_eq!(warnings.len(), 1);
    assert!(
        warnings[0].contains(&local.root.display().to_string()),
        "{}",
        warnings[0]
    );

    // …and in `app check`.
    let outcome = assess_agent(&h.paths, "alpha", &pinned, Mode::Check).unwrap();
    let row = agent_row(outcome, None);
    assert_eq!(row.invalid_candidates.len(), 1);
    let json = serde_json::to_value(&row).unwrap();
    assert_eq!(
        json["invalid-candidates"].as_array().unwrap().len(),
        1,
        "{json}"
    );
}

#[test]
fn stored_versions_reports_packages_whose_record_cannot_be_read() {
    let h = home();
    let v1 = write_agent(&h.paths, "alpha", "1.0.0", "");
    let package = crate::agent_store::snapshot(&h.paths, &v1).unwrap();
    std::fs::remove_file(package.root.join(PACKAGE_FILE)).unwrap();
    let listing = crate::agent_store::stored_versions(&h.paths, "alpha");
    assert!(listing.stored.is_empty());
    assert_eq!(listing.unreadable.len(), 1, "{:?}", listing.unreadable);
    assert!(
        listing.unreadable[0]
            .path
            .contains(&package.root.display().to_string())
    );
}
