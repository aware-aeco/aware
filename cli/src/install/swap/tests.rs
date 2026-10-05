//! Swap transaction tests (#627 plan §6 "Swap", R1-R4, 8 R1-1..3, 9 R2-3/R2-4).

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::Duration;

use super::*;

fn home() -> (tempfile::TempDir, Paths) {
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths {
        aware_home: tmp.path().to_path_buf(),
    };
    (tmp, paths)
}

/// Write a complete agent tree at `dir`. `extra` lands in a skill so two trees
/// of one version can differ in bytes; `files` adds bulk so a partial tree
/// would be visible.
fn write_tree(dir: &Path, id: &str, version: &str, extra: &str) {
    std::fs::create_dir_all(dir.join("skills")).unwrap();
    std::fs::write(
        dir.join("manifest.yaml"),
        format!(
            "agent: {id}\nversion: {version}\ndescription: x\nstateful: false\nlicense: MIT\n\
             transport:\n  cli:\n    binary: aware-{id}\n\
             commands:\n  go:\n    lifecycle: single\n    mode: read\n    description: x\n"
        ),
    )
    .unwrap();
    for n in 0..8 {
        std::fs::write(
            dir.join("skills").join(format!("s{n}.md")),
            format!("{id} {version} {extra} {n}"),
        )
        .unwrap();
    }
}

/// Every file under `dir`, by relative path → bytes.
fn tree_bytes(dir: &Path) -> BTreeMap<String, Vec<u8>> {
    crate::fs::plain_files_under(dir, "test tree")
        .unwrap()
        .into_iter()
        .map(|(relative, path)| (relative, std::fs::read(path).unwrap()))
        .collect()
}

fn maybe_tree(dir: &Path) -> Option<BTreeMap<String, Vec<u8>>> {
    dir.exists().then(|| tree_bytes(dir))
}

/// Stage `id` at `version` into a fresh transaction; returns it and its digest.
fn stage(paths: &Paths, id: &str, version: &str, extra: &str) -> (Staged, String) {
    let staged = Staged::new(paths).unwrap();
    write_tree(&staged.incoming(), id, version, extra);
    let digest = crate::install::integrity::tree_digest(&staged.incoming()).unwrap();
    (staged, digest)
}

fn outgoing(paths: &Paths, id: &str) -> Outgoing {
    Outgoing {
        id: id.to_string(),
        digest: crate::install::integrity::tree_digest(&paths.agents_dir().join(id)).ok(),
    }
}

