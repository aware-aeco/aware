mod common;

use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;

// ── tarball + registry fixture helpers ────────────────────────────────────────

/// Build a `.tar.gz` containing the tekla manifest + all its skill files,
/// laid out under the `aware-main/20-agents/tekla/` subdir path that the
/// registry index entry references.
fn build_tekla_tarball(path: &std::path::Path) {
    let tar_gz = std::fs::File::create(path).unwrap();
    let enc = flate2::write::GzEncoder::new(tar_gz, flate2::Compression::default());
    let mut tar = tar::Builder::new(enc);

    let repo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let manifest_text =
        std::fs::read_to_string(repo.join("20-agents/aeco/engineering/tekla/manifest.yaml"))
            .unwrap();

    // Add manifest.yaml
    {
        let mut h = tar::Header::new_gnu();
        h.set_size(manifest_text.len() as u64);
        h.set_mode(0o644);
        h.set_cksum();
        tar.append_data(
            &mut h,
            "aware-main/20-agents/tekla/manifest.yaml",
            manifest_text.as_bytes(),
        )
        .unwrap();
    }

    // Add each skill file listed in the manifest.
    let agent: serde_yaml::Value = serde_yaml::from_str(&manifest_text).unwrap();
    let skills = agent["skills"].as_sequence().unwrap();
    let skills_src = repo.join("20-agents/aeco/engineering/tekla/skills");
    for skill in skills {
        let skill_name = skill.as_str().unwrap();
        let body = std::fs::read_to_string(skills_src.join(skill_name)).unwrap();
        let mut h = tar::Header::new_gnu();
        h.set_size(body.len() as u64);
        h.set_mode(0o644);
        h.set_cksum();
        tar.append_data(
            &mut h,
            format!("aware-main/20-agents/tekla/skills/{skill_name}"),
            body.as_bytes(),
        )
        .unwrap();
    }

    // Flush gzip footer — required or the extract step fails.
    let gz_writer = tar.into_inner().unwrap();
    let mut file = gz_writer.finish().unwrap();
    file.flush().unwrap();
}

/// Convert a path to a `file://` URL with forward slashes (Windows-safe).
fn to_file_url(p: &std::path::Path) -> String {
    format!("file://{}", p.display().to_string().replace('\\', "/"))
}

/// Write a `registry-index.json` to `dir` that references the given tarball
/// via a `file://` URL. Returns the path to the written index file.
fn write_registry_fixture(dir: &std::path::Path, tarball: &std::path::Path) -> std::path::PathBuf {
    let idx_path = dir.join("registry-index.json");
    let tarball_url = to_file_url(tarball);
    let body = format!(
        r#"{{
    "version": "1.0",
    "updated-at": "2026-05-16T00:00:00Z",
    "agents": {{
        "tekla": {{
            "versions": {{
                "2025.0.1": {{ "tarball": "{tarball_url}", "subdir": "aware-main/20-agents/tekla" }}
            }}
        }}
    }},
    "bundles": {{}}
}}"#
    );
    std::fs::write(&idx_path, body).unwrap();
    idx_path
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[test]
fn updates_installed_agent() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let tarball = tmp.path().join("tekla.tar.gz");
    build_tekla_tarball(&tarball);
    let idx_path = write_registry_fixture(tmp.path(), &tarball);
    let idx_url = to_file_url(&idx_path);

    // First: install from registry.
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_REGISTRY", &idx_url)
        .args(["agent", "install", "tekla"])
        .assert()
        .success();

    // Then: update — should succeed (re-pulls and replaces).
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_REGISTRY", &idx_url)
        .args(["agent", "update", "tekla"])
        .assert()
        .success()
        .stdout(predicate::str::contains("updated tekla"));

    // Manifest still present after update.
    assert!(aware.join("agents/tekla/manifest.yaml").is_file());
}

#[test]
fn updates_agent_that_was_not_installed() {
    // `update` should work even if the agent was never installed —
    // the uninstall step is best-effort; then install proceeds normally.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let tarball = tmp.path().join("tekla.tar.gz");
    build_tekla_tarball(&tarball);
    let idx_path = write_registry_fixture(tmp.path(), &tarball);
    let idx_url = to_file_url(&idx_path);

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_REGISTRY", &idx_url)
        .args(["agent", "update", "tekla"])
        .assert()
        .success()
        .stdout(predicate::str::contains("updated tekla"));

    assert!(aware.join("agents/tekla/manifest.yaml").is_file());
}

