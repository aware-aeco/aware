//! Run leases through the real binary (#627-b): a run in progress holds a
//! lease naming the stored packages it uses; `aware agent leases` shows it;
//! uninstalling the tool says the run keeps its stored copy; finishing the run
//! releases the lease and stamps the package; a killed run leaves a stale one.
//!
//! The fixture agent's command is a tiny compiled helper that waits for a
//! "go" file before answering, so a test can look at a run while it runs.

use assert_cmd::Command;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

struct Fixture {
    _tmp: tempfile::TempDir,
    aware: PathBuf,
    go: PathBuf,
}

fn which(name: &str) -> Option<PathBuf> {
    std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths)
            .map(|dir| dir.join(name))
            .find(|candidate| candidate.is_file())
    })
}

/// A helper that waits (up to 60 s) for `go` to exist, then answers.
fn compile_waiter(dir: &Path, go: &Path) -> Option<PathBuf> {
    let rustc = which(if cfg!(windows) { "rustc.exe" } else { "rustc" })?;
    let src = dir.join("waiter.rs");
    let go = go.display().to_string().replace('\\', "/");
    std::fs::write(
        &src,
        format!(
            "use std::io::Read;\nfn main() {{ let mut s = String::new(); let _ = std::io::stdin().read_to_string(&mut s); \
             let start = std::time::Instant::now(); \
             while !std::path::Path::new(\"{go}\").exists() && start.elapsed().as_secs() < 60 {{ std::thread::sleep(std::time::Duration::from_millis(50)); }} \
             println!(\"{{{{\\\"version\\\":\\\"done\\\"}}}}\"); }}\n"
        ),
    )
    .unwrap();
    let bin = dir.join(if cfg!(windows) {
        "waiter.exe"
    } else {
        "waiter"
    });
    let status = std::process::Command::new(rustc)
        .arg(&src)
        .arg("-o")
        .arg(&bin)
        .status()
        .unwrap();
    assert!(status.success(), "rustc failed");
    Some(bin)
}

fn fixture() -> Option<Fixture> {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().to_path_buf();
    let go = root.join("go");
    let bin = compile_waiter(&root, &go)?;
    let aware = root.join("aware");
    let agent = aware.join("agents").join("waiter");
    std::fs::create_dir_all(&agent).unwrap();
    std::fs::write(
        agent.join("manifest.yaml"),
        format!(
            "agent: waiter\nversion: 1.0.0\ndescription: waits\nstateful: false\nlicense: MIT\n\
             transport:\n  cli:\n    binary: {}\n\
             commands:\n  say:\n    lifecycle: single\n    mode: read\n    description: wait then answer\n    \
             outputs:\n      type: single\n      schema:\n        version: {{ type: string }}\n",
            bin.display().to_string().replace('\\', "/")
        ),
    )
    .unwrap();
    let app = aware.join("apps").join("w");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("w.flo"),
        "app: w\nversion: 0.1.0\ndescription: waits for go\n\
         nodes:\n  - id: say\n    agent: waiter\n    command: say\nconnections: []\nrequires: []\n",
    )
    .unwrap();
    let fx = Fixture {
        _tmp: tmp,
        aware,
        go,
    };
    let source = app.join("w.flo");
    fx.ok(&["app", "compile", source.to_str().unwrap()]);
    Some(fx)
}

impl Fixture {
    fn aware(&self) -> Command {
        let mut command = Command::cargo_bin("aware").unwrap();
        command.env("AWARE_HOME", &self.aware);
        command
    }

