mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn prints_drawing_identity_skill() {
    let home = common::aware_home();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["agent", "skill", "tekla", "drawing-identity"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Mark"));
}

#[test]
fn accepts_filename_with_extension() {
    let home = common::aware_home();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["agent", "skill", "tekla", "drawing-identity.md"])
        .assert()
        .success();
}

#[test]
fn missing_agent_exits_7() {
    let home = common::aware_home();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["agent", "skill", "missing-agent", "anything"])
        .assert()
        .failure()
        .code(7);
}

#[test]
fn missing_skill_exits_7() {
    let home = common::aware_home();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["agent", "skill", "tekla", "nope-not-real"])
        .assert()
        .failure()
        .code(7);
}
