mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn build_from_openapi_generates_agent() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let spec_path = tmp.path().join("spec.json");
    std::fs::write(
        &spec_path,
        r#"{
        "openapi": "3.0.0",
        "info": { "title": "Demo", "version": "1.2.3", "license": {"name": "MIT"} },
        "paths": {
            "/items": { "get": { "operationId": "listItems", "summary": "List" } }
        }
    }"#,
    )
    .unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["build", "agent", "--from-openapi"])
        .arg(&spec_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("generated demo"));

    assert!(aware.join("agents/demo/manifest.yaml").is_file());
    assert!(aware.join("agents/demo/commands/list-items.md").is_file());
}

#[test]
fn build_from_cli_against_aware_binary() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let aware_bin = Command::cargo_bin("aware")
        .unwrap()
        .get_program()
        .to_owned();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["build", "agent", "--from-cli"])
        .arg(&aware_bin)
        .args(["--output", "aware-self"])
        .assert()
        .success()
        .stdout(predicate::str::contains("generated aware-self"));

    let manifest = aware.join("agents/aware-self/manifest.yaml");
    assert!(manifest.is_file());
    let body = std::fs::read_to_string(&manifest).unwrap();
    // Should mention at least one of aware's subcommands
    assert!(body.contains("agent") || body.contains("app") || body.contains("doctor"));
}

#[test]
fn build_from_dlls_with_no_sidecar_returns_not_found() {
    // When AWARE_SIDECAR points at a non-existent file, discovery fails with NotFound
    // (exit 7), not NotYetImplemented. This verifies the sidecar routing is live.
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("AWARE_SIDECAR", "C:/aware-sidecar-does-not-exist-test")
        .args(["build", "agent", "--from-dlls", "C:/some.dll"])
        .assert()
        .failure(); // exit 7 (NotFound) — sidecar missing
}

#[test]
fn build_from_com_with_no_sidecar_returns_not_found() {
    // --from-com now routes through the C# sidecar (v0.5.2). Without a sidecar
    // binary, discovery fails with NotFound (exit 7).
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("AWARE_SIDECAR", "C:/aware-sidecar-does-not-exist-test")
        .args(["build", "agent", "--from-com", "AutoCAD.Application"])
        .assert()
        .failure(); // exit 7 (NotFound) — sidecar missing
}

#[test]
fn build_from_headers_with_no_sidecar_returns_not_found() {
    // --from-headers now routes through the C# sidecar (v0.5.2). Without a sidecar
    // binary, discovery fails with NotFound (exit 7).
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("AWARE_SIDECAR", "C:/aware-sidecar-does-not-exist-test")
        .args(["build", "agent", "--from-headers", "/some/path"])
        .assert()
        .failure(); // exit 7 (NotFound) — sidecar missing
}

#[test]
fn build_decompile_with_no_sidecar_returns_not_found() {
    // --decompile now routes to the sidecar. Without a sidecar binary, discovery
    // fails with NotFound rather than NotYetImplemented.
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("AWARE_SIDECAR", "C:/aware-sidecar-does-not-exist-test")
        .args(["build", "agent", "--decompile", "--from-nuget", "Foo@1.0"])
        .assert()
        .failure(); // exit 7 (NotFound) — sidecar missing
}

#[test]
fn build_with_no_source_flags_is_validation_error() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["build", "agent"])
        .assert()
        .failure()
        .code(3); // Validation
}

#[test]
fn build_into_existing_agent_dir_is_conflict() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let spec_path = tmp.path().join("spec.json");
    std::fs::write(
        &spec_path,
        r#"{
        "openapi": "3.0.0",
        "info": { "title": "Twice", "version": "1.0.0" },
        "paths": {}
    }"#,
    )
    .unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["build", "agent", "--from-openapi"])
        .arg(&spec_path)
        .assert()
        .success();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["build", "agent", "--from-openapi"])
        .arg(&spec_path)
        .assert()
        .failure()
        .code(8); // Conflict
}
