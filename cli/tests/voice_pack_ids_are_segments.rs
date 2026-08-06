//! A voice-pack coordinate becomes three directory levels under
//! `~/.aware/voices/`, so every part of it must be one plain segment — the same
//! rule `apps/<id>/` got in #365 and `agents/<id>/` got in #370.
//!
//! `aware voice` shipped without that check, and `uninstall` ends in
//! `remove_dir_all`. Against the pre-fix binary:
//!
//! ```text
//! $ aware voice uninstall '../..'
//! ✓ uninstalled voice pack at <AWARE_HOME>/voices/../../home
//! $ ls <AWARE_HOME>          # apps, agents, credentials — gone
//! ```
//!
//! exit 0, reported as a success. `install` is the same escape one step
//! earlier: a manifest declaring `id: ../../../pwned` had its whole folder
//! copied there and printed `✓ installed`.
//!
//! These drive the real binary rather than the resolver, because the damage is
//! in what reaches the filesystem, not in what a function returns — and because
//! the unit tests beside `commands::voice` cannot see the exit code the shell
//! would have believed.

use assert_cmd::Command;
use predicates::prelude::*;

fn aware(home: &std::path::Path) -> Command {
    let mut c = Command::cargo_bin("aware").unwrap();
    c.env("AWARE_HOME", home);
    c
}

/// An AWARE home holding one real pack plus the neighbours a traversal would
/// reach: `apps/` beside `voices/`, and a directory one level ABOVE the home,
/// so `../..` has somewhere to land that the test can then assert survived.
fn home_with_neighbours() -> (tempfile::TempDir, std::path::PathBuf) {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("aware");
    std::fs::create_dir_all(home.join("voices/ise/reviewer/1.0.0")).unwrap();
    std::fs::write(
        home.join("voices/ise/reviewer/1.0.0/manifest.yaml"),
        "id: reviewer\nversion: 1.0.0\nscope: ise\n",
    )
    .unwrap();
    std::fs::write(
        home.join("voices/ise/reviewer/1.0.0/system-prompt.md"),
        "you are a UK structural reviewer\n",
    )
    .unwrap();
    std::fs::create_dir_all(home.join("apps/keep-me")).unwrap();
    std::fs::write(home.join("apps/keep-me/keep-me.flo"), "app: keep-me\n").unwrap();
    std::fs::create_dir_all(home.join("credentials")).unwrap();
    std::fs::write(home.join("credentials/tekla.json"), "{}\n").unwrap();
    std::fs::create_dir_all(tmp.path().join("sibling")).unwrap();
    std::fs::write(tmp.path().join("sibling/precious.txt"), "keep\n").unwrap();
    (tmp, home)
}

fn assert_neighbours_intact(tmp: &std::path::Path, home: &std::path::Path) {
    assert!(
        home.join("apps/keep-me/keep-me.flo").is_file(),
        "apps/ was destroyed"
    );
    assert!(
        home.join("credentials/tekla.json").is_file(),
        "credentials/ were destroyed"
    );
    assert!(
        home.join("voices/ise/reviewer/1.0.0/manifest.yaml")
            .is_file(),
        "the installed pack was destroyed"
    );
    assert!(
        tmp.join("sibling/precious.txt").is_file(),
        "the escape reached above AWARE_HOME"
    );
}

#[test]
fn uninstall_refuses_a_pack_id_that_points_outside_voices() {
    // `../..` is the shape that emptied the whole home: `voices/../..`.
    for pack in ["../..", "../../apps/keep-me", "ise/../../apps"] {
        let (tmp, home) = home_with_neighbours();
        aware(&home)
            .args(["voice", "uninstall", pack])
            .assert()
            .failure()
            .code(3)
            // Naming the offending part is the difference between a fix and a
            // shrug — the operator has to know WHICH field is wrong.
            .stderr(predicate::str::contains("not a plain name"));
        assert_neighbours_intact(tmp.path(), &home);
    }
}

