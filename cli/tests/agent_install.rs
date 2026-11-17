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

/// The `version:` of the Tekla manifest the fixture tarball packs. The registry entry that
/// should match the packed payload takes its `manifest-version` from this, so bumping the real
/// agent never needs this test edited.
fn packed_tekla_version() -> String {
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("20-agents/aeco/engineering/tekla/manifest.yaml");
    let agent: serde_yaml::Value =
        serde_yaml::from_str(&std::fs::read_to_string(manifest).unwrap()).unwrap();
    agent["version"].as_str().unwrap().to_string()
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
    let packed_version = packed_tekla_version();
    let body = format!(
        r#"{{
    "version": "1.0",
    "updated-at": "2026-05-16T00:00:00Z",
    "agents": {{
        "tekla": {{
            "versions": {{
                "2025.0.1": {{ "tarball": "{tarball_url}", "subdir": "aware-main/20-agents/tekla", "manifest-agent": "tekla", "manifest-version": "{packed_version}" }}
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
fn bundle_install_refuses_a_manifest_binding_mismatch_without_partial_install() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let tarball = tmp.path().join("tekla.tar.gz");
    build_tekla_tarball(&tarball);
    let idx_path = write_registry_fixture(tmp.path(), &tarball);
    let mut index: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&idx_path).unwrap()).unwrap();
    index["agents"]["tekla"]["versions"]["2025.0.1"]["manifest-version"] = "9.9.9".into();
    std::fs::write(&idx_path, serde_json::to_string_pretty(&index).unwrap()).unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_REGISTRY", to_file_url(&idx_path))
        .args(["agent", "install", "aware-tiny"])
        .assert()
        .success()
        .stdout(predicate::str::contains("0 installed, 1 failed"))
        .stdout(predicate::str::contains("expects manifest-version 9.9.9"));
    assert!(!aware.join("agents/tekla").exists());
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

// ── #845: a version-less install takes the newest release this CLI runs ────

/// A registry of `floor-probe` releases, each `(version, cli floor)`, all served from one
/// archive. `index_floors` says whether the index records each floor (the official registry
/// does) or leaves it to the payload manifest (a legacy or custom registry). Returns the index.
fn write_floor_registry(
    dir: &std::path::Path,
    releases: &[(&str, &str)],
    index_floors: bool,
) -> std::path::PathBuf {
    use std::io::Write;

    let archive = dir.join("floor-probe.tar.gz");
    let enc = flate2::write::GzEncoder::new(
        std::fs::File::create(&archive).unwrap(),
        flate2::Compression::default(),
    );
    let mut tar = tar::Builder::new(enc);
    let mut versions = serde_json::Map::new();
    for (version, floor) in releases {
        let subdir = format!("aware-main/20-agents/floor-probe-{version}");
        let manifest = format!(
            "agent: floor-probe\nversion: {version}\ndescription: Runtime floor fixture.\n\
             stateful: false\nlicense: MIT\nstatus: requires-runtime\nminimum-cli-version: {floor}\n\
             transport:\n  cli:\n    binary: floor-probe\n\
             commands:\n  ping:\n    lifecycle: single\n    description: Ping.\n"
        );
        let mut h = tar::Header::new_gnu();
        h.set_size(manifest.len() as u64);
        h.set_mode(0o644);
        h.set_cksum();
        tar.append_data(
            &mut h,
            format!("{subdir}/manifest.yaml"),
            manifest.as_bytes(),
        )
        .unwrap();
        let mut entry = serde_json::json!({
            "tarball": to_file_url(&archive),
            "subdir": subdir,
            "manifest-agent": "floor-probe",
            "manifest-version": version,
        });
        if index_floors {
            entry["minimum-cli-version"] = (*floor).into();
        }
        versions.insert((*version).to_string(), entry);
    }
    let mut file = tar.into_inner().unwrap().finish().unwrap();
    file.flush().unwrap();

    let index = dir.join("registry-index.json");
    std::fs::write(
        &index,
        serde_json::to_string_pretty(&serde_json::json!({
            "version": "1.0",
            "updated-at": "2026-11-17T00:00:00Z",
            "agents": { "floor-probe": { "versions": versions } },
            "bundles": {}
        }))
        .unwrap(),
    )
    .unwrap();
    index
}

#[test]
fn a_bare_install_installs_the_newest_release_this_cli_runs_and_says_what_it_held_back() {
    for index_floors in [true, false] {
        let tmp = tempfile::tempdir().unwrap();
        let aware = tmp.path().join("aware");
        let index = write_floor_registry(
            tmp.path(),
            &[("0.1.0", "0.1.0"), ("0.2.0", "999.0.0")],
            index_floors,
        );

        Command::cargo_bin("aware")
            .unwrap()
            .env("AWARE_HOME", &aware)
            .env("AWARE_REGISTRY", to_file_url(&index))
            .args(["agent", "install", "floor-probe"])
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "✓ installed floor-probe 0.1.0; floor-probe 0.2.0 needs AWARE 999.0.0 or newer",
            ));

        let installed =
            std::fs::read_to_string(aware.join("agents/floor-probe/manifest.yaml")).unwrap();
        assert!(installed.contains("version: 0.1.0"), "{installed}");
    }
}

#[test]
fn a_bare_install_with_no_release_this_cli_runs_still_refuses() {
    for index_floors in [true, false] {
        let tmp = tempfile::tempdir().unwrap();
        let aware = tmp.path().join("aware");
        let index = write_floor_registry(tmp.path(), &[("0.2.0", "999.0.0")], index_floors);

        Command::cargo_bin("aware")
            .unwrap()
            .env("AWARE_HOME", &aware)
            .env("AWARE_REGISTRY", to_file_url(&index))
            .args(["agent", "install", "floor-probe"])
            .assert()
            .failure()
            .code(3)
            .stderr(predicate::str::contains("E_AGENT_RUNTIME_TOO_OLD"))
            .stderr(predicate::str::contains("999.0.0"));
        assert!(!aware.join("agents/floor-probe").exists());
    }
}

#[test]
fn a_pinned_install_keeps_todays_refusal_and_output() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let index = write_floor_registry(
        tmp.path(),
        &[("0.1.0", "0.1.0"), ("0.2.0", "999.0.0")],
        true,
    );

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_REGISTRY", to_file_url(&index))
        .args(["agent", "install", "floor-probe@0.2.0"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("E_AGENT_RUNTIME_TOO_OLD"));
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_REGISTRY", to_file_url(&index))
        .args(["agent", "install", "floor-probe@0.1.0"])
        .assert()
        .success()
        .stdout(predicate::str::contains("✓ installed floor-probe\n"))
        .stdout(predicate::str::contains("needs AWARE").not());
}

#[test]
fn a_bare_update_moves_to_the_newest_release_this_cli_runs_and_says_what_it_held_back() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    let index = write_floor_registry(
        tmp.path(),
        &[("0.1.0", "0.1.0"), ("0.1.5", "0.1.0"), ("0.2.0", "999.0.0")],
        true,
    );
    let aware_cmd = || {
        let mut cmd = Command::cargo_bin("aware").unwrap();
        cmd.env("AWARE_HOME", &aware)
            .env("AWARE_REGISTRY", to_file_url(&index));
        cmd
    };
    aware_cmd()
        .args(["agent", "install", "floor-probe@0.1.0"])
        .assert()
        .success();

    aware_cmd()
        .args(["agent", "update", "floor-probe"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "✓ updated floor-probe to 0.1.5; floor-probe 0.2.0 needs AWARE 999.0.0 or newer",
        ));
    aware_cmd()
        .args(["agent", "update", "--all"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "✓ floor-probe 0.1.5; floor-probe 0.2.0 needs AWARE 999.0.0 or newer",
        ));
}
