mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn valid_agent_exits_0() {
    let src = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("20-agents/aeco/engineering/tekla");
    Command::cargo_bin("aware")
        .unwrap()
        .args(["agent", "validate"])
        .arg(&src)
        .assert()
        .success()
        .stdout(predicate::str::contains("is valid"));
}

#[test]
fn missing_manifest_exits_nonzero() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .args(["agent", "validate"])
        .arg(tmp.path())
        .assert()
        .failure();
}

#[test]
fn bad_manifest_exits_3() {
    let tmp = tempfile::tempdir().unwrap();
    // Create a manifest missing required `transport` field — should fail serde load.
    let bad =
        "agent: x\nversion: 1.0\ndescription: y\nstateful: false\nlicense: MIT\ncommands: {}\n";
    std::fs::write(tmp.path().join("manifest.yaml"), bad).unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .args(["agent", "validate"])
        .arg(tmp.path())
        .assert()
        .failure()
        .code(3);
}

#[test]
fn missing_skill_file_exits_3_with_error_code() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("skills")).unwrap();
    let manifest = r#"agent: x
version: 1.0
description: y
stateful: false
license: MIT
transport: { cli: { binary: x } }
commands: { do: { lifecycle: single, description: z } }
skills:
  - missing.md
"#;
    std::fs::write(tmp.path().join("manifest.yaml"), manifest).unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .args(["agent", "validate"])
        .arg(tmp.path())
        .assert()
        .failure()
        .code(3)
        .stdout(predicate::str::contains("E_SKILL_FILE_MISSING"));
}