#[test]
fn describe_refuses_a_pack_id_that_points_outside_voices() {
    let (_tmp, home) = home_with_neighbours();
    aware(&home)
        .args(["voice", "describe", "../../apps"])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("not a plain name"));
}

#[test]
fn install_refuses_a_manifest_whose_id_points_outside_voices() {
    let (tmp, home) = home_with_neighbours();
    let src = tmp.path().join("pack");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(
        src.join("manifest.yaml"),
        "id: ../../../pwned\nversion: 1.0.0\nscope: ise\n",
    )
    .unwrap();
    std::fs::write(src.join("system-prompt.md"), "hi\n").unwrap();

    aware(&home)
        .args(["voice", "install"])
        .arg(&src)
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("not a plain name"))
        .stderr(predicate::str::contains("../../../pwned"));

    // The assertion that matters: nothing was written anywhere under the tree,
    // not merely that the command exited non-zero. Before the fix this existed
    // and the command reported success.
    assert!(
        !tmp.path().join("pwned").exists(),
        "the pack escaped to {}",
        tmp.path().join("pwned").display()
    );
    assert_neighbours_intact(tmp.path(), &home);
}

#[test]
fn install_refuses_a_traversing_scope_flag() {
    let (tmp, home) = home_with_neighbours();
    let src = tmp.path().join("pack");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join("manifest.yaml"), "id: reviewer\nversion: 1.0.0\n").unwrap();
    std::fs::write(src.join("system-prompt.md"), "hi\n").unwrap();

    aware(&home)
        .args(["voice", "install"])
        .arg(&src)
        .args(["--scope", "../../../pwned-scope"])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("not a plain name"));

    assert!(!tmp.path().join("pwned-scope").exists());
    assert_neighbours_intact(tmp.path(), &home);
}

/// The fence has to leave the command working, or "nothing escaped" is
/// satisfied by refusing everything. Install → list → describe → uninstall,
/// end to end, on the ids a real pack uses.
#[test]
fn a_legitimate_pack_still_installs_lists_describes_and_uninstalls() {
    let (tmp, home) = home_with_neighbours();
    let src = tmp.path().join("pack");
    std::fs::create_dir_all(src.join("references")).unwrap();
    std::fs::write(
        src.join("manifest.yaml"),
        "id: uk-structural-reviewer\nversion: 2.0.0\n",
    )
    .unwrap();
    std::fs::write(
        src.join("system-prompt.md"),
        "you review to BS 5950 and EN 1993\n",
    )
    .unwrap();
    std::fs::write(src.join("references/bs5950.md"), "clause 4.2\n").unwrap();

    aware(&home)
        .args(["voice", "install"])
        .arg(&src)
        .args(["--scope", "ise"])
        .assert()
        .success();

    aware(&home)
        .args(["voice", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("uk-structural-reviewer"))
        .stdout(predicate::str::contains("2.0.0"))
        // The pack that was already installed is still listed too.
        .stdout(predicate::str::contains("reviewer"));

    // `describe` prints the pack's actual contents, not just its name.
    aware(&home)
        .args(["voice", "describe", "@ise/uk-structural-reviewer@2.0.0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("you review to BS 5950"))
        .stdout(predicate::str::contains("bs5950.md"));

    aware(&home)
        .args(["voice", "uninstall", "ise/uk-structural-reviewer"])
        .assert()
        .success();

    assert!(
        !home
            .join("voices/ise/uk-structural-reviewer/2.0.0")
            .exists(),
        "uninstall reported success without removing the pack"
    );
    assert_neighbours_intact(tmp.path(), &home);
}

/// An id naming nothing installed must fail as not-found (exit 7), not be
/// waved through and not be reported as a traversal.
#[test]
fn an_unknown_pack_is_a_not_found() {
    let (_tmp, home) = home_with_neighbours();
    aware(&home)
        .args(["voice", "describe", "ise/no-such-pack"])
        .assert()
        .failure()
        .code(7);
}
