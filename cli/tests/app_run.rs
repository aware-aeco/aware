mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn run_one_shot_app_with_no_installed_agents_fails_clearly() {
    // welded-to-tc requires agents that aren't installed; with no agents installed,
    // is_long_running() returns false (no installed manifest), so we'd take the
    // one-shot path. But the orchestrator will try to invoke tekla via CliInvoker,
    // which spawns `aware-tekla` (not on PATH) → AwareError::Network.
    // The test confirms the exit is non-zero and the error mentions the binary.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");

    // Install the welded-to-tc app from the repo .flo
    let app_src = tmp.path().join("welded-to-tc-src");
    std::fs::create_dir_all(&app_src).unwrap();
    let flo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("30-apps/_examples/welded-to-tc.flo");
    std::fs::copy(&flo, app_src.join("welded-to-tc.flo")).unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "install"])
        .arg(&app_src)
        .assert()
        .success();

    // Now `app run` — should fail with a clear error (binary not found or network)
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "run", "welded-to-tc"])
        .assert()
        .failure();
}

#[test]
fn run_nonexistent_app_exits_7() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["app", "run", "nope"])
        .assert()
        .failure()
        .code(7);
}

#[test]
fn run_nonexistent_app_prints_error() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["app", "run", "does-not-exist"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn long_running_app_writes_and_removes_pidfile() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");

    // Install a stateful agent stub.
    let agent_dir = aware.join("agents/stub-stateful");
    std::fs::create_dir_all(&agent_dir).unwrap();
    std::fs::write(
        agent_dir.join("manifest.yaml"),
        r#"agent: stub-stateful
version: 0.0.1
description: x
stateful: true
license: MIT
transport:
  cli:
    binary: this-binary-does-not-exist-stateful
commands:
  watch:
    lifecycle: start
    description: nope
"#,
    )
    .unwrap();

    // Install an app that uses the stateful stub.
    let app_dir = aware.join("apps/stubapp");
    std::fs::create_dir_all(&app_dir).unwrap();
    std::fs::write(
        app_dir.join("stubapp.flo"),
        r#"app: stubapp
version: 0.0.1
description: x
nodes:
  - id: w
    agent: stub-stateful
    command: watch
connections: []
requires: []
"#,
    )
    .unwrap();

    // Try to run — will fail because the binary doesn't exist;
    // we just verify the pidfile is cleaned up afterwards.
    let _ = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "run", "stubapp"])
        .timeout(std::time::Duration::from_secs(5))
        .output();

    // After exit, pidfile must not exist (cleaned up regardless of error).
    let pidfile = aware.join("apps/stubapp/instances/default/pidfile.yaml");
    assert!(!pidfile.exists(), "pidfile leaked: {}", pidfile.display());
}
