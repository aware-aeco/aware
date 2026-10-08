//! One damaged installed agent (#660).
//!
//! An agent whose `manifest.yaml` does not parse used to make every verb that
//! walks `agents/` fail — `agent list`, `agent describe`, `search`, `tree`,
//! `app compile`, `app explain` — with exit 3 and nothing on stdout, and made
//! `doctor` report zero agents. Worse, `app install`/`app validate` swallowed
//! the walk's error and skipped their safety and command checks entirely.
//!
//! Two tiers now:
//! - listings list every readable agent and report each damaged one, exit 0;
//! - a caller that needs a SPECIFIC agent ignores damage elsewhere, and fails
//!   with the needed agent's own load error — inside the `--json` envelope
//!   where the verb has one — when that agent is the damaged one.

use std::path::{Path, PathBuf};

use assert_cmd::Command;
use serde_json::Value;

const BROKEN: &str = "agent: broken\nversion: [unclosed\n";

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn aware(home: &Path) -> Command {
    let mut cmd = Command::cargo_bin("aware").unwrap();
    cmd.env("AWARE_HOME", home);
    cmd
}

fn run(home: &Path, args: &[&str]) -> std::process::Output {
    aware(home).args(args).output().unwrap()
}

fn stderr(out: &std::process::Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

fn envelope(out: &std::process::Output) -> Value {
    let stdout = String::from_utf8_lossy(&out.stdout);
    serde_json::from_str(stdout.trim()).unwrap_or_else(|e| {
        panic!(
            "stdout is not one JSON envelope ({e}): {stdout:?}\nstderr: {}",
            stderr(out)
        )
    })
}

fn write_broken(home: &Path, dir: &str, body: &str) {
    let d = home.join("agents").join(dir);
    std::fs::create_dir_all(&d).unwrap();
    std::fs::write(d.join("manifest.yaml"), body).unwrap();
}

/// A home with the real `html-report` agent installed and one damaged agent
/// beside it.
fn home() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    let src = repo().join("20-agents/_core/html-report");
    let out = run(tmp.path(), &["agent", "install", src.to_str().unwrap()]);
    assert!(out.status.success(), "install: {}", stderr(&out));
    write_broken(tmp.path(), "broken", BROKEN);
    tmp
}

/// An app source directory `<root>/<id>/<id>.flo` with one node on `agent`.
fn write_app(root: &Path, id: &str, agent: &str, command: &str) -> PathBuf {
    let dir = root.join(id);
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(format!("{id}.flo"));
    std::fs::write(
        &path,
        format!(
            "app: {id}\nversion: 0.1.0\ndescription: fixture {id}\nrequires: []\n\
             layout: linear\nnodes:\n  - id: step\n    agent: {agent}\n    command: {command}\n\
             \x20   config:\n      title: Hi\n      data: [1]\nconnections: []\n"
        ),
    )
    .unwrap();
    path
}

// ── tier (a): listings ────────────────────────────────────────────────────────

#[test]
fn agent_list_json_lists_readable_agents_and_reports_the_damaged_one() {
    let home = home();
    let out = run(home.path(), &["agent", "list", "--json"]);
    assert_eq!(out.status.code(), Some(0), "stderr: {}", stderr(&out));
    let env = envelope(&out);
    assert_eq!(env["ok"], true);
    let ids: Vec<&str> = env["data"]["agents"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["id"].as_str().unwrap())
        .collect();
    assert_eq!(ids, ["html-report"]);
    let invalid = env["data"]["invalid"].as_array().unwrap();
    assert_eq!(invalid.len(), 1, "{invalid:?}");
    assert_eq!(invalid[0]["id"], "broken");
    assert_eq!(invalid[0]["code"], "E_AGENT_MANIFEST_INVALID");
    assert!(
        invalid[0]["path"]
            .as_str()
            .unwrap()
            .ends_with("manifest.yaml")
    );
    assert!(
        invalid[0]["message"]
            .as_str()
            .unwrap()
            .contains("invalid type"),
        "{}",
        invalid[0]
    );
}

