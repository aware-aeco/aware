//! The store reference table (#629-a, plan §3 and §6 "Refs/GC: truth table").
//! Every reference kind is shown keeping a package on its own, against a
//! negative control in which nothing references it.

use super::*;
use crate::app_lock::candidate::tests::{
    Home, approve, home, update_agent, write_agent, write_app,
};

const ONE_TOOL: &str = "requires: []\nnodes:\n  - { id: a, agent: tool, command: go }\n";

fn days(n: i64) -> chrono::Duration {
    chrono::Duration::days(n)
}

fn window(text: &str) -> Window {
    Window::parse(text).unwrap()
}

fn build(paths: &Paths, window_text: &str, now: DateTime<Utc>) -> RefTable {
    let guard = crate::agent_store::open(paths).unwrap();
    table(paths, &guard, &window(window_text), now).unwrap()
}

/// A stored package of `id` that no working copy has: snapshotted from a
/// scratch folder outside `agents/`.
fn orphan(paths: &Paths, id: &str, version: &str) -> String {
    let dir = paths
        .aware_home
        .join("scratch")
        .join(format!("{id}-{version}"));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("manifest.yaml"),
        format!(
            "agent: {id}\nversion: {version}\ndescription: x\nstateful: false\nlicense: MIT\n\
             transport:\n  cli:\n    binary: aware-test\ncommands:\n  go:\n    lifecycle: single\n    mode: read\n    description: x\n"
        ),
    )
    .unwrap();
    let guard = crate::agent_store::open(paths).unwrap();
    let digest = crate::agent_store::snapshot(paths, &dir, &guard)
        .unwrap()
        .digest;
    drop(guard);
    std::fs::remove_dir_all(&dir).unwrap();
    digest
}

fn lock_text(app: &str, id: &str, digest: &str) -> String {
    format!(
        "source-hash: sha256:{}\ncompiled-at: t\ncompiler-version: v\napp: {app}\nversion: 0.1.0\n\
         agent-pins: {{ {id}: 1.0.0 }}\nagent-digests: {{ {id}: '{digest}' }}\nnodes: []\n",
        "0".repeat(64)
    )
}

/// An app folder at `dir` with a source and a lock pinning `digest`.
fn lock_beside_source(dir: &Path, app: &str, id: &str, digest: &str) -> PathBuf {
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(dir.join(format!("{app}.flo")), format!("app: {app}\n")).unwrap();
    let lock = dir.join(format!("{app}.lock"));
    std::fs::write(&lock, lock_text(app, id, digest)).unwrap();
    lock
}

fn row<'t>(table: &'t RefTable, digest: &str) -> &'t PackageRow {
    table
        .packages
        .iter()
        .find(|p| p.digest == digest)
        .unwrap_or_else(|| panic!("{digest} not in {table:#?}"))
}

fn kinds(row: &PackageRow) -> Vec<String> {
    row.references
        .iter()
        .map(|r| {
            serde_json::to_value(r).unwrap()["kind"]
                .as_str()
                .unwrap()
                .to_string()
        })
        .collect()
}

fn far(now: DateTime<Utc>) -> DateTime<Utc> {
    now + days(400)
}

#[test]
fn the_window_parses_units_and_refuses_anything_else() {
    assert_eq!(window("30d").duration(), days(30));
    assert_eq!(window("12h").duration(), chrono::Duration::hours(12));
    assert_eq!(window("5m").duration(), chrono::Duration::minutes(5));
    assert_eq!(window("0s").duration(), chrono::Duration::zero());
    for bad in [
        "",
        "d",
        "30",
        "-1d",
        "1.5d",
        "30w",
        "1 d",
        "99999999999999d",
    ] {
        let error = Window::parse(bad).unwrap_err().to_string();
        assert!(
            error.contains("E_AGENT_REFS_WINDOW_INVALID"),
            "{bad}: {error}"
        );
    }
}

