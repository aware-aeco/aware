mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn stop_with_no_pidfile_exits_nonzero() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["app", "stop", "nonexistent"])
        .assert()
        .failure();
}

#[test]
fn stop_with_pidfile_removes_pidfile_even_if_process_gone() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let instance_dir = aware.join("apps/myapp/instances/default");
    std::fs::create_dir_all(&instance_dir).unwrap();
    // Write a fake pidfile pointing at PID 999999 (which we don't have permission to signal —
    // that's fine; the test just verifies the pidfile gets removed after the stop call regardless).
    std::fs::write(
        instance_dir.join("pidfile.yaml"),
        r#"app: myapp
instance: default
pid: 999999
started-at: 2026-05-16T00:00:00Z
run-id: r_fake
"#,
    )
    .unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "stop", "myapp"])
        .assert()
        .success()
        .stdout(predicate::str::contains("stopped myapp"));
    assert!(!instance_dir.join("pidfile.yaml").exists());
}
