use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde_yaml::Value;

const AGENT_RELATIVE: &str = "20-agents/aeco/cross-cutting/google-workspace";
const PLANNED_COMMAND_COUNT: usize = 23;

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("cli crate must live directly below the repository root")
        .to_path_buf()
}

fn agent_path(relative: &str) -> PathBuf {
    repository_root().join(AGENT_RELATIVE).join(relative)
}

fn load_manifest() -> Value {
    let path = agent_path("manifest.yaml");
    serde_yaml::from_str(
        &fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display())),
    )
    .unwrap_or_else(|error| panic!("parse {}: {error}", path.display()))
}

#[test]
fn first_party_bundle_installs_the_corrected_immutable_release() {
    let path = repository_root().join("registry-index.json");
    let index: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display())),
    )
    .unwrap_or_else(|error| panic!("parse {}: {error}", path.display()));
    let agents = index["bundles"]["aware-aeco"]["agents"]
        .as_array()
        .expect("aware-aeco bundle must list agents");

    assert!(agents.iter().any(|entry| entry == "google-workspace@1.1.1"));
    assert!(!agents.iter().any(|entry| entry == "google-workspace@1.0.0"));
}

fn mapping_keys(value: &Value) -> BTreeSet<String> {
    value
        .as_mapping()
        .expect("expected YAML mapping")
        .keys()
        .map(|key| key.as_str().expect("string mapping key").to_string())
        .collect()
}

#[test]
fn manifest_is_rest_only_runtime_gated_and_least_privilege() {
    let manifest = load_manifest();

    assert_eq!(manifest["version"].as_str(), Some("0.3.0"));
    assert_eq!(manifest["status"].as_str(), Some("requires-runtime"));
    assert_eq!(manifest["minimum-cli-version"].as_str(), Some("0.136.0"));

    assert_eq!(
        mapping_keys(&manifest["transport"]),
        BTreeSet::from(["rest".to_string()]),
        "the removed aware-google executable must not remain preferred over REST"
    );
    assert_eq!(
        manifest["transport"]["rest"]["base"].as_str(),
        Some("https://gmail.googleapis.com/gmail/v1/")
    );
    assert_eq!(manifest["auth"]["scheme"].as_str(), Some("oauth2"));
    assert_eq!(
        manifest["auth"]["secret"].as_str(),
        Some("google-workspace")
    );

    let declared_hosts = manifest["requires"]["network"]
        .as_sequence()
        .expect("requires.network sequence")
        .iter()
        .map(|host| host.as_str().expect("network host").to_string())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        declared_hosts,
        BTreeSet::from([
            "https://gmail.googleapis.com".to_string(),
            "https://oauth2.googleapis.com".to_string(),
            "https://openidconnect.googleapis.com".to_string(),
        ])
    );
}

#[test]
fn gmail_send_is_the_only_available_command_and_freezes_the_v1_shape() {
    let manifest = load_manifest();
    let commands = manifest["commands"].as_mapping().expect("commands mapping");
    assert_eq!(commands.len(), PLANNED_COMMAND_COUNT + 1);

    let mut planned = Vec::new();
    for (name, command) in commands {
        let name = name.as_str().expect("command name");
        if name == "gmail.send" {
            assert!(
                command.get("status").is_none(),
                "gmail.send inherits the available command default"
            );
        } else {
            assert_eq!(
                command["status"].as_str(),
                Some("planned"),
                "{name} must not be advertised as runnable"
            );
            planned.push(name);
        }
    }
    assert_eq!(planned.len(), PLANNED_COMMAND_COUNT);

    let send = &manifest["commands"]["gmail.send"];
    assert_eq!(send["mode"].as_str(), Some("write"));
    assert_eq!(send["rollback"].as_str(), Some("unsupported"));
    assert_eq!(send["method"].as_str(), Some("POST"));
    assert_eq!(send["path"].as_str(), Some("users/me/messages/send"));

    let expected_inputs = BTreeSet::from([
        "attempt-id".to_string(),
        "bcc".to_string(),
        "body".to_string(),
        "cc".to_string(),
        "content-type".to_string(),
        "subject".to_string(),
        "to".to_string(),
    ]);
    assert_eq!(mapping_keys(&send["inputs"]), expected_inputs);
    assert!(send["inputs"].get("attachments").is_none());
    assert!(send["inputs"].get("from").is_none());
    for required in ["to", "subject", "body", "attempt-id"] {
        assert_eq!(
            send["inputs"][required]["required"].as_bool(),
            Some(true),
            "{required} must be explicitly required"
        );
    }

    let expected_outputs = BTreeSet::from([
        "attempt-id".to_string(),
        "gmail-message-id".to_string(),
        "message-id".to_string(),
        "rfc-message-id".to_string(),
        "status".to_string(),
        "thread-id".to_string(),
    ]);
    assert_eq!(mapping_keys(&send["outputs"]["schema"]), expected_outputs);
    assert_eq!(
        send["outputs"]["schema"]["status"]["values"][0].as_str(),
        Some("accepted")
    );
}

