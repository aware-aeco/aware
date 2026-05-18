//! Integration tests for `aware build agent --from-coverage` (v0.30 A9).
//!
//! These verify the clap surface (flag parses, mutually exclusive routing) and
//! the IR-validation precondition. Full end-to-end coverage of the sidecar
//! path lives in cli-sidecar's own test suite plus `sidecar_integration.rs`.

mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn build_from_coverage_missing_ir_is_not_found() {
    // The Rust side validates that the IR file exists before spawning the
    // sidecar — so a missing path returns NotFound (exit 7) with a message
    // mentioning "IR file", regardless of whether the sidecar is installed.
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args([
            "build",
            "agent",
            "--agent-id",
            "test-agent",
            "--vendor",
            "test",
            "--vertical",
            "engineering",
            "--from-coverage",
            "C:/this-ir-file-does-not-exist-12345.json",
        ])
        .assert()
        .failure()
        .code(7)
        .stderr(predicate::str::contains("IR file"));
}

#[test]
fn build_from_coverage_requires_agent_id() {
    // Without --agent-id, dispatch returns a Validation error (exit 3).
    let tmp = tempfile::tempdir().unwrap();
    let ir_path = tmp.path().join("test.ir.json");
    std::fs::write(&ir_path, "{}").unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["build", "agent", "--from-coverage"])
        .arg(&ir_path)
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("--agent-id"));
}

#[test]
fn build_from_coverage_with_no_sidecar_returns_not_found() {
    // With a real IR file but no sidecar binary, discovery fails with NotFound
    // (exit 7) — the verb routed correctly, the wall is "sidecar missing".
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let ir_path = tmp.path().join("test.ir.json");
    std::fs::write(
        &ir_path,
        r#"{
        "host":"test","host_version":"1.0",
        "source":{"kind":"scrape","urls":[],"extracted_at":"2026-01-01T00:00:00Z"},
        "types":[],
        "metadata":{"page_count":0,"type_count":0,"method_count":0,"event_count":0,"property_count":0}
    }"#,
    )
    .unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env(
            "AWARE_SIDECAR",
            "C:/aware-sidecar-does-not-exist-for-coverage-test.exe",
        )
        .args([
            "build",
            "agent",
            "--agent-id",
            "test-agent",
            "--vendor",
            "test",
            "--vertical",
            "engineering",
            "--from-coverage",
        ])
        .arg(&ir_path)
        .assert()
        .failure()
        .code(7); // exit 7 (NotFound) — sidecar missing
}
