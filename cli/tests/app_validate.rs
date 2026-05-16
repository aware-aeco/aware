mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn valid_app_exits_0() {
    let tmp = tempfile::tempdir().unwrap();
    let flo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("30-apps/_examples/welded-to-tc.flo");
    std::fs::copy(&flo, tmp.path().join("welded-to-tc.flo")).unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .args(["app", "validate"])
        .arg(tmp.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("is valid"));
}

#[test]
fn cyclic_app_exits_3() {
    let tmp = tempfile::tempdir().unwrap();
    let cyclic = r#"app: cyc
version: 0.0.1
description: x
nodes:
  - id: a
  - id: b
connections:
  - { from: a, to: b }
  - { from: b, to: a }
requires: []
"#;
    std::fs::write(tmp.path().join("cyc.flo"), cyclic).unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .args(["app", "validate"])
        .arg(tmp.path())
        .assert()
        .failure()
        .code(3)
        .stdout(predicate::str::contains("E_APP_CYCLE"));
}
