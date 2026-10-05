//! `aware agent gc` (#629-b, plan §4 and §6 "Refs/GC").

use super::*;
use crate::app_lock::candidate::tests::{approve, home, update_agent, write_agent, write_app};

const ONE_TOOL: &str = "requires: []\nnodes:\n  - { id: a, agent: tool, command: go }\n";

fn window(text: &str) -> Window {
    Window::parse(text).unwrap()
}

fn options(apply: bool, window_text: &str) -> Options {
    Options {
        apply,
        window: window(window_text),
        agent: None,
        only: None,
        wait: Duration::ZERO,
    }
}

/// A stored package nothing references: snapshotted from a scratch folder.
fn orphan(paths: &Paths, id: &str, version: &str) -> (String, PathBuf) {
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
    let package = crate::agent_store::snapshot(paths, &dir, &guard).unwrap();
    drop(guard);
    std::fs::remove_dir_all(&dir).unwrap();
    (package.digest, package.root)
}

fn lock_beside_source(dir: &Path, app: &str, id: &str, digest: &str) {
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(dir.join(format!("{app}.flo")), format!("app: {app}\n")).unwrap();
    std::fs::write(
        dir.join(format!("{app}.lock")),
        format!(
            "source-hash: sha256:{}\ncompiled-at: t\ncompiler-version: v\napp: {app}\nversion: 0.1.0\n\
             agent-pins: {{ {id}: 1.0.0 }}\nagent-digests: {{ {id}: '{digest}' }}\nnodes: []\n",
            "0".repeat(64)
        ),
    )
    .unwrap();
}

fn home_bytes(paths: &Paths) -> BTreeMap<String, Vec<u8>> {
    crate::fs::plain_files_under(&paths.aware_home, "home")
        .unwrap()
        .into_iter()
        .filter(|(rel, _)| !rel.ends_with(".flock"))
        .map(|(rel, path)| (rel, std::fs::read(path).unwrap()))
        .collect()
}

use std::collections::BTreeMap;

#[test]
fn a_dry_run_lists_what_apply_would_remove_and_writes_nothing() {
    let h = home();
    let (removable, _) = orphan(&h.paths, "tool", "1.0.0");
    let (kept, _) = orphan(&h.paths, "tool", "2.0.0");
    lock_beside_source(&h.paths.apps_dir().join("demo"), "demo", "tool", &kept);
    drop(crate::agent_store::open(&h.paths).unwrap());
    let before = home_bytes(&h.paths);
    let report = collect(&h.paths, &options(false, "0s")).unwrap();
    assert!(!report.applied);
    assert_eq!(report.removed.len(), 1, "{report:#?}");
    assert_eq!(report.removed[0].digest, removable);
    assert_eq!(report.kept[0].digest, kept);
    assert_eq!(home_bytes(&h.paths), before, "a dry run writes nothing");
}

#[test]
fn apply_removes_only_removable_packages_whole_and_records_each_removal() {
    let h = home();
    let (removable, gone_path) = orphan(&h.paths, "tool", "1.0.0");
    let (kept, kept_path) = orphan(&h.paths, "tool", "2.0.0");
    let (recent, recent_path) = orphan(&h.paths, "other", "1.0.0");
    lock_beside_source(&h.paths.apps_dir().join("demo"), "demo", "tool", &kept);
    // `other` was snapshotted just now: in its window at 30d. Run GC "later"
    // for `tool` with 0s... the window is global, so stamp `other` instead.
    crate::agent_store::stamps::stamp(
        &h.paths,
        "other",
        &recent,
        Some(Utc::now() + chrono::Duration::days(10)),
    )
    .unwrap();
    let report = collect_at(
        &h.paths,
        &options(true, "1d"),
        Utc::now() + chrono::Duration::days(2),
    )
    .unwrap();
    assert!(
        report.applied && report.complete == Some(true),
        "{report:#?}"
    );
    assert_eq!(report.removed.len(), 1, "{report:#?}");
    assert_eq!(report.removed[0].digest, removable);
    assert!(!gone_path.exists(), "removed whole");
    assert!(kept_path.exists(), "an approved lock's package stays");
    assert!(recent_path.exists(), "a package in its window stays");
    assert_eq!(report.in_window[0].digest, recent);
    assert!(report.pending_delete.is_empty());
    // No trash left behind; a tombstone says what happened.
    let container = gone_path.parent().unwrap();
    let names: Vec<String> = std::fs::read_dir(container)
        .unwrap()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(names, [format!("{TOMBSTONE_PREFIX}no-receipt.yaml")]);
    let stone = tombstone(&h.paths, "tool", &removable).unwrap();
    assert_eq!(stone.version.as_deref(), Some("1.0.0"));
    assert_eq!(stone.removed_by, "aware agent gc");
    // Nothing else is removable now.
    let again = collect(&h.paths, &options(true, "1d")).unwrap();
    assert!(again.removed.is_empty(), "{again:#?}");
}

