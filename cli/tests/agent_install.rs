mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn installs_tekla_from_local_path() {
    let tmp = tempfile::tempdir().unwrap();
    let src = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("20-agents/aeco/engineering/tekla");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["agent", "install"])
        .arg(&src)
        .assert()
        .success()
        .stdout(predicate::str::contains("tekla"));
    assert!(tmp.path().join("agents/tekla/manifest.yaml").is_file());
    assert!(
        tmp.path()
            .join("agents/tekla/skills/drawing-identity.md")
            .is_file()
    );
}

#[test]
fn rejects_install_of_already_installed_agent() {
    let tmp = tempfile::tempdir().unwrap();
    let src = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("20-agents/aeco/engineering/tekla");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["agent", "install"])
        .arg(&src)
        .assert()
        .success();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["agent", "install"])
        .arg(&src)
        .assert()
        .failure()
        .code(8); // Conflict
}