#[test]
fn update_requires_agent_or_all_flag() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["agent", "update"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("missing <agent>"));
}

#[test]
fn update_rejects_both_agent_and_all_flag() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["agent", "update", "tekla", "--all"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("either <agent> or --all"));
}

#[test]
fn update_all_with_no_agents_installed() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["agent", "update", "--all"])
        .assert()
        .success()
        .stdout(predicate::str::contains("no agents installed"));
}

#[test]
fn update_all_reinstalls_every_installed_agent() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let tarball = tmp.path().join("tekla.tar.gz");
    build_tekla_tarball(&tarball);
    let idx_path = write_registry_fixture(tmp.path(), &tarball);
    let idx_url = to_file_url(&idx_path);

    // Install the agent first.
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_REGISTRY", &idx_url)
        .args(["agent", "install", "tekla"])
        .assert()
        .success();

    // `--all` should iterate every installed id and update each.
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_REGISTRY", &idx_url)
        .args(["agent", "update", "--all"])
        .assert()
        .success()
        .stdout(predicate::str::contains("updating 1 installed agents"))
        .stdout(predicate::str::contains("tekla"))
        .stdout(predicate::str::contains("1 updated"));

    assert!(aware.join("agents/tekla/manifest.yaml").is_file());
}

// ── #174 regression coverage ──────────────────────────────────────────────────

/// Like `build_tekla_tarball`, but rewrites the manifest's `agent:` id so the
/// registry key (`tekla`) differs from the installed id (`tekla.0`) — the exact
/// shape that broke `aware agent update` for `allplan-2024` (#174).
fn build_tekla_tarball_with_agent_id(path: &std::path::Path, agent_id: &str) {
    let tar_gz = std::fs::File::create(path).unwrap();
    let enc = flate2::write::GzEncoder::new(tar_gz, flate2::Compression::default());
    let mut tar = tar::Builder::new(enc);

    let repo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let manifest_text = tekla_manifest_with_agent_id(agent_id);

    {
        let mut h = tar::Header::new_gnu();
        h.set_size(manifest_text.len() as u64);
        h.set_mode(0o644);
        h.set_cksum();
        tar.append_data(
            &mut h,
            "aware-main/20-agents/tekla/manifest.yaml",
            manifest_text.as_bytes(),
        )
        .unwrap();
    }

    let agent: serde_yaml::Value = serde_yaml::from_str(&manifest_text).unwrap();
    let skills = agent["skills"].as_sequence().unwrap();
    let skills_src = repo.join("20-agents/aeco/engineering/tekla/skills");
    for skill in skills {
        let skill_name = skill.as_str().unwrap();
        let body = std::fs::read_to_string(skills_src.join(skill_name)).unwrap();
        let mut h = tar::Header::new_gnu();
        h.set_size(body.len() as u64);
        h.set_mode(0o644);
        h.set_cksum();
        tar.append_data(
            &mut h,
            format!("aware-main/20-agents/tekla/skills/{skill_name}"),
            body.as_bytes(),
        )
        .unwrap();
    }

    let gz_writer = tar.into_inner().unwrap();
    let mut file = gz_writer.finish().unwrap();
    file.flush().unwrap();
}

