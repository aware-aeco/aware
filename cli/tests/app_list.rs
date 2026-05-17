mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn lists_two_fixture_apps() {
    let home = common::aware_home();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["app", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ID"))
        .stdout(stdout_contains_version())
        .stdout(predicate::str::contains("NODES"))
        .stdout(predicate::str::contains("CONNS"))
        .stdout(predicate::str::contains("LAYOUT"))
        .stdout(predicate::str::contains("welded-to-tc"))
        .stdout(predicate::str::contains("qa-drawings-to-tekla"))
        .stdout(predicate::str::contains("linear"))
        .stdout(predicate::str::contains("dag"));
}

fn stdout_contains_version() -> predicates::str::ContainsPredicate {
    predicate::str::contains("VERSION")
}

#[test]
fn json_output_has_seven_apps() {
    let home = common::aware_home();
    let out = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["--json", "app", "list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let v: serde_json::Value = serde_json::from_slice(&out).unwrap();
    assert_eq!(v["ok"], true);
    assert_eq!(v["data"]["apps"].as_array().unwrap().len(), 7);
}