#[test]
fn floless_1090_fixture_freezes_the_complete_request_and_identifier_semantics() {
    let fixture_path = agent_path("fixtures/floless-1090-gmail-send-contract.json");
    let fixture: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&fixture_path)
            .unwrap_or_else(|error| panic!("read {}: {error}", fixture_path.display())),
    )
    .unwrap_or_else(|error| panic!("parse {}: {error}", fixture_path.display()));

    assert_eq!(fixture["source"]["downstream"], "FloLess#1090");
    assert_eq!(fixture["source"]["adoption-required"], true);
    assert_eq!(fixture["request"]["agent"], "google-workspace");
    assert_eq!(fixture["request"]["command"], "gmail.send");

    let inputs = &fixture["request"]["inputs"];
    assert_eq!(inputs["attempt-id"], "floless-rfi-001-email-v1");
    assert_eq!(
        inputs
            .as_object()
            .expect("fixture inputs")
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "attempt-id".to_string(),
            "bcc".to_string(),
            "body".to_string(),
            "cc".to_string(),
            "content-type".to_string(),
            "subject".to_string(),
            "to".to_string(),
        ])
    );

    let accepted = &fixture["accepted-response"];
    assert_eq!(accepted["status"], "accepted");
    assert_eq!(accepted["attempt-id"], inputs["attempt-id"]);
    assert_eq!(accepted["message-id"], accepted["gmail-message-id"]);
    assert_ne!(accepted["message-id"], accepted["rfc-message-id"]);
    assert_eq!(fixture["semantics"]["rollback"], "unsupported");
    assert_eq!(fixture["semantics"]["automatic-retry"], false);
}

#[test]
fn docs_and_canary_preserve_acceptance_retry_and_live_verification_boundaries() {
    let command_doc = fs::read_to_string(agent_path("commands/gmail.send.md"))
        .expect("read Gmail send command documentation")
        .to_ascii_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    for required in [
        "rollback: unsupported",
        "does not prove delivery",
        "attachments are deliberately unavailable",
        "callers cannot provide or override a `from` address",
        "message-id` is retained for floless #1090",
        "gmail.send.outcome-unknown",
        "retryable: false",
        "never automatically retries",
        "cached accepted output",
    ] {
        assert!(
            command_doc.contains(required),
            "missing command contract: {required}"
        );
    }

    let auth_doc = fs::read_to_string(agent_path("skills/auth.md"))
        .expect("read Google auth documentation")
        .to_ascii_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    for required in [
        "gmail.send` is a google sensitive scope",
        "gmail.readonly` is restricted",
        "aware connect google-workspace --oauth",
        "fails closed",
        "aware disconnect google-workspace",
        "openidconnect.googleapis.com",
        "oauth2.googleapis.com",
        "gmail.googleapis.com",
    ] {
        assert!(
            auth_doc.contains(required),
            "missing auth contract: {required}"
        );
    }

    let canary: Value = serde_yaml::from_str(
        &fs::read_to_string(agent_path("fixtures/issue-495-gmail-send-canary.app"))
            .expect("read Gmail canary fixture"),
    )
    .expect("parse Gmail canary fixture");
    let config = &canary["nodes"][0]["config"];
    assert!(
        config["attempt-id"]
            .as_str()
            .is_some_and(|id| !id.is_empty())
    );
    assert!(
        config["bcc"]
            .as_sequence()
            .is_some_and(|bcc| !bcc.is_empty())
    );
    assert!(config.get("attachments").is_none());

    let qa = fs::read_to_string(repository_root().join("docs/qa/google-workspace-gmail-send.md"))
        .expect("read live Gmail QA runbook")
        .to_ascii_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    for required in [
        "exactly one matching message",
        "sent mailbox",
        "recipient mailbox",
        "do not contain a `bcc` header",
        "decoded headers and body",
        "profile request, process exit code, or aware node output alone",
        "refresh token",
        "do not mark issue #495 `qa-ready`",
    ] {
        assert!(
            qa.contains(required),
            "missing live-canary check: {required}"
        );
    }
}
