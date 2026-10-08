//! `aware agent call-capabilities` and `aware agent call` (#618), driven through
//! the real binary: the argv FloLess sends, the envelope and exit code of each
//! refusal that needs no network, the code-only log line, and the shipped
//! google-workspace manifest declaring `list-files` runnable and read-only.
//!
//! The transport rules (slot resolution, no fallback, origin pin, redirect
//! refusal, token non-disclosure, caps, deadline, identity binding, replacement
//! boundary, pinned refresh endpoint) are unit-tested in
//! `src/runtime/agent_call_tests.rs` against local HTTP fixtures.

mod common;

use assert_cmd::Command;
use sha2::{Digest, Sha256};

const TOKEN: &str = "integration-secret-token-618";
const GENERATION: &str = "0b9c3f53-6a3e-4c0f-9d0e-2f3b4a5c6d7e";

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

fn repo_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn shipped_manifest_bytes() -> Vec<u8> {
    std::fs::read(repo_root().join("20-agents/aeco/cross-cutting/google-workspace/manifest.yaml"))
        .unwrap()
}

fn install_google(home: &std::path::Path) -> String {
    let dir = home.join("agents").join("google-workspace");
    std::fs::create_dir_all(&dir).unwrap();
    let bytes = shipped_manifest_bytes();
    std::fs::write(dir.join("manifest.yaml"), &bytes).unwrap();
    format!("{:x}", Sha256::digest(&bytes))
}

/// A fresh mail-only Google credential in the file store (no Drive scope).
fn store_mail_only(home: &std::path::Path) {
    let dir = home.join("credentials");
    std::fs::create_dir_all(&dir).unwrap();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let token = serde_json::json!({
        "access_token": TOKEN,
        "refresh_token": null,
        "expires_at": now + 3600,
        "scope": "https://www.googleapis.com/auth/gmail.send https://www.googleapis.com/auth/userinfo.email openid",
        "token_type": "Bearer",
        "integration": "google-workspace",
        "obtained_at": now,
        "generation": GENERATION,
        "source": "oauth",
    });
    std::fs::write(dir.join("google-workspace.json"), token.to_string()).unwrap();
}

fn log_lines(home: &std::path::Path) -> Vec<String> {
    std::fs::read_to_string(home.join("logs").join("agent-call.log"))
        .unwrap_or_default()
        .lines()
        .map(str::to_string)
        .collect()
}

#[test]
fn capabilities_for_an_uninstalled_agent_is_an_error_envelope_with_a_log_line() {
    let home = tempfile::tempdir().unwrap();
    let out = aware(home.path())
        .args([
            "--json",
            "agent",
            "call-capabilities",
            "google-workspace",
            "list-files",
        ])
        .assert()
        .failure()
        .code(7)
        .get_output()
        .stdout
        .clone();
    let v = json_of(&out);
    assert_eq!(v["ok"], false);
    assert_eq!(v["error"]["code"], "E_AGENT_NOT_INSTALLED");
    assert_eq!(v["meta"]["command"], "agent call-capabilities");
    let lines = log_lines(home.path());
    assert_eq!(lines.len(), 1, "{lines:?}");
    assert!(
        lines[0].ends_with(
            " agent-call call-capabilities google-workspace list-files E_AGENT_NOT_INSTALLED"
        ),
        "{}",
        lines[0]
    );
}

#[test]
fn an_unreviewed_operation_is_unsupported() {
    let home = tempfile::tempdir().unwrap();
    let out = aware(home.path())
        .args([
            "--json",
            "agent",
            "call-capabilities",
            "microsoft-365",
            "list-folder",
        ])
        .assert()
        .failure()
        .code(3)
        .get_output()
        .stdout
        .clone();
    assert_eq!(json_of(&out)["error"]["code"], "E_CALL_UNSUPPORTED");
    // The unreviewed command name is not echoed into the log.
    assert!(log_lines(home.path())[0].contains(" microsoft-365 - E_CALL_UNSUPPORTED"));
}

#[test]
fn an_alias_is_one_valid_slot_name_never_an_option_or_another_slot() {
    let home = tempfile::tempdir().unwrap();
    install_google(home.path());
    // `--json` after `--as` is not swallowed as an alias.
    aware(home.path())
        .args([
            "agent",
            "call-capabilities",
            "google-workspace",
            "list-files",
            "--as",
            "--json",
        ])
        .assert()
        .failure()
        .code(2);
    aware(home.path())
        .args([
            "agent",
            "call-capabilities",
            "google-workspace",
            "list-files",
            "--as",
            "-x",
        ])
        .assert()
        .failure()
        .code(2);
    for alias in ["--as=-x", "--as=Team", "--as=a.b"] {
        let out = aware(home.path())
            .args([
                "--json",
                "agent",
                "call-capabilities",
                "google-workspace",
                "list-files",
                alias,
            ])
            .assert()
            .failure()
            .code(3)
            .get_output()
            .stdout
            .clone();
        assert_eq!(
            json_of(&out)["error"]["code"],
            "E_CALL_ALIAS_INVALID",
            "{alias}"
        );
    }
}

