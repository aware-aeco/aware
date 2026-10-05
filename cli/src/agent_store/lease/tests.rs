//! Run leases (#627-b, plan §2, §8 R1-9).

use super::*;

fn home() -> (tempfile::TempDir, Paths) {
    let tmp = tempfile::tempdir().unwrap();
    let paths = Paths {
        aware_home: tmp.path().to_path_buf(),
    };
    (tmp, paths)
}

fn package(agent: &str, hex: char) -> LeasePackage {
    LeasePackage {
        agent: agent.into(),
        version: "1.0.0".into(),
        digest: format!("sha256:{}", hex.to_string().repeat(64)),
        receipt_key: "no-receipt".into(),
        root: format!("/store/{agent}"),
        via: None,
    }
}

#[test]
fn a_held_lease_is_live_readable_and_released_with_its_packages_stamped() {
    let (_tmp, paths) = home();
    let guard = crate::agent_store::open(&paths).unwrap();
    let mut nested = package("alpha", 'b');
    nested.via = Some("inner".into());
    let record = LeaseRecord::new(
        "run-1",
        "demo",
        "default",
        vec![package("tekla", 'a'), nested],
    );
    let lease = RunLease::acquire(&paths, &guard, record.clone()).unwrap();
    drop(guard);

    // Readable while held, and live (another handle cannot take it).
    let listed = list(&paths).unwrap();
    assert_eq!(listed.leases.len(), 1, "{listed:?}");
    assert!(listed.stale.is_empty() && listed.unreadable.is_empty());
    let row = &listed.leases[0];
    assert!(row.live);
    assert_eq!(row.run_id, "run-1");
    assert_eq!(row.pid, std::process::id());
    assert_eq!(row.packages, record.packages);
    let path = lease.path().to_path_buf();
    assert!(is_live(&path).unwrap());

    // Released strictly later than acquired: the stamp must say so.
    std::thread::sleep(std::time::Duration::from_millis(20));
    let released_after = chrono::Utc::now() - chrono::Duration::milliseconds(1);
    drop(lease);
    assert!(!path.exists(), "a released lease is removed");
    for p in &record.packages {
        let stamped = crate::agent_store::stamps::last_needed(&paths, &p.agent, &p.digest)
            .unwrap_or_else(|| panic!("{} stamped", p.agent));
        let stamped = chrono::DateTime::parse_from_rfc3339(&stamped).unwrap();
        assert!(
            stamped >= released_after,
            "{} stamped on release ({stamped}), not only on acquire",
            p.agent
        );
    }
    assert!(list(&paths).unwrap().leases.is_empty());
}

/// A run that dies leaves its lease file but not its lock: it reads as stale,
/// never live — no heartbeat, no PID guess.
#[test]
fn a_lease_file_nobody_holds_is_stale() {
    let (_tmp, paths) = home();
    let dir = leases_dir(&paths);
    std::fs::create_dir_all(&dir).unwrap();
    let record = LeaseRecord::new("dead-run", "demo", "default", vec![package("tekla", 'a')]);
    std::fs::write(
        dir.join("dead-run.lease"),
        serde_json::to_vec(&record).unwrap(),
    )
    .unwrap();
    std::fs::write(dir.join("garbled.lease"), "{ not json").unwrap();
    let listed = list(&paths).unwrap();
    assert!(listed.leases.is_empty());
    assert_eq!(listed.stale.len(), 1);
    assert!(!listed.stale[0].live);
    assert_eq!(listed.unreadable.len(), 1);
    assert_eq!(listed.unreadable[0].live, Some(false));
}

/// A lease left by a process that was killed: a real child process holds it,
/// is killed, and the lease turns stale (the OS lock died with it).
#[test]
fn a_lease_turns_stale_when_its_process_is_killed() {
    let (_tmp, paths) = home();
    let dir = leases_dir(&paths);
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("child.lease");
    std::fs::write(&path, b"{}").unwrap();
    // A child that opens the file, takes a shared lock and waits.
    #[cfg(windows)]
    let mut child = std::process::Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-Command",
            &format!(
                "$f=[System.IO.File]::Open('{}','Open','ReadWrite','ReadWrite,Delete'); $f.Lock(0,1); Start-Sleep -Seconds 60",
                path.display()
            ),
        ])
        .spawn()
        .unwrap();
    #[cfg(unix)]
    if std::process::Command::new("flock")
        .arg("--version")
        .output()
        .is_err()
    {
        eprintln!("[skip] flock(1) is not installed");
        return;
    }
    #[cfg(unix)]
    let mut child = std::process::Command::new("flock")
        .args(["-s"])
        .arg(&path)
        .args(["sleep", "60"])
        .spawn()
        .unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    while !is_live(&path).unwrap() {
        assert!(
            std::time::Instant::now() < deadline,
            "the child never took the lock"
        );
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    child.kill().unwrap();
    child.wait().unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    while is_live(&path).unwrap() {
        assert!(
            std::time::Instant::now() < deadline,
            "the lock outlived its process"
        );
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

#[test]
fn a_lease_needs_the_store_guard_of_its_home_and_a_plain_run_id() {
    let (_tmp, paths) = home();
    let (_other_tmp, other) = home();
    let wrong = crate::agent_store::open(&other).unwrap();
    let record = LeaseRecord::new("run-2", "demo", "default", Vec::new());
    assert!(RunLease::acquire(&paths, &wrong, record).is_err());
    let guard = crate::agent_store::open(&paths).unwrap();
    let bad = LeaseRecord::new("../escape", "demo", "default", Vec::new());
    assert!(RunLease::acquire(&paths, &guard, bad).is_err());
    assert!(list(&paths).unwrap().leases.is_empty());
}

/// Review round 1: a lister probing liveness (`aware agent leases`, uninstall)
/// can hold the lease's lock for an instant between the run writing the file
/// and locking it. The run waits for it; it never aborts.
#[test]
fn a_liveness_probe_racing_a_starting_run_never_aborts_it() {
    let (_tmp, paths) = home();
    let guard = crate::agent_store::open(&paths).unwrap();
    let (released_tx, released_rx) = std::sync::mpsc::channel::<()>();
    on_before_lock(Box::new(move |path: &Path| {
        // A probe takes the lock exclusive (as `is_live` does) and lets go
        // a moment later, from another thread.
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .unwrap();
        file.try_lock_exclusive().unwrap();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(200));
            fs2::FileExt::unlock(&file).unwrap();
            drop(file);
            released_tx.send(()).unwrap();
        });
    }));
    let started = std::time::Instant::now();
    let lease = RunLease::acquire(
        &paths,
        &guard,
        LeaseRecord::new("run-race", "demo", "default", vec![package("tekla", 'a')]),
    )
    .expect("the run waited for the probe instead of failing");
    released_rx.recv().unwrap();
    assert!(started.elapsed() >= std::time::Duration::from_millis(150));
    assert!(is_live(lease.path()).unwrap());
}
