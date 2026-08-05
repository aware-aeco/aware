//! An app id becomes a directory name and a file name, so it must be one plain
//! segment (#365).
//!
//! `write_lockfile` puts the lock at `<source-dir>/<app-id>.lock`, and until this
//! check the only judgement on the id was "not empty". So a `.flo` declaring
//! `app: ../../pwned` compiled clean and wrote its lockfile two directories above
//! the source — reported as a success, exit 0. `aware app install` has the same
//! shape one step later (`apps/<id>/`).
//!
//! These drive the real binary, because the escape is in what reaches the
//! filesystem, not in what the validator returns.

mod common;

use assert_cmd::Command;
use predicates::prelude::*;

fn aware(home: &std::path::Path) -> Command {
    let mut c = Command::cargo_bin("aware").unwrap();
    c.env("AWARE_HOME", home);
    c
}

/// A source tree two levels deep, so a `../../` id has somewhere to escape TO
/// that the test can then assert is empty.
fn fixture(app_id: &str) -> (tempfile::TempDir, std::path::PathBuf) {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("home")).unwrap();
    let src = tmp.path().join("proj").join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(
        src.join("evil.flo"),
        format!(
            "app: {app_id}\nversion: 0.1.0\ndescription: an id that is not a plain name\n\
             nodes:\n  - id: n\n    agent: probe-agent\n    command: probe\n    frozen: {{}}\n"
        ),
    )
    .unwrap();
    (tmp, src)
}

#[test]
fn compile_refuses_an_app_id_that_would_write_outside_its_directory() {
    let (tmp, src) = fixture("../../pwned");
    aware(&tmp.path().join("home"))
        .args(["app", "compile"])
        .arg(src.join("evil.flo"))
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("E_APP_ID_NOT_A_SEGMENT"))
        // Names the id, or the author cannot tell which field is wrong.
        .stderr(predicate::str::contains("../../pwned"));

    // The assertion that matters: nothing was written ANYWHERE under the tree,
    // not merely that the command exited non-zero. Before the fix this file
    // existed and the command reported success.
    assert!(
        !tmp.path().join("pwned.lock").exists(),
        "the lockfile escaped to {}",
        tmp.path().join("pwned.lock").display()
    );
    let strays: Vec<_> = walk(tmp.path())
        .into_iter()
        .filter(|p| p.extension().is_some_and(|e| e == "lock"))
        .collect();
    assert!(strays.is_empty(), "a refused compile wrote {strays:?}");
}

#[test]
fn compile_still_accepts_an_ordinary_id() {
    // The negative control. A check that refused every id would pass the test
    // above without closing anything.
    let (tmp, src) = fixture("pwned");
    aware(&tmp.path().join("home"))
        .args(["app", "compile"])
        .arg(src.join("evil.flo"))
        .assert()
        .success();
    assert!(
        src.join("pwned.lock").is_file(),
        "the lock belongs beside the source it was compiled from"
    );
}

#[test]
fn install_refuses_it_too() {
    // `install` turns the id into `apps/<id>/`, so it needs the same answer —
    // and it validates separately from `compile`, so it needs its own test.
    let (tmp, src) = fixture("../../pwned");
    let home = tmp.path().join("home");
    aware(&home)
        .args(["app", "install"])
        .arg(&src)
        .assert()
        .failure()
        .stderr(predicate::str::contains("E_APP_ID_NOT_A_SEGMENT"));
    assert!(
        !tmp.path().join("pwned").exists(),
        "install created a directory outside apps/"
    );
}

#[cfg(windows)]
#[test]
fn a_drive_relative_id_is_refused_as_well() {
    // The escape a hand-rolled "no separators" check misses: `C:evil` carries
    // none, and `Path::join` discards the whole base when the appended path has
    // a prefix. Windows-only because on POSIX `C:evil` is an ordinary filename.
    let (tmp, src) = fixture("C:evil");
    aware(&tmp.path().join("home"))
        .args(["app", "compile"])
        .arg(src.join("evil.flo"))
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("E_APP_ID_NOT_A_SEGMENT"));
}

fn walk(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return out;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            out.extend(walk(&p));
        } else {
            out.push(p);
        }
    }
    out
}
