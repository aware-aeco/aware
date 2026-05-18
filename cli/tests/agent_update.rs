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
