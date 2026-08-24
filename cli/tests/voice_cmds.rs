//! `aware voice ...` end to end.
//!
//! The voice-pack command group shipped in v0.25 with four subcommands and no
//! integration coverage at all: the only tests in `commands/voice.rs` exercised the
//! version comparator added later by #377, so every guard in `install`, every branch
//! of `describe`, the depth `list` walks, and the directory `uninstall` actually
//! deletes were unpinned. These drive the real binary against a real `AWARE_HOME`.

use assert_cmd::Command;
use predicates::prelude::*;
use std::path::Path;

/// Write a voice-pack source folder: `manifest.yaml` (+ optional `scope:`), a
/// `system-prompt.md`, and one file under `references/` so the recursive copy has
/// something nested to carry.
fn write_pack(dir: &Path, id: &str, version: &str, scope: Option<&str>) {
    std::fs::create_dir_all(dir.join("references")).unwrap();
    // Quoted, because YAML would read a calendar version as a float and `2025.10` would
    // land in a folder called `2025.1`. Coercion of genuinely non-string scalars has its
    // own test below; here the folder names need to be exactly what the caller asked for.
    let mut manifest = format!("id: \"{id}\"\nversion: \"{version}\"\n");
    if let Some(scope) = scope {
        manifest.push_str(&format!("scope: \"{scope}\"\n"));
    }
    std::fs::write(dir.join("manifest.yaml"), manifest).unwrap();
    std::fs::write(
        dir.join("system-prompt.md"),
        format!("You are the {id} reviewer.\n"),
    )
    .unwrap();
    std::fs::write(dir.join("references/bs-5950.md"), "clause 4.2\n").unwrap();
}

fn aware(home: &Path) -> Command {
    let mut cmd = Command::cargo_bin("aware").unwrap();
    cmd.env("AWARE_HOME", home);
    cmd
}

/// Install `dir` and return the home it landed in, asserting the command succeeded.
fn install(home: &Path, dir: &Path, scope_flag: Option<&str>) {
    let mut cmd = aware(home);
    cmd.args(["voice", "install"]).arg(dir);
    if let Some(scope) = scope_flag {
        cmd.args(["--scope", scope]);
    }
    cmd.assert().success();
}

#[test]
fn install_lays_the_pack_out_under_scope_id_version_and_copies_nested_files() {
    // The storage contract from the module docs: `<home>/voices/<scope>/<id>/<version>/`,
    // with the pack's own tree copied in — including subdirectories, which is the only
    // part of `copy_dir_recursive` that recurses.
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    let src = tmp.path().join("src");
    write_pack(&src, "uk-structural-reviewer", "2025.1", Some("ise"));

    install(&home, &src, None);

    let dst = home.join("voices/ise/uk-structural-reviewer/2025.1");
    assert!(dst.join("manifest.yaml").is_file(), "manifest copied");
    assert!(dst.join("system-prompt.md").is_file(), "prompt copied");
    assert_eq!(
        std::fs::read_to_string(dst.join("references/bs-5950.md")).unwrap(),
        "clause 4.2\n",
        "a nested reference file is copied with its contents, not just its directory"
    );
}

#[test]
fn an_explicit_scope_flag_wins_over_the_manifests_own() {
    // `--scope` is checked before the manifest's `scope:`, so an operator installing
    // someone else's pack under their own namespace gets that namespace and nothing is
    // left behind in the pack author's.
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    let src = tmp.path().join("src");
    write_pack(&src, "rev", "1.0", Some("ise"));

    install(&home, &src, Some("acme"));

    assert!(home.join("voices/acme/rev/1.0/manifest.yaml").is_file());
    assert!(
        !home.join("voices/ise").exists(),
        "the manifest's own scope must not also be written"
    );
}

#[test]
fn without_the_flag_the_manifests_scope_is_used() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    let src = tmp.path().join("src");
    write_pack(&src, "rev", "1.0", Some("ise"));

    install(&home, &src, None);

    assert!(home.join("voices/ise/rev/1.0/manifest.yaml").is_file());
}