/// Read the real tekla manifest and rewrite its top-level `agent:` id (which is
/// column-aligned in the source, so a literal replace won't match). `.lines()`
/// also normalizes CRLF → LF.
fn tekla_manifest_with_agent_id(agent_id: &str) -> String {
    let repo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let original =
        std::fs::read_to_string(repo.join("20-agents/aeco/engineering/tekla/manifest.yaml"))
            .unwrap();
    let mut rewrote = false;
    let text = original
        .lines()
        .map(|line| {
            if line.starts_with("agent:") {
                rewrote = true;
                format!("agent: {agent_id}")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert!(rewrote, "fixture: no top-level `agent:` line to rewrite");
    text
}

/// Write a local agent folder (manifest + skills) with a custom `agent:` id,
/// suitable for `aware agent install <path>`.
fn write_tekla_agent_dir(dir: &std::path::Path, agent_id: &str) {
    let repo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let manifest_text = tekla_manifest_with_agent_id(agent_id);
    std::fs::create_dir_all(dir.join("skills")).unwrap();
    std::fs::write(dir.join("manifest.yaml"), &manifest_text).unwrap();

    let agent: serde_yaml::Value = serde_yaml::from_str(&manifest_text).unwrap();
    let skills_src = repo.join("20-agents/aeco/engineering/tekla/skills");
    for skill in agent["skills"].as_sequence().unwrap() {
        let skill_name = skill.as_str().unwrap();
        std::fs::copy(
            skills_src.join(skill_name),
            dir.join("skills").join(skill_name),
        )
        .unwrap();
    }
}

/// Write a single-agent registry index with a custom key / version / tarball.
fn write_registry_index(
    path: &std::path::Path,
    key: &str,
    version: &str,
    tarball_url: &str,
    subdir: &str,
) {
    let body = format!(
        r#"{{
    "version": "1.0",
    "updated-at": "2026-05-16T00:00:00Z",
    "agents": {{
        "{key}": {{
            "versions": {{
                "{version}": {{ "tarball": "{tarball_url}", "subdir": "{subdir}" }}
            }}
        }}
    }},
    "bundles": {{}}
}}"#
    );
    std::fs::write(path, body).unwrap();
}

#[test]
fn update_resolves_base_name_and_replaces_in_place() {
    // #174 (bugs 2 & 3): registry key `tekla` installs as `tekla.0`. Updating by
    // the *installed* id must resolve back to the base-name (no "not in
    // registry") and replace in place (no duplicate `tekla` folder left behind).
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let tarball = tmp.path().join("tekla.tar.gz");
    build_tekla_tarball_with_agent_id(&tarball, "tekla.0");
    let idx_path = write_registry_fixture(tmp.path(), &tarball); // registry key = `tekla`
    let idx_url = to_file_url(&idx_path);

    // Install via base-name `tekla` → lands under the manifest id `tekla.0`.
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_REGISTRY", &idx_url)
        .args(["agent", "install", "tekla"])
        .assert()
        .success()
        .stdout(predicate::str::contains("installed tekla.0"));
    assert!(aware.join("agents/tekla.0/manifest.yaml").is_file());

    // Update using the INSTALLED id — the exact repro that used to fail
    // "not in registry" and delete the agent.
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_REGISTRY", &idx_url)
        .args(["agent", "update", "tekla.0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("updated tekla.0"));

    // Replaced in place: still exactly one folder, no `tekla` duplicate.
    assert!(aware.join("agents/tekla.0/manifest.yaml").is_file());
    assert!(
        !aware.join("agents/tekla").exists(),
        "#174: update must not leave a duplicate base-name folder"
    );
    let count = std::fs::read_dir(aware.join("agents")).unwrap().count();
    assert_eq!(
        count, 1,
        "expected exactly one installed agent, found {count}"
    );
}

#[test]
fn update_when_dropped_from_registry_preserves_install() {
    // #174 (bug 1): if the agent is no longer in the registry, `update` must
    // fail WITHOUT deleting the existing install.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let tarball = tmp.path().join("tekla.tar.gz");
    build_tekla_tarball(&tarball);
    let good_idx = write_registry_fixture(tmp.path(), &tarball);
    let good_url = to_file_url(&good_idx);

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_REGISTRY", &good_url)
        .args(["agent", "install", "tekla"])
        .assert()
        .success();
    assert!(aware.join("agents/tekla/manifest.yaml").is_file());

    // Expire the cached index so `update` re-reads AWARE_REGISTRY.
    let _ = std::fs::remove_file(aware.join("cache/registry-index.json"));

    // Registry that no longer lists tekla.
    let empty_idx = tmp.path().join("empty-index.json");
    std::fs::write(
        &empty_idx,
        r#"{ "version": "1.0", "updated-at": "x", "agents": {}, "bundles": {} }"#,
    )
    .unwrap();
    let empty_url = to_file_url(&empty_idx);

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_REGISTRY", &empty_url)
        .args(["agent", "update", "tekla"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not in registry"));

    assert!(
        aware.join("agents/tekla/manifest.yaml").is_file(),
        "#174: a failed update must not delete the installed agent"
    );
}

#[test]
fn update_with_failed_repull_preserves_install() {
    // #174 (bug 1), network-timeout flavour: the agent is still in the registry
    // but the re-pull fails (unreachable tarball). The existing install must
    // survive untouched rather than being deleted before the failed fetch.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let tarball = tmp.path().join("tekla.tar.gz");
    build_tekla_tarball(&tarball);
    let good_idx = write_registry_fixture(tmp.path(), &tarball); // installs 2025.0.1
    let good_url = to_file_url(&good_idx);

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_REGISTRY", &good_url)
        .args(["agent", "install", "tekla"])
        .assert()
        .success();
    assert!(aware.join("agents/tekla/manifest.yaml").is_file());

    // Expire the cached index so `update` re-reads AWARE_REGISTRY.
    let _ = std::fs::remove_file(aware.join("cache/registry-index.json"));

    // A NEWER version (so the install's tarball cache can't satisfy it) whose
    // tarball does not exist → the fetch fails after the key resolves.
    let broken_idx = tmp.path().join("broken-index.json");
    let missing_url = to_file_url(&tmp.path().join("does-not-exist.tar.gz"));
    write_registry_index(
        &broken_idx,
        "tekla",
        "2025.0.2",
        &missing_url,
        "aware-main/20-agents/tekla",
    );
    let broken_url = to_file_url(&broken_idx);

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_REGISTRY", &broken_url)
        .args(["agent", "update", "tekla"])
        .assert()
        .failure();

    assert!(
        aware.join("agents/tekla/manifest.yaml").is_file(),
        "#174: a failed re-pull must not delete the installed agent"
    );
}

#[test]
fn update_does_not_hijack_custom_agent_sharing_a_key_prefix() {
    // #174 follow-up: a locally-installed agent whose id merely *looks* like
    // `<registry-key>.<suffix>` (here `tekla.dev`) must NOT be resolved to the
    // `tekla` registry entry and replaced. The base-name fallback only applies
    // when the registry entry actually produces that exact install id.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");

    // Registry serves `tekla` (whose manifest id is plain `tekla`).
    let tarball = tmp.path().join("tekla.tar.gz");
    build_tekla_tarball(&tarball);
    let idx_path = write_registry_fixture(tmp.path(), &tarball);
    let idx_url = to_file_url(&idx_path);

    // Install a CUSTOM local agent `tekla.dev` (not from the registry).
    let custom_src = tmp.path().join("src/tekla.dev");
    write_tekla_agent_dir(&custom_src, "tekla.dev");
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["agent", "install", custom_src.to_str().unwrap()])
        .assert()
        .success();
    assert!(aware.join("agents/tekla.dev/manifest.yaml").is_file());

    // Updating it must fail non-destructively — not pull `tekla` over it.
    //
    // Since #370 it is refused one step EARLIER and for a more specific reason: this agent was
    // installed from a folder, which is true and is the thing the operator needs to know. The
    // #174 suffix guard still stands behind it — the `--force` arm below is what proves that,
    // because forcing past the provenance check is the only way to reach it now.
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_REGISTRY", &idx_url)
        .args(["agent", "update", "tekla.dev"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("installed from a local folder"));

    // #174 itself, still enforced: with the provenance check waived, the suffix fallback must
    // STILL refuse rather than resolve `tekla.dev` to key `tekla` and replace it. Without this
    // arm, #370's guard would have quietly become the only thing standing between a custom agent
    // and the registry — and `--force` would walk straight through it.
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_REGISTRY", &idx_url)
        .args(["agent", "update", "tekla.dev", "--force"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not in registry"));

    assert!(
        aware.join("agents/tekla.dev/manifest.yaml").is_file(),
        "#174: a custom agent must not be hijacked/deleted by suffix matching"
    );
    assert!(
        !aware.join("agents/tekla").exists(),
        "registry `tekla` must not have been installed over the custom agent"
    );
}

// ── #363: reaching a version that is not the newest ───────────────────────────

/// A minimal one-command agent tarball declaring `version`, laid out under the
/// subdir its index entry references. Two of these at different versions are what
/// makes "reach an OLDER one" testable at all — the tekla fixture above ships a
/// single version, so it cannot tell "picked the one I asked for" from "picked
/// the only one there is".
fn build_probe_tarball(path: &std::path::Path, version: &str) {
    let tar_gz = std::fs::File::create(path).unwrap();
    let enc = flate2::write::GzEncoder::new(tar_gz, flate2::Compression::default());
    let mut tar = tar::Builder::new(enc);
    let manifest = format!(
        "agent: probe-agent\nversion: {version}\ndescription: a two-version fixture\n\
         stateful: false\nlicense: MIT\ntransport:\n  cli:\n    binary: aware-probe\n\
         commands:\n  probe:\n    lifecycle: single\n    description: x\n    mode: read\n"
    );
    let mut h = tar::Header::new_gnu();
    h.set_size(manifest.len() as u64);
    h.set_mode(0o644);
    h.set_cksum();
    tar.append_data(
        &mut h,
        "aware-main/20-agents/probe-agent/manifest.yaml",
        manifest.as_bytes(),
    )
    .unwrap();
    let gz_writer = tar.into_inner().unwrap();
    let mut file = gz_writer.finish().unwrap();
    file.flush().unwrap();
}

/// A registry carrying `probe-agent` at 1.2.0 and 1.3.0, each from its own
/// tarball, so which one landed is visible in the installed manifest.
fn two_version_registry(dir: &std::path::Path) -> String {
    let old = dir.join("probe-1.2.0.tar.gz");
    let new = dir.join("probe-1.3.0.tar.gz");
    build_probe_tarball(&old, "1.2.0");
    build_probe_tarball(&new, "1.3.0");
    let idx_path = dir.join("registry-index.json");
    std::fs::write(
        &idx_path,
        format!(
            r#"{{
    "version": "1.0",
    "updated-at": "2026-05-16T00:00:00Z",
    "agents": {{
        "probe-agent": {{
            "versions": {{
                "1.2.0": {{ "tarball": "{}", "subdir": "aware-main/20-agents/probe-agent" }},
                "1.3.0": {{ "tarball": "{}", "subdir": "aware-main/20-agents/probe-agent" }}
            }}
        }}
    }},
    "bundles": {{}}
}}"#,
            to_file_url(&old),
            to_file_url(&new)
        ),
    )
    .unwrap();
    to_file_url(&idx_path)
}

fn installed_version(aware: &std::path::Path) -> String {
    let text = std::fs::read_to_string(aware.join("agents/probe-agent/manifest.yaml")).unwrap();
    let m: serde_yaml::Value = serde_yaml::from_str(&text).unwrap();
    m["version"].as_str().unwrap().to_string()
}

fn probe(aware: &std::path::Path, idx: &str) -> Command {
    let mut c = Command::cargo_bin("aware").unwrap();
    c.env("AWARE_HOME", aware).env("AWARE_REGISTRY", idx);
    c
}

#[test]
fn update_reaches_a_version_that_is_not_the_newest() {
    // The issue's own repro. There was no single command that moved an installed
    // agent to a chosen version: `install` refuses while a copy is on disk
    // ("already installed; use `aware agent update`") and `update` only ever
    // pulled the newest — the two messages pointing at each other, with
    // `uninstall` + `install <id>@<v>` the only way through, which destroys a
    // locally-installed agent before failing.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let idx = two_version_registry(tmp.path());

    probe(&aware, &idx)
        .args(["agent", "install", "probe-agent"])
        .assert()
        .success();
    assert_eq!(
        installed_version(&aware),
        "1.3.0",
        "install takes the newest"
    );

    probe(&aware, &idx)
        .args(["agent", "update", "probe-agent@1.2.0"])
        .assert()
        .success()
        // The version is echoed, so the operator can see which one landed
        // without going to look.
        .stdout(predicate::str::contains("updated probe-agent to 1.2.0"));
    assert_eq!(
        installed_version(&aware),
        "1.2.0",
        "the DOWNGRADE is the point — this is what needed uninstall-then-install"
    );
}

#[test]
fn update_without_a_version_still_takes_the_newest() {
    // The control, and the compatibility guarantee: adding the argument must not
    // change what the bare command has always done. Run from the downgraded
    // state, so "newest" and "unchanged" are different answers.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let idx = two_version_registry(tmp.path());

    probe(&aware, &idx)
        .args(["agent", "install", "probe-agent"])
        .assert()
        .success();
    probe(&aware, &idx)
        .args(["agent", "update", "probe-agent@1.2.0"])
        .assert()
        .success();
    assert_eq!(installed_version(&aware), "1.2.0");

    probe(&aware, &idx)
        .args(["agent", "update", "probe-agent"])
        .assert()
        .success()
        .stdout(predicate::str::contains("updated probe-agent"));
    assert_eq!(installed_version(&aware), "1.3.0", "bare update = newest");
}

#[test]
fn a_version_the_registry_lacks_leaves_the_install_untouched() {
    // The property that makes this the right verb to carry a version, and the one
    // the remedy in `validate.rs` now tells operators to rely on: the resolve
    // happens before any filesystem work, so a miss cannot cost them the agent
    // they have. `install`+`uninstall` could not offer that.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let idx = two_version_registry(tmp.path());

    probe(&aware, &idx)
        .args(["agent", "install", "probe-agent"])
        .assert()
        .success();

    probe(&aware, &idx)
        .args(["agent", "update", "probe-agent@9.9.9"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "probe-agent@9.9.9 not in registry",
        ));

    assert_eq!(
        installed_version(&aware),
        "1.3.0",
        "a failed version-selecting update must leave the existing install alone"
    );
}

#[test]
fn a_spec_missing_one_side_of_the_at_is_refused() {
    // `@` with nothing on one side is a typo, not a request. Refused with a
    // message that names the shape, rather than resolving an empty id or an
    // empty version into a registry lookup.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let idx = two_version_registry(tmp.path());
    probe(&aware, &idx)
        .args(["agent", "install", "probe-agent"])
        .assert()
        .success();

    for bad in ["probe-agent@", "@1.2.0"] {
        probe(&aware, &idx)
            .args(["agent", "update", bad])
            .assert()
            .failure()
            .code(3)
            .stderr(predicate::str::contains("<agent>[@<version>]"));
    }
    assert_eq!(installed_version(&aware), "1.3.0", "and nothing moved");
}

// ── #370: update must not destroy something the registry did not put there ────

/// Install a LOCAL agent whose id is also a registry key — the collision the #174 hijack guard
/// misses, because that one only covers the suffix case (`tekla.dev` resolving to key `tekla`).
fn install_local_fork(aware: &std::path::Path, tmp: &std::path::Path, version: &str) {
    let src = tmp.join("fork").join("probe-agent");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(
        src.join("manifest.yaml"),
        format!(
            "agent: probe-agent\nversion: {version}\ndescription: a LOCAL fork\nstateful: false\n\
             license: MIT\ntransport:\n  cli:\n    binary: aware-probe\ncommands:\n  probe:\n    \
             lifecycle: single\n    description: my local change\n    mode: read\n"
        ),
    )
    .unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", aware)
        .args(["agent", "install"])
        .arg(&src)
        .assert()
        .success();
}

fn installed_manifest(aware: &std::path::Path) -> String {
    std::fs::read_to_string(aware.join("agents/probe-agent/manifest.yaml")).unwrap()
}

#[test]
fn update_refuses_to_replace_a_locally_installed_agent() {
    // The issue reproduction. `update` resolves the id in the registry and replaces whatever sits
    // at `agents/<id>/` — right for a registry install, unrecoverable for a folder one, because
    // the staging directory holds the registry payload and not the local work. Exit 0, no warning.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let idx = two_version_registry(tmp.path());
    install_local_fork(&aware, tmp.path(), "0.0.1-my-local-fork");

    probe(&aware, &idx)
        .args(["agent", "update", "probe-agent"])
        .assert()
        .failure()
        .code(8)
        // Names the folder, so the operator can act rather than guess.
        .stderr(predicate::str::contains("installed from a local folder"))
        .stderr(predicate::str::contains("fork"))
        .stderr(predicate::str::contains("--force"));

    assert!(
        installed_manifest(&aware).contains("0.0.1-my-local-fork"),
        "the local fork must still be on disk — that is the whole issue"
    );
}

#[test]
fn a_version_selecting_update_is_refused_the_same_way() {
    // #363 gave `update` a version argument, which makes this destructive path EASIER to reach:
    // the remedy an unsatisfied pin prints leads straight to it. Same guard, same answer.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let idx = two_version_registry(tmp.path());
    install_local_fork(&aware, tmp.path(), "0.0.1-my-local-fork");

    probe(&aware, &idx)
        .args(["agent", "update", "probe-agent@1.2.0"])
        .assert()
        .failure()
        .code(8);
    assert!(installed_manifest(&aware).contains("0.0.1-my-local-fork"));
}

#[test]
fn force_takes_the_registry_copy_when_that_is_what_you_want() {
    // The guard must not be a wall. An operator who genuinely wants the registry version over
    // their own says so once — and the marker is rewritten, so the agent is registry-sourced after.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let idx = two_version_registry(tmp.path());
    install_local_fork(&aware, tmp.path(), "0.0.1-my-local-fork");

    probe(&aware, &idx)
        .args(["agent", "update", "probe-agent@1.2.0", "--force"])
        .assert()
        .success();
    assert_eq!(installed_version(&aware), "1.2.0");

    // …and now it updates with no ceremony, because it IS registry-sourced now.
    probe(&aware, &idx)
        .args(["agent", "update", "probe-agent"])
        .assert()
        .success();
    assert_eq!(installed_version(&aware), "1.3.0");
}

#[test]
fn a_registry_install_still_updates_with_no_ceremony() {
    // The negative control that matters most: a guard that refused everything would pass every
    // test above while breaking the command for its ordinary use.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let idx = two_version_registry(tmp.path());
    probe(&aware, &idx)
        .args(["agent", "install", "probe-agent"])
        .assert()
        .success();
    probe(&aware, &idx)
        .args(["agent", "update", "probe-agent@1.2.0"])
        .assert()
        .success();
    assert_eq!(installed_version(&aware), "1.2.0");
}

#[test]
fn an_install_predating_the_marker_is_judged_by_its_version() {
    // Back-compat, and the half that needed a judgement call. An agent installed before provenance
    // existed carries no marker, and "unknown" must not silently mean "registry" — that assumption
    // is exactly what destroyed forks, for precisely the people most likely to have one.
    //
    // So ask the registry a different question: does it publish the version that is installed? A
    // registry install always matches a published key; a hand-built one almost never does.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let idx = two_version_registry(tmp.path());
    probe(&aware, &idx)
        .args(["agent", "install", "probe-agent"])
        .assert()
        .success();
    let marker = aware.join("agents/probe-agent/.aware-install.yaml");
    std::fs::remove_file(&marker).unwrap();

    // 1.3.0 IS published, so this reads as a registry install and proceeds — no false refusal for
    // everyone who installed before this shipped.
    probe(&aware, &idx)
        .args(["agent", "update", "probe-agent@1.2.0"])
        .assert()
        .success();

    // Now a version the registry does not publish, still with no marker: refused, and the message
    // says it is inferring rather than claiming a record it does not have.
    std::fs::remove_file(&marker).unwrap();
    let m = aware.join("agents/probe-agent/manifest.yaml");
    let body = std::fs::read_to_string(&m).unwrap();
    std::fs::write(
        &m,
        body.replace("version: 1.2.0", "version: 9.9.9-hand-built"),
    )
    .unwrap();
    probe(&aware, &idx)
        .args(["agent", "update", "probe-agent"])
        .assert()
        .failure()
        .code(8)
        .stderr(predicate::str::contains("does not publish"))
        .stderr(predicate::str::contains("inference, not a record"));
}

#[test]
fn install_records_where_it_came_from() {
    // The fact the guard reads. Asserted directly, so a regression in the WRITE side is not
    // diagnosed through a confusing failure in the guard.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let idx = two_version_registry(tmp.path());
    probe(&aware, &idx)
        .args(["agent", "install", "probe-agent"])
        .assert()
        .success();
    let marker =
        std::fs::read_to_string(aware.join("agents/probe-agent/.aware-install.yaml")).unwrap();
    assert!(marker.contains("source: registry"), "got: {marker}");
    assert!(
        marker.contains("1.3.0"),
        "the resolved version belongs in it: {marker}"
    );

    let tmp2 = tempfile::tempdir().unwrap();
    let aware2 = tmp2.path().join("aware");
    install_local_fork(&aware2, tmp2.path(), "0.0.1-mine");
    let marker2 =
        std::fs::read_to_string(aware2.join("agents/probe-agent/.aware-install.yaml")).unwrap();
    assert!(marker2.contains("source: local"), "got: {marker2}");
    assert!(
        marker2.contains("fork"),
        "the folder belongs in it: {marker2}"
    );
}

/// A registry whose KEY differs from the id its payload installs under — `probe` publishing an
/// agent whose `manifest.agent` is `probe-agent`. Both supported shapes produce this: a key that
/// installs under a suffixed id (`allplan-2024` -> `allplan-2024.0`) and an `alias-of` rename.
fn renaming_registry(dir: &std::path::Path) -> String {
    let tar = dir.join("renaming.tar.gz");
    build_probe_tarball(&tar, "2.0.0");
    let idx_path = dir.join("renaming-index.json");
    std::fs::write(
        &idx_path,
        format!(
            r#"{{
    "version": "1.0",
    "updated-at": "2026-05-16T00:00:00Z",
    "agents": {{
        "probe": {{
            "versions": {{
                "2.0.0": {{ "tarball": "{}", "subdir": "aware-main/20-agents/probe-agent" }}
            }}
        }}
    }},
    "bundles": {{}}
}}"#,
            to_file_url(&tar)
        ),
    )
    .unwrap();
    to_file_url(&idx_path)
}

#[test]
fn update_by_registry_key_cannot_delete_a_local_agent_at_the_resulting_id() {
    // The route that defeated the FIRST cut of this fix, and the reason it needed a second.
    //
    // `update_agent_from_registry` removes TWO directories: `agents/<id>` (the spec you typed) and
    // `agents/<new_name>` (the id the payload installs under). Guarding only the first left the
    // whole bug alive whenever they differ — which they do for a suffixed install id and for an
    // `alias-of` rename, both supported and both already tested elsewhere in this file.
    //
    // Reproduced against the real binary before the second fix: `aware agent update probe` replaced
    // a local fork at `agents/probe-agent/` and exited 0.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let idx = renaming_registry(tmp.path());
    install_local_fork(&aware, tmp.path(), "0.0.1-my-local-fork");

    probe(&aware, &idx)
        .args(["agent", "update", "probe"])
        .assert()
        .failure()
        .code(8)
        .stderr(predicate::str::contains("installed from a local folder"));

    assert!(
        installed_manifest(&aware).contains("0.0.1-my-local-fork"),
        "the fork sits at the id the PAYLOAD installs under, which the first guard never looked at"
    );
}

#[test]
fn an_installed_agent_whose_manifest_will_not_parse_is_not_deleted() {
    // The unknown-provenance branch reads the installed version to infer whether this came from the
    // registry. When the manifest will not parse there is no version to read — and treating that as
    // "nothing installed" deleted the directory, which is the opposite of the rule this guard
    // exists to enforce. An unparseable manifest is a likelier sign of a hand-edited fork than of a
    // registry install.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let idx = two_version_registry(tmp.path());
    let dir = aware.join("agents").join("probe-agent");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("manifest.yaml"), "agent: [not\n  yaml\n").unwrap();

    probe(&aware, &idx)
        .args(["agent", "update", "probe-agent"])
        .assert()
        .failure()
        .code(8)
        .stderr(predicate::str::contains("manifest cannot be read"));
    // The CONTENT, not merely a file of that name — a successful replacement would also leave a
    // `manifest.yaml` here, so asserting existence alone would pass either way.
    assert!(
        std::fs::read_to_string(dir.join("manifest.yaml"))
            .unwrap()
            .contains("[not"),
        "the unparseable manifest must still be there, untouched"
    );
}

#[test]
fn a_registry_update_records_that_it_came_from_the_registry() {
    // The WRITE side of the update path, which nothing pinned: deleting it left the whole suite
    // green, because the fixture publishes both versions so the unknown-inference branch proceeds
    // anyway and masks the missing marker. A regression there would silently make every
    // registry-updated agent read as "unknown" forever.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let idx = two_version_registry(tmp.path());
    probe(&aware, &idx)
        .args(["agent", "install", "probe-agent"])
        .assert()
        .success();
    std::fs::remove_file(aware.join("agents/probe-agent/.aware-install.yaml")).unwrap();

    probe(&aware, &idx)
        .args(["agent", "update", "probe-agent@1.2.0"])
        .assert()
        .success();

    let marker =
        std::fs::read_to_string(aware.join("agents/probe-agent/.aware-install.yaml")).unwrap();
    assert!(
        marker.contains("source: registry"),
        "an update must leave the agent MARKED, not merely replaced: {marker}"
    );
    assert!(
        marker.contains("1.2.0"),
        "with the version it landed: {marker}"
    );
}

#[test]
fn a_local_install_over_a_copied_registry_agent_does_not_inherit_its_marker() {
    // `install <dir>` copies the whole directory, so if the source is itself an installed registry
    // agent it carries a `source: registry` marker saying the wrong thing about this install.
    //
    // What this pins is that the marker is written AFTER the copy, so the inherited one is
    // overwritten. It does NOT pin the belt-and-braces `remove_file` that precedes the write —
    // that line only matters when the write itself FAILS, which no test can induce without fault
    // injection. Said plainly so nobody later reads this as coverage of that line.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let idx = two_version_registry(tmp.path());
    probe(&aware, &idx)
        .args(["agent", "install", "probe-agent"])
        .assert()
        .success();

    // Copy the INSTALLED agent (marker and all) into a source folder, then install it elsewhere.
    let src = tmp.path().join("copied").join("probe-agent");
    std::fs::create_dir_all(&src).unwrap();
    for f in ["manifest.yaml", ".aware-install.yaml"] {
        std::fs::copy(aware.join("agents/probe-agent").join(f), src.join(f)).unwrap();
    }
    let aware2 = tmp.path().join("aware2");
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware2)
        .args(["agent", "install"])
        .arg(&src)
        .assert()
        .success();

    let marker =
        std::fs::read_to_string(aware2.join("agents/probe-agent/.aware-install.yaml")).unwrap();
    assert!(
        marker.contains("source: local"),
        "installing FROM a folder is a local install, whatever that folder used to be: {marker}"
    );
}
