mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn shows_welded_to_tc_linear_topology() {
    let home = common::aware_home();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["app", "show", "welded-to-tc"])
        .assert()
        .success()
        .stdout(predicate::str::contains("app:"))
        .stdout(predicate::str::contains("welded-to-tc"))
        .stdout(predicate::str::contains("0.3.1"))
        .stdout(predicate::str::contains("Topology"))
        .stdout(predicate::str::contains("linear"))
        .stdout(predicate::str::contains("tekla-watch"))
        .stdout(predicate::str::contains("filter-welded"))
        .stdout(predicate::str::contains("tc-upload"))
        .stdout(predicate::str::contains("Requires:"));
}

#[test]
fn shows_qa_drawings_dag_topology() {
    let home = common::aware_home();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["app", "show", "qa-drawings-to-tekla"])
        .assert()
        .success()
        .stdout(predicate::str::contains("dag"))
        .stdout(predicate::str::contains("match-build"))
        .stdout(predicate::str::contains("pdf-extract → match-build"));
}

#[test]
fn missing_app_exits_7() {
    let home = common::aware_home();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["app", "show", "nope"])
        .assert()
        .failure()
        .code(7);
}
