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
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_REGISTRY", &idx_url)
        .args(["agent", "update", "tekla.dev"])
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
