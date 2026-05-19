//! End-to-end test for `aware coverage validate` against the published
//! `aware-sidecar.exe`. Brings the codex-coverage review-protocol Step 4
//! schema validation in-tree (was ajv@8 / Python jsonschema before).
//!
//! Skipped automatically when the sidecar binary isn't built — the test
//! exercises the published native binary, not the C# DLL, so CI must run
//! `dotnet publish` before `cargo test`. The same pattern is used by
//! `coverage_e2e.rs` and `sidecar_integration.rs`.

mod common;

use assert_cmd::Command;
use predicates::prelude::*;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has parent")
        .to_path_buf()
}

fn sidecar_release() -> PathBuf {
    repo_root().join("cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe")
}

fn skip_if_no_sidecar(path: &Path) -> bool {
    if !path.exists() {
        eprintln!(
            "[skip] sidecar not found at {}; run: \
             dotnet publish cli-sidecar -c Release -r win-x64 --self-contained",
            path.display()
        );
        return true;
    }
    false
}

#[test]
fn validate_minimal_ir_succeeds() {
    let sidecar = sidecar_release();
    if skip_if_no_sidecar(&sidecar) {
        return;
    }
    let tmp = tempfile::tempdir().unwrap();
    let ir_fixture =
        repo_root().join("cli-sidecar/Ingest/Generator/Tests/fixtures/minimal.ir.json");
    assert!(
        ir_fixture.is_file(),
        "fixture not found at {}",
        ir_fixture.display()
    );

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("AWARE_SIDECAR", &sidecar)
        .args(["coverage", "validate"])
        .arg(&ir_fixture)
        .assert()
        .success()
        .stdout(predicate::str::contains("\"ok\":true"))
        .stdout(predicate::str::contains("status: ok"));
}

#[test]
fn validate_real_tekla_ir_succeeds() {
    let sidecar = sidecar_release();
    if skip_if_no_sidecar(&sidecar) {
        return;
    }
    let ir = repo_root().join("cli-sidecar/Ingest/Output/tekla-2025.0.ir.json");
    if !ir.is_file() {
        eprintln!("[skip] tekla-2025.0.ir.json not present in cli-sidecar/Ingest/Output/");
        return;
    }
    let tmp = tempfile::tempdir().unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("AWARE_SIDECAR", &sidecar)
        .args(["coverage", "validate"])
        .arg(&ir)
        .assert()
        .success()
        .stdout(predicate::str::contains("\"ok\":true"));
}

#[test]
fn validate_broken_ir_fails_with_violations() {
    let sidecar = sidecar_release();
    if skip_if_no_sidecar(&sidecar) {
        return;
    }
    let tmp = tempfile::tempdir().unwrap();
    let broken = tmp.path().join("broken.ir.json");
    // Missing host_version, source, types, metadata — schema requires all four.
    std::fs::write(&broken, r#"{"host":"demo"}"#).unwrap();

    let assertion = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("AWARE_SIDECAR", &sidecar)
        .args(["coverage", "validate"])
        .arg(&broken)
        .assert()
        .failure();

    // The CLI prints the JSON line to stdout, then the human summary, then
    // returns a Validation error (exit code 3) on the way out. We assert
    // BOTH outputs are present so we know the user sees the violations.
    let output = assertion.get_output();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stdout.contains("\"ok\":false"),
        "expected ok:false in stdout, got: {stdout}"
    );
    assert!(
        stdout.contains("IR violations"),
        "expected human summary on stdout, got: {stdout}"
    );
    assert!(
        stderr.contains("validation failed") || stderr.contains("violations"),
        "expected error on stderr, got: {stderr}"
    );
    // Exit code 3 per cli-spec.md § Exit codes (Validation error).
    assert_eq!(
        output.status.code(),
        Some(3),
        "expected exit code 3 (Validation), got {:?}",
        output.status.code()
    );
}

#[test]
fn validate_missing_ir_returns_not_found() {
    let sidecar = sidecar_release();
    if skip_if_no_sidecar(&sidecar) {
        return;
    }
    let tmp = tempfile::tempdir().unwrap();

    let assertion = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("AWARE_SIDECAR", &sidecar)
        .args(["coverage", "validate"])
        .arg(tmp.path().join("does-not-exist.ir.json"))
        .assert()
        .failure();

    // Exit code 7 (NotFound) per cli-spec.md.
    assert_eq!(assertion.get_output().status.code(), Some(7));
}

#[test]
fn validate_with_agent_dir_walks_catalog() {
    let sidecar = sidecar_release();
    if skip_if_no_sidecar(&sidecar) {
        return;
    }
    let tmp = tempfile::tempdir().unwrap();
    let ir_fixture =
        repo_root().join("cli-sidecar/Ingest/Generator/Tests/fixtures/minimal.ir.json");
    assert!(ir_fixture.is_file());

    // Build a synthetic agent dir with one bogus catalog file. The IR itself
    // is valid (we re-use the Generator's minimal fixture) but the catalog
    // file fails the per-Type schema, so the verb should return ok:false +
    // exit code 3.
    let agent_dir = tmp.path().join("synthetic-agent");
    let catalog = agent_dir.join("catalog");
    std::fs::create_dir_all(&catalog).unwrap();
    std::fs::write(catalog.join("Bogus.json"), r#"{"name":"X"}"#).unwrap();

    let assertion = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("AWARE_SIDECAR", &sidecar)
        .args(["coverage", "validate"])
        .arg(&ir_fixture)
        .args(["--agent-dir"])
        .arg(&agent_dir)
        .assert()
        .failure();

    let stdout = String::from_utf8_lossy(&assertion.get_output().stdout);
    assert!(
        stdout.contains("Catalog violations"),
        "expected catalog summary on stdout, got: {stdout}"
    );
    assert!(
        stdout.contains("Bogus.json"),
        "expected bogus catalog path on stdout, got: {stdout}"
    );
    assert_eq!(assertion.get_output().status.code(), Some(3));
}
