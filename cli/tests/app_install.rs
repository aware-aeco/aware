mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn installs_app_with_lockfile() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");

    let repo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let tekla_src = repo.join("20-agents/aeco/engineering/tekla");
    let tc_src = repo.join("20-agents/aeco/construction/trimble-connect");

    // Install the two agents the app requires
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["agent", "install"])
        .arg(&tekla_src)
        .assert()
        .success();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["agent", "install"])
        .arg(&tc_src)
        .assert()
        .success();

    // Build a one-file app dir with welded-to-tc.app
    let app_src = tmp.path().join("welded-to-tc-src");
    std::fs::create_dir_all(&app_src).unwrap();
    let flo = repo.join("30-apps/_examples/welded-to-tc.app");
    std::fs::copy(&flo, app_src.join("welded-to-tc.app")).unwrap();

    // Install the app
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "install"])
        .arg(&app_src)
        .assert()
        .success()
        .stdout(predicate::str::contains("installed welded-to-tc"))
        .stdout(predicate::str::contains("lockfile"));

    // Lockfile exists with both agents pinned
    let lockfile = aware.join("apps/welded-to-tc/lockfile.yaml");
    assert!(
        lockfile.is_file(),
        "lockfile missing at {}",
        lockfile.display()
    );
    let body = std::fs::read_to_string(&lockfile).unwrap();
    assert!(body.contains("app: welded-to-tc"));
    assert!(body.contains("version: 0.3.1"));
    assert!(body.contains("tekla:"));
    assert!(body.contains("trimble-connect:"));
    assert!(body.contains("resolved-at:"));
}

#[test]
fn app_install_rejects_invalid_path() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["app", "install", "does-not-exist-anywhere"])
        .assert()
        .failure();
}

/// End-to-end #502: a source folder holding two top-level manifests is refused
/// by the CLI itself, and leaves no app behind for `app list` to disagree about.
///
/// Before the fix this install SUCCEEDED — it wrote a lock for `alpha` and then
/// `app list` reported `decoy` from the same installed directory, because
/// install and discovery each ran their own manifest selector.
#[test]
fn app_install_refuses_a_folder_with_two_manifests() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let app_src = tmp.path().join("bundle");
    std::fs::create_dir_all(&app_src).unwrap();
    write_gate_app(&app_src.join("bundle.flo"), "alpha", "selected manifest");
    write_gate_app(
        &app_src.join("alpha.flo"),
        "decoy",
        "sibling decoy manifest",
    );

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "install"])
        .arg(&app_src)
        .assert()
        .failure()
        .stderr(predicate::str::contains("bundle.flo"))
        .stderr(predicate::str::contains("alpha.flo"))
        .stderr(predicate::str::contains("exactly one"));

    assert!(
        !aware.join("apps/alpha").exists() && !aware.join("apps/decoy").exists(),
        "a refused install must copy nothing"
    );

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alpha").not())
        .stdout(predicate::str::contains("decoy").not());
}

/// The single-manifest case the refusal protects: a manifest NOT named after
/// the app it declares, so the installed directory is renamed out from under
/// it. The lock, `app show` and `app list` must all name the same app.
#[test]
fn app_install_agrees_with_discovery_when_the_manifest_is_not_named_after_the_app() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let app_src = tmp.path().join("bundle");
    std::fs::create_dir_all(&app_src).unwrap();
    write_gate_app(&app_src.join("bundle.flo"), "alpha", "selected manifest");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "install"])
        .arg(&app_src)
        .assert()
        .success()
        .stdout(predicate::str::contains("installed alpha"));

    let lock = std::fs::read_to_string(aware.join("apps/alpha/lockfile.yaml")).unwrap();
    assert!(lock.contains("app: alpha"), "lockfile says: {lock}");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "show", "alpha"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alpha"))
        .stdout(predicate::str::contains("decoy").not());

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("alpha"));
}

/// A minimal valid app: one inline predicate node, no agents required.
fn write_gate_app(path: &std::path::Path, id: &str, description: &str) {
    std::fs::write(
        path,
        format!(
            "app: {id}\nversion: 0.1.0\ndescription: {description}\n\
             nodes:\n  - id: gate\n    inline:\n      kind: predicate\n\
             \x20     description: always pass\n      code: 'true'\nrequires: []\n"
        ),
    )
    .unwrap();
}