#[test]
fn a_non_string_scalar_still_names_the_folder() {
    // `version: 2025` and `scope: 2026` are YAML *numbers*, not strings — a manifest
    // author has no reason to quote a calendar version. The install must coerce them
    // rather than report the field missing, or a perfectly ordinary manifest is refused.
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    let src = tmp.path().join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(
        src.join("manifest.yaml"),
        "id: rev\nversion: 2025\nscope: 42\n",
    )
    .unwrap();
    std::fs::write(src.join("system-prompt.md"), "prompt\n").unwrap();

    install(&home, &src, None);

    assert!(
        home.join("voices/42/rev/2025/manifest.yaml").is_file(),
        "a numeric version and scope name the folders they would as strings"
    );
}

#[test]
fn install_refuses_each_thing_it_needs_and_says_which() {
    // One case per guard in `install`, each asserting the message that names it — a
    // guard that stops firing fails here rather than silently writing a broken pack.
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");

    let not_a_dir = tmp.path().join("plain.txt");
    std::fs::write(&not_a_dir, "x").unwrap();

    let empty = tmp.path().join("empty");
    std::fs::create_dir_all(&empty).unwrap();

    let bad_yaml = tmp.path().join("bad-yaml");
    std::fs::create_dir_all(&bad_yaml).unwrap();
    std::fs::write(bad_yaml.join("manifest.yaml"), "id: [unclosed\n").unwrap();

    let no_id = tmp.path().join("no-id");
    std::fs::create_dir_all(&no_id).unwrap();
    std::fs::write(no_id.join("manifest.yaml"), "version: 1\nscope: s\n").unwrap();

    let no_version = tmp.path().join("no-version");
    std::fs::create_dir_all(&no_version).unwrap();
    std::fs::write(no_version.join("manifest.yaml"), "id: rev\nscope: s\n").unwrap();

    let no_scope = tmp.path().join("no-scope");
    std::fs::create_dir_all(&no_scope).unwrap();
    std::fs::write(no_scope.join("manifest.yaml"), "id: rev\nversion: 1\n").unwrap();

    // A mapping value where a scalar is required is *present* but uncoercible — it must
    // be refused the same way an absent field is, not stringified into a folder name.
    let map_id = tmp.path().join("map-id");
    std::fs::create_dir_all(&map_id).unwrap();
    std::fs::write(map_id.join("manifest.yaml"), "id:\n  a: b\nversion: 1\n").unwrap();

    for (src, needle) in [
        (&not_a_dir, "is not a directory"),
        (&empty, "no manifest.yaml in"),
        (&bad_yaml, "manifest YAML:"),
        (&no_id, "manifest missing `id`"),
        (&no_version, "manifest missing `version`"),
        (&no_scope, "scope required"),
        (&map_id, "manifest missing `id`"),
    ] {
        aware(&home)
            .args(["voice", "install"])
            .arg(src)
            .assert()
            .failure()
            .stderr(predicate::str::contains(needle));
    }
    assert!(
        !home.join("voices").exists(),
        "no refused install may leave anything behind under voices/"
    );
}

