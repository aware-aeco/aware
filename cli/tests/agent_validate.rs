mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn valid_agent_exits_0() {
    let src = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("20-agents/aeco/engineering/tekla");
    Command::cargo_bin("aware")
        .unwrap()
        .args(["agent", "validate"])
        .arg(&src)
        .assert()
        .success()
        .stdout(predicate::str::contains("is valid"));
}

#[test]
fn missing_manifest_exits_nonzero() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .args(["agent", "validate"])
        .arg(tmp.path())
        .assert()
        .failure();
}

#[test]
fn bad_manifest_exits_3() {
    let tmp = tempfile::tempdir().unwrap();
    // Create a manifest missing required `transport` field — should fail serde load.
    let bad =
        "agent: x\nversion: 1.0\ndescription: y\nstateful: false\nlicense: MIT\ncommands: {}\n";
    std::fs::write(tmp.path().join("manifest.yaml"), bad).unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .args(["agent", "validate"])
        .arg(tmp.path())
        .assert()
        .failure()
        .code(3);
}

#[test]
fn missing_skill_file_exits_3_with_error_code() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp.path().join("skills")).unwrap();
    let manifest = r#"agent: x
version: 1.0
description: y
stateful: false
license: MIT
transport: { cli: { binary: x } }
commands: { do: { lifecycle: single, description: z } }
skills:
  - missing.md
"#;
    std::fs::write(tmp.path().join("manifest.yaml"), manifest).unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .args(["agent", "validate"])
        .arg(tmp.path())
        .assert()
        .failure()
        .code(3)
        .stdout(predicate::str::contains("E_SKILL_FILE_MISSING"));
}

/// A manifest declaring exactly `transport_block`, in a fresh temp dir. Raw literals so
/// the YAML in a test reads as YAML.
fn agent_dir(id: &str, transport_block: &str) -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    let manifest = format!(
        r#"agent: {id}
version: 1.0.0
description: y
stateful: false
license: MIT
transport:
{transport_block}commands:
  do:
    lifecycle: single
    description: z
"#
    );
    std::fs::write(tmp.path().join("manifest.yaml"), manifest).unwrap();
    tmp
}

#[test]
fn mcp_only_agent_is_refused_by_the_real_binary_at_validate_and_at_install() {
    // #366. The unit test asserts `validate_agent` returns the issue; this asserts that
    // the two things a user actually does are both refused — the agent-spec now promises
    // rejection "at validate/install" and only the validate half was pinned.
    let src = agent_dir("mcponly", "  mcp:\n    server: s\n");

    Command::cargo_bin("aware")
        .unwrap()
        .args(["agent", "validate"])
        .arg(src.path())
        .assert()
        .failure()
        .code(3)
        .stdout(predicate::str::contains("E_AGENT_MCP_ONLY"))
        // The message must say WHY: an author who sees only a code cannot act on it.
        .stdout(predicate::str::contains(
            "not a dispatchable transport on its own",
        ));

    // Install is the destructive half — it must refuse BEFORE writing the agent dir,
    // rather than installing something that dies at the first invoke.
    let home = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .args(["agent", "install"])
        .arg(src.path())
        .assert()
        .failure()
        // `install` reports on stderr where `validate` reports on stdout.
        .stderr(predicate::str::contains("E_AGENT_MCP_ONLY"));
    assert!(
        !home.path().join("agents/mcponly").exists(),
        "a refused install must leave nothing behind"
    );

    // The control: the same agent WITH a runnable transport beside `mcp` — the shape the
    // agent-spec's own example shows — still installs. Without this, the assertions above
    // would pass just as well on a build that had broken `mcp` manifests outright.
    let ok = agent_dir("climcp", "  cli:\n    binary: b\n  mcp:\n    server: s\n");
    Command::cargo_bin("aware")
        .unwrap()
        .args(["agent", "validate"])
        .arg(ok.path())
        .assert()
        .success();
    let home = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .args(["agent", "install"])
        .arg(ok.path())
        .assert()
        .success();
    assert!(home.path().join("agents/climcp/manifest.yaml").is_file());
}
