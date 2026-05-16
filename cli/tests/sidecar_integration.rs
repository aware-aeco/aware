mod common;

use assert_cmd::Command;
use predicates::prelude::*;

fn sidecar_binary() -> Option<std::path::PathBuf> {
    let repo_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let candidate =
        repo_root.join("cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe");
    if candidate.is_file() {
        return Some(candidate);
    }
    // Also try non-Windows path layouts
    let alt = repo_root.join("cli-sidecar/bin/Release/net9.0/linux-x64/publish/aware-sidecar");
    if alt.is_file() {
        return Some(alt);
    }
    None
}

fn fixture_dll() -> Option<std::path::PathBuf> {
    let repo_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    // Try the test fixture build output (Debug build is fine; we only need the assembly metadata)
    for cfg in ["Debug", "Release"] {
        let candidate = repo_root.join(format!(
            "cli-sidecar/Tests/FixtureAssembly/bin/{cfg}/net9.0/FixtureAssembly.dll"
        ));
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

#[test]
fn sidecar_e2e_reflects_fixture_dll_into_agent() {
    let Some(sidecar) = sidecar_binary() else {
        eprintln!(
            "[skip] sidecar binary not found; run `dotnet publish cli-sidecar -c Release -r win-x64 -p:PublishAot=true`"
        );
        return;
    };
    let Some(fixture) = fixture_dll() else {
        eprintln!(
            "[skip] fixture DLL not found; run `dotnet build cli-sidecar/Tests/FixtureAssembly`"
        );
        return;
    };

    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_SIDECAR", &sidecar)
        .args(["build", "agent", "--from-dlls"])
        .arg(&fixture)
        .args(["--output", "e2e-fixture"])
        .assert()
        .success()
        .stdout(predicate::str::contains("generated e2e-fixture"));

    // Verify the agent landed on disk
    assert!(aware.join("agents/e2e-fixture/manifest.yaml").is_file());
    let manifest = std::fs::read_to_string(aware.join("agents/e2e-fixture/manifest.yaml")).unwrap();
    // Should have at least one of Greeter's methods as a command
    assert!(
        manifest.contains("greeter-greet") || manifest.contains("greeter-count-occurrences"),
        "manifest did not contain expected Greeter commands:\n{manifest}"
    );
}

#[test]
fn sidecar_missing_binary_returns_not_found() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env(
            "AWARE_SIDECAR",
            "C:/this-binary-does-not-exist-12345-aware-sidecar.exe",
        )
        .args(["build", "agent", "--from-dlls", "C:/some.dll"])
        .assert()
        .failure(); // exit 7 (NotFound) — sidecar missing
}