/// Swap transaction directories still on disk (settled or not).
fn txn_dirs(paths: &Paths) -> Vec<PathBuf> {
    match std::fs::read_dir(paths.agent_swap_dir()) {
        Ok(entries) => entries
            .flatten()
            .filter(|e| is_txn_name(&e.file_name().to_string_lossy()))
            .map(|e| e.path())
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// Run an update of `from` → `to` (two ids when they differ: the dotted-id
/// rename shape, where `agents/<to>` already holds a stale copy).
fn update(paths: &Paths, from: &str, to: &str, extra: &str) -> Result<(), AwareError> {
    let guard = crate::agent_store::open(paths).unwrap();
    let (staged, digest) = stage(paths, to, "2.0.0", extra);
    let txn = begin(paths, &guard, &[from, to], Some(staged))?;
    let mut out = vec![outgoing(paths, from)];
    if from != to && paths.agents_dir().join(to).exists() {
        out.push(outgoing(paths, to));
    }
    txn.execute(Op::Update, Some(to), Some(digest), out)
}

/// The lines a two-directory update writes, in order.
fn two_dir_lines(from: &str, to: &str) -> Vec<String> {
    vec![
        "intent".into(),
        format!("pending out {from}"),
        format!("done out {from}"),
        format!("pending out {to}"),
        format!("done out {to}"),
        format!("pending in {to}"),
        format!("done in {to}"),
        COMMIT.into(),
    ]
}

/// Plan §6: a crash after EVERY journal line leaves, once a reader has
/// recovered, each id of the set a complete old or complete new tree — byte
/// for byte — and the set consistent (all old or all new). Before recovery
/// nothing under `agents/` is ever a partial tree.
#[test]
fn a_crash_after_every_journal_line_recovers_to_a_complete_old_or_new_set() {
    for (from, to) in [("alpha", "alpha"), ("alpha", "alpha.v2")] {
        let lines: Vec<String> = if from == to {
            vec![
                "intent".into(),
                format!("pending out {from}"),
                format!("done out {from}"),
                format!("pending in {to}"),
                format!("done in {to}"),
                COMMIT.into(),
            ]
        } else {
            two_dir_lines(from, to)
        };
        for (n, line) in lines.iter().enumerate() {
            let (_tmp, paths) = home();
            write_tree(&paths.agents_dir().join(from), from, "1.0.0", "old");
            if from != to {
                write_tree(&paths.agents_dir().join(to), to, "0.9.0", "stale");
            }
            let old_from = tree_bytes(&paths.agents_dir().join(from));
            let old_to = maybe_tree(&paths.agents_dir().join(to));

            inject_fault(Fault::CrashAfter(line.clone()));
            let error = update(&paths, from, to, "new").unwrap_err();
            clear_fault();
            assert!(error.to_string().contains(CRASHED), "{line}: {error}");

            // Before recovery: whatever is present is a whole tree.
            for id in [from, to] {
                if let Some(now) = maybe_tree(&paths.agents_dir().join(id)) {
                    let allowed = [Some(&old_from), old_to.as_ref()];
                    assert!(
                        allowed.contains(&Some(&now))
                            || now.get("manifest.yaml").is_some_and(
                                |m| String::from_utf8_lossy(m).contains("version: 2.0.0")
                            ) && now.len() == 9,
                        "{from}->{to} crash after {line:?}: agents/{id} is a partial tree"
                    );
                }
            }

            // A reader of `from` recovers the WHOLE transaction (both ids).
            let guard = crate::agent_store::open(&paths).unwrap();
            drop(read_lock(&paths, &guard, &[from]).unwrap());
            drop(guard);
            assert!(
                pending_naming(&paths, &[from.to_string(), to.to_string()])
                    .unwrap()
                    .is_empty(),
                "{line}: recovered"
            );
            if line == COMMIT {
                // A swap that committed is inert even if its final delete never
                // ran; readers leave it, a writer of its ids or doctor removes it.
                assert_eq!(txn_dirs(&paths).len(), 1, "{line}");
                let guard = crate::agent_store::open(&paths).unwrap();
                let findings = recover_all(&paths, &guard).unwrap();
                assert_eq!(findings[0].outcome, "cleaned", "{findings:?}");
            }
            assert!(txn_dirs(&paths).is_empty(), "{line}: transaction removed");

            let committed = n >= lines.iter().position(|l| l.starts_with("done in")).unwrap();
            let now_to = maybe_tree(&paths.agents_dir().join(to));
            if committed {
                let new = now_to.expect("the new tree is installed");
                assert!(
                    String::from_utf8_lossy(&new["manifest.yaml"]).contains("version: 2.0.0"),
                    "{line}"
                );
                assert_eq!(new.len(), 9, "{line}: complete new tree");
                if from != to {
                    assert!(
                        !paths.agents_dir().join(from).exists(),
                        "{line}: the renamed-from copy is gone after commit"
                    );
                }
            } else {
                assert_eq!(
                    tree_bytes(&paths.agents_dir().join(from)),
                    old_from,
                    "{from}->{to} crash after {line:?}: old tree restored byte for byte"
                );
                assert_eq!(now_to, old_to, "{line}: the second directory too");
            }
        }
    }
}

/// R2-3: a crash BETWEEN a rename and its `done` line is reconciled from the
/// paths — the move-in happened (the incoming tree is gone and `agents/<name>`
/// hashes to the intent's digest), so recovery commits rather than rolling a
/// completed swap back over a missing outgoing copy.
#[test]
fn a_rename_without_its_done_line_is_reconciled_from_the_paths() {
    let (_tmp, paths) = home();
    write_tree(&paths.agents_dir().join("alpha"), "alpha", "1.0.0", "old");
    // Crash after `done in` and then erase that line: the journal now says
    // `pending in alpha` with no `done`, while the rename did happen.
    inject_fault(Fault::CrashAfter("done in alpha".into()));
    update(&paths, "alpha", "alpha", "new").unwrap_err();
    clear_fault();
    let txn = txn_dirs(&paths).pop().unwrap();
    let journal = std::fs::read_to_string(txn.join(JOURNAL_FILE)).unwrap();
    std::fs::write(
        txn.join(JOURNAL_FILE),
        journal
            .replace("done in alpha\n", "")
            .replace("done in alpha\r\n", ""),
    )
    .unwrap();

    let guard = crate::agent_store::open(&paths).unwrap();
    drop(read_lock(&paths, &guard, &["alpha"]).unwrap());
    let now = tree_bytes(&paths.agents_dir().join("alpha"));
    assert!(String::from_utf8_lossy(&now["manifest.yaml"]).contains("version: 2.0.0"));
    assert!(txn_dirs(&paths).is_empty());
}

/// A state recovery cannot explain is reported and left untouched — never
/// "repaired" by deleting one of two copies.
#[test]
fn an_unexplainable_state_is_refused_and_left_alone() {
    let (_tmp, paths) = home();
    write_tree(&paths.agents_dir().join("alpha"), "alpha", "1.0.0", "old");
    inject_fault(Fault::CrashAfter("done out alpha".into()));
    update(&paths, "alpha", "alpha", "new").unwrap_err();
    clear_fault();
    // Someone puts a different tree at agents/alpha by hand.
    write_tree(&paths.agents_dir().join("alpha"), "alpha", "1.5.0", "hand");
    let hand = tree_bytes(&paths.agents_dir().join("alpha"));
    let guard = crate::agent_store::open(&paths).unwrap();
    let error = read_lock(&paths, &guard, &["alpha"])
        .unwrap_err()
        .to_string();
    assert!(error.contains("E_AGENT_SWAP_RECOVERY"), "{error}");
    assert_eq!(tree_bytes(&paths.agents_dir().join("alpha")), hand);
    assert_eq!(txn_dirs(&paths).len(), 1, "the evidence stays");
}

/// Plan §6: a failed incoming rename restores every outgoing tree.
#[test]
fn an_incoming_rename_failure_restores_the_outgoing_trees() {
    for (from, to) in [("alpha", "alpha"), ("alpha", "alpha.v2")] {
        let (_tmp, paths) = home();
        write_tree(&paths.agents_dir().join(from), from, "1.0.0", "old");
        if from != to {
            write_tree(&paths.agents_dir().join(to), to, "0.9.0", "stale");
        }
        let old_from = tree_bytes(&paths.agents_dir().join(from));
        let old_to = maybe_tree(&paths.agents_dir().join(to));
        inject_fault(Fault::FailRename(format!("in {to}")));
        let error = update(&paths, from, to, "new").unwrap_err().to_string();
        clear_fault();
        assert!(error.contains("injected rename failure"), "{error}");
        assert_eq!(tree_bytes(&paths.agents_dir().join(from)), old_from);
        assert_eq!(maybe_tree(&paths.agents_dir().join(to)), old_to);
        assert!(txn_dirs(&paths).is_empty(), "rolled back and removed");
    }
}

/// A failed SECOND move-out puts the first one back.
#[test]
fn a_failed_second_move_out_puts_the_first_back() {
    let (_tmp, paths) = home();
    write_tree(&paths.agents_dir().join("alpha"), "alpha", "1.0.0", "old");
    write_tree(
        &paths.agents_dir().join("alpha.v2"),
        "alpha.v2",
        "0.9.0",
        "stale",
    );
    let before = (
        tree_bytes(&paths.agents_dir().join("alpha")),
        tree_bytes(&paths.agents_dir().join("alpha.v2")),
    );
    inject_fault(Fault::FailRename("out alpha.v2".into()));
    update(&paths, "alpha", "alpha.v2", "new").unwrap_err();
    clear_fault();
    assert_eq!(
        (
            tree_bytes(&paths.agents_dir().join("alpha")),
            tree_bytes(&paths.agents_dir().join("alpha.v2")),
        ),
        before
    );
    assert!(txn_dirs(&paths).is_empty());
}

/// An uninstall interrupted before its move-out rolls back (the agent stays);
/// one interrupted after it commits (the agent is gone) — never half-deleted.
#[test]
fn an_interrupted_uninstall_is_all_or_nothing() {
    for (line, gone) in [
        ("intent", false),
        ("pending out alpha", false),
        ("done out alpha", true),
        (COMMIT, true),
    ] {
        let (_tmp, paths) = home();
        write_tree(&paths.agents_dir().join("alpha"), "alpha", "1.0.0", "old");
        let old = tree_bytes(&paths.agents_dir().join("alpha"));
        let guard = crate::agent_store::open(&paths).unwrap();
        inject_fault(Fault::CrashAfter(line.into()));
        crate::install::uninstall_agent("alpha", &paths, &guard).unwrap_err();
        clear_fault();
        let findings = recover_all(&paths, &guard).unwrap();
        assert!(
            findings.iter().all(|f| f.outcome != "needs-you"),
            "{line}: {findings:?}"
        );
        if gone {
            assert!(!paths.agents_dir().join("alpha").exists(), "{line}");
        } else {
            assert_eq!(tree_bytes(&paths.agents_dir().join("alpha")), old, "{line}");
        }
        assert!(txn_dirs(&paths).is_empty(), "{line}");
    }
}

/// Plan §6: `discover_agents_in` never lists the swap area — not a staged
/// incoming tree, not a moved-aside one, not even a manifest planted at its root.
#[test]
fn agent_discovery_never_lists_the_swap_area() {
    let (_tmp, paths) = home();
    write_tree(&paths.agents_dir().join("alpha"), "alpha", "1.0.0", "old");
    inject_fault(Fault::CrashAfter("done out alpha".into()));
    update(&paths, "alpha", "alpha", "new").unwrap_err();
    clear_fault();
    let (_staged, _) = stage(&paths, "beta", "1.0.0", "x");
    write_tree(&paths.agent_swap_dir(), SWAP_DIR, "1.0.0", "planted");
    let listed: Vec<String> = crate::manifest::loader::discover_agents_in(&paths.agents_dir())
        .unwrap()
        .into_iter()
        .map(|a| a.manifest.agent)
        .collect();
    assert!(listed.is_empty(), "{listed:?}");
    // …and the swap area is never an agent id.
    assert!(crate::manifest::loader::load_agent_by_id(&paths.agents_dir(), SWAP_DIR).is_err());
    let guard = crate::agent_store::open(&paths).unwrap();
    for name in [SWAP_DIR, ".AWARE-SWAP"] {
        assert!(matches!(
            crate::install::uninstall_agent(name, &paths, &guard),
            Err(AwareError::NotFound(_))
        ));
    }
    assert!(paths.agent_swap_dir().exists());
}

/// Plan §6: concurrent updates of one id serialize — never two inside the
/// swap at once, every one complete, nothing left behind.
#[test]
fn concurrent_updates_of_one_id_serialize() {
    let (_tmp, paths) = home();
    write_tree(&paths.agents_dir().join("alpha"), "alpha", "1.0.0", "seed");
    let inside = Arc::new(AtomicBool::new(false));
    let overlaps = Arc::new(AtomicUsize::new(0));
    let threads: Vec<_> = (0..6)
        .map(|n| {
            let paths = paths.clone();
            let inside = inside.clone();
            let overlaps = overlaps.clone();
            std::thread::spawn(move || {
                let guard = crate::agent_store::open(&paths).unwrap();
                let (staged, digest) = stage(&paths, "alpha", "2.0.0", &format!("t{n}"));
                let txn = begin(&paths, &guard, &["alpha"], Some(staged)).unwrap();
                if inside.swap(true, Ordering::SeqCst) {
                    overlaps.fetch_add(1, Ordering::SeqCst);
                }
                std::thread::sleep(Duration::from_millis(20));
                let out = vec![outgoing(&paths, "alpha")];
                inside.store(false, Ordering::SeqCst);
                txn.execute(Op::Update, Some("alpha"), Some(digest), out)
                    .unwrap();
            })
        })
        .collect();
    for thread in threads {
        thread.join().unwrap();
    }
    assert_eq!(overlaps.load(Ordering::SeqCst), 0, "two swaps overlapped");
    let now = tree_bytes(&paths.agents_dir().join("alpha"));
    assert_eq!(now.len(), 9);
    assert!(String::from_utf8_lossy(&now["manifest.yaml"]).contains("version: 2.0.0"));
    assert!(txn_dirs(&paths).is_empty());
}

/// A reader holding the swap lock shared keeps every writer of that id out.
#[test]
fn a_shared_reader_holds_writers_off() {
    let (_tmp, paths) = home();
    write_tree(&paths.agents_dir().join("alpha"), "alpha", "1.0.0", "old");
    let guard = crate::agent_store::open(&paths).unwrap();
    let reading = read_lock(&paths, &guard, &["alpha"]).unwrap();
    let done = Arc::new(AtomicBool::new(false));
    let writer = {
        let paths = paths.clone();
        let done = done.clone();
        std::thread::spawn(move || {
            update(&paths, "alpha", "alpha", "new").unwrap();
            done.store(true, Ordering::SeqCst);
        })
    };
    std::thread::sleep(Duration::from_millis(300));
    assert!(
        !done.load(Ordering::SeqCst),
        "the writer must wait for the reader"
    );
    assert!(
        String::from_utf8_lossy(
            &std::fs::read(paths.agents_dir().join("alpha/manifest.yaml")).unwrap()
        )
        .contains("1.0.0")
    );
    drop(reading);
    writer.join().unwrap();
    assert!(done.load(Ordering::SeqCst));
}

/// R2-4: a reader that finds a crashed MULTI-id transaction recovers it whole,
/// even when it reads only one of the ids.
#[test]
fn a_reader_of_one_id_recovers_the_whole_multi_id_transaction() {
    let (_tmp, paths) = home();
    write_tree(&paths.agents_dir().join("alpha"), "alpha", "1.0.0", "old");
    write_tree(
        &paths.agents_dir().join("alpha.v2"),
        "alpha.v2",
        "0.9.0",
        "stale",
    );
    let before = (
        tree_bytes(&paths.agents_dir().join("alpha")),
        tree_bytes(&paths.agents_dir().join("alpha.v2")),
    );
    inject_fault(Fault::CrashAfter("done out alpha.v2".into()));
    update(&paths, "alpha", "alpha.v2", "new").unwrap_err();
    clear_fault();
    assert!(!paths.agents_dir().join("alpha").exists());
    assert!(!paths.agents_dir().join("alpha.v2").exists());
    let guard = crate::agent_store::open(&paths).unwrap();
    drop(read_lock(&paths, &guard, &["alpha.v2"]).unwrap());
    assert_eq!(
        (
            tree_bytes(&paths.agents_dir().join("alpha")),
            tree_bytes(&paths.agents_dir().join("alpha.v2")),
        ),
        before,
        "both ids are rolled back together"
    );
}

/// Plan §6 (R1 regression): a lock that pins only a VERSION runs a snapshot of
/// the working copy. An update interrupted mid-swap must never let that
/// snapshot be taken of a partial tree — the run recovers first and snapshots a
/// complete old or new copy, whose digest is exactly one of the two trees'.
#[test]
fn a_version_only_lock_never_snapshots_a_partial_tree() {
    for line in [
        "pending out alpha",
        "done out alpha",
        "pending in alpha",
        "done in alpha",
    ] {
        let (_tmp, paths) = home();
        let current = paths.agents_dir().join("alpha");
        write_tree(&current, "alpha", "1.0.0", "old");
        let old_digest = crate::install::integrity::tree_digest(&current).unwrap();
        // Same version, other bytes: a version-only lock accepts either.
        let guard = crate::agent_store::open(&paths).unwrap();
        let (staged, new_digest) = stage(&paths, "alpha", "1.0.0", "new");
        let txn = begin(&paths, &guard, &["alpha"], Some(staged)).unwrap();
        inject_fault(Fault::CrashAfter(line.into()));
        txn.execute(
            Op::Update,
            Some("alpha"),
            Some(new_digest.clone()),
            vec![outgoing(&paths, "alpha")],
        )
        .unwrap_err();
        clear_fault();

        let app: crate::manifest::App = serde_yaml::from_str(
            "app: demo\nversion: 0.1.0\ndescription: x\nnodes:\n  - id: n\n    agent: alpha\n    command: go\nconnections: []\n",
        )
        .unwrap();
        let lock = crate::app_lock::LockFile {
            source_hash: "sha256:x".into(),
            compiled_at: "t".into(),
            compiler_version: "t".into(),
            app: "demo".into(),
            version: "0.1.0".into(),
            agent_pins: [("alpha".to_string(), "1.0.0".to_string())].into(),
            agent_bundle_pins: BTreeMap::new(),
            agent_digests: BTreeMap::new(),
            nodes: Vec::new(),
            schedule: None,
            engineering: None,
            front_door: None,
            approval: None,
        };
        let resolved = crate::agent_resolution::resolve_agents(
            &paths,
            &app,
            &lock,
            crate::agent_resolution::Selection::Default,
            &guard,
        )
        .unwrap_or_else(|e| panic!("{line}: the run must recover and resolve: {e}"));
        let digest = &resolved.info("alpha").unwrap().digest;
        assert!(
            digest == &old_digest || digest == &new_digest,
            "{line}: snapshot {digest} is neither the old nor the new tree"
        );
        let expected = if line.starts_with("done in") {
            &new_digest
        } else {
            &old_digest
        };
        assert_eq!(digest, expected, "{line}");
        assert_eq!(
            crate::install::integrity::tree_digest(&resolved.get("alpha").unwrap().root).unwrap(),
            *expected
        );
    }
}

/// One race of updates, a run and a compile over the same two agents. Every
/// worker reports its own outcome — `Err` carries the error or panic message —
/// so a worker that FAILS is told apart from one that never finishes (a
/// deadlock, bounded by `timeout`).
fn race_updates_runs_and_compiles(timeout: Duration) -> Result<(), String> {
    let (_tmp, paths) = home();
    write_tree(&paths.agents_dir().join("alpha"), "alpha", "1.0.0", "seed");
    write_tree(&paths.agents_dir().join("beta"), "beta", "1.0.0", "seed");
    let source_dir = paths.aware_home.join("src");
    std::fs::create_dir_all(&source_dir).unwrap();
    let source = source_dir.join("demo.flo");
    std::fs::write(
        &source,
        "app: demo\nversion: 0.1.0\ndescription: x\nnodes:\n  - id: a\n    agent: alpha\n    command: go\n  - id: b\n    agent: beta\n    command: go\nconnections: []\n",
    )
    .unwrap();
    {
        let guard = crate::agent_store::open(&paths).unwrap();
        crate::app_lock::compile_to_disk(&source, &paths, &guard).unwrap();
    }

    type Outcome = (&'static str, Result<(), String>);
    let (tx, rx) = std::sync::mpsc::channel::<Outcome>();
    let spawn = |name: &'static str, work: Box<dyn FnOnce() -> Result<(), AwareError> + Send>| {
        let tx = tx.clone();
        std::thread::spawn(move || {
            let outcome = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(work)) {
                Ok(Ok(())) => Ok(()),
                Ok(Err(error)) => Err(error.to_string()),
                Err(panic) => Err(panic
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| panic.downcast_ref::<&str>().map(|s| s.to_string()))
                    .unwrap_or_else(|| "panic".into())),
            };
            let _ = tx.send((name, outcome));
        })
    };
    let mut handles = Vec::new();
    for (name, id) in [("update-alpha", "alpha"), ("update-beta", "beta")] {
        let paths = paths.clone();
        handles.push(spawn(
            name,
            Box::new(move || {
                for n in 0..6 {
                    let guard = crate::agent_store::open(&paths)?;
                    // Same version, new bytes: the version-only fallback of
                    // the compiled lock is not what is under test.
                    let (staged, digest) = stage(&paths, id, "1.0.0", &format!("{name}{n}"));
                    let txn = begin(&paths, &guard, &[id], Some(staged))?;
                    txn.execute(
                        Op::Update,
                        Some(id),
                        Some(digest),
                        vec![outgoing(&paths, id)],
                    )?;
                }
                Ok(())
            }),
        ));
    }
    {
        let paths = paths.clone();
        let source = source.clone();
        handles.push(spawn(
            "compile",
            Box::new(move || {
                for _ in 0..6 {
                    let guard = crate::agent_store::open(&paths)?;
                    crate::app_lock::compile_to_disk(&source, &paths, &guard)?;
                }
                Ok(())
            }),
        ));
    }
    {
        let paths = paths.clone();
        let source = source.clone();
        handles.push(spawn(
            "run",
            Box::new(move || {
                for _ in 0..6 {
                    let guard = crate::agent_store::open(&paths)?;
                    // A run reads the approval under the guard, then resolves.
                    let (app, lock) = crate::app_lock::load_approved_app_with_lock(&source)?;
                    // The lock may be one compile behind the bytes: a refusal
                    // (a validation error) is a clean outcome; a hang or a raw
                    // IO error from a vanishing tree is not.
                    if let Err(error @ AwareError::Io(_)) = crate::agent_resolution::resolve_agents(
                        &paths,
                        &app,
                        &lock,
                        crate::agent_resolution::Selection::Default,
                        &guard,
                    ) {
                        return Err(error);
                    }
                }
                Ok(())
            }),
        ));
    }
    drop(tx);
    let mut finished = Vec::new();
    let mut failures = Vec::new();
    for _ in 0..handles.len() {
        match rx.recv_timeout(timeout) {
            Ok((name, Ok(()))) => finished.push(name),
            Ok((name, Err(error))) => failures.push(format!("{name}: {error}")),
            Err(_) => {
                return Err(format!(
                    "deadlock: only {finished:?} finished (failures: {failures:?}) within {timeout:?}"
                ));
            }
        }
    }
    for handle in handles {
        let _ = handle.join();
    }
    if !failures.is_empty() {
        return Err(format!("a racing worker failed: {}", failures.join("; ")));
    }
    if !txn_dirs(&paths).is_empty() {
        return Err("a swap transaction was left behind".into());
    }
    Ok(())
}

