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

/// A self-contained app source with no `requires:`, so these tests exercise
/// manifest SELECTION without dragging agent installs in.
fn write_standalone_app(path: &std::path::Path, id: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(
        path,
        format!(
            "app: {id}\nversion: 0.1.0\ndescription: a selection fixture\n\
             nodes:\n  - id: gate\n    inline:\n      kind: predicate\n\
             \x20     description: always pass\n      code: 'true'\nrequires: []\n"
        ),
    )
    .unwrap();
}

/// `aware app install .` installs the directory you are standing in.
///
/// The manifest selector prefers `<dir-name>.flo`, and it used to take that
/// name from `Path::file_name`, which is `None` for `.` and `..` — so the
/// selector returned "no manifest" without ever reading the directory, and a
/// perfectly ordinary invocation failed (Codex, #499). The name now comes from
/// the path resolved against the working directory.
#[test]
fn app_install_accepts_a_dot_path() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let app_src = tmp.path().join("dotted");
    write_standalone_app(&app_src.join("dotted.flo"), "dotted");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .current_dir(&app_src)
        .args(["app", "install", "."])
        .assert()
        .success();

    assert!(aware.join("apps/dotted/dotted.flo").is_file());
}

/// The manifest that gets validated and copied is the one whose lockfile is
/// written and whose id the app is discovered under — all four the same file.
///
/// Install copies the source folder to `apps/<app-id>`, which RENAMES the
/// directory, and the selector prefers `<dir-name>.flo`. Re-running the
/// selector afterwards therefore asks a different question: here `bundle/`
/// holds `bundle.flo` (declaring `app: alpha`) beside an `alpha.flo`, so the
/// pre-flight picks `bundle.flo` and a second lookup under `apps/alpha/` would
/// pick `alpha.flo` — locking an app nobody installed (Codex, #499).
#[test]
fn app_install_locks_the_manifest_it_actually_installed() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let app_src = tmp.path().join("bundle");
    write_standalone_app(&app_src.join("bundle.flo"), "alpha");
    write_standalone_app(&app_src.join("alpha.flo"), "decoy");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "install"])
        .arg(&app_src)
        .assert()
        .success()
        .stdout(predicate::str::contains("alpha"));

    // Installed under the id `bundle.flo` declared...
    let lockfile = aware.join("apps/alpha/lockfile.yaml");
    let body = std::fs::read_to_string(&lockfile).unwrap();
    // ...and the lockfile describes THAT app, not the decoy beside it.
    assert!(
        body.contains("app: alpha"),
        "lockfile must describe the installed manifest, got: {body}"
    );
    assert!(
        !body.contains("decoy"),
        "the decoy manifest must not be what got locked: {body}"
    );
}
