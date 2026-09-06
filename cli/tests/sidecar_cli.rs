//! End-to-end coverage for the generic managed-sidecar CLI contract.

use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;

#[test]
fn list_json_publishes_the_versioned_managed_sidecar_inventory() {
    let tmp = tempfile::tempdir().unwrap();
    let nowhere = empty_path_dir();
    let output = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        // Prevent real user-level PATH sidecars from affecting this inventory.
        .env("PATH", nowhere.path())
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

/// The name a legacy on-PATH bridge has to carry to be found by `which_binary`,
/// which appends `.exe` only on Windows.
fn legacy_name(binary: &str) -> String {
    if cfg!(windows) {
        format!("{binary}.exe")
    } else {
        binary.to_string()
    }
}

/// A directory with nothing in it, to stand in for "this machine has no bridges
/// anywhere on PATH".
///
/// `PATH=""` does NOT mean that: `env::split_paths("")` yields one *empty*
/// entry, so the lookup probes the relative name against the child's working
/// directory instead of probing nothing. Pointing at a real empty directory is
/// what makes these tests independent of where they are run from.
fn empty_path_dir() -> tempfile::TempDir {
    tempfile::tempdir().unwrap()
}

#[test]
fn list_json_reports_current_and_stale_copies_and_marks_only_the_stale_one_for_repair() {
    let tmp = tempfile::tempdir().unwrap();
    let nowhere = empty_path_dir();
    plant_managed(tmp.path(), "aware-tekla", env!("CARGO_PKG_VERSION"));
    plant_managed(tmp.path(), "aware-rhino", "0.0.1-old");

    let output = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("PATH", nowhere.path())
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
fn list_json_reports_a_path_only_copy_as_legacy_and_never_offers_to_repair_it() {
    // The one status that needs a populated PATH, and the reason `repair` is not
    // simply "everything that is not current": repair reinstalls into the managed
    // dir, and doing that for a copy AWARE never installed would silently shadow
    // the user's own binary.
    let home = tempfile::tempdir().unwrap();
    let on_path = tempfile::tempdir().unwrap();
    std::fs::write(on_path.path().join(legacy_name("aware-tekla")), b"fake").unwrap();

    let output = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .env("PATH", on_path.path())
        .args(["sidecar", "list", "--json"])
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let body: Value = serde_json::from_slice(&output.stdout).unwrap();
    let tekla = body["data"]["sidecars"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["id"] == "tekla")
        .expect("no row for tekla")
        .clone();

    assert_eq!(
        tekla["status"], "legacy",
        "a copy resolvable only through PATH is legacy, not current and not missing"
    );
    assert_eq!(tekla["installed-version"], Value::Null);
    assert_eq!(tekla["repair-eligible"], false);
}

#[test]
fn list_names_every_catalogue_state_in_the_form_a_user_types() {
    // `--json` is the consumer contract; this is the human one, and it had no
    // test at all. Each state prints a different line, and the install hint is
    // the whole point of the missing and stale ones.
    let home = tempfile::tempdir().unwrap();
    let on_path = tempfile::tempdir().unwrap();
    std::fs::write(on_path.path().join(legacy_name("aware-sketchup")), b"fake").unwrap();
    plant_managed(home.path(), "aware-tekla", env!("CARGO_PKG_VERSION"));
    plant_managed(home.path(), "aware-rhino", "0.0.1-old");

    let output = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .env("PATH", on_path.path())
        .args(["sidecar", "list"])
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains(&format!(
            "install dir: {}",
            home.path().join("bridges").display()
        )),
        "the header names the install dir, which is off PATH and otherwise unguessable:\n{stdout}"
    );

    // Per line, not per buffer: `contains` over the whole output cannot tell a
    // hint attached to the right state from one attached to the wrong bridge.
    let line = |id: &str| -> &str {
        stdout
            .lines()
            .find(|l| l.contains(&format!(" {id} ")) || l.trim_end().ends_with(&format!(" {id}")))
            .unwrap_or_else(|| panic!("no line for {id}:\n{stdout}"))
    };

    assert!(
        line("tekla").contains('\u{2713}'),
        "current: {}",
        line("tekla")
    );
    assert!(
        !line("tekla").contains("aware sidecar install"),
        "a current bridge must not be told to reinstall: {}",
        line("tekla")
    );
    assert!(
        line("rhino").contains('\u{21bb}') && line("rhino").contains("aware sidecar install rhino"),
        "stale: {}",
        line("rhino")
    );
    assert!(
        line("sketchup").contains('\u{26a0}') && line("sketchup").contains("legacy"),
        "PATH-only, flagged for migration rather than reported installed: {}",
        line("sketchup")
    );
    assert!(
        line("revit").contains('\u{2717}') && line("revit").contains("aware sidecar install revit"),
        "missing: {}",
        line("revit")
    );

    // Every catalogue entry is listed. Without this a bridge could drop out of
    // the human listing entirely and only the `--json` count would notice.
    for id in ["tekla", "rhino", "sketchup", "revit", "connection-reader"] {
        let _ = line(id);
    }
}

#[test]
fn install_is_a_no_op_when_the_managed_copy_already_matches_this_cli_version() {
    // The #148 guard, and the branch that decides whether `install` reaches the
    // network at all. Nothing else in the suite reaches it, and it needs no
    // network to test: with a matching marker in place the command must report
    // the existing install and stop.
    let home = tempfile::tempdir().unwrap();
    let nowhere = empty_path_dir();
    let exe = plant_managed(home.path(), "aware-tekla", env!("CARGO_PKG_VERSION"));

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .env("PATH", nowhere.path())
        .args(["sidecar", "install", "tekla"])
        .assert()
        .success()
        .stdout(predicates::str::contains(format!(
            "already installed (v{})",
            env!("CARGO_PKG_VERSION")
        )))
        .stdout(predicates::str::contains("Downloading").not());

    assert_eq!(
        std::fs::read(&exe).unwrap(),
        b"fake",
        "the existing binary is left exactly as it was, not re-fetched over"
    );
}

#[test]
fn uninstall_removes_the_managed_binary_and_its_version_marker() {
    let tmp = tempfile::tempdir().unwrap();
    let nowhere = empty_path_dir();
    let exe = plant_managed(tmp.path(), "aware-tekla", env!("CARGO_PKG_VERSION"));
    let marker = tmp.path().join("bridges").join("aware-tekla.version");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("PATH", nowhere.path())
        .args(["sidecar", "uninstall", "tekla"])
        .assert()
        .success()
        .stdout(predicates::str::contains(format!("{}", exe.display())));

    assert!(!exe.exists(), "the managed binary is gone");
    assert!(
        !marker.exists(),
        "the version marker must go too, or `list` keeps claiming a version for a bridge \
         that is no longer there"
    );
}

#[test]
fn uninstall_removes_the_managed_copy_and_never_the_one_on_path() {
    // `uninstall` is dir-only on purpose: reaching out to PATH would delete a
    // binary AWARE never installed and does not manage.
    //
    // Both directions in one environment, because either alone is satisfiable
    // without running the lookup at all. The first invocation proves the managed
    // copy is found and removed while the identically-named PATH copy is left
    // alone; the second proves that with nothing left to manage the command
    // refuses rather than falling through to PATH. An `uninstall` stubbed to fail
    // before consulting the dir passes the second and fails the first.
    let home = tempfile::tempdir().unwrap();
    let on_path = tempfile::tempdir().unwrap();
    let legacy = on_path.path().join(legacy_name("aware-tekla"));
    std::fs::write(&legacy, b"fake").unwrap();
    let managed = plant_managed(home.path(), "aware-tekla", env!("CARGO_PKG_VERSION"));

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .env("PATH", on_path.path())
        .args(["sidecar", "uninstall", "tekla"])
        .assert()
        .success()
        .stdout(predicates::str::contains(format!("{}", managed.display())));

    assert!(
        !managed.exists(),
        "the managed copy is what uninstall removes"
    );
    assert!(
        legacy.exists(),
        "the PATH copy shares the bridge id and must still be untouched"
    );

    // Nothing managed now. The PATH copy is still there and still must not be
    // reached — this is the arm that regresses if `find_bridge_in_dir` is ever
    // swapped for the PATH-reaching `find_bridge`.
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .env("PATH", on_path.path())
        .args(["sidecar", "uninstall", "tekla"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("aware-tekla is not installed"));

    assert!(
        legacy.exists(),
        "the on-PATH copy must survive an uninstall that found nothing it manages"
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
    let nowhere = empty_path_dir();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("PATH", nowhere.path())
        .args(["sidecar", "repair", "--installed"])
        .assert()
        .success()
        .stdout(predicates::str::contains(
            "No installed managed sidecars need repair",
        ));
}