#[test]
fn the_window_comes_from_the_flag_then_config_then_the_default() {
    let h = home();
    assert_eq!(
        recovery_window(&h.paths, None).unwrap().text(),
        DEFAULT_WINDOW
    );
    std::fs::write(
        h.paths.config_path(),
        "agent-store:\n  recovery-window: 7d\n",
    )
    .unwrap();
    assert_eq!(recovery_window(&h.paths, None).unwrap().duration(), days(7));
    assert_eq!(recovery_window(&h.paths, Some("1h")).unwrap().text(), "1h");
    // A malformed setting is an error, never a silent 30 days.
    std::fs::write(
        h.paths.config_path(),
        "agent-store:\n  recovery-window: 7\n",
    )
    .unwrap();
    assert!(recovery_window(&h.paths, None).is_err());
    std::fs::write(h.paths.config_path(), "agent-store: [\n").unwrap();
    assert!(recovery_window(&h.paths, None).is_err());
    // Unrelated config is fine.
    std::fs::write(h.paths.config_path(), "other: 1\n").unwrap();
    assert_eq!(
        recovery_window(&h.paths, None).unwrap().text(),
        DEFAULT_WINDOW
    );
}

#[test]
fn an_unreferenced_package_is_in_window_then_removable() {
    let h = home();
    let digest = orphan(&h.paths, "tool", "1.0.0");
    let now = Utc::now();
    let t = build(&h.paths, "30d", now);
    assert!(t.complete, "{t:#?}");
    let r = row(&t, &digest);
    assert_eq!(r.state, State::InWindow, "{r:#?}");
    assert_eq!(kinds(r), ["recent"]);
    let until = parse_time(r.kept_until.as_deref().unwrap()).unwrap();
    assert!(until > now + days(29) && until <= now + days(30), "{until}");
    assert_eq!(r.version, "1.0.0");
    assert!(r.bytes > 0);

    let r = row(&build(&h.paths, "30d", far(now)), &digest).clone();
    assert_eq!(r.state, State::Removable, "{r:#?}");
    assert!(r.references.is_empty());
    assert!(r.kept_until.is_none());
}

#[test]
fn a_last_needed_stamp_restarts_the_window() {
    let h = home();
    let digest = orphan(&h.paths, "tool", "1.0.0");
    let later = Utc::now() + days(100);
    crate::agent_store::stamps::stamp(&h.paths, "tool", &digest, Some(later)).unwrap();
    let r = row(&build(&h.paths, "30d", later + days(10)), &digest).clone();
    assert_eq!(r.state, State::InWindow, "{r:#?}");
    assert_eq!(
        parse_time(r.kept_until.as_deref().unwrap()).unwrap(),
        parse_time(&stamp(later)).unwrap() + days(30)
    );
    assert!(r.last_needed_at.is_some());
    // The window boundary itself: kept strictly before, removable at it.
    let edge = parse_time(r.kept_until.as_deref().unwrap()).unwrap();
    assert_eq!(
        row(
            &build(&h.paths, "30d", edge - chrono::Duration::seconds(1)),
            &digest
        )
        .state,
        State::InWindow
    );
    assert_eq!(
        row(&build(&h.paths, "30d", edge), &digest).state,
        State::Removable
    );
}

#[test]
fn an_approved_lock_keeps_its_pins_and_a_lock_with_no_source_beside_it_does_not() {
    let h = home();
    let digest = orphan(&h.paths, "tool", "1.0.0");
    let later = far(Utc::now());
    let lock = lock_beside_source(&h.paths.apps_dir().join("demo"), "demo", "tool", &digest);
    let t = build(&h.paths, "30d", later);
    let r = row(&t, &digest);
    assert_eq!(r.state, State::Kept);
    assert_eq!(
        r.references,
        [Reference::ApprovedLock {
            lock: lock.display().to_string()
        }]
    );
    assert_eq!(t.roots[0].kind, "apps");
    assert_eq!(t.roots[0].locks, 1);

    // Negative control: the same lock with no app source in its folder is
    // somebody else's `.lock` file, not an approval.
    std::fs::remove_file(h.paths.apps_dir().join("demo").join("demo.flo")).unwrap();
    let t = build(&h.paths, "30d", later);
    assert_eq!(row(&t, &digest).state, State::Removable);
    assert!(t.complete);
}

#[test]
fn an_app_whose_folder_name_differs_from_its_id_is_still_found() {
    let h = home();
    let digest = orphan(&h.paths, "tool", "1.0.0");
    lock_beside_source(
        &h.paths.apps_dir().join("renamed-folder"),
        "demo",
        "tool",
        &digest,
    );
    let t = build(&h.paths, "30d", far(Utc::now()));
    assert_eq!(row(&t, &digest).state, State::Kept);
}

