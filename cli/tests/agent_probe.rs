//! `aware agent probe` and the surfaces around it (#617), driven through the
//! real binary: the envelope and exit code of a refusal, the code-only log line,
//! the probe block `agent describe --json` publishes, `agent validate` refusing
//! an invalid block, `credential status` reporting the slot generation, and
//! `build agent --probe`.
//!
//! The transport mechanics (origin pin, redirect refusal, token non-disclosure,
//! caps, slot rule, tree-kill) are unit-tested in `src/runtime/probe_tests.rs`
//! against a local HTTP fixture; the real Tekla run is recorded in the PR.

mod common;

use assert_cmd::Command;
use sha2::{Digest, Sha256};

fn aware(home: &std::path::Path) -> Command {
    let mut cmd = Command::cargo_bin("aware").unwrap();
    cmd.env("AWARE_HOME", home)
        .env("AWARE_DISABLE_KEYRING", "1")
        .env_remove("AWARE_REGISTRY");
    cmd
}

fn json_of(bytes: &[u8]) -> serde_json::Value {
    serde_json::from_slice(bytes)
        .unwrap_or_else(|e| panic!("not JSON ({e}): {}", String::from_utf8_lossy(bytes)))
}

fn install_manifest(home: &std::path::Path, id: &str, manifest: &str) {
    let dir = home.join("agents").join(id);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("manifest.yaml"), manifest).unwrap();
}

const HOST_AGENT: &str = "agent: hostx\nversion: 1.0.0\ndescription: x\nstateful: false\nlicense: MIT\n\
transport: { cli: { binary: aware-no-such-bridge-617 } }\n\
commands:\n  model-info: { lifecycle: single, description: Reads the model. }\n\
probe: { command: model-info, describe: Reads the model name., kind: host }\n";

#[test]
fn a_refused_probe_is_an_error_envelope_with_a_code_only_log_line() {
    let home = tempfile::tempdir().unwrap();
    install_manifest(home.path(), "hostx", HOST_AGENT);
    let out = aware(home.path())
        .args(["--json", "agent", "probe", "hostx"])
        .assert()
        .failure()
        .code(4)
        .get_output()
        .stdout
        .clone();
    let v = json_of(&out);
    assert_eq!(v["ok"], false);
    assert!(v["data"].is_null());
    assert_eq!(v["error"]["code"], "E_HOST_UNAVAILABLE");
    assert_eq!(
        v["error"]["details"],
        serde_json::json!({"reason": "bridge-missing"})
    );
    assert_eq!(v["meta"]["command"], "agent probe");

    let log = std::fs::read_to_string(home.path().join("logs/agent-probe.log")).unwrap();
    assert_eq!(log.lines().count(), 1);
    assert!(
        log.trim_end()
            .ends_with("agent-probe hostx E_HOST_UNAVAILABLE"),
        "{log}"
    );
}

#[test]
fn an_agent_without_a_probe_and_an_absent_agent_are_refused_by_code() {
    let home = tempfile::tempdir().unwrap();
    install_manifest(
        home.path(),
        "plain",
        "agent: plain\nversion: 1.0.0\ndescription: x\nstateful: false\nlicense: MIT\n\
         transport: { cli: { binary: x } }\ncommands:\n  r: { lifecycle: single, description: d }\n",
    );
    for (id, code, exit) in [
        ("plain", "E_PROBE_UNDECLARED", 3),
        ("ghost", "E_AGENT_NOT_INSTALLED", 7),
    ] {
        let out = aware(home.path())
            .args(["--json", "agent", "probe", id])
            .assert()
            .failure()
            .code(exit)
            .get_output()
            .stdout
            .clone();
        assert_eq!(json_of(&out)["error"]["code"], code, "{id}");
    }
}

#[test]
fn the_timeout_flag_is_bounded() {
    let home = tempfile::tempdir().unwrap();
    for bad in ["999", "60001"] {
        aware(home.path())
            .args(["agent", "probe", "hostx", "--timeout-ms", bad])
            .assert()
            .failure()
            .code(2);
    }
}

