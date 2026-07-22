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
fn json_output_has_every_example_app() {
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
    // Mirrors every `.app` under `30-apps/_examples/`. A strict equality keeps this
    // honest — adjust it whenever an example app lands or retires.
    // 7 → 8 (`model-to-renders` — IFC in, headless still + turntable out).
    assert_eq!(v["data"]["apps"].as_array().unwrap().len(), 8);
}
