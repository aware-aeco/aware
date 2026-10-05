//! `aware agent refs` through the real binary (#629-a): the table of stored
//! versions and what needs each one, and the registered-roots verbs.

use assert_cmd::Command;
use std::path::Path;

fn aware(home: &Path) -> Command {
    let mut cmd = Command::cargo_bin("aware").unwrap();
    cmd.env("AWARE_HOME", home);
    cmd
}

fn json(home: &Path, args: &[&str]) -> serde_json::Value {
    let output = aware(home).arg("--json").args(args).output().unwrap();
    assert!(
        output.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let envelope: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&output.stdout).trim()).unwrap();
    assert_eq!(envelope["ok"], true, "{envelope}");
    envelope["data"].clone()
}

fn write_agent(home: &Path, version: &str) {
    let agent = home.join("agents").join("alpha");
    std::fs::create_dir_all(&agent).unwrap();
    std::fs::write(
        agent.join("manifest.yaml"),
        format!(
            "agent: alpha\nversion: {version}\ndescription: build {version}\nstateful: false\nlicense: MIT\n\
             transport:\n  cli:\n    binary: aware-test\n\
             commands:\n  go:\n    lifecycle: single\n    mode: read\n    description: x\n"
        ),
    )
    .unwrap();
}

fn write_app(dir: &Path, app: &str) -> std::path::PathBuf {
    std::fs::create_dir_all(dir).unwrap();
    let source = dir.join(format!("{app}.flo"));
    std::fs::write(
        &source,
        format!(
            "app: {app}\nversion: 0.1.0\ndescription: x\nnodes:\n  - id: n\n    agent: alpha\n    command: go\nconnections: []\nrequires: []\n"
        ),
    )
    .unwrap();
    source
}

fn compile(home: &Path, source: &Path) {
    aware(home)
        .args(["app", "compile"])
        .arg(source)
        .assert()
        .success();
}

fn package<'v>(table: &'v serde_json::Value, version: &str) -> &'v serde_json::Value {
    table["packages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["version"] == version)
        .unwrap_or_else(|| panic!("{version} not in {table}"))
}

#[test]
fn refs_reports_what_keeps_each_stored_version_and_registered_folders_count() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("aware");
    write_agent(&home, "1.0.0");
    compile(&home, &write_app(&home.join("apps").join("a"), "a"));
    // A workflow kept outside AWARE's apps/, approved on 1.0.0 too.
    let workspace = tmp.path().join("workspace");
    compile(&home, &write_app(&workspace.join("flows").join("w"), "w"));
    write_agent(&home, "1.0.1");
    compile(&home, &write_app(&home.join("apps").join("b"), "b"));

    let table = json(&home, &["agent", "refs", "--recovery-window", "0s"]);
    assert_eq!(table["format"], "aware.agent-refs/v1");
    assert_eq!(table["complete"], true, "{table}");
    assert_eq!(table["recovery-window"], "0s");
    let old = package(&table, "1.0.0");
    assert_eq!(old["state"], "kept", "{old}");
    assert_eq!(old["references"][0]["kind"], "approved-lock");
    let new = package(&table, "1.0.1");
    assert_eq!(new["state"], "kept");
    let kinds: Vec<&str> = new["references"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["kind"].as_str().unwrap())
        .collect();
    assert_eq!(kinds, ["current", "approved-lock"]);

    // Remove app a: 1.0.0 is only pinned by the workspace lock AWARE cannot see.
    std::fs::remove_dir_all(home.join("apps").join("a")).unwrap();
    let table = json(&home, &["agent", "refs", "--recovery-window", "0s"]);
    assert_eq!(package(&table, "1.0.0")["state"], "removable");

    // Registering the workspace makes its lock count.
    let added = json(
        &home,
        &[
            "agent",
            "refs",
            "roots",
            "add",
            workspace.to_str().unwrap(),
            "--label",
            "floless",
        ],
    );
    assert_eq!(added["added"], true);
    let listed = json(&home, &["agent", "refs", "roots", "list"]);
    assert_eq!(listed["roots"][0]["label"], "floless");
    let table = json(&home, &["agent", "refs", "--recovery-window", "0s"]);
    assert_eq!(package(&table, "1.0.0")["state"], "kept");
    assert_eq!(table["roots"][1]["kind"], "registered");
    assert_eq!(table["roots"][1]["locks"], 1);

    // A registered folder that has gone makes the table incomplete — and the
    // command still succeeds: it is a report, not a refusal.
    std::fs::remove_dir_all(&workspace).unwrap();
    let table = json(&home, &["agent", "refs"]);
    assert_eq!(table["complete"], false);
    assert_eq!(table["roots"][1]["status"], "missing");
    let removed = json(
        &home,
        &[
            "agent",
            "refs",
            "roots",
            "remove",
            workspace.to_str().unwrap(),
        ],
    );
    assert_eq!(removed["removed"], true);
    assert_eq!(json(&home, &["agent", "refs"])["complete"], true);

    // Human output names the reasons.
    let text = aware(&home).args(["agent", "refs"]).output().unwrap();
    let text = String::from_utf8_lossy(&text.stdout);
    assert!(text.contains("alpha 1.0.1"), "{text}");
    assert!(
        text.contains("current copy, approved lock")
            || text.contains("approved lock, current copy"),
        "{text}"
    );
}

#[test]
fn a_malformed_window_is_an_error() {
    let tmp = tempfile::tempdir().unwrap();
    let output = aware(tmp.path())
        .args(["--json", "agent", "refs", "--recovery-window", "30x"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(said.contains("E_AGENT_REFS_WINDOW_INVALID"), "{said}");
}

/// #629-b review round 3: `--only` names its agent; combined with `--agent`
/// it could only conflict, so the two are refused together as a usage error.
#[test]
fn gc_refuses_agent_together_with_only() {
    let tmp = tempfile::tempdir().unwrap();
    let only = format!("tool@sha256:{}", "a".repeat(64));
    let output = aware(tmp.path())
        .args(["agent", "gc", "--agent", "other", "--only", &only])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let said = String::from_utf8_lossy(&output.stderr);
    assert!(said.contains("cannot be used with"), "{said}");
}
