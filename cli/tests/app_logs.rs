mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn logs_for_missing_app_exits_7() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["app", "logs", "nope"])
        .assert()
        .failure()
        .code(7);
}

#[test]
fn logs_prints_jsonl_content() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let logs_dir = aware.join("logs/myapp/default");
    std::fs::create_dir_all(&logs_dir).unwrap();
    let fake_event = r#"{"kind":"run-start","ts":"2026-05-16T00:00:00Z","run_id":"r1","app":"myapp","instance":"default","config":{}}"#;
    std::fs::write(logs_dir.join("r1.jsonl"), format!("{fake_event}\n")).unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "logs", "myapp"])
        .assert()
        .success()
        .stdout(predicate::str::contains("run-start"))
        .stdout(predicate::str::contains("r1"));
}