#[test]
fn apply_removes_nothing_while_the_table_is_incomplete() {
    let h = home();
    let (digest, path) = orphan(&h.paths, "tool", "1.0.0");
    let dir = h.paths.apps_dir().join("demo");
    lock_beside_source(&dir, "demo", "tool", &digest);
    std::fs::write(dir.join("demo.lock"), "nodes: [").unwrap();
    let error = collect(&h.paths, &options(true, "0s"))
        .unwrap_err()
        .to_string();
    assert!(error.contains("E_AGENT_GC_REFS_INCOMPLETE"), "{error}");
    assert!(error.contains("demo.lock"), "{error}");
    assert!(path.exists());
    // A dry run still reports, as data, and lists nothing as removable:
    // `--apply` would remove nothing (review round 1).
    let report = collect(&h.paths, &options(false, "0s")).unwrap();
    assert_eq!(report.complete, Some(false));
    assert_eq!(report.blockers.len(), 1);
    assert!(report.removed.is_empty(), "{report:#?}");
}

#[test]
fn only_refuses_a_referenced_package_and_removes_just_the_one_named() {
    let h = home();
    let (one, one_path) = orphan(&h.paths, "tool", "1.0.0");
    let (two, two_path) = orphan(&h.paths, "tool", "1.5.0");
    let (kept, kept_path) = orphan(&h.paths, "tool", "2.0.0");
    lock_beside_source(&h.paths.apps_dir().join("demo"), "demo", "tool", &kept);
    let only = |digest: &str| Options {
        only: Some(("tool".into(), digest.into())),
        ..options(true, "0s")
    };
    let error = collect(&h.paths, &only(&kept)).unwrap_err().to_string();
    assert!(error.contains("E_AGENT_GC_REFERENCED"), "{error}");
    assert!(error.contains("the approved workflow"), "{error}");
    assert!(error.contains("demo.lock"), "{error}");
    assert!(kept_path.exists());
    let error = collect(&h.paths, &only(&format!("sha256:{}", "f".repeat(64))))
        .unwrap_err()
        .to_string();
    assert!(error.contains("E_AGENT_GC_NOT_STORED"), "{error}");

    let report = collect(&h.paths, &only(&one)).unwrap();
    assert_eq!(report.removed.len(), 1);
    assert!(!one_path.exists());
    assert!(two_path.exists(), "--only removes just the one named");
    let _ = two;

    assert!(parse_only("tool@sha256:abc").is_err());
    assert!(parse_only("../x@sha256:".to_string().as_str()).is_err());
    assert_eq!(parse_only(&format!("tool@{one}")).unwrap().1, one);
}

#[test]
fn agent_limits_removal_to_one_agent() {
    let h = home();
    let (_, tool) = orphan(&h.paths, "tool", "1.0.0");
    let (_, other) = orphan(&h.paths, "other", "1.0.0");
    // An old leftover of the agent NOT named stays too (review round 4).
    let trash = tool.parent().unwrap().join(format!("{TRASH_PREFIX}x"));
    std::fs::create_dir_all(&trash).unwrap();
    let report = collect_at(
        &h.paths,
        &Options {
            agent: Some("other".into()),
            ..options(true, "0s")
        },
        Utc::now() + chrono::Duration::hours(2),
    )
    .unwrap();
    assert_eq!(report.removed.len(), 1);
    assert!(tool.exists() && !other.exists());
    assert!(trash.exists(), "--agent touches nothing outside its agent");
}

/// GC waits for no run: while anything holds the store lock shared it
/// reports `deferred: store-busy` and removes nothing.
#[test]
fn a_busy_store_defers_apply() {
    let h = home();
    let (_, path) = orphan(&h.paths, "tool", "1.0.0");
    let paths = h.paths.clone();
    let (held_tx, held_rx) = std::sync::mpsc::channel();
    let (done_tx, done_rx) = std::sync::mpsc::channel::<()>();
    let holder = std::thread::spawn(move || {
        let guard = crate::agent_store::open(&paths).unwrap();
        held_tx.send(()).unwrap();
        done_rx.recv().unwrap();
        drop(guard);
    });
    held_rx.recv().unwrap();
    let report = collect(&h.paths, &options(true, "0s")).unwrap();
    assert!(!report.applied);
    assert_eq!(report.deferred, Some("store-busy"));
    assert_eq!(report.complete, None, "deferred: no table was built");
    assert!(path.exists());
    done_tx.send(()).unwrap();
    holder.join().unwrap();
    let report = collect(&h.paths, &options(true, "0s")).unwrap();
    assert!(report.applied && !path.exists(), "{report:#?}");
}

