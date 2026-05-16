mod common;

use assert_cmd::Command;
use predicates::prelude::*;

// ── registry / bundle helpers ──────────────────────────────────────────────

/// Build a `.tar.gz` containing the tekla manifest + all its skill files,
/// laid out under the `aware-main/20-agents/tekla/` subdir path that the
/// registry index entry references.
fn build_tekla_tarball(path: &std::path::Path) {
    use std::io::Write;

    let tar_gz = std::fs::File::create(path).unwrap();
    let enc = flate2::write::GzEncoder::new(tar_gz, flate2::Compression::default());
    let mut tar = tar::Builder::new(enc);

    let repo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let manifest_src = repo.join("20-agents/aeco/engineering/tekla/manifest.yaml");
    let manifest_text = std::fs::read_to_string(&manifest_src).unwrap();

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

/// Write a `registry-index.json` to `dir` that references the given tarball
/// via a `file://` URL. Returns the path to the written index file.
///
/// The index has:
///   - agent `tekla@2025.0.1` → tarball
///   - bundle `aware-tiny` → `["tekla@2025.0.1"]`
fn write_registry_fixture(dir: &std::path::Path, tarball: &std::path::Path) -> std::path::PathBuf {
    let idx_path = dir.join("registry-index.json");
    // Convert backslashes to forward slashes so `file://C:/...` is valid.
    let tarball_url = format!(
        "file://{}",
        tarball.display().to_string().replace('\\', "/")
    );
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
    "bundles": {{
        "aware-tiny": {{
            "description": "One-agent test bundle",
            "agents": ["tekla@2025.0.1"]
        }}
    }}
}}"#
    );
    std::fs::write(&idx_path, body).unwrap();
    idx_path
}

/// Convert a path to a `file://` URL with forward slashes (Windows-safe).
fn to_file_url(p: &std::path::Path) -> String {
    format!("file://{}", p.display().to_string().replace('\\', "/"))
}

// ── existing tests ─────────────────────────────────────────────────────────

#[test]
fn installs_tekla_from_local_path() {
    let tmp = tempfile::tempdir().unwrap();
    let src = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("20-agents/aeco/engineering/tekla");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["agent", "install"])
        .arg(&src)
        .assert()
        .success()
        .stdout(predicate::str::contains("tekla"));
    assert!(tmp.path().join("agents/tekla/manifest.yaml").is_file());
    assert!(
        tmp.path()
            .join("agents/tekla/skills/drawing-identity.md")
            .is_file()
    );
}

#[test]
fn rejects_install_of_already_installed_agent() {
    let tmp = tempfile::tempdir().unwrap();
    let src = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("20-agents/aeco/engineering/tekla");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["agent", "install"])
        .arg(&src)
        .assert()
        .success();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["agent", "install"])
        .arg(&src)
        .assert()
        .failure()
        .code(8); // Conflict
}

// ── registry tests ─────────────────────────────────────────────────────────

#[test]
fn installs_tekla_from_registry() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let tarball = tmp.path().join("tekla.tar.gz");
    build_tekla_tarball(&tarball);
    let idx_path = write_registry_fixture(tmp.path(), &tarball);

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_REGISTRY", to_file_url(&idx_path))
        .args(["agent", "install", "tekla"])
        .assert()
        .success()
        .stdout(predicate::str::contains("installed tekla"));

    assert!(aware.join("agents/tekla/manifest.yaml").is_file());
    assert!(
        aware
            .join("agents/tekla/skills/drawing-identity.md")
            .is_file()
    );
}

#[test]
fn installs_one_agent_bundle() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let tarball = tmp.path().join("tekla.tar.gz");
    build_tekla_tarball(&tarball);
    let idx_path = write_registry_fixture(tmp.path(), &tarball);

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_REGISTRY", to_file_url(&idx_path))
        .args(["agent", "install", "aware-tiny"])
        .assert()
        .success()
        .stdout(predicate::str::contains("bundle aware-tiny: 1 installed"));

    assert!(aware.join("agents/tekla/manifest.yaml").is_file());
}

#[test]
fn install_auto_regenerates_claude_plugin_when_target_exists() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let claude_dir = tmp.path().join("claude_plugins");
    std::fs::create_dir_all(&claude_dir).unwrap();

    let tekla_src = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("20-agents/aeco/engineering/tekla");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_PLUGINS_CLAUDE", &claude_dir)
        .args(["agent", "install"])
        .arg(&tekla_src)
        .assert()
        .success();

    // Plugin should have been auto-regenerated after install
    assert!(claude_dir.join("aware-aeco/plugin.json").is_file());
}
