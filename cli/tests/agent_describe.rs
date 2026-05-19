mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn describes_tekla_agent() {
    let home = common::aware_home();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["agent", "describe", "tekla"])
        .assert()
        .success()
        .stdout(predicate::str::contains("agent:"))
        .stdout(predicate::str::contains("tekla"))
        // The curated tekla manifest uses the version-agnostic `2025+` shorthand for
        // sdk-target (a structured form will land in a later hardening pass).
        .stdout(predicate::str::contains("2025+"))
        .stdout(predicate::str::contains("stateful:"))
        .stdout(predicate::str::contains("watch"))
        .stdout(predicate::str::contains("insert"))
        .stdout(predicate::str::contains("save-attributes"))
        .stdout(predicate::str::contains("drawing-identity.md"));
}

#[test]
fn missing_agent_returns_not_found_exit_7() {
    let home = common::aware_home();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["agent", "describe", "does-not-exist"])
        .assert()
        .failure()
        .code(7);
}

#[test]
fn json_describe_returns_envelope() {
    let home = common::aware_home();
    let output = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["--json", "agent", "describe", "tekla"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let v: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(v["ok"], true);
    assert_eq!(v["data"]["agent"], "tekla");
    // v0.29 added 2 runtime skills + 2 runtime verbs to the curated tekla manifest
    // (33 skills total, 22 commands). The v0.28 baseline was 31 / 20. The agent.rs unit
    // tests have the same expectation — see `cli/src/manifest/agent.rs`.
    assert_eq!(v["data"]["skill_count"], 33);
    assert_eq!(v["data"]["command_count"], 22);
}