#[test]
fn list_says_nothing_is_installed_before_anything_is() {
    let tmp = tempfile::tempdir().unwrap();
    aware(&tmp.path().join("home"))
        .args(["voice", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("(no voice packs installed)"))
        .stdout(predicate::str::contains("aware voice install"));
}

#[test]
fn list_reports_the_scope_id_and_version_of_each_installed_pack() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    for (id, version, scope) in [("rev", "2025.1", "ise"), ("critic", "0.2", "acme")] {
        let src = tmp.path().join(format!("src-{id}"));
        write_pack(&src, id, version, Some(scope));
        install(&home, &src, None);
    }

    aware(&home)
        .args(["voice", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("SCOPE"))
        .stdout(predicate::str::contains("ise"))
        .stdout(predicate::str::contains("rev"))
        .stdout(predicate::str::contains("2025.1"))
        .stdout(predicate::str::contains("acme"))
        .stdout(predicate::str::contains("critic"))
        .stdout(predicate::str::contains("0.2"));
}

#[test]
fn list_walks_exactly_three_directory_levels_and_ignores_stray_files() {
    // `list` reports a pack only at scope/id/version depth. Files sitting at any of
    // those levels (a `.DS_Store`, an editor backup, a stray README) are not packs and
    // must not be reported as one — each level has its own `is_dir` check.
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    let voices = home.join("voices");
    std::fs::create_dir_all(voices.join("ise/rev/2025.1")).unwrap();
    std::fs::write(voices.join("STRAY-SCOPE.txt"), "x").unwrap();
    std::fs::write(voices.join("ise/STRAY-ID.txt"), "x").unwrap();
    std::fs::write(voices.join("ise/rev/STRAY-VERSION.txt"), "x").unwrap();

    aware(&home)
        .args(["voice", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2025.1"))
        .stdout(predicate::str::contains("STRAY-SCOPE").not())
        .stdout(predicate::str::contains("STRAY-ID").not())
        .stdout(predicate::str::contains("STRAY-VERSION").not());
}

#[test]
fn list_reports_nothing_when_voices_exists_but_holds_no_pack() {
    // A `voices/` directory left behind by an uninstall is not "one empty pack" — the
    // header must not print with nothing under it.
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    std::fs::create_dir_all(home.join("voices/ise")).unwrap();

    aware(&home)
        .args(["voice", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("(no voice packs installed)"))
        .stdout(predicate::str::contains("SCOPE").not());
}

#[test]
fn describe_prints_the_manifest_the_prompt_and_the_reference_names() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    let src = tmp.path().join("src");
    write_pack(&src, "rev", "2025.1", Some("ise"));
    install(&home, &src, None);

    aware(&home)
        .args(["voice", "describe", "ise/rev"])
        .assert()
        .success()
        .stdout(predicate::str::contains("# manifest.yaml"))
        .stdout(predicate::str::contains("id: \"rev\""))
        .stdout(predicate::str::contains("# system-prompt.md"))
        .stdout(predicate::str::contains("You are the rev reviewer."))
        .stdout(predicate::str::contains("# references/"))
        .stdout(predicate::str::contains("bs-5950.md"));
}

#[test]
fn describe_omits_the_sections_a_pack_does_not_have() {
    // A pack may ship a manifest alone. The prompt and reference headings are each
    // gated on the file/dir existing, so neither may be printed for this pack.
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    let dst = home.join("voices/ise/bare/1.0");
    std::fs::create_dir_all(&dst).unwrap();
    std::fs::write(dst.join("manifest.yaml"), "id: bare\nversion: 1.0\n").unwrap();

    aware(&home)
        .args(["voice", "describe", "ise/bare"])
        .assert()
        .success()
        .stdout(predicate::str::contains("id: bare"))
        .stdout(predicate::str::contains("# system-prompt.md").not())
        .stdout(predicate::str::contains("# references/").not());
}

#[test]
fn describe_refuses_a_version_directory_with_no_manifest() {
    // The resolver only asks whether the directory exists; `describe` is what notices
    // there is no pack in it, and it names the path so the operator can see what it
    // looked for.
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    std::fs::create_dir_all(home.join("voices/ise/rev/2025.1")).unwrap();

    aware(&home)
        .args(["voice", "describe", "ise/rev"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("voice pack manifest at"))
        .stderr(predicate::str::contains("manifest.yaml"));
}

#[test]
fn a_pack_id_without_a_scope_is_refused_as_malformed() {
    // `<scope>/<id>` is the whole addressing scheme; a bare word cannot be resolved, and
    // saying so beats hunting for a directory that could never exist.
    let tmp = tempfile::tempdir().unwrap();
    aware(&tmp.path().join("home"))
        .args(["voice", "describe", "noslash"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid pack id: noslash"));
}

#[test]
fn an_uninstalled_pack_is_reported_missing_rather_than_empty() {
    let tmp = tempfile::tempdir().unwrap();
    aware(&tmp.path().join("home"))
        .args(["voice", "describe", "ise/absent"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("ise/absent not installed"));
}

#[test]
fn a_pinned_version_resolves_to_that_version_and_only_that_one() {
    // The `@<version>` branch of the resolver, which the comparator tests never reach:
    // it must take the named version even when a newer one is installed, and refuse a
    // version that is not there instead of silently falling back to the newest.
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    for version in ["2025.9", "2025.10"] {
        let src = tmp.path().join(format!("src-{version}"));
        write_pack(&src, "rev", version, Some("ise"));
        install(&home, &src, None);
    }

    aware(&home)
        .args(["voice", "describe", "@ise/rev@2025.9"])
        .assert()
        .success()
        .stdout(predicate::str::contains("version: \"2025.9\""));

    aware(&home)
        .args(["voice", "describe", "ise/rev@2024.1"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("ise/rev@2024.1 not installed"));
}

#[test]
fn uninstall_removes_only_the_version_it_was_pinned_to() {
    // `uninstall` feeds the resolver's answer straight to `remove_dir_all`, so which
    // directory the resolver returns is a destructive decision. Pinned, it must take
    // that version and leave its siblings — and the pack itself — on disk.
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    for version in ["2025.9", "2025.10"] {
        let src = tmp.path().join(format!("src-{version}"));
        write_pack(&src, "rev", version, Some("ise"));
        install(&home, &src, None);
    }

    aware(&home)
        .args(["voice", "uninstall", "ise/rev@2025.9"])
        .assert()
        .success()
        .stdout(predicate::str::contains("uninstalled"));

    assert!(
        !home.join("voices/ise/rev/2025.9").exists(),
        "the pinned version is gone"
    );
    assert!(
        home.join("voices/ise/rev/2025.10/manifest.yaml").is_file(),
        "the sibling version is untouched"
    );
}

#[test]
fn an_unpinned_uninstall_deletes_the_newest_version_and_leaves_the_rest() {
    // #377 changed which directory an unpinned `uninstall` DELETES: before it, `2025.9`
    // beat `2025.10` as a string. The old test for this resolved a path and then
    // asserted the loser still existed without ever deleting anything, so it held
    // whatever `uninstall` did. This runs the command.
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    for version in ["2025.9", "2025.10"] {
        let src = tmp.path().join(format!("src-{version}"));
        write_pack(&src, "rev", version, Some("ise"));
        install(&home, &src, None);
    }

    aware(&home)
        .args(["voice", "uninstall", "ise/rev"])
        .assert()
        .success();

    assert!(
        !home.join("voices/ise/rev/2025.10").exists(),
        "the newest version is the one deleted"
    );
    assert!(
        home.join("voices/ise/rev/2025.9/manifest.yaml").is_file(),
        "the older version survives"
    );
}

#[test]
fn uninstalling_a_pack_that_is_not_installed_removes_nothing() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    let src = tmp.path().join("src");
    write_pack(&src, "rev", "2025.1", Some("ise"));
    install(&home, &src, None);

    aware(&home)
        .args(["voice", "uninstall", "ise/other"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("ise/other not installed"));

    assert!(
        home.join("voices/ise/rev/2025.1/manifest.yaml").is_file(),
        "a refused uninstall leaves the installed pack alone"
    );
}

#[test]
fn reinstalling_a_version_refreshes_its_files_in_place() {
    // Install writes into `<scope>/<id>/<version>/` without clearing it first, so a
    // re-install of the same version overwrites what it carries. That is what makes
    // `install` usable as "refresh this pack", and it is worth pinning because the
    // alternative — refusing, as `agent install` does — would be a silent behaviour flip.
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("home");
    let src = tmp.path().join("src");
    write_pack(&src, "rev", "2025.1", Some("ise"));
    install(&home, &src, None);

    std::fs::write(src.join("system-prompt.md"), "REVISED PROMPT\n").unwrap();
    install(&home, &src, None);

    assert_eq!(
        std::fs::read_to_string(home.join("voices/ise/rev/2025.1/system-prompt.md")).unwrap(),
        "REVISED PROMPT\n",
        "the second install replaced the prompt rather than keeping the first"
    );
}