/// Plan 8 R1-1: one lock order everywhere (store, then swap locks sorted), so
/// updates, runs and compiles of the same agents racing each other always
/// finish — and none of them fails. Bounded by a timeout: a deadlock fails the
/// test instead of hanging. Repeated, because an interleaving bug shows up in
/// a fraction of runs only (review #627-a round 1: compile failed in ~1 of 5).
#[test]
fn updates_runs_and_compiles_racing_never_deadlock_or_fail() {
    for round in 0..40 {
        if let Err(error) = race_updates_runs_and_compiles(Duration::from_secs(120)) {
            panic!("round {round}: {error}");
        }
    }
}

/// The swap-lock API refuses a guard of another AWARE_HOME.
#[test]
fn a_guard_of_another_home_is_refused() {
    let (_a, one) = home();
    let (_b, two) = home();
    let guard = crate::agent_store::open(&one).unwrap();
    assert!(read_lock(&two, &guard, &["alpha"]).is_err());
    assert!(begin(&two, &guard, &["alpha"], None).is_err());
}

/// K7: readers of one agent never wait on each other (they never take its
/// swap lock exclusive unless there is a crashed swap to recover).
#[test]
fn concurrent_readers_of_one_agent_do_not_wait_on_each_other() {
    let (_tmp, paths) = home();
    write_tree(&paths.agents_dir().join("alpha"), "alpha", "1.0.0", "old");
    let guard = crate::agent_store::open(&paths).unwrap();
    let held = read_lock(&paths, &guard, &["alpha"]).unwrap();
    let other = paths.clone();
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let guard = crate::agent_store::open(&other).unwrap();
        let _second = read_lock(&other, &guard, &["alpha"]).unwrap();
        tx.send(()).unwrap();
    });
    rx.recv_timeout(Duration::from_secs(10))
        .expect("a second reader must not wait for the first");
    drop(held);
}

