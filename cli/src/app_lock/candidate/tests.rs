//! `compile_candidate` + `plan_digest` (#628 plan §8: "compile_candidate
//! (approved bytes identical; header; blocked cases); plan_digest stability").

use super::*;
use crate::agent_resolution::PinTarget;
use std::path::PathBuf;

pub(crate) struct Home {
    _tmp: tempfile::TempDir,
    pub(crate) paths: Paths,
}

pub(crate) fn home() -> Home {
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths {
        aware_home: tmp.path().to_path_buf(),
    };
    Home { _tmp: tmp, paths }
}

/// Write `agents/<id>/` at `version`; command `go` gets `mode_line` verbatim
/// (e.g. `mode: read`), command `exec` is the Tekla-style overridable write.
pub(crate) fn write_agent(paths: &Paths, id: &str, version: &str, mode_line: &str) -> PathBuf {
    let dir = paths.agents_dir().join(id);
    if dir.exists() {
        std::fs::remove_dir_all(&dir).unwrap();
    }
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("manifest.yaml"),
        format!(
            "agent: {id}\nversion: {version}\ndescription: build {version}\nstateful: false\nlicense: MIT\n\
             transport:\n  cli:\n    binary: aware-test-bridge\n\
             commands:\n  go:\n    lifecycle: single\n    {mode_line}\n    description: x\n    \
             outputs:\n      type: single\n      schema:\n        ok: {{ type: string }}\n  \
             exec:\n    lifecycle: single\n    mode: write\n    mode-overridable: true\n    description: x\n"
        ),
    )
    .unwrap();
    dir
}

/// Install a new version of `id` the way install/update do: replace the
/// working copy and snapshot it into the store. Returns its digest.
pub(crate) fn update_agent(paths: &Paths, id: &str, version: &str, mode_line: &str) -> String {
    let dir = write_agent(paths, id, version, mode_line);
    crate::agent_store::snapshot(paths, &dir, &crate::agent_store::open(paths).unwrap())
        .unwrap()
        .digest
}

pub(crate) fn write_app(paths: &Paths, id: &str, body: &str) -> PathBuf {
    let dir = paths.apps_dir().join(id);
    std::fs::create_dir_all(&dir).unwrap();
    let source = dir.join(format!("{id}.flo"));
    std::fs::write(
        &source,
        format!("app: {id}\nversion: 0.1.0\ndescription: x\n{body}connections: []\n"),
    )
    .unwrap();
    source
}

/// Compile as a person would; return the lock and its exact bytes.
pub(crate) fn approve(paths: &Paths, source: &Path) -> (LockFile, Vec<u8>) {
    let (path, lock) =
        compile_to_disk_with_lock(source, paths, &crate::agent_store::open(paths).unwrap())
            .unwrap();
    (lock, std::fs::read(path).unwrap())
}

fn target(id: &str, digest: &str) -> BTreeMap<String, PinTarget> {
    [(id.to_string(), PinTarget::Digest(digest.to_string()))].into()
}

fn home_state(paths: &Paths) -> BTreeMap<String, Vec<u8>> {
    crate::fs::plain_files_under(&paths.aware_home, "home")
        .unwrap()
        .into_iter()
        .map(|(rel, path)| (rel, std::fs::read(path).unwrap()))
        .collect()
}

const TWO_TOOLS: &str = "requires: []\nnodes:\n  - { id: a, agent: tool, command: go }\n  - { id: b, agent: other, command: go }\n";