/// Plan §9 R2-5: a stale lease is stamped first, then removed; its packages
/// get their window from that stamp.
#[test]
fn a_stale_lease_is_stamped_then_removed_and_its_packages_get_their_window() {
    let h = home();
    let (digest, path) = orphan(&h.paths, "tool", "1.0.0");
    let record = crate::agent_store::lease::LeaseRecord::new(
        "run-x",
        "demo",
        "default",
        vec![crate::agent_store::lease::LeasePackage {
            agent: "tool".into(),
            version: "1.0.0".into(),
            digest: digest.clone(),
            receipt_key: "no-receipt".into(),
            root: path.display().to_string(),
            via: None,
        }],
    );
    let dir = crate::agent_store::lease::leases_dir(&h.paths);
    std::fs::create_dir_all(&dir).unwrap();
    let lease = dir.join("run-x.lease");
    std::fs::write(&lease, serde_json::to_vec(&record).unwrap()).unwrap();
    let later = Utc::now() + chrono::Duration::days(100);
    let report = collect_at(&h.paths, &options(true, "30d"), later).unwrap();
    assert_eq!(report.stale_leases_removed, [lease.display().to_string()]);
    assert!(!lease.exists());
    assert!(path.exists(), "kept: its window starts at the stamp");
    assert_eq!(report.in_window[0].digest, digest);
    assert_eq!(
        crate::agent_store::stamps::last_needed(&h.paths, "tool", &digest).as_deref(),
        Some(now_text(later).as_str())
    );
}

/// Leftovers of dead processes go once they are old enough to be nobody's:
/// trash after 10 minutes (another GC may be deleting its own), interrupted
/// snapshots after an hour (an older CLI may still be writing one).
/// Unrecognized names are never removed.
#[test]
fn leftovers_of_dead_processes_are_removed_once_old_enough() {
    let h = home();
    let (digest, _) = orphan(&h.paths, "tool", "1.0.0");
    let container = crate::agent_store::digest_container(&h.paths, "tool", &digest).unwrap();
    let trash = container.join(format!("{TRASH_PREFIX}x"));
    let temp = container.join(".tmp-x");
    let odd = container.join("weird");
    for dir in [&trash, &temp, &odd] {
        std::fs::create_dir_all(dir.join("sub")).unwrap();
        std::fs::write(dir.join("sub").join("f"), "x").unwrap();
    }
    let now = Utc::now();
    let report = collect_at(&h.paths, &options(true, "30d"), now).unwrap();
    assert!(trash.exists() && temp.exists(), "too young: {report:#?}");
    let report = collect_at(
        &h.paths,
        &options(true, "30d"),
        now + chrono::Duration::minutes(15),
    )
    .unwrap();
    assert!(!trash.exists(), "{report:#?}");
    assert!(temp.exists(), "a young snapshot may be a live older CLI's");
    let report = collect_at(
        &h.paths,
        &options(true, "30d"),
        now + chrono::Duration::hours(2),
    )
    .unwrap();
    assert!(!temp.exists(), "{report:#?}");
    assert_eq!(report.leftovers_removed.len(), 1, "{report:#?}");
    assert!(odd.exists(), "unrecognized names are never removed");
    // `--only` touches nothing but the one package.
    std::fs::create_dir_all(&trash).unwrap();
    let error = collect_at(
        &h.paths,
        &Options {
            only: Some(("tool".into(), digest.clone())),
            ..options(true, "30d")
        },
        now + chrono::Duration::hours(2),
    )
    .unwrap_err()
    .to_string();
    assert!(
        error.contains("--recovery-window 0s"),
        "kept only by its window: {error}"
    );
    assert!(trash.exists(), "a refused --only changed nothing");
    // A successful --only removes its package and nothing else, however old
    // the leftovers beside it.
    let report = collect_at(
        &h.paths,
        &Options {
            only: Some(("tool".into(), digest.clone())),
            ..options(true, "0s")
        },
        now + chrono::Duration::hours(2),
    )
    .unwrap();
    assert_eq!(report.removed.len(), 1, "{report:#?}");
    assert!(report.leftovers_removed.is_empty(), "{report:#?}");
    assert!(trash.exists(), "--only touches nothing but its package");
}

