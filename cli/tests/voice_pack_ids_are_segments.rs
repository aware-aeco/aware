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
/// reach: `apps/` beside `voices/`, and a directory ABOVE the home, so `../..`
/// has somewhere to land that the test can then assert survived.
///
/// The home is nested several levels inside the tempdir on purpose. An earlier
/// version put it one level down, so `--scope ../../../pwned-scope` escaped
/// clean past the tempdir into the system `/tmp` — while the assertion written
/// to catch that looked for `<tempdir>/pwned-scope`, where nothing could ever
/// appear. It passed by construction, and a real fence regression wrote into
/// `/tmp` on every machine that ran the suite. Any `..`-chain a test uses must
/// stay inside the directory the test inspects.
fn home_with_neighbours() -> (tempfile::TempDir, std::path::PathBuf) {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("deep/enough/to/catch/aware");
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

/// Run the command and hand back its output WITHOUT asserting on it.
///
/// `assert_cmd`'s `.assert().failure().code(3)` panics the instant the exit
/// code is wrong, so anything written after it — including the neighbour
/// checks — never runs on precisely the regression it was written to judge.
/// Reproduced: with the fence deleted, all four escape tests aborted inside the
/// `.assert()` chain and no neighbour assertion was ever reached, leaving the
/// exit code doing all the work. Splitting the run from the judgement lets the
/// filesystem be inspected FIRST, so "nothing was deleted" is a real assertion
/// rather than a line the failing case skips over.
fn run(home: &std::path::Path, args: &[&str]) -> std::process::Output {
    let mut c = aware(home);
    c.args(args);
    c.output().unwrap()
}

fn assert_refused_with(out: &std::process::Output, code: i32, needle: &str) {
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(code),
        "expected exit {code}, got {:?}; stderr: {stderr}",
        out.status.code()
    );
    assert!(
        stderr.contains(needle),
        "stderr did not name the problem ({needle:?}): {stderr}"
    );
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
        let out = run(&home, &["voice", "uninstall", pack]);
        // Filesystem FIRST — this is the assertion that would otherwise be
        // skipped on the very run that regressed.
        assert_neighbours_intact(tmp.path(), &home);
        // Naming the offending part is the difference between a fix and a
        // shrug — the operator has to know WHICH field is wrong.
        assert_refused_with(&out, 3, "not a plain name");
    }
}

#[test]
fn describe_refuses_a_pack_id_that_points_outside_voices() {
    let (tmp, home) = home_with_neighbours();
    let out = run(&home, &["voice", "describe", "../../apps"]);
    assert_neighbours_intact(tmp.path(), &home);
    assert_refused_with(&out, 3, "not a plain name");
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

    let out = run(&home, &["voice", "install", src.to_str().unwrap()]);

    // The assertion that matters: nothing was written anywhere under the tree,
    // not merely that the command exited non-zero. Before the fix this existed
    // and the command reported success. Checked before the exit-code assert,
    // which would otherwise panic first and skip it.
    assert!(
        !tmp.path().join("pwned").exists(),
        "the pack escaped to {}",
        tmp.path().join("pwned").display()
    );
    assert_neighbours_intact(tmp.path(), &home);
    assert_refused_with(&out, 3, "not a plain name");
    assert_refused_with(&out, 3, "../../../pwned");
}

#[test]
fn install_refuses_a_traversing_scope_flag() {
    let (tmp, home) = home_with_neighbours();
    let src = tmp.path().join("pack");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join("manifest.yaml"), "id: reviewer\nversion: 1.0.0\n").unwrap();
    std::fs::write(src.join("system-prompt.md"), "hi\n").unwrap();

    let out = run(
        &home,
        &[
            "voice",
            "install",
            src.to_str().unwrap(),
            "--scope",
            "../../../pwned-scope",
        ],
    );

    // Where the escape actually LANDS. `home` is `<tmp>/deep/enough/to/catch/
    // aware`, so `voices/../../../pwned-scope` is `<tmp>/deep/enough/to/
    // pwned-scope` — inside the tempdir, which is the whole point of nesting
    // it. The earlier one-level home put this in the system `/tmp`, where the
    // assertion could not see it and so could never fail.
    let landing = home.parent().unwrap().parent().unwrap().join("pwned-scope");
    assert!(
        !landing.exists(),
        "the scope flag escaped to {}",
        landing.display()
    );
    // Nothing anywhere else under the tempdir either.
    assert!(!tmp.path().join("pwned-scope").exists());
    assert_neighbours_intact(tmp.path(), &home);
    assert_refused_with(&out, 3, "not a plain name");
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
        // The pack that was already installed is still listed too. Anchored on
        // the column layout: a bare `contains("reviewer")` is a substring of
        // `uk-structural-reviewer` above, so it could not fail independently
        // and did not check what this line claims to check.
        .stdout(predicate::str::contains("ise          reviewer"));

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

/// The lexical fence's blind spot, through the real binary. Every part of
/// `ise/secret-pack` is a plain segment, so the segment check passes it — and
/// `remove_dir_all` would then follow the symlinked scope directory and empty
/// a folder outside `voices/` entirely, reporting success.
#[cfg(unix)]
#[test]
fn uninstall_does_not_reach_through_a_symlinked_scope_directory() {
    let (tmp, home) = home_with_neighbours();
    let outside = tmp.path().join("outside");
    std::fs::create_dir_all(outside.join("secret-pack/1.0.0")).unwrap();
    std::fs::write(outside.join("secret-pack/1.0.0/creds.txt"), "secret\n").unwrap();
    std::os::unix::fs::symlink(&outside, home.join("voices/ise-linked")).unwrap();

    let out = run(&home, &["voice", "uninstall", "ise-linked/secret-pack"]);

    assert!(
        outside.join("secret-pack/1.0.0/creds.txt").is_file(),
        "uninstall followed the symlinked scope directory and deleted {}",
        outside.join("secret-pack/1.0.0/creds.txt").display()
    );
    assert_neighbours_intact(tmp.path(), &home);
    assert_refused_with(&out, 3, "outside");
}

/// A pack that ships `up -> ..` walked `pack/up/pack/up/…` until the OS
/// stopped it with ELOOP, leaving ~85 directories and half a megabyte under
/// `voices/` with no rollback, because `Path::is_dir` follows links.
#[cfg(unix)]
#[test]
fn install_refuses_a_pack_containing_a_symlink_instead_of_following_it() {
    let (tmp, home) = home_with_neighbours();
    let src = tmp.path().join("src").join("pack");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join("manifest.yaml"), "id: p\nversion: 1.0.0\n").unwrap();
    std::os::unix::fs::symlink("..", src.join("up")).unwrap();

    let out = run(
        &home,
        &["voice", "install", src.to_str().unwrap(), "--scope", "s"],
    );

    let deep = home.join("voices/s/p/1.0.0/up/pack/up");
    assert!(
        !deep.exists(),
        "the copy followed the link into {}",
        deep.display()
    );
    assert_neighbours_intact(tmp.path(), &home);
    assert_refused_with(&out, 3, "symlink");
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
