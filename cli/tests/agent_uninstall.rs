mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn uninstalls_agent_from_local_install() {
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
    assert!(tmp.path().join("agents/tekla/manifest.yaml").is_file());

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["agent", "uninstall", "tekla"])
        .assert()
        .success()
        .stdout(predicate::str::contains("uninstalled"));
    assert!(!tmp.path().join("agents/tekla").exists());
}

#[test]
fn missing_agent_exits_7() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["agent", "uninstall", "nope"])
        .assert()
        .failure()
        .code(7);
}
