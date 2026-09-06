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

/// Plant a managed bridge (binary + version marker) under `<home>/bridges`.
fn plant_managed(home: &std::path::Path, binary: &str, version: &str) -> std::path::PathBuf {
    let bridges = home.join("bridges");
    std::fs::create_dir_all(&bridges).unwrap();
    let exe = bridges.join(format!("{binary}.exe"));
    std::fs::write(&exe, b"fake").unwrap();
    std::fs::write(bridges.join(format!("{binary}.version")), version).unwrap();
    exe
}

#[test]
fn list_json_reports_current_and_stale_copies_and_marks_only_the_stale_one_for_repair() {
    let tmp = tempfile::tempdir().unwrap();
    plant_managed(tmp.path(), "aware-tekla", env!("CARGO_PKG_VERSION"));
    plant_managed(tmp.path(), "aware-rhino", "0.0.1-old");

    let output = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
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
    let row = |id: &str| -> Value {
        body["data"]["sidecars"]
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["id"] == id)
            .unwrap_or_else(|| panic!("no row for {id}"))
            .clone()
    };

    let tekla = row("tekla");
    assert_eq!(tekla["status"], "current");
    assert_eq!(tekla["installed-version"], env!("CARGO_PKG_VERSION"));
    assert_eq!(tekla["repair-eligible"], false);

    let rhino = row("rhino");
    assert_eq!(rhino["status"], "stale");
    assert_eq!(rhino["installed-version"], "0.0.1-old");
    assert_eq!(
        rhino["repair-eligible"], true,
        "a version-drifted managed copy is what repair --installed targets"
    );

    let revit = row("revit");
    assert_eq!(revit["status"], "missing");
    assert_eq!(revit["installed-version"], Value::Null);
    assert_eq!(revit["repair-eligible"], false);
}

#[test]
fn uninstall_removes_the_managed_binary_and_its_version_marker() {
    let tmp = tempfile::tempdir().unwrap();
    let exe = plant_managed(tmp.path(), "aware-tekla", env!("CARGO_PKG_VERSION"));
    let marker = tmp.path().join("bridges").join("aware-tekla.version");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("PATH", "")
        .args(["sidecar", "uninstall", "tekla"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Removed"));

    assert!(!exe.exists(), "the managed binary is gone");
    assert!(
        !marker.exists(),
        "the version marker must go too, or `list` keeps claiming a version for a bridge \
         that is no longer there"
    );
}

#[test]
fn uninstall_refuses_to_touch_a_legacy_copy_that_only_exists_on_path() {
    // `uninstall` is dir-only on purpose: reaching out to PATH would delete a
    // binary AWARE never installed and does not manage.
    let home = tempfile::tempdir().unwrap();
    let on_path = tempfile::tempdir().unwrap();
    let legacy = on_path.path().join(if cfg!(windows) {
        "aware-tekla.exe"
    } else {
        "aware-tekla"
    });
    std::fs::write(&legacy, b"fake").unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .env("PATH", on_path.path())
        .args(["sidecar", "uninstall", "tekla"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("is not installed"));

    assert!(
        legacy.exists(),
        "the on-PATH copy must survive an uninstall that found nothing it manages"
    );
}

#[test]
fn uninstall_rejects_an_unknown_host_and_names_the_ones_it_knows() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["sidecar", "uninstall", "autocad"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("unknown host 'autocad'"))
        .stderr(predicates::str::contains("connection-reader"));
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
