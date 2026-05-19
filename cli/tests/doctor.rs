mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn doctor_reports_installed_agents_and_apps() {
    // Counts grew from (57 agents / 7 apps) at v0.28 to (63 / 7) as v0.29 broadened vendor
    // coverage. The fixture mirrors every `manifest.yaml` under `20-agents/` and every `.flo`/
    // `.app` under `30-apps/_examples/`; bump the agent count whenever a new agent lands.
    let home = common::aware_home();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .arg("doctor")
        .assert()
        .success()
        .stdout(predicate::str::contains("CLI:"))
        .stdout(predicate::str::contains("aware v0.28.1"))
        .stdout(predicate::str::contains("Filesystem:"))
        .stdout(predicate::str::contains("63 installed"))
        .stdout(predicate::str::contains("7 installed"));
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