    fn ok(&self, args: &[&str]) -> String {
        let output = self.aware().args(args).output().unwrap();
        assert!(
            output.status.success(),
            "aware {args:?} failed:\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8_lossy(&output.stdout).into_owned()
    }

    fn leases(&self) -> serde_json::Value {
        let out = self.ok(&["--json", "agent", "leases"]);
        let envelope: serde_json::Value = serde_json::from_str(out.trim()).unwrap();
        assert_eq!(envelope["ok"], true, "{envelope}");
        envelope["data"].clone()
    }

    /// Start `aware app run w` in the background.
    fn start_run(&self) -> std::process::Child {
        std::process::Command::new(assert_cmd::cargo::cargo_bin("aware"))
            .env("AWARE_HOME", &self.aware)
            .args(["app", "run", "w"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .unwrap()
    }

    fn wait_for_lease(&self) -> serde_json::Value {
        let deadline = Instant::now() + Duration::from_secs(60);
        loop {
            let leases = self.leases();
            if let Some(lease) = leases["leases"].as_array().and_then(|l| l.first()) {
                return lease.clone();
            }
            assert!(Instant::now() < deadline, "no lease appeared: {leases}");
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    fn approved_digest(&self) -> String {
        let lock: serde_yaml::Value =
            serde_yaml::from_slice(&std::fs::read(self.aware.join("apps/w/w.lock")).unwrap())
                .unwrap();
        lock["agent-digests"]["waiter"]
            .as_str()
            .unwrap()
            .to_string()
    }
}

#[test]
fn a_run_holds_a_lease_until_it_finishes_and_then_stamps_its_packages() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    assert_eq!(fx.leases()["leases"], serde_json::json!([]));
    let mut run = fx.start_run();
    let lease = fx.wait_for_lease();
    assert_eq!(lease["app"], "w");
    assert_eq!(lease["instance"], "default");
    assert_eq!(lease["live"], true);
    assert_eq!(lease["pid"], run.id());
    let digest = fx.approved_digest();
    assert_eq!(lease["packages"][0]["agent"], "waiter");
    assert_eq!(lease["packages"][0]["digest"], digest.as_str());
    assert!(
        lease["packages"][0]["root"]
            .as_str()
            .unwrap()
            .replace('\\', "/")
            .contains("/agent-store-v2/waiter/"),
        "{lease}"
    );

    // Uninstalling the tool mid-run: the run keeps its stored copy.
    let out = fx.ok(&["agent", "uninstall", "waiter"]);
    assert!(
        out.contains("1 run(s) in progress still use waiter"),
        "{out}"
    );

    std::fs::write(&fx.go, "go").unwrap();
    let status = run.wait().unwrap();
    assert!(status.success(), "the run finished on its stored copy");
    let leases = fx.leases();
    assert_eq!(leases["leases"], serde_json::json!([]), "{leases}");
    assert_eq!(leases["stale"], serde_json::json!([]), "{leases}");
    let hex = digest.strip_prefix("sha256:").unwrap();
    let stamp = fx
        .aware
        .join("agent-store-control/refs/waiter")
        .join(format!("{hex}.last-needed"));
    assert!(stamp.is_file(), "the package was stamped when released");
}

#[test]
fn a_killed_run_leaves_a_stale_lease_never_a_live_one() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    let mut run = fx.start_run();
    let lease = fx.wait_for_lease();
    let run_id = lease["run-id"].as_str().unwrap().to_string();
    run.kill().unwrap();
    run.wait().unwrap();
    // Let the orphaned helper finish too.
    std::fs::write(&fx.go, "go").unwrap();
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let leases = fx.leases();
        let stale = leases["stale"].as_array().cloned().unwrap_or_default();
        if leases["leases"] == serde_json::json!([]) && !stale.is_empty() {
            assert_eq!(stale[0]["run-id"], run_id.as_str());
            assert_eq!(stale[0]["live"], false);
            break;
        }
        assert!(
            Instant::now() < deadline,
            "the lease outlived its process: {leases}"
        );
        std::thread::sleep(Duration::from_millis(100));
    }
}

impl Fixture {
    fn gc(&self, args: &[&str]) -> serde_json::Value {
        let mut full = vec!["--json", "agent", "gc"];
        full.extend_from_slice(args);
        let out = self.ok(&full);
        let envelope: serde_json::Value = serde_json::from_str(out.trim()).unwrap();
        assert_eq!(envelope["ok"], true, "{envelope}");
        envelope["data"].clone()
    }

    /// Nothing but the run itself needs the approved package any more.
    fn drop_every_other_reference(&self) {
        self.ok(&["agent", "uninstall", "waiter"]);
        std::fs::remove_dir_all(self.aware.join("apps/w")).unwrap();
    }
}

fn digests(rows: &serde_json::Value) -> Vec<String> {
    rows.as_array()
        .unwrap()
        .iter()
        .map(|r| r["digest"].as_str().unwrap().to_string())
        .collect()
}

/// #629-b: GC never removes what a run in progress is using; once the run
/// has finished, the package is removable like any other.
#[test]
fn gc_keeps_what_a_run_in_progress_uses_and_removes_it_after() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    let digest = fx.approved_digest();
    let mut run = fx.start_run();
    fx.wait_for_lease();
    fx.drop_every_other_reference();
    let report = fx.gc(&["--apply", "--recovery-window", "0s"]);
    assert_eq!(report["applied"], true, "{report}");
    assert!(digests(&report["removed"]).is_empty(), "{report}");
    assert_eq!(
        digests(&report["kept"]),
        std::slice::from_ref(&digest),
        "{report}"
    );
    assert_eq!(report["kept"][0]["references"][0]["kind"], "lease");

    std::fs::write(&fx.go, "go").unwrap();
    assert!(
        run.wait().unwrap().success(),
        "the run finished on its copy"
    );
    let report = fx.gc(&["--apply", "--recovery-window", "0s"]);
    assert_eq!(digests(&report["removed"]), [digest], "{report}");
}

/// #629-b / plan §9 R2-5: a killed run's stale lease keeps its package until
/// GC stamps it; GC then removes the lease. (That the window then counts from
/// GC's stamp is shown with a moved clock by the unit test
/// `a_stale_lease_is_stamped_then_removed_and_its_packages_get_their_window`;
/// here the run's own acquire stamp is recent too — review round 1.)
#[test]
fn gc_stamps_and_releases_a_killed_run_s_lease() {
    let Some(fx) = fixture() else {
        eprintln!("[skip] rustc not on PATH");
        return;
    };
    let digest = fx.approved_digest();
    let mut run = fx.start_run();
    fx.wait_for_lease();
    run.kill().unwrap();
    run.wait().unwrap();
    std::fs::write(&fx.go, "go").unwrap();
    let deadline = Instant::now() + Duration::from_secs(30);
    while fx.leases()["stale"].as_array().is_none_or(|s| s.is_empty()) {
        assert!(Instant::now() < deadline, "no stale lease");
        std::thread::sleep(Duration::from_millis(100));
    }
    fx.drop_every_other_reference();
    let report = fx.gc(&["--apply", "--recovery-window", "30d"]);
    assert_eq!(
        report["stale-leases-removed"].as_array().unwrap().len(),
        1,
        "{report}"
    );
    assert_eq!(digests(&report["in-window"]), [digest], "{report}");
    assert!(digests(&report["removed"]).is_empty());
    assert_eq!(fx.leases()["stale"], serde_json::json!([]));
}