#[test]
fn a_migration_candidate_and_a_promotion_in_progress_keep_their_pins() {
    let h = home();
    let candidate_digest = orphan(&h.paths, "tool", "1.0.0");
    let staged_digest = orphan(&h.paths, "tool", "2.0.0");
    let later = far(Utc::now());
    let dir = h.paths.apps_dir().join("demo");
    std::fs::create_dir_all(dir.join(".aware-migration")).unwrap();
    std::fs::write(
        dir.join(".aware-migration").join("demo.candidate.lock"),
        lock_text("demo", "tool", &candidate_digest),
    )
    .unwrap();
    let txn = dir.join(".aware-approvals").join(".txn").join("demo.abc");
    std::fs::create_dir_all(&txn).unwrap();
    std::fs::write(
        txn.join("next.lock"),
        lock_text("demo", "tool", &staged_digest),
    )
    .unwrap();

    let t = build(&h.paths, "30d", later);
    assert!(t.complete, "{t:#?}");
    let c = row(&t, &candidate_digest);
    assert_eq!(c.state, State::Kept);
    assert_eq!(kinds(c), ["candidate-lock"]);
    let s = row(&t, &staged_digest);
    assert_eq!(s.state, State::Kept);
    assert_eq!(kinds(s), ["promotion-in-progress"]);
}

#[test]
fn a_candidates_evidence_keeps_the_pins_it_moves_away_from() {
    let h = home();
    let base = orphan(&h.paths, "tool", "1.0.0");
    let dir = h.paths.apps_dir().join("demo").join(".aware-migration");
    std::fs::create_dir_all(&dir).unwrap();
    let evidence = serde_json::json!({
        "format": crate::migration::files::EVIDENCE_FORMAT,
        "app": "demo",
        "prepared-at": "2026-10-05T00:00:00Z",
        "cli-version": "x",
        "header": {
            "format": crate::app_lock::candidate::CANDIDATE_FORMAT,
            "app": "demo",
            "base-lock-digest": format!("sha256:{}", "1".repeat(64)),
            "base-source-hash": format!("sha256:{}", "2".repeat(64)),
            "targets": { "tool": {
                "from": { "version": "1.0.0", "digest": base },
                "to": { "version": "2.0.0", "digest": format!("sha256:{}", "3".repeat(64)) }
            }},
            "candidate-digest": format!("sha256:{}", "4".repeat(64)),
            "plan-digest": format!("sha256:{}", "5".repeat(64)),
        },
        "row": {},
    });
    std::fs::write(dir.join("demo.evidence.json"), evidence.to_string()).unwrap();
    let t = build(&h.paths, "30d", far(Utc::now()));
    assert_eq!(kinds(row(&t, &base)), ["candidate-base"]);
    assert_eq!(row(&t, &base).state, State::Kept);
}

#[test]
fn an_archived_approval_keeps_its_pins_for_the_window_and_for_ever_on_hold() {
    let h = home();
    let digest = orphan(&h.paths, "tool", "1.0.0");
    let dir = h.paths.apps_dir().join("demo");
    lock_beside_source(&dir, "demo", "tool", &orphan(&h.paths, "tool", "3.0.0"));
    let approvals = dir.join(".aware-approvals");
    std::fs::create_dir_all(&approvals).unwrap();
    let archive = approvals.join(format!("{}.lock", "a".repeat(64)));
    std::fs::write(&archive, lock_text("demo", "tool", &digest)).unwrap();
    // Evidence JSON beside it is not a lock.
    std::fs::write(approvals.join(format!("{}.json", "b".repeat(64))), "{}").unwrap();
    let now = Utc::now();

    // No successor names it: anchored at the archive's own time.
    let t = build(&h.paths, "30d", now + days(10));
    assert!(t.complete, "{t:#?}");
    let r = row(&t, &digest);
    assert_eq!(r.state, State::InWindow, "{r:#?}");
    assert!(kinds(r).contains(&"approval-archive".to_string()));
    assert_eq!(
        row(&build(&h.paths, "30d", far(now)), &digest).state,
        State::Removable
    );

    // A HOLD on the app keeps it for as long as the hold exists.
    std::fs::write(approvals.join("HOLD.demo"), "format: 1\n").unwrap();
    let t = build(&h.paths, "30d", far(now));
    let r = row(&t, &digest);
    assert_eq!(r.state, State::Kept, "{r:#?}");
    assert!(r.references.iter().any(|r| matches!(
        r,
        Reference::ApprovalArchive {
            held: true,
            until: None,
            ..
        }
    )));
}

