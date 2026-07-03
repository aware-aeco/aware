mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn lists_all_installed_fixture_agents() {
    let home = common::aware_home();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["agent", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ID"))
        .stdout(predicate::str::contains("VERSION"))
        .stdout(predicate::str::contains("KIND"))
        .stdout(predicate::str::contains("SKILLS"))
        .stdout(predicate::str::contains("COMMANDS"))
        .stdout(predicate::str::contains("tekla"))
        .stdout(predicate::str::contains("trimble-connect"))
        .stdout(predicate::str::contains("microsoft-365"))
        .stdout(predicate::str::contains("google-workspace"))
        .stdout(predicate::str::contains("html-report"))
        .stdout(predicate::str::contains("aware-agent-builder"))
        .stdout(predicate::str::contains("aware-skill-builder"));
}

#[test]
fn empty_home_lists_nothing_but_header() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["agent", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("ID"));
}

#[test]
fn json_output_returns_envelope() {
    let home = common::aware_home();
    let output = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["--json", "agent", "list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let body = std::str::from_utf8(&output).unwrap();
    let v: serde_json::Value = serde_json::from_str(body).unwrap();
    assert_eq!(v["ok"], true);
    assert_eq!(v["meta"]["command"], "agent list");
    let agents = v["data"]["agents"].as_array().unwrap();
    // The fixture mirrors every `manifest.yaml` under `20-agents/` (keyed by unique `agent:` id).
    // Count grew 57 (v0.28) → 63 (v0.29 vendor coverage) → 64 (the generic `http` agent, #101)
    //              → 65 (bcf-file + ifc-inspector curated to v0.2.0, PR #93)
    //              → 67 (tekla plugin-SDK agents, PR #149)
    //              → 66 (idea-statica-25.0/26.0 retired; registry 1:1 with tree, #187)
    //              → 67 (the declarative-UI `ui` builtin, #215).
    //              → 68 (the `vision.extract` curated runtime-extraction agent, RFC #223).
    //              → 69 (the `viewer-3d` builtin scene renderer — generic interactive 3D HTML).
    //              → 70 (the `ifc` builtin scene-to-IFC4 writer — host-free SPF export, sibling to viewer-3d).
    //              → 73 (the three `steel-detailer-{us,uk,eu}` curated knowledge agents, PR #232;
    //                    the US agent landed as `-aisc` and was renamed `-us` for region-consistency).
    //              → 74 (the generic `file` filesystem agent — watch/read/write/write-csv, #240).
    //              → 75 (the generic `shell` agent — open a path with the OS default handler, #241).
    //              → 76 (the `connection-reader` cli agent — extract steel connections from IFC as mesh).
    // A strict equality keeps this honest — adjust it whenever an agent lands or retires.
    assert_eq!(agents.len(), 76);
    assert!(agents.iter().any(|a| a["id"] == "tekla"));
    assert!(agents.iter().any(|a| a["id"] == "vision"));
    assert!(agents.iter().any(|a| a["id"] == "viewer-3d"));
    assert!(agents.iter().any(|a| a["id"] == "ifc"));
    assert!(agents.iter().any(|a| a["id"] == "steel-detailer-us"));
    assert!(agents.iter().any(|a| a["id"] == "connection-reader"));
}
