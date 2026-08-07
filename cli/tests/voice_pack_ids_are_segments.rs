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

/// Assert nothing whose name begins with `prefix` exists ANYWHERE under `root`.
///
/// The escape assertions used to name one path and check that, and got the
/// depth wrong: `<tmp>/pwned` where the escape actually lands at
/// `<tmp>/deep/enough/to/catch/pwned`. That is the same failure
/// `home_with_neighbours`'s own doc comment warns about — an assertion satisfied
/// by the fixture rather than by the code — repeated one level up. Searching the
/// tree takes the arithmetic out of the assertion instead of getting it right
/// once and leaving it to be got wrong again.
fn assert_nothing_escaped(root: &std::path::Path, prefix: &str) {
    let mut found = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            if entry.file_name().to_string_lossy().starts_with(prefix) {
                found.push(entry.path());
            }
            // `file_type` does not follow links, so a symlinked fixture cannot
            // walk this out of `root` or into a cycle.
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                stack.push(entry.path());
            }
        }
    }
    assert!(
        found.is_empty(),
        "the pack escaped to {found:?} — nothing named {prefix:?}* may exist under {}",
        root.display()
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
    assert_nothing_escaped(tmp.path(), "pwned");
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

    // `home` is `<tmp>/deep/enough/to/catch/aware`, so the escape would land at
    // `<tmp>/deep/enough/to/pwned-scope` — inside the tempdir, which is the
    // whole point of nesting it. The earlier one-level home put this in the
    // system `/tmp`, where the assertion could not see it and so could never
    // fail. Searched rather than spelled out, so the depth cannot drift.
    assert_nothing_escaped(tmp.path(), "pwned-scope");
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
    // Named `elsewhere`, not `outside`: the refusal message echoes the resolved
    // path back, so a fixture called `outside` satisfied a
    // `stderr.contains("outside")` needle by its own name — the assertion passed
    // on the path, never on the message's own words.
    let elsewhere = tmp.path().join("elsewhere");
    std::fs::create_dir_all(elsewhere.join("secret-pack/1.0.0")).unwrap();
    std::fs::write(elsewhere.join("secret-pack/1.0.0/creds.txt"), "secret\n").unwrap();
    std::os::unix::fs::symlink(&elsewhere, home.join("voices/ise-linked")).unwrap();

    let out = run(&home, &["voice", "uninstall", "ise-linked/secret-pack"]);

    assert!(
        elsewhere.join("secret-pack/1.0.0/creds.txt").is_file(),
        "uninstall followed the symlinked scope directory and deleted {}",
        elsewhere.join("secret-pack/1.0.0/creds.txt").display()
    );
    assert_neighbours_intact(tmp.path(), &home);
    assert_refused_with(&out, 3, "resolves through a symlink");
}

/// The install half of the same blind spot, through the real binary. `--scope
/// ise-linked` is a plain segment, so the lexical fence passes it; only the
/// containment check stands between a manifest and a write outside `voices/`.
/// Deleting that one call left the whole suite green before this existed.
#[cfg(unix)]
#[test]
fn install_does_not_write_through_a_symlinked_scope_directory() {
    let (tmp, home) = home_with_neighbours();
    let elsewhere = tmp.path().join("elsewhere");
    std::fs::create_dir_all(&elsewhere).unwrap();
    std::os::unix::fs::symlink(&elsewhere, home.join("voices/ise-linked")).unwrap();

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
            "ise-linked",
        ],
    );

    // Filesystem first: not one byte through the link, and not even the empty
    // directories `create_dir_all` makes on the way to the check.
    assert!(
        !elsewhere.join("reviewer/1.0.0/system-prompt.md").exists(),
        "the pack was written through the symlinked scope directory"
    );
    assert!(
        !elsewhere.join("reviewer").exists(),
        "a refused install left {} behind",
        elsewhere.join("reviewer").display()
    );
    assert_neighbours_intact(tmp.path(), &home);
    assert_refused_with(&out, 3, "resolves through a symlink");
}

/// A link that stays INSIDE `voices/` — the case a `starts_with` containment
/// test waves through, and the one that turns the fence itself into a delete of
/// something the operator never named.
///
/// `resolve_pack_dir` returns the canonical path and `uninstall` calls
/// `remove_dir_all` on it, so with only the prefix test `uninstall
/// 'ise/reviewer@1.0.0'` reported success while destroying 2.0.0 and leaving
/// the dangling 1.0.0 link standing.
#[cfg(unix)]
#[test]
fn uninstall_does_not_delete_the_pack_a_sibling_symlink_points_at() {
    let (tmp, home) = home_with_neighbours();
    let parent = home.join("voices/ise/reviewer");
    std::fs::create_dir_all(parent.join("2.0.0")).unwrap();
    std::fs::write(parent.join("2.0.0/manifest.yaml"), "id: reviewer\n").unwrap();
    std::os::unix::fs::symlink("2.0.0", parent.join("1.0.0-link")).unwrap();

    let out = run(&home, &["voice", "uninstall", "ise/reviewer@1.0.0-link"]);

    assert!(
        parent.join("2.0.0/manifest.yaml").is_file(),
        "uninstalling the LINK deleted the real pack at {}",
        parent.join("2.0.0").display()
    );
    assert_neighbours_intact(tmp.path(), &home);
    assert_refused_with(&out, 3, "resolves through a symlink");
}

/// A refused install must leave nothing that `list` will call installed.
/// `list` reads the directory tree, so a bare `voices/s/p/1.0.0/` left by a
/// failed copy is reported as a pack — exit 0, with none of its files present.
#[cfg(unix)]
#[test]
fn a_refused_install_is_not_reported_as_installed_afterwards() {
    let (tmp, home) = home_with_neighbours();
    let src = tmp.path().join("src").join("pack");
    std::fs::create_dir_all(src.join("references")).unwrap();
    std::fs::write(src.join("manifest.yaml"), "id: husk\nversion: 1.0.0\n").unwrap();
    std::fs::write(src.join("system-prompt.md"), "hi\n").unwrap();
    std::fs::write(src.join("references/bs5950.md"), "clause 4\n").unwrap();
    std::os::unix::fs::symlink("/etc", src.join("etc-link")).unwrap();

    let out = run(
        &home,
        &["voice", "install", src.to_str().unwrap(), "--scope", "s"],
    );
    assert_refused_with(&out, 3, "symlink");

    assert!(
        !home.join("voices/s").exists(),
        "a refused install left {} standing",
        home.join("voices/s").display()
    );
    let listed = run(&home, &["voice", "list"]);
    let stdout = String::from_utf8_lossy(&listed.stdout);
    assert!(
        !stdout.contains("husk"),
        "`voice list` reports a pack whose install was refused: {stdout}"
    );
    assert_neighbours_intact(tmp.path(), &home);
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