/// Codex P1: an update whose installed id and payload id differ only by case
/// (`Alpha` → `alpha`) names ONE directory on a case-insensitive filesystem.
/// Its swap must take one lock for it — two handles on one lock file, the
/// second exclusive request waiting on the first, would hang the update.
#[test]
fn ids_differing_only_by_case_take_one_swap_lock_and_never_hang() {
    let (_tmp, paths) = home();
    write_tree(&paths.agents_dir().join("Alpha"), "Alpha", "1.0.0", "old");
    let (tx, rx) = std::sync::mpsc::channel();
    let thread_paths = paths.clone();
    std::thread::spawn(move || {
        let guard = crate::agent_store::open(&thread_paths).unwrap();
        let outcome = begin(&thread_paths, &guard, &["Alpha", "alpha"], None)
            .map(|txn| txn.locks.ids().len());
        tx.send(outcome.map_err(|e| e.to_string())).unwrap();
    });
    let locked = rx
        .recv_timeout(Duration::from_secs(20))
        .expect("begin hung: the same lock file was locked twice by one transaction")
        .unwrap();
    assert_eq!(locked, 1, "one lock for one directory");
}

/// The outgoing set of a case-differing update is the ONE directory both ids
/// name where the filesystem folds case; where it does not and both exist as
/// two directories, the swap refuses rather than guess (nothing is changed).
#[test]
fn a_case_differing_update_moves_the_one_directory_or_refuses_two() {
    let (_tmp, paths) = home();
    write_tree(&paths.agents_dir().join("Alpha"), "Alpha", "1.0.0", "old");
    let entries = existing_dirs(&paths, &["Alpha", "alpha"]);
    let folds = paths.agents_dir().join("alpha").exists();
    if folds {
        assert_eq!(entries.unwrap(), vec!["Alpha".to_string()]);
        // And the whole update goes through.
        let guard = crate::agent_store::open(&paths).unwrap();
        let (staged, digest) = stage(&paths, "alpha", "2.0.0", "new");
        let txn = begin(&paths, &guard, &["Alpha", "alpha"], Some(staged)).unwrap();
        let out = existing_dirs(&paths, &["Alpha", "alpha"])
            .unwrap()
            .into_iter()
            .map(|id| outgoing(&paths, &id))
            .collect();
        txn.execute(Op::Update, Some("alpha"), Some(digest), out)
            .unwrap();
        let names: Vec<String> = std::fs::read_dir(paths.agents_dir())
            .unwrap()
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| n != SWAP_DIR)
            .collect();
        assert_eq!(names, vec!["alpha".to_string()]);
    } else {
        write_tree(&paths.agents_dir().join("alpha"), "alpha", "0.9.0", "other");
        let error = existing_dirs(&paths, &["Alpha", "alpha"]).unwrap_err();
        assert!(error.to_string().contains("differ only by case"), "{error}");
    }
}

