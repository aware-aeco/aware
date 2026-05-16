mod common;

use assert_cmd::Command;
use predicates::prelude::*;

fn install_tekla(aware: &std::path::Path) {
    let tekla_src = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("20-agents/aeco/engineering/tekla");
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", aware)
        .args(["agent", "install"])
        .arg(&tekla_src)
        .assert()
        .success();
}

#[test]
fn skill_create_writes_template() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    install_tekla(&aware);
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["skill", "create", "tekla", "my-new-skill"])
        .assert()
        .success()
        .stdout(predicate::str::contains("created"));
    let path = aware.join("agents/tekla/skills/my-new-skill.md");
    assert!(path.is_file());
    let body = std::fs::read_to_string(&path).unwrap();
    assert!(body.contains("name: tekla-my-new-skill"));
}

#[test]
fn skill_port_between_agents() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    install_tekla(&aware);

    // Create a second target agent via the trimble-connect fixture
    let tc_src = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("20-agents/aeco/construction/trimble-connect");
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["agent", "install"])
        .arg(&tc_src)
        .assert()
        .success();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["skill", "port", "tekla/drawing-identity", "trimble-connect"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ported"));
    let ported = aware.join("agents/trimble-connect/skills/drawing-identity.md");
    assert!(ported.is_file());
    let body = std::fs::read_to_string(&ported).unwrap();
    assert!(body.contains("name: trimble-connect-drawing-identity"));
}

#[test]
fn skill_eval_creates_template_when_missing() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    install_tekla(&aware);
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["skill", "eval", "tekla", "drawing-identity"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Creating template"));
    let eval_path = aware.join("agents/tekla/skills/drawing-identity.eval.md");
    assert!(eval_path.is_file());
}

#[test]
fn skill_missing_is_not_found() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    install_tekla(&aware);
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["skill", "modify", "tekla", "does-not-exist"])
        .assert()
        .failure()
        .code(7);
}
