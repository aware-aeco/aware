//! `aware app rename <old> <new>` — rename an installed app in place (#226).

mod common;

use assert_cmd::Command;
use predicates::prelude::*;

fn seed(aware: &std::path::Path, id: &str) {
    std::fs::create_dir_all(aware.join(format!("apps/{id}"))).unwrap();
    std::fs::write(
        aware.join(format!("apps/{id}/{id}.app")),
        format!(
            "app: {id}\nversion: 0.0.1\ndescription: x\nnodes: [{{id: a}}]\nconnections: []\nrequires: []\n"
        ),
    )
    .unwrap();
}

#[test]
fn renames_installed_app_and_remaps_identity() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    seed(&aware, "old-app");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "rename", "old-app", "new-app"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("renamed old-app").and(predicate::str::contains("new-app")),
        );

    // Old directory gone; new directory holds the renamed source.
    assert!(!aware.join("apps/old-app").exists());
    assert!(aware.join("apps/new-app/new-app.app").is_file());
    assert!(!aware.join("apps/new-app/old-app.app").exists());

    // The new id resolves; the old one no longer does.
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "show", "new-app"])
        .assert()
        .success()
        .stdout(predicate::str::contains("app:           new-app"));
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "show", "old-app"])
        .assert()
        .failure();
}

#[test]
fn rename_to_existing_name_conflicts() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    seed(&aware, "a");
    seed(&aware, "b");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "rename", "a", "b"])
        .assert()
        .failure()
        .code(8); // Conflict
    // Both apps left intact.
    assert!(aware.join("apps/a/a.app").is_file());
    assert!(aware.join("apps/b/b.app").is_file());
}

#[test]
fn rename_missing_app_is_not_found() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    std::fs::create_dir_all(aware.join("apps")).unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "rename", "ghost", "new"])
        .assert()
        .failure()
        .code(7); // NotFound
}

#[test]
fn rename_to_unsafe_id_is_rejected_and_leaves_original() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    seed(&aware, "ok");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "rename", "ok", "../escape"])
        .assert()
        .failure()
        .code(3); // Validation
    // The original app is untouched after a rejected rename.
    assert!(aware.join("apps/ok/ok.app").is_file());
    assert!(!aware.join("apps/escape").exists());
}
