mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn regenerate_creates_claude_plugin_when_target_dir_exists() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");
    // Install the tekla agent so there's something to regenerate from.
    let tekla_src = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("20-agents/aeco/engineering/tekla");
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["agent", "install"])
        .arg(&tekla_src)
        .assert()
        .success();

    let claude_dir = tmp.path().join("claude_plugins");
    std::fs::create_dir_all(&claude_dir).unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .env("AWARE_PLUGINS_CLAUDE", &claude_dir)
        .args(["plugins", "regenerate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("claude-code"));

    assert!(claude_dir.join("aware-aeco/plugin.json").is_file());
}

#[test]
fn regenerate_with_no_agents_succeeds() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["plugins", "regenerate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Regenerating"));
}
