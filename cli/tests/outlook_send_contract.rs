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
    // Runnable only on a CLI with the reviewed Graph read (list-folder, #667);
    // the send commands themselves stay planned.
    assert_eq!(response["data"]["status"], "requires-runtime");
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
        Some("requires-runtime"),
        "only the reviewed in-process Graph read is runnable, on a new enough CLI"
    );
    assert!(
        manifest["transport"].get("cli").is_none(),
        "the agent must not advertise the never-shipped aware-m365 executable"
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

#[test]
fn first_party_bundle_installs_the_newest_microsoft_365_release_pinned_to_its_runtime() {
    let path = repository_root().join("registry-index.json");
    let index: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display())),
    )
    .unwrap_or_else(|error| panic!("parse {}: {error}", path.display()));
    let agents = index["bundles"]["aware-aeco"]["agents"]
        .as_array()
        .expect("aware-aeco bundle must list agents");

    // 1.2.0 (manifest 0.4.0, #709 slice B1) adds the mail, Planner, SharePoint and
    // drive-item reads and requires CLI 0.167.0; the bundle tracks it.
    assert!(agents.iter().any(|entry| entry == "microsoft-365@1.2.0"));
    assert!(
        !agents
            .iter()
            .any(|entry| entry == "microsoft-365@1.0.0" || entry == "microsoft-365@1.1.0")
    );
    let versions = &index["agents"]["microsoft-365"]["versions"];
    assert_eq!(
        versions
            .as_object()
            .expect("Microsoft 365 versions must be an object")
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        vec!["1.0.0", "1.1.0", "1.2.0"],
        "earlier releases remain addressable while 0.4.0 becomes current"
    );
    let current = &versions["1.2.0"];
    assert_eq!(current["manifest-version"], "0.4.0");
    // Pinned to the v0.167.0 release commit, whose manifest names that CLI as its
    // floor, so an older CLI refuses the new reads instead of misrunning them.
    let sha = "c38dbee512538155a3e461c931b8d4e77abfaf6e";
    assert_eq!(
        current["tarball"],
        format!("https://github.com/aware-aeco/aware/archive/{sha}.tar.gz")
    );
    assert_eq!(
        current["subdir"],
        format!("aware-{sha}/20-agents/aeco/cross-cutting/microsoft-365")
    );
    for (version, entry) in versions.as_object().unwrap() {
        let tarball = entry["tarball"].as_str().unwrap_or_default();
        assert!(
            !tarball.contains("refs/heads/"),
            "microsoft-365@{version} must download an immutable archive, not a branch: {tarball}"
        );
    }
}
