//! End-to-end coverage for the generic managed-sidecar CLI contract.

use assert_cmd::Command;
use serde_json::Value;

#[test]
fn list_json_publishes_the_versioned_managed_sidecar_inventory() {
    let tmp = tempfile::tempdir().unwrap();
    let output = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        // Prevent real user-level PATH sidecars from affecting this inventory.
        .env("PATH", "")
        .args(["sidecar", "list", "--json"])
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let body: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(body["ok"], true);
    assert_eq!(body["meta"]["command"], "sidecar list");
    assert_eq!(body["data"]["schema-version"], 1);
    assert_eq!(body["data"]["runtime-version"], env!("CARGO_PKG_VERSION"));

    let sidecars = body["data"]["sidecars"].as_array().unwrap();
    assert_eq!(sidecars.len(), 5, "the whole managed catalogue is exposed");
    for sidecar in sidecars {
        assert!(sidecar["id"].is_string());
        assert!(sidecar["binary"].is_string());
        assert!(sidecar["description"].is_string());
        assert_eq!(sidecar["status"], "missing");
        assert!(sidecar.get("installed-version").is_some());
        assert_eq!(sidecar["installed-version"], Value::Null);
        assert_eq!(sidecar["repair-eligible"], false);
    }
    assert!(
        sidecars
            .iter()
            .any(|sidecar| sidecar["id"] == "connection-reader")
    );
}

#[test]
fn repair_requires_an_explicit_installed_scope() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["sidecar", "repair"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("requires --installed"));
}

#[test]
fn repair_installed_leaves_missing_sidecars_uninstalled() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("PATH", "")
        .args(["sidecar", "repair", "--installed"])
        .assert()
        .success()
        .stdout(predicates::str::contains(
            "No installed managed sidecars need repair",
        ));
}
