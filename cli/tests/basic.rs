//! Top-level smoke tests for the `aware` binary.
//!
//! These exercise the clap-derived argument surface without depending on
//! any subcommand's implementation. Commands still stubbed with
//! `NotYetImplemented` return exit code 1 with the message in stderr;
//! this file verifies routing, help-text, version reporting, and exit-code
//! mapping.
//!
//! NOTE: All v0.2 commands are now implemented, and all v0.3 runtime commands
//! (`app run`, `app stop`, `app logs`) are wired. All v0.4 commands (`connect`,
//! `disconnect`) are wired. All v0.5 tier-A and tier-B build sources and all
//! skill commands are wired. All v0.5.2 tier-C build sources (--from-dlls,
//! --from-com, --from-headers) are now wired through the C# sidecar.
//! The unimplemented-smoke test uses `agent publish` which is deferred to v0.6+.

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
fn version_prints_version() {
    Command::cargo_bin("aware")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("aware"))
        .stdout(predicate::str::contains("0.24.0"));
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
    // `agent publish` is a genuine NotYetImplemented stub (v0.6+).
    // All tier-C build sources (--from-dlls, --from-com, --from-headers) now
    // route through the C# sidecar and return NotFound when the sidecar is absent,
    // not NotYetImplemented — so they are no longer usable as sentinels here.
    Command::cargo_bin("aware")
        .unwrap()
        .args(["agent", "publish", "some-agent"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not yet implemented"));
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