#[test]
fn call_reads_only_an_at_file_request() {
    let home = tempfile::tempdir().unwrap();
    for (arg, reason) in [
        (
            "{\"schema\":\"aware.agent-call/v1\"}",
            "not-a-file-reference",
        ),
        ("@", "not-a-file-reference"),
        ("@does-not-exist.json", "unreadable"),
    ] {
        let out = aware(home.path())
            .args(["--json", "agent", "call", arg])
            .assert()
            .failure()
            .code(3)
            .get_output()
            .stdout
            .clone();
        let v = json_of(&out);
        assert_eq!(v["error"]["code"], "E_CALL_REQUEST_INVALID", "{arg}");
        assert_eq!(
            v["error"]["details"],
            serde_json::json!({"reason": reason}),
            "{arg}"
        );
    }
    let dir = tempfile::tempdir().unwrap();
    let big = dir.path().join("big.json");
    std::fs::write(&big, vec![b' '; 64 * 1024 + 1]).unwrap();
    let out = aware(home.path())
        .args(["--json", "agent", "call"])
        .arg(format!("@{}", big.display()))
        .assert()
        .failure()
        .code(3)
        .get_output()
        .stdout
        .clone();
    assert_eq!(json_of(&out)["error"]["details"]["reason"], "too-large");
}

/// The whole pre-network path through the binary: a FloLess-shaped request
/// file, the installed shipped manifest, a mail-only credential. It is refused
/// for the missing Drive scope before any request is made — this test has no
/// network fixture, so a request would fail it differently.
#[test]
fn a_mail_only_slot_is_refused_for_scope_before_any_request() {
    let home = tempfile::tempdir().unwrap();
    let manifest_sha = install_google(home.path());
    store_mail_only(home.path());
    let inputs = serde_json::json!({"page-size": 5});
    let request = serde_json::json!({
        "schema": "aware.agent-call/v1",
        "invocationId": "6f1c2b3a-4d5e-5f60-8a1b-2c3d4e5f6a7b",
        "agent": "google-workspace",
        "command": "list-files",
        "expectedAgentVersion": "2.2.0",
        "expectedManifestSha256": manifest_sha,
        "executionTransport": "rest",
        "operationSchemaVersion": 1,
        "expectedOperationSha256": operation_sha(home.path()),
        "integration": "google-workspace",
        "alias": null,
        "bindingId": "6f1c2b3a-4d5e-8f60-8a1b-2c3d4e5f6a7b",
        "bindingRevision": 1,
        "credentialGeneration": GENERATION,
        "inputs": inputs,
        "inputsSha256": format!("{:x}", Sha256::digest(b"{\"page-size\":5}")),
        "owner": {"rootId": "r"},
        "connectionRevision": "rev",
        "approvalReceiptId": "6f1c2b3a-4d5e-5f60-8a1b-2c3d4e5f6a7c",
        "timeoutMs": 5000,
        "maxOutputBytes": 262144
    });
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("request.json");
    std::fs::write(&path, request.to_string()).unwrap();
    let assert = aware(home.path())
        .args(["--json", "agent", "call"])
        .arg(format!("@{}", path.display()))
        .assert()
        .failure()
        .code(6);
    let output = assert.get_output();
    let v = json_of(&output.stdout);
    assert_eq!(v["error"]["code"], "E_CALL_SCOPE_MISSING", "{v}");
    assert_eq!(v["meta"]["command"], "agent call");
    let everything = format!(
        "{}{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
        log_lines(home.path()).join("\n")
    );
    assert!(!everything.contains(TOKEN), "{everything}");
    assert!(
        log_lines(home.path())[0]
            .ends_with(" agent-call call google-workspace list-files E_CALL_SCOPE_MISSING")
    );
}

/// The operation digest, read from the CLI itself so the test cannot drift from
/// the code: a capability call on a home with no credential reports it without
/// any network request (the slot is missing).
fn operation_sha(home: &std::path::Path) -> String {
    let other = tempfile::tempdir().unwrap();
    install_google(other.path());
    let out = aware(other.path())
        .args([
            "--json",
            "agent",
            "call-capabilities",
            "google-workspace",
            "list-files",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let v = json_of(&out);
    assert_eq!(v["data"]["credential"]["status"], "missing", "{v}");
    assert_eq!(v["data"]["available"], false);
    assert_eq!(v["data"]["unavailableCode"], "credential-missing");
    let _ = home;
    v["data"]["operationSha256"].as_str().unwrap().to_string()
}

#[test]
fn the_shipped_manifest_declares_list_files_read_only_and_validates() {
    let manifest: serde_yaml::Value = serde_yaml::from_slice(&shipped_manifest_bytes()).unwrap();
    let list = &manifest["commands"]["list-files"];
    assert!(list.get("status").is_none(), "list-files must be runnable");
    assert_eq!(list["mode"].as_str(), Some("read"));
    assert_eq!(list["method"].as_str(), Some("GET"));
    assert_eq!(
        list["path"].as_str(),
        Some("https://www.googleapis.com/drive/v3/files")
    );
    let network: Vec<&str> = manifest["requires"]["network"]
        .as_sequence()
        .unwrap()
        .iter()
        .filter_map(|v| v.as_str())
        .collect();
    assert!(network.contains(&"https://www.googleapis.com"));
    let home = tempfile::tempdir().unwrap();
    aware(home.path())
        .args(["agent", "validate"])
        .arg(repo_root().join("20-agents/aeco/cross-cutting/google-workspace"))
        .assert()
        .success();
}