#[test]
fn a_run_lease_keeps_its_packages_and_a_stale_one_keeps_them_until_gc_stamps_it() {
    let h = home();
    let digest = orphan(&h.paths, "tool", "1.0.0");
    let later = far(Utc::now());
    let package = crate::agent_store::lease::LeasePackage {
        agent: "tool".into(),
        version: "1.0.0".into(),
        digest: digest.clone(),
        receipt_key: "no-receipt".into(),
        root: "x".into(),
        via: None,
    };
    let record =
        crate::agent_store::lease::LeaseRecord::new("run-1", "demo", "default", vec![package]);
    let guard = crate::agent_store::open(&h.paths).unwrap();
    let lease =
        crate::agent_store::lease::RunLease::acquire(&h.paths, &guard, record.clone()).unwrap();
    drop(guard);
    let t = build(&h.paths, "30d", later);
    let r = row(&t, &digest);
    assert_eq!(r.state, State::Kept);
    assert_eq!(
        r.references,
        [Reference::Lease {
            run_id: "run-1".into(),
            app: "demo".into()
        }]
    );
    drop(lease);

    // A lease file nobody holds: the run ended without releasing it.
    let path = crate::agent_store::lease::leases_dir(&h.paths).join("run-2.lease");
    let mut stale = record.clone();
    stale.run_id = "run-2".into();
    std::fs::write(&path, serde_json::to_vec(&stale).unwrap()).unwrap();
    let t = build(&h.paths, "30d", later);
    assert_eq!(kinds(row(&t, &digest)), ["stale-lease-unstamped"]);
    assert_eq!(row(&t, &digest).state, State::Kept);
    assert_eq!(t.stale_leases.len(), 1);
    assert!(t.complete);

    // An unreadable lease: what it holds is unknown, so the table is incomplete.
    std::fs::write(&path, b"not json").unwrap();
    let t = build(&h.paths, "30d", later);
    assert!(!t.complete);
    assert!(
        t.blockers
            .iter()
            .any(|b| b.path == path.display().to_string()),
        "{t:#?}"
    );
}

#[test]
fn the_current_copy_keeps_its_bytes_and_an_unhashable_one_keeps_every_version() {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    let old = update_agent(&h.paths, "tool", "1.0.0", "mode: read");
    let new = update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    let later = far(Utc::now());
    let t = build(&h.paths, "30d", later);
    assert_eq!(kinds(row(&t, &new)), ["current"]);
    assert_eq!(row(&t, &new).state, State::Kept);
    assert_eq!(row(&t, &old).state, State::Removable);

    // A link inside the working copy makes it unhashable: every stored
    // version of the agent is kept, because which one it is cannot be known.
    let outside = h.paths.aware_home.join("outside");
    std::fs::create_dir_all(&outside).unwrap();
    let link = h.paths.agents_dir().join("tool").join("linked");
    #[cfg(windows)]
    assert!(
        std::process::Command::new("cmd")
            .args(["/C", "mklink", "/J"])
            .arg(&link)
            .arg(&outside)
            .output()
            .unwrap()
            .status
            .success()
    );
    #[cfg(unix)]
    std::os::unix::fs::symlink(&outside, &link).unwrap();
    let t = build(&h.paths, "30d", later);
    for digest in [&old, &new] {
        let r = row(&t, digest);
        assert_eq!(r.state, State::Kept, "{r:#?}");
        assert_eq!(kinds(r), ["current-unhashable"]);
    }
}

