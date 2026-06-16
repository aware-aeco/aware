//! `aware agent catalog --json` must surface each agent's registry `vendor` and
//! `keywords` so a UI (e.g. floless.app's "Available agents" tab) can group/facet
//! the catalog without re-deriving categories from the id. Drives the real binary
//! against a `file://` fixture catalog (AWARE_CATALOG override) in an isolated home.

use assert_cmd::Command;

/// A minimal two-agent catalog: one carries vendor+keywords, one carries neither
/// (so we also pin the "omit when absent/empty" behaviour).
const FIXTURE: &str = r#"{
  "version": "1",
  "updated-at": "2026-06-16T00:00:00Z",
  "agents": {
    "slack": {
      "display-name": "Slack",
      "vendor": "slack",
      "keywords": ["messaging", "chat", "productivity"],
      "versions": {
        "1.0.0": {
          "description": "Post messages and read channels.",
          "status": "available",
          "stateful": false,
          "transport": "rest",
          "command_count": 9,
          "skills": ["slack-auth"]
        }
      }
    },
    "bare": {
      "versions": {
        "0.1.0": {
          "description": "No vendor, no keywords.",
          "status": "available",
          "stateful": false,
          "transport": "cli",
          "command_count": 1,
          "skills": []
        }
      }
    }
  }
}"#;

#[test]
fn catalog_json_surfaces_vendor_and_keywords() {
    let tmp = tempfile::tempdir().unwrap();
    let cat = tmp.path().join("fixture-catalog.json");
    std::fs::write(&cat, FIXTURE).unwrap();

    let out = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .env("AWARE_CATALOG", format!("file://{}", cat.display()))
        .args(["agent", "catalog", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let v: serde_json::Value = serde_json::from_slice(&out).unwrap();
    let agents = v["data"]["agents"].as_array().expect("agents array");

    let slack = agents
        .iter()
        .find(|a| a["id"] == "slack")
        .expect("slack row");
    assert_eq!(slack["vendor"], "slack");
    assert_eq!(
        slack["keywords"]
            .as_array()
            .unwrap()
            .iter()
            .map(|k| k.as_str().unwrap())
            .collect::<Vec<_>>(),
        vec!["messaging", "chat", "productivity"],
    );

    // The agent with no vendor/keywords omits both keys entirely (minimal row).
    let bare = agents.iter().find(|a| a["id"] == "bare").expect("bare row");
    assert!(bare.get("vendor").is_none(), "vendor omitted when absent");
    assert!(
        bare.get("keywords").is_none(),
        "keywords omitted when empty"
    );
}
