//! `aware app duplicate <src> <new>` — copy an installed app to a new id (#226).

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
fn duplicates_app_leaving_original_intact() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    seed(&aware, "orig");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "duplicate", "orig", "copy"])
        .assert()
        .success()
        .stdout(predicate::str::contains("duplicated orig").and(predicate::str::contains("copy")));

    // Both exist; the copy carries its own identity, the original is unchanged.
    assert!(aware.join("apps/orig/orig.app").is_file());
    assert!(aware.join("apps/copy/copy.app").is_file());

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "show", "copy"])
        .assert()
        .success()
        .stdout(predicate::str::contains("app:           copy"));
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "show", "orig"])
        .assert()
        .success()
        .stdout(predicate::str::contains("app:           orig"));
}

#[test]
fn duplicate_to_existing_name_conflicts() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    seed(&aware, "a");
    seed(&aware, "b");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "duplicate", "a", "b"])
        .assert()
        .failure()
        .code(8); // Conflict
}

#[test]
fn duplicate_missing_app_is_not_found() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    std::fs::create_dir_all(aware.join("apps")).unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "duplicate", "ghost", "copy"])
        .assert()
        .failure()
        .code(7); // NotFound
}
