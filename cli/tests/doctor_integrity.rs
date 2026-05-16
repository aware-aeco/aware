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

#[test]
fn doctor_lists_running_instances() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");

    // Create a fake pidfile
    let instance_dir = aware.join("apps/welded-to-tc/instances/default");
    std::fs::create_dir_all(&instance_dir).unwrap();
    std::fs::write(
        instance_dir.join("pidfile.yaml"),
        r#"app: welded-to-tc
instance: default
pid: 99999
started-at: 2026-05-16T14:23:00Z
run-id: r_abc123
"#,
    )
    .unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("Running:"))
        .stdout(predicate::str::contains("welded-to-tc/default"))
        .stdout(predicate::str::contains("99999"))
        .stdout(predicate::str::contains("r_abc123"));
}

#[test]
fn doctor_reports_no_running_instances_when_empty() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("Running:"))
        .stdout(predicate::str::contains("no running instances"));
}
