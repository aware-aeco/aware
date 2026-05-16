mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn uninstalls_app() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    // Set up a dummy installed app
    std::fs::create_dir_all(aware.join("apps/welded-to-tc")).unwrap();
    std::fs::write(
        aware.join("apps/welded-to-tc/welded-to-tc.flo"),
        "app: welded-to-tc\nversion: 0.0.1\ndescription: x\nnodes: [{id: a}]\nconnections: []\nrequires: []",
    )
    .unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "uninstall", "welded-to-tc"])
        .assert()
        .success()
        .stdout(predicate::str::contains("uninstalled welded-to-tc"));
    assert!(!aware.join("apps/welded-to-tc").exists());
}

#[test]
fn missing_app_exits_7() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["app", "uninstall", "nope"])
        .assert()
        .failure()
        .code(7);
}
