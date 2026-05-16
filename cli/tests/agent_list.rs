mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn lists_all_seven_fixture_agents() {
    let home = common::aware_home();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["agent", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ID"))
        .stdout(predicate::str::contains("VERSION"))
        .stdout(predicate::str::contains("KIND"))
        .stdout(predicate::str::contains("SKILLS"))
        .stdout(predicate::str::contains("COMMANDS"))
        .stdout(predicate::str::contains("tekla"))
        .stdout(predicate::str::contains("trimble-connect"))
        .stdout(predicate::str::contains("microsoft-365"))
        .stdout(predicate::str::contains("google-workspace"))
        .stdout(predicate::str::contains("html-report"))
        .stdout(predicate::str::contains("aware-agent-builder"))
        .stdout(predicate::str::contains("aware-skill-builder"));
}

#[test]
fn empty_home_lists_nothing_but_header() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["agent", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ID"));
}

#[test]
fn json_output_returns_envelope() {
    let home = common::aware_home();
    let output = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["--json", "agent", "list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let body = std::str::from_utf8(&output).unwrap();
    let v: serde_json::Value = serde_json::from_str(body).unwrap();
    assert_eq!(v["ok"], true);
    assert_eq!(v["meta"]["command"], "agent list");
    let agents = v["data"]["agents"].as_array().unwrap();
    assert_eq!(agents.len(), 19);
    assert!(agents.iter().any(|a| a["id"] == "tekla"));
}
