mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn describes_tekla_agent() {
    let home = common::aware_home();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["agent", "describe", "tekla"])
        .assert()
        .success()
        .stdout(predicate::str::contains("agent:"))
        .stdout(predicate::str::contains("tekla"))
        .stdout(predicate::str::contains("2025.0.1"))
        .stdout(predicate::str::contains("stateful:"))
        .stdout(predicate::str::contains("watch"))
        .stdout(predicate::str::contains("insert"))
        .stdout(predicate::str::contains("save-attributes"))
        .stdout(predicate::str::contains("drawing-identity.md"));
}

#[test]
fn missing_agent_returns_not_found_exit_7() {
    let home = common::aware_home();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["agent", "describe", "does-not-exist"])
        .assert()
        .failure()
        .code(7);
}

#[test]
fn json_describe_returns_envelope() {
    let home = common::aware_home();
    let output = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["--json", "agent", "describe", "tekla"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let v: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(v["ok"], true);
    assert_eq!(v["data"]["agent"], "tekla");
    assert_eq!(v["data"]["skill_count"], 31);
    assert_eq!(v["data"]["command_count"], 20);
}
