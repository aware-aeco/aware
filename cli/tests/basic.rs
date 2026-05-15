//! Top-level smoke tests for the `aware` binary.
//!
//! These exercise the clap-derived argument surface without depending on
//! any subcommand's implementation. Every subcommand stub returns
//! `NotYetImplemented` in v0.1-dev; the smoke tests verify routing,
//! help-text generation, version reporting, and exit-code mapping.

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn top_level_help_works() {
    Command::cargo_bin("aware")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("aware"))
        .stdout(predicate::str::contains("agent"))
        .stdout(predicate::str::contains("app"))
        .stdout(predicate::str::contains("connect"))
        .stdout(predicate::str::contains("skill"))
        .stdout(predicate::str::contains("build"))
        .stdout(predicate::str::contains("doctor"));
}

#[test]
fn version_prints_dev_version() {
    Command::cargo_bin("aware")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("aware"))
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn agent_subcommand_help_works() {
    Command::cargo_bin("aware")
        .unwrap()
        .args(["agent", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("describe"))
        .stdout(predicate::str::contains("install"))
        .stdout(predicate::str::contains("validate"));
}

#[test]
fn app_subcommand_help_works() {
    Command::cargo_bin("aware")
        .unwrap()
        .args(["app", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("show"))
        .stdout(predicate::str::contains("run"))
        .stdout(predicate::str::contains("stop"));
}

#[test]
fn unimplemented_subcommand_exits_nonzero_with_message() {
    Command::cargo_bin("aware")
        .unwrap()
        .args(["agent", "install", "tekla"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not yet implemented"))
        .stderr(predicate::str::contains("agent install"));
}

#[test]
fn missing_subcommand_shows_help() {
    // `aware` with no subcommand should print help and exit successfully
    // (we configured `arg_required_else_help = true`).
    Command::cargo_bin("aware")
        .unwrap()
        .assert()
        .failure() // clap exits 2 on missing required arg
        .stderr(predicate::str::contains("Usage:"));
}