#[test]
fn a_carried_forward_lock_keeps_its_earlier_approvals_for_the_window_from_the_promotion() {
    use crate::app_lock::approval::SuccessorKind;
    use crate::app_lock::approval::test_support::{By, promote};
    let h: Home = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    let source = write_app(&h.paths, "demo", ONE_TOOL);
    let (base, _) = approve(&h.paths, &source);
    let old = crate::app_lock::pinned_digest(&base, "tool")
        .unwrap()
        .clone();
    let new = update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    let promoted = promote(
        &h.paths,
        &source,
        [(
            "tool".to_string(),
            crate::agent_resolution::PinTarget::Digest(new.clone()),
        )]
        .into(),
        By::Person("pawel"),
        SuccessorKind::CarriedForward,
    );
    // Re-date the promotion to now, so the window is measured from a time
    // this test controls (the helper writes a fixed date).
    let promoted_at = Utc::now();
    let text = std::fs::read_to_string(&promoted.lock_path).unwrap();
    let text = text.replace("2026-10-05T00:00:00Z", &stamp(promoted_at));
    std::fs::write(&promoted.lock_path, text).unwrap();
    // Snapshotting happened just before; the package's own recent use runs
    // out a little earlier than the promotion's window.
    let at = promoted_at + days(2);

    let t = build(&h.paths, "3d", at);
    assert!(t.complete, "{t:#?}");
    let r = row(&t, &old);
    assert_eq!(r.state, State::InWindow, "{r:#?}");
    let k = kinds(r);
    for kind in ["approval-original", "successor-from", "approval-archive"] {
        assert!(k.contains(&kind.to_string()), "{kind} missing from {k:?}");
    }
    assert_eq!(
        parse_time(r.kept_until.as_deref().unwrap()).unwrap(),
        parse_time(&stamp(promoted_at)).unwrap() + days(3)
    );
    assert_eq!(row(&t, &new).state, State::Kept);

    // After the window, nothing needs the old version.
    let t = build(&h.paths, "1d", at);
    assert_eq!(
        row(&t, &old).state,
        State::Removable,
        "{:#?}",
        row(&t, &old)
    );

    // A HOLD on the app keeps the earlier approval's version for ever.
    crate::migration::files::write_hold(
        crate::fs::containing_dir(&source),
        &serde_yaml::from_str(&format!(
            "format: 1\napp: demo\nheld-by: pawel\nheld-at: '{}'\n",
            stamp(Utc::now())
        ))
        .unwrap(),
    )
    .unwrap();
    let t = build(&h.paths, "1d", far(at));
    assert_eq!(row(&t, &old).state, State::Kept, "{:#?}", row(&t, &old));

    // The hold keeps it through the lock's own record too, not only through
    // the archives beside it.
    let approvals = crate::fs::containing_dir(&source).join(".aware-approvals");
    for entry in std::fs::read_dir(&approvals).unwrap().flatten() {
        if entry.path().extension().is_some_and(|e| e == "lock") {
            std::fs::remove_file(entry.path()).unwrap();
        }
    }
    let t = build(&h.paths, "1d", far(at));
    let r = row(&t, &old);
    assert_eq!(r.state, State::Kept, "{r:#?}");
    assert!(
        r.references.iter().all(|r| matches!(
            r,
            Reference::ApprovalOriginal { held: true, .. }
                | Reference::SuccessorFrom { held: true, .. }
        )),
        "{r:#?}"
    );
}

#[test]
fn an_expiring_reference_ends_exactly_at_its_time() {
    let h = home();
    let digest = orphan(&h.paths, "tool", "1.0.0");
    let approvals = h.paths.apps_dir().join("demo").join(".aware-approvals");
    std::fs::create_dir_all(&approvals).unwrap();
    let archive = approvals.join(format!("{}.lock", "a".repeat(64)));
    std::fs::write(&archive, lock_text("demo", "tool", &digest)).unwrap();
    // Push the archive's own time past the package's recent use, so the
    // archive is the only reference left near its end.
    let file = std::fs::OpenOptions::new()
        .write(true)
        .open(&archive)
        .unwrap();
    file.set_modified((Utc::now() + days(5)).into()).unwrap();
    drop(file);
    let end = modified(&archive).unwrap() + days(30);
    let before = row(
        &build(&h.paths, "30d", end - chrono::Duration::milliseconds(1)),
        &digest,
    )
    .clone();
    assert_eq!(before.state, State::InWindow, "{before:#?}");
    assert_eq!(kinds(&before), ["approval-archive"]);
    assert_eq!(
        row(&build(&h.paths, "30d", end), &digest).state,
        State::Removable
    );
}