/// Review #627-a: a kill DURING the final delete of a committed transaction
/// (`remove_dir_all` gone part-way: the journal and some of a moved-aside copy
/// deleted, the intent still there) must leave nothing a later recovery could
/// mistake for an unfinished swap — never rolling a half-deleted old copy back
/// into `agents/`, never E_AGENT_SWAP_RECOVERY for ever.
#[test]
fn a_kill_during_the_final_delete_never_resurrects_or_wedges_the_swap() {
    for op in ["uninstall", "update"] {
        let (_tmp, paths) = home();
        write_tree(&paths.agents_dir().join("alpha"), "alpha", "1.0.0", "old");
        inject_fault(Fault::InterruptFinish);
        let result = if op == "uninstall" {
            let guard = crate::agent_store::open(&paths).unwrap();
            crate::install::uninstall_agent("alpha", &paths, &guard)
        } else {
            update(&paths, "alpha", "alpha", "new")
        };
        clear_fault();
        result.unwrap();

        let guard = crate::agent_store::open(&paths).unwrap();
        let read = read_lock(&paths, &guard, &["alpha"]);
        assert!(read.is_ok(), "{op}: {:?}", read.err());
        drop(read);
        let now = maybe_tree(&paths.agents_dir().join("alpha"));
        if op == "uninstall" {
            assert_eq!(now, None, "{op}: the uninstalled agent came back");
        } else {
            let now = now.expect("updated");
            assert_eq!(now.len(), 9, "{op}: complete");
            assert!(String::from_utf8_lossy(&now["manifest.yaml"]).contains("2.0.0"));
        }
        let findings = recover_all(&paths, &guard).unwrap();
        assert!(
            findings.iter().all(|f| f.outcome != "needs-you"),
            "{op}: {findings:?}"
        );
        let left: Vec<_> = std::fs::read_dir(paths.agent_swap_dir())
            .unwrap()
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| n != LOCKS_DIR)
            .collect();
        assert!(
            left.is_empty(),
            "{op}: doctor cleans the leftover: {left:?}"
        );
    }
}

