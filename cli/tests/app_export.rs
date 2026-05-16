mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn exports_installed_app() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    std::fs::create_dir_all(aware.join("apps/welded-to-tc")).unwrap();
    let original = "app: welded-to-tc\nversion: 0.0.1\ndescription: x\nnodes: [{id: a}]\nconnections: []\nrequires: []";
    std::fs::write(aware.join("apps/welded-to-tc/welded-to-tc.flo"), original).unwrap();

    let out = tmp.path().join("exported.flo");
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "export", "welded-to-tc"])
        .arg(&out)
        .assert()
        .success()
        .stdout(predicate::str::contains("exported welded-to-tc"));
    let exported = std::fs::read_to_string(&out).unwrap();
    assert_eq!(exported, original);
}