#[test]
fn a_candidate_moves_only_its_target_and_writes_nothing() {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    write_agent(&h.paths, "other", "1.0.0", "mode: read");
    let source = write_app(&h.paths, "demo", TWO_TOOLS);
    let (base, base_bytes) = approve(&h.paths, &source);
    let tool_new = update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    update_agent(&h.paths, "other", "1.0.1", "mode: read");
    let lock_path = source.with_file_name("demo.lock");
    let mtime = std::fs::metadata(&lock_path).unwrap().modified().unwrap();
    let before = home_state(&h.paths);

    let pins = PinSet::from_lock(&base, target("tool", &tool_new)).unwrap();
    let candidate = compile_candidate(&source, &h.paths, &base, &base_bytes, &pins).unwrap();

    assert_eq!(
        home_state(&h.paths),
        before,
        "compile_candidate wrote to AWARE_HOME"
    );
    assert_eq!(std::fs::read(&lock_path).unwrap(), base_bytes);
    assert_eq!(
        std::fs::metadata(&lock_path).unwrap().modified().unwrap(),
        mtime
    );
    assert!(candidate.blocked.is_empty(), "{:?}", candidate.blocked);

    // The target moved; the other agent kept the APPROVED bytes, not 1.0.1.
    assert_eq!(candidate.lock.agent_pins["tool"], "1.0.1");
    assert_eq!(candidate.lock.agent_digests["tool"], tool_new);
    assert_eq!(candidate.lock.agent_pins["other"], "1.0.0");
    assert_eq!(
        candidate.lock.agent_digests["other"],
        base.agent_digests["other"]
    );

    let header = &candidate.header;
    assert_eq!(header.format, CANDIDATE_FORMAT);
    assert_eq!(header.base_lock_digest, lock_digest(&base_bytes));
    assert_eq!(header.base_source_hash, base.source_hash);
    assert_eq!(header.candidate_digest, lock_digest(&candidate.bytes));
    assert_eq!(header.plan_digest, plan_digest(&candidate.lock).unwrap());
    let moved: Vec<&String> = header.targets.keys().collect();
    assert_eq!(moved, ["tool"]);
    assert_eq!(header.targets["tool"].from.version, "1.0.0");
    assert_eq!(
        header.targets["tool"].from.digest,
        base.agent_digests["tool"]
    );
    assert_eq!(header.targets["tool"].to.digest, tool_new);
    let text = String::from_utf8(candidate.bytes.clone()).unwrap();
    assert!(text.starts_with("# demo.candidate.lock"), "{text}");
    assert!(text.contains("not an approval"));
}

#[test]
fn a_candidate_with_no_target_compiles_to_the_approved_plan() {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    write_agent(&h.paths, "other", "1.0.0", "mode: read");
    let source = write_app(&h.paths, "demo", TWO_TOOLS);
    let (base, base_bytes) = approve(&h.paths, &source);
    let pins = PinSet::from_lock(&base, BTreeMap::new()).unwrap();
    let candidate = compile_candidate(&source, &h.paths, &base, &base_bytes, &pins).unwrap();
    assert!(candidate.header.targets.is_empty());
    assert_eq!(
        candidate.header.plan_digest,
        plan_digest(&base).unwrap(),
        "same source + same bytes = same plan"
    );
}

#[test]
fn plan_digest_ignores_when_and_by_whom_and_nothing_else() {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    write_agent(&h.paths, "other", "1.0.0", "mode: read");
    let source = write_app(&h.paths, "demo", TWO_TOOLS);
    let (mut lock, _) = approve(&h.paths, &source);
    let original = plan_digest(&lock).unwrap();
    assert_eq!(plan_digest(&lock).unwrap(), original, "stable");
    assert!(original.starts_with("sha256:") && original.len() == 71);

    lock.compiled_at = "1999-01-01T00:00:00Z".into();
    lock.compiler_version = "0.0.1".into();
    assert_eq!(plan_digest(&lock).unwrap(), original);

    let mutations: [fn(&mut LockFile); 4] = [
        |l: &mut LockFile| l.nodes[0].mode = "write".into(),
        |l: &mut LockFile| {
            l.agent_pins.insert("tool".into(), "9.9.9".into());
        },
        |l: &mut LockFile| {
            l.agent_digests
                .insert("tool".into(), format!("sha256:{}", "0".repeat(64)));
        },
        |l: &mut LockFile| l.source_hash = "sha256:other".into(),
    ];
    for mutate in mutations {
        let mut changed =
            serde_yaml::from_value::<LockFile>(serde_yaml::to_value(&lock).unwrap()).unwrap();
        mutate(&mut changed);
        assert_ne!(plan_digest(&changed).unwrap(), original);
    }
}

#[test]
fn plan_digest_does_not_depend_on_key_order() {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    write_agent(&h.paths, "other", "1.0.0", "mode: read");
    let source = write_app(
        &h.paths,
        "demo",
        "requires: []\nnodes:\n  - id: a\n    agent: tool\n    command: go\n    inputs: { zeta: 1, alpha: 2 }\n",
    );
    let (lock, _) = approve(&h.paths, &source);
    let reordered = write_app(
        &h.paths,
        "demo",
        "requires: []\nnodes:\n  - id: a\n    agent: tool\n    command: go\n    inputs: { alpha: 2, zeta: 1 }\n",
    );
    let (mut again, _) = approve(&h.paths, &reordered);
    // Only the source hash differs (the bytes differ); equalise it.
    again.source_hash = lock.source_hash.clone();
    assert_eq!(plan_digest(&again).unwrap(), plan_digest(&lock).unwrap());
}

