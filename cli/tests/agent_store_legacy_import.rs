//! The legacy store import through the real binary (#627-b): on a home
//! upgraded from AWARE 0.149-0.151 — stored versions only in `agent-store/` —
//! the very first `aware agent list --json` already reports them (review
//! round 3), and the legacy store is left byte-identical.

use assert_cmd::Command;
use std::collections::BTreeMap;
use std::path::Path;

fn tree(dir: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut out = BTreeMap::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for entry in std::fs::read_dir(&d).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                out.insert(
                    path.strip_prefix(dir).unwrap().display().to_string(),
                    std::fs::read(&path).unwrap(),
                );
            }
        }
    }
    out
}

#[test]
fn the_first_agent_list_after_an_upgrade_reports_the_older_clis_stored_versions() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("aware");
    let agent = home.join("agents").join("alpha");
    std::fs::create_dir_all(&agent).unwrap();
    std::fs::write(
        agent.join("manifest.yaml"),
        "agent: alpha\nversion: 1.0.0\ndescription: x\nstateful: false\nlicense: MIT\n\
         transport:\n  cli:\n    binary: aware-test\n\
         commands:\n  go:\n    lifecycle: single\n    mode: read\n    description: x\n",
    )
    .unwrap();
    let app = home.join("apps").join("a");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("a.flo"),
        "app: a\nversion: 0.1.0\ndescription: x\nnodes:\n  - id: n\n    agent: alpha\n    command: go\nconnections: []\nrequires: []\n",
    )
    .unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &home)
        .args(["app", "compile"])
        .arg(app.join("a.flo"))
        .assert()
        .success();
    // Make it look like an older CLI's home: its store is `agent-store/`.
    std::fs::rename(home.join("agent-store-v2"), home.join("agent-store")).unwrap();
    let _ = std::fs::remove_file(home.join("agent-store-control/legacy-import.json"));
    let legacy_before = tree(&home.join("agent-store"));

    let output = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &home)
        .args(["--json", "agent", "list"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let envelope: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&output.stdout).trim()).unwrap();
    let row = envelope["data"]["agents"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["id"] == "alpha")
        .cloned()
        .unwrap();
    assert_eq!(row["stored"][0]["version"], "1.0.0", "{row}");
    assert_eq!(tree(&home.join("agent-store")), legacy_before);
}