#[test]
fn describe_publishes_the_probe_and_the_same_manifest_digest() {
    let home = common::aware_home();
    let out = aware(&home)
        .args(["--json", "agent", "describe", "tekla"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let data = json_of(&out)["data"].clone();
    let bytes = std::fs::read(home.join("agents/tekla/manifest.yaml")).unwrap();
    assert_eq!(
        data["manifestSha256"],
        format!("{:x}", Sha256::digest(&bytes))
    );
    assert_eq!(
        data["probe"],
        serde_json::json!({
            "command": "model-info",
            "describe": "Reads the name of the model open in Tekla Structures.",
            "kind": "host",
            "reviewed": false,
            "origin": null,
            "credentialHandle": null,
        })
    );

    let out = aware(&home)
        .args(["--json", "agent", "describe", "google-workspace"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let probe = json_of(&out)["data"]["probe"].clone();
    assert_eq!(probe["command"], "account.userinfo");
    assert_eq!(probe["kind"], "account");
    assert_eq!(probe["origin"], "https://openidconnect.googleapis.com");
    assert_eq!(probe["credentialHandle"], "google-workspace");
    assert_eq!(probe["reviewed"], false);
}

#[test]
fn validate_refuses_a_probe_on_a_caller_determined_command() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(
        tmp.path().join("manifest.yaml"),
        "agent: bad\nversion: 1.0.0\ndescription: x\nstateful: false\nlicense: MIT\n\
         transport: { cli: { binary: x } }\n\
         commands:\n  exec: { lifecycle: single, description: d, mode: write, mode-overridable: true }\n\
         probe: { command: exec, describe: Runs something., kind: host }\n",
    )
    .unwrap();
    let assert = aware(tmp.path())
        .args(["agent", "validate"])
        .arg(tmp.path())
        .assert()
        .failure();
    let output = assert.get_output();
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(text.contains("E_PROBE_INVALID"), "{text}");
    assert!(text.contains("command-mode-overridable"), "{text}");
}

#[test]
fn the_shipped_probes_validate() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    for agent in [
        "20-agents/aeco/engineering/tekla",
        "20-agents/aeco/cross-cutting/google-workspace",
    ] {
        let home = tempfile::tempdir().unwrap();
        aware(home.path())
            .args(["agent", "validate"])
            .arg(root.join(agent))
            .assert()
            .success();
    }
}

#[test]
fn credential_status_reports_one_stable_generation_even_for_a_legacy_credential() {
    let home = tempfile::tempdir().unwrap();
    let creds = home.path().join("credentials");
    std::fs::create_dir_all(&creds).unwrap();
    // Hand-written, pre-generation shape.
    std::fs::write(creds.join("legacy-svc.json"), r#"{"token":"abc"}"#).unwrap();
    let status = |handle: &str| {
        let out = aware(home.path())
            .args(["--json", "credential", "status", handle])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();
        json_of(&out)
    };
    let first = status("legacy-svc");
    assert_eq!(first["status"], "present");
    let generation = first["generation"]
        .as_str()
        .expect("a generation is written")
        .to_string();
    assert_eq!(status("legacy-svc")["generation"], generation.as_str());

    let missing = status("never-provisioned");
    assert_eq!(missing["status"], "missing");
    assert!(missing["generation"].is_null());
}

#[test]
fn build_agent_writes_a_validated_probe_or_nothing() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("aware");
    let spec = tmp.path().join("spec.json");
    std::fs::write(
        &spec,
        r#"{
        "openapi": "3.0.0",
        "info": { "title": "Demo", "version": "1.2.3", "license": {"name": "MIT"} },
        "servers": [ { "url": "https://api.demo.example/v1" } ],
        "paths": {
            "/me": { "get": { "operationId": "getMe", "summary": "Who am I" } },
            "/items": { "post": { "operationId": "createItem", "summary": "Create" } }
        }
    }"#,
    )
    .unwrap();

    // A write-mode command cannot be a probe — and nothing is written.
    aware(&home)
        .args(["build", "agent", "--from-openapi"])
        .arg(&spec)
        .args(["--probe", "create-item"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("E_PROBE_INVALID"));
    assert!(!home.join("agents/demo").exists());

    aware(&home)
        .args(["build", "agent", "--from-openapi"])
        .arg(&spec)
        .args(["--probe", "get-me"])
        .assert()
        .success();
    let out = aware(&home)
        .args(["--json", "agent", "describe", "demo"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let probe = json_of(&out)["data"]["probe"].clone();
    assert_eq!(probe["command"], "get-me");
    assert_eq!(probe["kind"], "host");
    assert_eq!(probe["origin"], "https://api.demo.example");
    assert_eq!(probe["reviewed"], false);
}