#[test]
fn a_node_that_becomes_write_without_safety_blocks_the_candidate() {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    let source = write_app(
        &h.paths,
        "demo",
        "requires: []\nnodes:\n  - { id: a, agent: tool, command: go }\n",
    );
    let (base, base_bytes) = approve(&h.paths, &source);
    let new = update_agent(&h.paths, "tool", "1.0.1", "mode: write");
    let pins = PinSet::from_lock(&base, target("tool", &new)).unwrap();
    let candidate = compile_candidate(&source, &h.paths, &base, &base_bytes, &pins).unwrap();
    assert_eq!(candidate.blocked.len(), 1, "{:?}", candidate.blocked);
    assert_eq!(candidate.blocked[0].code, "needs-source-edit");
    assert!(
        candidate.blocked[0].text.contains("safety"),
        "{:?}",
        candidate.blocked
    );
    assert!(
        candidate.blocked[0]
            .text
            .contains("E_APP_WRITE_WITHOUT_SAFETY")
    );
}

#[test]
fn a_requires_pin_the_target_does_not_satisfy_blocks_the_candidate() {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    let source = write_app(
        &h.paths,
        "demo",
        "requires: [tool@1.0.0]\nnodes:\n  - { id: a, agent: tool, command: go }\n",
    );
    let (base, base_bytes) = approve(&h.paths, &source);
    let new = update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    let pins = PinSet::from_lock(&base, target("tool", &new)).unwrap();
    let candidate = compile_candidate(&source, &h.paths, &base, &base_bytes, &pins).unwrap();
    assert!(
        candidate
            .blocked
            .iter()
            .any(|r| r.code == "needs-source-edit" && r.text.contains("requires:")),
        "{:?}",
        candidate.blocked
    );
}

#[test]
fn a_changed_source_cannot_be_carried_forward() {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    let source = write_app(
        &h.paths,
        "demo",
        "requires: []\nnodes:\n  - { id: a, agent: tool, command: go }\n",
    );
    let (base, base_bytes) = approve(&h.paths, &source);
    write_app(
        &h.paths,
        "demo",
        "requires: []\nnodes:\n  - { id: renamed, agent: tool, command: go }\n",
    );
    let pins = PinSet::from_lock(&base, BTreeMap::new()).unwrap();
    let error = compile_candidate(&source, &h.paths, &base, &base_bytes, &pins).unwrap_err();
    assert!(
        error.to_string().contains("E_MIGRATE_SOURCE_CHANGED"),
        "{error}"
    );
}

#[test]
fn a_frozen_only_agent_keeps_its_base_pin_even_when_its_bytes_are_gone() {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    write_agent(&h.paths, "ice", "1.0.0", "mode: read");
    let source = write_app(
        &h.paths,
        "demo",
        "requires: []\nnodes:\n  - { id: a, agent: tool, command: go }\n  - id: f\n    agent: ice\n    command: go\n    frozen: { ok: pinned }\n",
    );
    let (base, base_bytes) = approve(&h.paths, &source);
    assert!(
        base.agent_digests.contains_key("ice"),
        "compile pins frozen agents"
    );
    let new = update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    let pins = PinSet::from_lock(&base, target("tool", &new)).unwrap();

    // Stored: compiled against the stored base bytes.
    let stored = compile_candidate(&source, &h.paths, &base, &base_bytes, &pins).unwrap();
    assert_eq!(stored.lock.agent_digests["ice"], base.agent_digests["ice"]);

    // Gone from the store (and the working copy): carried over verbatim.
    std::fs::remove_dir_all(h.paths.agent_store_dir().join("ice")).unwrap();
    std::fs::remove_dir_all(h.paths.agents_dir().join("ice")).unwrap();
    let verbatim = compile_candidate(&source, &h.paths, &base, &base_bytes, &pins).unwrap();
    assert_eq!(verbatim.lock.agent_pins["ice"], base.agent_pins["ice"]);
    assert_eq!(
        verbatim.lock.agent_digests["ice"],
        base.agent_digests["ice"]
    );
    let node = |lock: &LockFile| {
        serde_yaml::to_string(lock.nodes.iter().find(|n| n.id == "f").unwrap()).unwrap()
    };
    assert_eq!(node(&verbatim.lock), node(&base));
    assert!(verbatim.blocked.is_empty(), "{:?}", verbatim.blocked);
}
