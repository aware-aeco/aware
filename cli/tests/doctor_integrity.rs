mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn doctor_reports_integrity_clean_for_real_fixtures() {
    let home = common::aware_home();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("Integrity:"))
        .stdout(predicate::str::contains(
            "all installed agents pass validation",
        ));
}

#[test]
fn doctor_flags_missing_skill_files() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let agent_dir = aware.join("agents/broken");
    std::fs::create_dir_all(agent_dir.join("skills")).unwrap();
    let manifest = r#"agent: broken
version: 1.0.0
description: an intentionally-broken agent
stateful: false
license: MIT
transport: { cli: { binary: x } }
commands: { do: { lifecycle: single, description: z } }
skills:
  - real.md
  - missing.md
"#;
    std::fs::write(agent_dir.join("manifest.yaml"), manifest).unwrap();
    std::fs::write(agent_dir.join("skills/real.md"), "# real skill\n").unwrap();
    // skills/missing.md intentionally absent

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("Integrity:"))
        .stdout(predicate::str::contains("Agent broken:"))
        .stdout(predicate::str::contains("E_SKILL_FILE_MISSING"))
        .stdout(predicate::str::contains("missing.md"));
}
