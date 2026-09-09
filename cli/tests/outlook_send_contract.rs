use std::fs;
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use serde_yaml::Value;

const SEND_COMMANDS: [&str; 2] = ["outlook.mail.send", "outlook.mail.send-with-attachment"];

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("cli crate must live directly below the repository root")
        .to_path_buf()
}

#[test]
fn describe_json_exposes_the_planned_agent_and_command_statuses() {
    let home = tempfile::tempdir().expect("temporary AWARE_HOME");
    let installed = home.path().join("agents/microsoft-365");
    fs::create_dir_all(&installed).expect("installed agent directory");
    fs::copy(
        repository_root().join("20-agents/aeco/cross-cutting/microsoft-365/manifest.yaml"),
        installed.join("manifest.yaml"),
    )
    .expect("install manifest fixture");

    let output = Command::cargo_bin("aware")
        .expect("aware test binary")
        .env("AWARE_HOME", home.path())
        .args(["agent", "describe", "microsoft-365", "--json"])
        .output()
        .expect("run agent describe");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let response: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("describe JSON envelope");
    assert_eq!(response["data"]["status"], "planned");
    for command_name in SEND_COMMANDS {
        let command = response["data"]["commands"]
            .as_array()
            .expect("commands array")
            .iter()
            .find(|command| command["name"] == command_name)
            .unwrap_or_else(|| panic!("missing {command_name}"));
        assert_eq!(command["status"], "planned", "{command_name}");
    }
}

#[test]
fn outlook_send_commands_are_planned_without_a_fictional_message_id() {
    let manifest_path =
        repository_root().join("20-agents/aeco/cross-cutting/microsoft-365/manifest.yaml");
    let manifest: Value = serde_yaml::from_str(
        &fs::read_to_string(&manifest_path).expect("read Microsoft 365 manifest"),
    )
    .expect("parse Microsoft 365 manifest");

    assert_eq!(
        manifest["status"].as_str(),
        Some("planned"),
        "the agent must not advertise a missing aware-m365 executable as runnable"
    );

    for command_name in SEND_COMMANDS {
        let command = &manifest["commands"][command_name];
        assert_eq!(
            command["status"].as_str(),
            Some("planned"),
            "{command_name} must stay unavailable until a real transport exists"
        );
        assert!(
            command.get("outputs").is_none(),
            "{command_name} must not promise output Graph cannot return"
        );
    }
}

#[test]
fn outlook_send_docs_preserve_acceptance_and_retry_boundaries() {
    let commands_dir =
        repository_root().join("20-agents/aeco/cross-cutting/microsoft-365/commands");

    for command_name in SEND_COMMANDS {
        let doc_path = commands_dir.join(format!("{command_name}.md"));
        let doc = fs::read_to_string(&doc_path)
            .unwrap_or_else(|error| panic!("read {}: {error}", doc_path.display()));
        let lowercase = doc.to_ascii_lowercase();
        let normalized = lowercase.split_whitespace().collect::<Vec<_>>().join(" ");

        assert!(
            normalized.contains("planned / unavailable"),
            "{command_name}"
        );
        assert!(normalized.contains("202 accepted"), "{command_name}");
        assert!(normalized.contains("no response body"), "{command_name}");
        assert!(
            normalized.contains("does not prove delivery"),
            "{command_name}"
        );
        assert!(normalized.contains("no message id"), "{command_name}");
        assert!(
            normalized.contains("must not automatically retry"),
            "{command_name}"
        );
        assert!(
            !normalized.contains("message-id:"),
            "{command_name} must not show a fabricated message-id output"
        );
    }
}
