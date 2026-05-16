mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn installs_app_with_lockfile() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");

    let repo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let tekla_src = repo.join("20-agents/aeco/engineering/tekla");
    let tc_src = repo.join("20-agents/aeco/construction/trimble-connect");

    // Install the two agents the app requires
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["agent", "install"])
        .arg(&tekla_src)
        .assert()
        .success();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["agent", "install"])
        .arg(&tc_src)
        .assert()
        .success();

    // Build a one-file app dir with welded-to-tc.flo
    let app_src = tmp.path().join("welded-to-tc-src");
    std::fs::create_dir_all(&app_src).unwrap();
    let flo = repo.join("30-apps/_examples/welded-to-tc.flo");
    std::fs::copy(&flo, app_src.join("welded-to-tc.flo")).unwrap();

    // Install the app
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "install"])
        .arg(&app_src)
        .assert()
        .success()
        .stdout(predicate::str::contains("installed welded-to-tc"))
        .stdout(predicate::str::contains("lockfile"));

    // Lockfile exists with both agents pinned
    let lockfile = aware.join("apps/welded-to-tc/lockfile.yaml");
    assert!(
        lockfile.is_file(),
        "lockfile missing at {}",
        lockfile.display()
    );
    let body = std::fs::read_to_string(&lockfile).unwrap();
    assert!(body.contains("app: welded-to-tc"));
    assert!(body.contains("version: 0.3.1"));
    assert!(body.contains("tekla:"));
    assert!(body.contains("trimble-connect:"));
    assert!(body.contains("resolved-at:"));
}

#[test]
fn app_install_rejects_invalid_path() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["app", "install", "does-not-exist-anywhere"])
        .assert()
        .failure();
}
