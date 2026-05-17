mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn doctor_reports_twentythree_agents_and_two_apps() {
    let home = common::aware_home();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("CLI:"))
        .stdout(predicate::str::contains("aware v0.8.1"))
        .stdout(predicate::str::contains("Filesystem:"))
        .stdout(predicate::str::contains("23 installed"))
        .stdout(predicate::str::contains("2 installed"));
}

#[test]
fn doctor_empty_home_succeeds() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("0 installed"));
}