#[test]
fn agent_list_json_carries_an_empty_invalid_array_when_every_agent_loads() {
    let home = home();
    std::fs::remove_dir_all(home.path().join("agents/broken")).unwrap();
    let out = run(home.path(), &["agent", "list", "--json"]);
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(envelope(&out)["data"]["invalid"], serde_json::json!([]));
}

#[test]
fn agent_list_human_names_the_damaged_agent() {
    let home = home();
    let out = run(home.path(), &["agent", "list"]);
    assert_eq!(out.status.code(), Some(0), "stderr: {}", stderr(&out));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("html-report"), "{stdout}");
    assert!(
        stdout
            .lines()
            .any(|l| l.starts_with("unreadable: broken") && l.contains("E_AGENT_MANIFEST_INVALID")),
        "{stdout}"
    );
}

#[test]
fn doctor_counts_the_readable_agents_and_names_the_damaged_one() {
    let home = home();
    let out = run(home.path(), &["doctor", "--json"]);
    assert!(out.status.success());
    let report: Value = serde_json::from_slice(&out.stdout).unwrap();
    let ids: Vec<&str> = report["agents"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["id"].as_str().unwrap())
        .collect();
    assert_eq!(ids, ["html-report"]);
    let invalid = report["invalid_agents"].as_array().unwrap();
    assert_eq!(invalid.len(), 1);
    assert_eq!(invalid[0]["id"], "broken");
    assert_eq!(invalid[0]["code"], "E_AGENT_MANIFEST_INVALID");
}

#[test]
fn search_searches_the_readable_agents_and_reports_the_damaged_one() {
    let home = home();
    let out = run(home.path(), &["search", "render", "--json"]);
    assert_eq!(out.status.code(), Some(0), "stderr: {}", stderr(&out));
    let env = envelope(&out);
    assert_eq!(env["data"]["searched_agents"], 1);
    assert_eq!(env["data"]["results"][0]["agent"], "html-report");
    let skipped = env["data"]["unreadable_agents"].as_array().unwrap();
    assert_eq!(skipped.len(), 1);
    assert_eq!(skipped[0]["id"], "broken");
}

// ── tier (b): callers that need a specific agent ──────────────────────────────

#[test]
fn describe_ignores_damage_in_another_agent() {
    let home = home();
    let out = run(home.path(), &["agent", "describe", "html-report", "--json"]);
    assert_eq!(out.status.code(), Some(0), "stderr: {}", stderr(&out));
    assert_eq!(envelope(&out)["ok"], true);
}

#[test]
fn describe_of_the_damaged_agent_returns_its_load_error_in_the_envelope() {
    let home = home();
    let out = run(home.path(), &["agent", "describe", "broken", "--json"]);
    assert_eq!(out.status.code(), Some(3), "stderr: {}", stderr(&out));
    let env = envelope(&out);
    assert_eq!(env["ok"], false);
    assert_eq!(env["error"]["code"], "E_AGENT_MANIFEST_INVALID");
    assert_eq!(env["meta"]["command"], "agent describe");
    assert!(
        env["error"]["message"]
            .as_str()
            .unwrap()
            .contains("manifest.yaml"),
        "{env}"
    );
}

/// The `agent:` field is what a caller names; a damaged agent whose directory
/// differs from its declared id is still recognised by that id.
#[test]
fn a_damaged_agent_is_recognised_by_its_declared_id_too() {
    let home = home();
    write_broken(
        home.path(),
        "moved-dir",
        "agent: declared\nversion: 1.0.0\ndescription: x\ncommands: 42\n",
    );
    let out = run(home.path(), &["agent", "describe", "declared", "--json"]);
    assert_eq!(out.status.code(), Some(3), "stderr: {}", stderr(&out));
    let env = envelope(&out);
    assert!(
        env["error"]["message"]
            .as_str()
            .unwrap()
            .contains("moved-dir"),
        "{env}"
    );
}