#[test]
fn whatever_cannot_be_read_makes_the_table_incomplete() {
    let later = far(Utc::now());

    // An unparseable lock beside a source.
    let h = home();
    let dir = h.paths.apps_dir().join("demo");
    lock_beside_source(&dir, "demo", "tool", &format!("sha256:{}", "a".repeat(64)));
    std::fs::write(dir.join("demo.lock"), "nodes: [").unwrap();
    let t = build(&h.paths, "30d", later);
    assert!(!t.complete);
    assert_eq!(
        t.blockers[0].path,
        dir.join("demo.lock").display().to_string()
    );

    // An approval record of a format this CLI cannot read.
    let h = home();
    let dir = h.paths.apps_dir().join("demo");
    let lock = lock_beside_source(&dir, "demo", "tool", &format!("sha256:{}", "a".repeat(64)));
    let mut text = std::fs::read_to_string(&lock).unwrap();
    text.push_str(
        "approval:\n  format: 99\n  original:\n    lock-digest: x\n    archive: x\n    compiled-at: t\n    compiler-version: v\n    agent-pins: {}\n",
    );
    std::fs::write(&lock, text).unwrap();
    let t = build(&h.paths, "30d", later);
    assert!(!t.complete);
    assert!(t.blockers[0].problem.contains("format 99"), "{t:#?}");
    assert_eq!(t.blockers[0].path, lock.display().to_string());

    // An unreadable candidate and an unreadable archive.
    for (rel, name) in [
        (".aware-migration", "demo.candidate.lock"),
        (".aware-approvals", &*format!("{}.lock", "c".repeat(64))),
    ] {
        let h = home();
        let dir = h.paths.apps_dir().join("demo").join(rel);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join(name), "{{{").unwrap();
        let t = build(&h.paths, "30d", later);
        assert!(!t.complete, "{rel}/{name}");
    }

    // A registered root that has gone, and a malformed roots file.
    let h = home();
    let gone = h.paths.aware_home.join("gone");
    std::fs::create_dir_all(&gone).unwrap();
    let guard = crate::agent_store::open(&h.paths).unwrap();
    add_root(&h.paths, &guard, &gone, Some("floless")).unwrap();
    drop(guard);
    std::fs::remove_dir(&gone).unwrap();
    let t = build(&h.paths, "30d", later);
    assert!(!t.complete);
    assert_eq!(t.roots[1].status, "missing");
    std::fs::write(roots_path(&h.paths), "roots: [").unwrap();
    assert!(!build(&h.paths, "30d", later).complete);

    // Negative control: a home with nothing unreadable is complete.
    assert!(build(&home().paths, "30d", later).complete);
}

#[test]
fn a_registered_root_is_searched_like_apps_and_can_be_removed_again() {
    let h = home();
    let digest = orphan(&h.paths, "tool", "1.0.0");
    let later = far(Utc::now());
    let workspace = h.paths.aware_home.join("workspace");
    lock_beside_source(
        &workspace.join("project").join("flows").join("w1"),
        "w1",
        "tool",
        &digest,
    );
    assert_eq!(
        row(&build(&h.paths, "30d", later), &digest).state,
        State::Removable
    );

    let guard = crate::agent_store::open(&h.paths).unwrap();
    let (root, added) = add_root(&h.paths, &guard, &workspace, Some("floless")).unwrap();
    assert!(added);
    assert_eq!(root.label.as_deref(), Some("floless"));
    // Idempotent; a new label replaces the old.
    let (again, added) = add_root(&h.paths, &guard, &workspace, Some("other")).unwrap();
    assert!(!added);
    assert_eq!(again.label.as_deref(), Some("other"));
    assert_eq!(read_roots(&h.paths).unwrap().len(), 1);
    // Not a directory: refused, nothing recorded.
    assert!(add_root(&h.paths, &guard, &workspace.join("nope"), None).is_err());
    drop(guard);

    let t = build(&h.paths, "30d", later);
    assert_eq!(row(&t, &digest).state, State::Kept);
    assert_eq!(t.roots[1].kind, "registered");
    assert_eq!(t.roots[1].locks, 1);

    let guard = crate::agent_store::open(&h.paths).unwrap();
    assert!(remove_root(&h.paths, &guard, &workspace).unwrap());
    assert!(!remove_root(&h.paths, &guard, &workspace).unwrap());
    drop(guard);
    assert_eq!(
        row(&build(&h.paths, "30d", later), &digest).state,
        State::Removable
    );
}

#[test]
fn a_link_below_a_root_is_not_followed_and_is_reported() {
    let h = home();
    let digest = orphan(&h.paths, "tool", "1.0.0");
    let elsewhere = h.paths.aware_home.join("elsewhere");
    lock_beside_source(&elsewhere, "w1", "tool", &digest);
    std::fs::create_dir_all(h.paths.apps_dir()).unwrap();
    let link = h.paths.apps_dir().join("linked");
    #[cfg(windows)]
    assert!(
        std::process::Command::new("cmd")
            .args(["/C", "mklink", "/J"])
            .arg(&link)
            .arg(&elsewhere)
            .output()
            .unwrap()
            .status
            .success()
    );
    #[cfg(unix)]
    std::os::unix::fs::symlink(&elsewhere, &link).unwrap();
    let t = build(&h.paths, "30d", far(Utc::now()));
    assert_eq!(row(&t, &digest).state, State::Removable);
    assert_eq!(t.roots[0].links_not_followed, [link.display().to_string()]);
}