/// A package Windows will not let go of (an open handle inside it) is
/// skipped, intact, with no tombstone; the next GC removes it.
#[cfg(windows)]
#[test]
fn a_package_in_use_is_skipped_intact() {
    use std::os::windows::fs::OpenOptionsExt;
    let h = home();
    let (digest, path) = orphan(&h.paths, "tool", "1.0.0");
    let held = std::fs::OpenOptions::new()
        .read(true)
        .share_mode(0)
        .open(path.join("manifest.yaml"))
        .unwrap();
    let report = collect(&h.paths, &options(true, "0s")).unwrap();
    assert!(report.removed.is_empty(), "{report:#?}");
    assert_eq!(report.skipped.len(), 1, "{report:#?}");
    assert!(path.join("manifest.yaml").exists());
    assert!(tombstone(&h.paths, "tool", &digest).is_none());
    drop(held);
    let report = collect(&h.paths, &options(true, "0s")).unwrap();
    assert_eq!(report.removed.len(), 1);
}

/// After GC removed an approved version, `app check` says so, and the run's
/// refusal names it — nothing else is run in its place (plan §4).
#[test]
fn app_check_says_gc_removed_the_approved_version() {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    let source = write_app(&h.paths, "demo", ONE_TOOL);
    let (_, lock_bytes) = approve(&h.paths, &source);
    update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    let lock = source.with_file_name("demo.lock");
    std::fs::remove_file(&lock).unwrap();
    let report = collect(&h.paths, &options(true, "0s")).unwrap();
    assert_eq!(report.removed.len(), 1, "{report:#?}");
    assert_eq!(report.removed[0].version.as_deref(), Some("1.0.0"));
    // The lock comes back (restored from a backup, say).
    std::fs::write(&lock, &lock_bytes).unwrap();
    let guard = crate::agent_store::open(&h.paths).unwrap();
    let check = crate::agent_resolution::check_app(&h.paths, &source, &guard).unwrap();
    let row = &check.agents[0];
    assert_eq!(
        row.resolution,
        crate::agent_resolution::Resolution::PinNotInstalled
    );
    assert_eq!(row.removed.as_ref().unwrap().by, "gc");
    assert!(
        row.detail.contains("removed by `aware agent gc`"),
        "{}",
        row.detail
    );
    assert!(
        row.detail.contains("compile the app again to use 1.0.1"),
        "{}",
        row.detail
    );
    assert!(!check.approval_current);
}

/// Review round 1: a legacy import that is due while a run holds the store
/// must not make `--apply` wait past `--wait`; it reports `store-busy`.
#[test]
fn a_due_legacy_import_does_not_make_apply_wait_past_its_bound() {
    let h = home();
    let paths = h.paths.clone();
    let (held_tx, held_rx) = std::sync::mpsc::channel();
    let (done_tx, done_rx) = std::sync::mpsc::channel::<()>();
    let holder = std::thread::spawn(move || {
        let guard = crate::agent_store::open(&paths).unwrap();
        held_tx.send(()).unwrap();
        done_rx.recv().unwrap();
        drop(guard);
    });
    held_rx.recv().unwrap();
    // An older CLI adds a package to its own store while the run is going.
    let legacy = h
        .paths
        .legacy_agent_store_dir()
        .join("tool")
        .join("a".repeat(64))
        .join(crate::agent_store::NO_RECEIPT);
    std::fs::create_dir_all(&legacy).unwrap();
    std::fs::write(legacy.join("manifest.yaml"), "x").unwrap();
    assert!(crate::agent_store::import::needed(&h.paths).unwrap());

    let paths = h.paths.clone();
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || tx.send(collect(&paths, &options(true, "0s"))).unwrap());
    let report = rx
        .recv_timeout(std::time::Duration::from_secs(20))
        .expect("gc --apply waited for the run instead of reporting store-busy")
        .unwrap();
    assert_eq!(report.deferred, Some("store-busy"));
    done_tx.send(()).unwrap();
    holder.join().unwrap();
}

/// Review round 2: the default dry run neither imports the legacy store nor
/// creates the new one, so it writes nothing and never waits for a run.
#[test]
fn a_dry_run_never_imports_or_creates_the_store() {
    let h = home();
    let legacy = h
        .paths
        .legacy_agent_store_dir()
        .join("tool")
        .join("a".repeat(64))
        .join(crate::agent_store::NO_RECEIPT);
    std::fs::create_dir_all(&legacy).unwrap();
    std::fs::write(legacy.join("manifest.yaml"), "x").unwrap();
    let report = collect(&h.paths, &options(false, "0s")).unwrap();
    assert!(report.removed.is_empty());
    assert!(
        !h.paths.agent_store_dir().exists(),
        "a dry run created the store"
    );
    assert!(
        crate::agent_store::import::needed(&h.paths).unwrap(),
        "a dry run imported the legacy store"
    );
}
