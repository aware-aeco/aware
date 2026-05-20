mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn doctor_reports_installed_agents_and_apps() {
    // The fixture mirrors every `manifest.yaml` under `20-agents/` and every `.flo`/`.app`
    // under `30-apps/_examples/`. We assert doctor reports installed agents/apps rather than a
    // hardcoded count — the count drifts whenever an agent lands, and pinning it is brittle.
    let home = common::aware_home();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("CLI:"))
        .stdout(predicate::str::contains(format!(
            "aware v{}",
            env!("CARGO_PKG_VERSION")
        )))
        .stdout(predicate::str::contains("Filesystem:"))
        .stdout(predicate::str::contains("installed"));
}

#[test]
fn doctor_empty_home_succeeds() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("0 installed"));
}