#[test]
fn tree_ignores_damage_elsewhere_and_reports_its_own_in_the_envelope() {
    let home = home();
    let ok = run(home.path(), &["tree", "html-report", "--json"]);
    assert_eq!(ok.status.code(), Some(0), "stderr: {}", stderr(&ok));
    let bad = run(home.path(), &["tree", "broken", "--json"]);
    assert_eq!(bad.status.code(), Some(3));
    assert_eq!(envelope(&bad)["error"]["code"], "E_AGENT_MANIFEST_INVALID");
}

#[test]
fn search_scoped_to_the_damaged_agent_fails_with_its_load_error() {
    let home = home();
    let out = run(home.path(), &["search", "x", "--agent", "broken"]);
    assert_eq!(out.status.code(), Some(3));
    assert!(stderr(&out).contains("manifest.yaml"), "{}", stderr(&out));
}

#[test]
fn compile_ignores_damage_in_an_agent_the_app_does_not_use() {
    let home = home();
    let src = write_app(home.path(), "rep", "html-report", "render");
    let out = run(home.path(), &["app", "compile", src.to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(0), "stderr: {}", stderr(&out));
    assert!(src.with_extension("lock").is_file(), "no lock written");
}

#[test]
fn compile_of_an_app_using_the_damaged_agent_reports_its_load_error() {
    let home = home();
    let src = write_app(home.path(), "uses-broken", "broken", "probe");
    let out = run(home.path(), &["app", "compile", src.to_str().unwrap()]);
    assert_eq!(out.status.code(), Some(3), "stderr: {}", stderr(&out));
    let err = stderr(&out);
    assert!(
        err.contains("broken") && err.contains("manifest.yaml"),
        "the error must name the damaged agent's manifest: {err}"
    );
}

#[test]
fn explain_reports_a_damaged_agent_it_uses_in_the_envelope() {
    let home = home();
    let src = write_app(&home.path().join("src"), "rep", "html-report", "render");
    let out = run(
        home.path(),
        &["app", "install", src.parent().unwrap().to_str().unwrap()],
    );
    assert!(out.status.success(), "install: {}", stderr(&out));
    let ok = run(home.path(), &["app", "explain", "rep", "--json"]);
    assert_eq!(ok.status.code(), Some(0), "stderr: {}", stderr(&ok));

    let bad = write_app(&home.path().join("src"), "uses-broken", "broken", "probe");
    std::fs::create_dir_all(home.path().join("apps/uses-broken")).unwrap();
    std::fs::copy(&bad, home.path().join("apps/uses-broken/uses-broken.flo")).unwrap();
    let out = run(home.path(), &["app", "explain", "uses-broken", "--json"]);
    assert_eq!(out.status.code(), Some(3), "stderr: {}", stderr(&out));
    let env = envelope(&out);
    assert_eq!(env["ok"], false);
    assert_eq!(env["error"]["code"], "E_AGENT_MANIFEST_INVALID");
}

/// The silent-waiver regression: `app validate` used to swallow the agent walk's
/// error and skip the safety contract whenever ANY agent was damaged, so a
/// write-mode node with no `safety:` block validated clean.
#[test]
fn validate_keeps_the_safety_check_when_an_unrelated_agent_is_damaged() {
    let home = home();
    let writer = home.path().join("agents/writer");
    std::fs::create_dir_all(&writer).unwrap();
    std::fs::write(
        writer.join("manifest.yaml"),
        "agent: writer\nversion: 1.0.0\ndescription: a write-mode fixture\nstateful: false\n\
         license: MIT\ntransport:\n  cli:\n    binary: aware-writer\n\
         commands:\n  push:\n    lifecycle: single\n    description: writes\n    mode: write\n",
    )
    .unwrap();
    let src = write_app(home.path(), "pusher", "writer", "push");
    let out = run(home.path(), &["app", "validate", src.to_str().unwrap()]);
    assert_eq!(
        out.status.code(),
        Some(3),
        "a write node without `safety:` must not validate; stdout: {} stderr: {}",
        String::from_utf8_lossy(&out.stdout),
        stderr(&out)
    );
}