#[test]
fn an_invalid_package_is_kept_when_referenced_and_removable_after_the_window_when_not() {
    let h = home();
    let digest = orphan(&h.paths, "tool", "1.0.0");
    let package = crate::agent_store::digest_container(&h.paths, "tool", &digest)
        .unwrap()
        .join(crate::agent_store::NO_RECEIPT);
    std::fs::write(package.join("manifest.yaml"), "tampered").unwrap();
    let later = far(Utc::now());
    let t = build(&h.paths, "30d", later);
    assert!(t.packages.is_empty());
    assert_eq!(t.invalid_packages.len(), 1);
    assert_eq!(t.invalid_packages[0].state, State::Removable);
    assert!(t.invalid_packages[0].reason.contains("hash"), "{t:#?}");

    lock_beside_source(&h.paths.apps_dir().join("demo"), "demo", "tool", &digest);
    let t = build(&h.paths, "30d", later);
    assert_eq!(t.invalid_packages[0].state, State::Kept);
    assert_eq!(t.invalid_packages[0].references.len(), 1);
}

#[test]
fn leftovers_are_listed_by_kind() {
    let h = home();
    let digest = orphan(&h.paths, "tool", "1.0.0");
    let container = crate::agent_store::digest_container(&h.paths, "tool", &digest).unwrap();
    std::fs::create_dir_all(container.join(".tmp-1")).unwrap();
    std::fs::create_dir_all(container.join(format!("{TRASH_PREFIX}2"))).unwrap();
    std::fs::write(container.join(format!("{TOMBSTONE_PREFIX}x.yaml")), "x").unwrap();
    std::fs::create_dir_all(container.join("weird")).unwrap();
    std::fs::create_dir_all(h.paths.agent_store_dir().join("tool").join("not-a-digest")).unwrap();
    let t = build(&h.paths, "30d", Utc::now());
    let mut found: Vec<(&str, String)> = t
        .leftovers
        .iter()
        .map(|l| {
            (
                l.kind,
                Path::new(&l.path)
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
            )
        })
        .collect();
    found.sort();
    assert_eq!(
        found,
        [
            ("temp", ".tmp-1".to_string()),
            ("trash", format!("{TRASH_PREFIX}2")),
            ("unrecognized", "not-a-digest".to_string()),
            ("unrecognized", "weird".to_string()),
        ]
    );
    assert_eq!(t.packages.len(), 1);
}

#[test]
fn building_the_table_changes_nothing_on_disk() {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    let source = write_app(&h.paths, "demo", ONE_TOOL);
    approve(&h.paths, &source);
    update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    orphan(&h.paths, "other", "1.0.0");
    // `open` itself creates the store root; take it once before measuring.
    drop(crate::agent_store::open(&h.paths).unwrap());
    let snapshot = || -> BTreeMap<String, Vec<u8>> {
        crate::fs::plain_files_under(&h.paths.aware_home, "home")
            .unwrap()
            .into_iter()
            .filter(|(rel, _)| !rel.ends_with(".flock")) // lock files are touched by design
            .map(|(rel, path)| (rel, std::fs::read(path).unwrap()))
            .collect()
    };
    let before = snapshot();
    let t = build(&h.paths, "30d", Utc::now());
    assert!(t.complete, "{t:#?}");
    assert_eq!(snapshot(), before);
}

#[test]
fn the_legacy_store_is_reported_with_its_size() {
    let h = home();
    assert!(build(&h.paths, "30d", Utc::now()).legacy_store.is_none());
    let legacy = h.paths.legacy_agent_store_dir().join("x");
    std::fs::create_dir_all(&legacy).unwrap();
    std::fs::write(legacy.join("f"), [0u8; 10]).unwrap();
    let t = build(&h.paths, "30d", Utc::now());
    assert_eq!(t.legacy_store.unwrap().bytes, 10);
}

#[test]
fn the_table_needs_the_guard_of_its_own_home() {
    let a = home();
    let b = home();
    let guard = crate::agent_store::open(&a.paths).unwrap();
    assert!(table(&b.paths, &guard, &window("1d"), Utc::now()).is_err());
}
