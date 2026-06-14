//! #226 footgun: when an app's directory name and `app:` field disagree (e.g.
//! after a manual `mv`), every by-id verb must resolve it the SAME way — by
//! directory name AND by the `app:` field (with a warning). Before the fix
//! `show`/`run` keyed on the field while `explain` keyed on the directory name,
//! so `show renamed-demo` failed while `explain renamed-demo` worked.

mod common;

use assert_cmd::Command;
use predicates::prelude::*;

/// Install an app whose `app:` field is `field_id` into a directory named
/// `dir_name` — the desynced state.
fn seed_desynced(aware: &std::path::Path, dir_name: &str, field_id: &str) {
    std::fs::create_dir_all(aware.join(format!("apps/{dir_name}"))).unwrap();
    std::fs::write(
        aware.join(format!("apps/{dir_name}/src.app")),
        format!(
            "app: {field_id}\nversion: 0.0.1\ndescription: x\nnodes: [{{id: a}}]\nconnections: []\nrequires: []\n"
        ),
    )
    .unwrap();
}

#[test]
fn show_and_explain_resolve_by_dir_name_and_by_field() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    seed_desynced(&aware, "renamed-demo", "hello-world");

    // By DIRECTORY name → direct hit, no warning.
    for verb in ["show", "explain"] {
        Command::cargo_bin("aware")
            .unwrap()
            .env("AWARE_HOME", &aware)
            .args(["app", verb, "renamed-demo"])
            .assert()
            .success();
    }

    // By the `app:` FIELD → fallback hit, with a desync warning on stderr. This
    // is the case that used to fail for `show`.
    for verb in ["show", "explain"] {
        Command::cargo_bin("aware")
            .unwrap()
            .env("AWARE_HOME", &aware)
            .args(["app", verb, "hello-world"])
            .assert()
            .success()
            .stderr(predicate::str::contains("disagree"));
    }
}

#[test]
fn list_warns_on_dir_field_mismatch() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    seed_desynced(&aware, "renamed-demo", "hello-world");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello-world"))
        .stderr(predicate::str::contains("disagree"));
}