/// Review #627-a: `aware app check` after an update was killed between its
/// two renames must answer what the run would do — the run recovers the swap
/// and runs — not report the agent "not installed" for ever.
#[test]
fn app_check_after_an_interrupted_swap_answers_like_the_run() {
    let (_tmp, paths) = home();
    write_tree(&paths.agents_dir().join("alpha"), "alpha", "1.0.0", "old");
    let source_dir = paths.aware_home.join("src");
    std::fs::create_dir_all(&source_dir).unwrap();
    let source = source_dir.join("demo.flo");
    std::fs::write(
        &source,
        "app: demo\nversion: 0.1.0\ndescription: x\nnodes:\n  - id: a\n    agent: alpha\n    command: go\nconnections: []\n",
    )
    .unwrap();
    {
        let guard = crate::agent_store::open(&paths).unwrap();
        crate::app_lock::compile_to_disk(&source, &paths, &guard).unwrap();
    }
    inject_fault(Fault::CrashAfter("done out alpha".into()));
    update(&paths, "alpha", "alpha", "new").unwrap_err();
    clear_fault();
    assert!(!paths.agents_dir().join("alpha").exists());

    let check = crate::agent_resolution::check_app(&paths, &source).unwrap();
    assert!(
        check.approval_current,
        "check must recover the interrupted swap as the run does: {:?}",
        check.agents.iter().map(|a| &a.detail).collect::<Vec<_>>()
    );
    assert!(paths.agents_dir().join("alpha/manifest.yaml").is_file());
    assert!(txn_dirs(&paths).is_empty());
}
