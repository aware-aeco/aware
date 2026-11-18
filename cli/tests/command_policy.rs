//! E2E coverage for the command policy (`aware policy …`, checked at
//! dispatch) through the real binary: a `rest` agent against a local stub
//! server, each test in its own temp AWARE_HOME with file-seeded credentials
//! (`AWARE_DISABLE_KEYRING`), as `agent_invoke_rest.rs` does.

use assert_cmd::Command;
use predicates::prelude::*;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

const BINDING: &str = "a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d";

/// A stub HTTP server that answers 200 to everything and records each request
/// line.
struct Stub {
    port: u16,
    seen: Arc<Mutex<Vec<String>>>,
}

fn stub() -> Stub {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let seen = Arc::new(Mutex::new(Vec::new()));
    let log = seen.clone();
    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let Ok(mut stream) = stream else { break };
            let mut data = Vec::new();
            let mut buf = [0u8; 1024];
            while !data.windows(4).any(|w| w == b"\r\n\r\n") {
                match stream.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => data.extend_from_slice(&buf[..n]),
                }
            }
            let text = String::from_utf8_lossy(&data).to_string();
            log.lock()
                .unwrap()
                .push(text.lines().next().unwrap_or("").to_string());
            let body = r#"{"ok":true}"#;
            let resp = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            let _ = stream.write_all(resp.as_bytes());
        }
    });
    Stub { port, seen }
}

fn seed(home: &std::path::Path, handle: &str) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let creds = home.join("credentials");
    std::fs::create_dir_all(&creds).unwrap();
    std::fs::write(
        creds.join(format!("{handle}.json")),
        serde_json::json!({
            "access_token": format!("{handle}-token"),
            "refresh_token": "rt",
            "expires_at": now + 86_400,
            "scope": "openid",
            "token_type": "Bearer",
            "integration": "trimble-connect",
            "obtained_at": now,
            "source": "oauth",
        })
        .to_string(),
    )
    .unwrap();
}

fn home_with_rest_agent(base: &str) -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("agents").join("demo-rest");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("manifest.yaml"),
        format!(
            r#"agent: demo-rest
version: 0.1.0
description: demo rest agent
stateful: false
license: MIT
transport:
  rest:
    base: {base}
auth:
  scheme: oauth2
  secret: trimble-connect
commands:
  list-projects:
    lifecycle: single
    description: List projects.
    method: GET
    path: /projects
  list-users:
    lifecycle: single
    description: List users.
    method: GET
    path: /users
  ping:
    lifecycle: single
    description: Public endpoint.
    method: GET
    path: /ping
    no-auth: true
"#
        ),
    )
    .unwrap();
    seed(tmp.path(), "trimble-connect");
    seed(tmp.path(), "trimble-connect.team");
    tmp
}

fn aware(home: &std::path::Path) -> Command {
    let mut cmd = Command::cargo_bin("aware").unwrap();
    cmd.env("AWARE_HOME", home)
        .env("AWARE_DISABLE_KEYRING", "1");
    cmd
}

fn invoke(
    home: &std::path::Path,
    command: &str,
    alias: Option<&str>,
) -> assert_cmd::assert::Assert {
    let mut cmd = aware(home);
    cmd.args(["--json", "agent", "invoke", "demo-rest", command]);
    if let Some(alias) = alias {
        cmd.args(["--as", alias]);
    }
    cmd.assert()
}

fn count(server: &Stub) -> usize {
    server.seen.lock().unwrap().len()
}

#[test]
fn a_denied_command_is_refused_at_dispatch_and_allowed_again_once_removed() {
    let server = stub();
    let home = home_with_rest_agent(&format!("http://127.0.0.1:{}", server.port));
    invoke(home.path(), "list-projects", None).success();
    assert_eq!(count(&server), 1);

    aware(home.path())
        .args([
            "policy",
            "deny",
            "projects-off",
            "--agent",
            "demo-rest",
            "--command",
            "list-projects",
        ])
        .assert()
        .success();
    invoke(home.path(), "list-projects", None)
        .code(5)
        .stderr(predicate::str::contains("E_COMMAND_DENIED_BY_POLICY"))
        .stderr(predicate::str::contains("projects-off"));
    assert_eq!(count(&server), 1, "a denied command must send nothing");
    // Another command of the same agent is untouched.
    invoke(home.path(), "list-users", None).success();
    assert_eq!(count(&server), 2);

    aware(home.path())
        .args(["policy", "remove", "projects-off"])
        .assert()
        .success();
    invoke(home.path(), "list-projects", None).success();
    assert_eq!(count(&server), 3);
}

#[test]
fn a_slot_rule_denies_only_that_slot_and_a_binding_exception_cannot_be_verified_here() {
    let server = stub();
    let home = home_with_rest_agent(&format!("http://127.0.0.1:{}", server.port));
    aware(home.path())
        .args([
            "policy",
            "deny",
            "team-off",
            "--agent",
            "demo-rest",
            "--command",
            "*",
            "--account",
            "trimble-connect.team",
        ])
        .assert()
        .success();
    invoke(home.path(), "list-projects", None).success();
    invoke(home.path(), "list-projects", Some("team"))
        .code(5)
        .stderr(predicate::str::contains("E_COMMAND_DENIED_BY_POLICY"));
    assert_eq!(count(&server), 1);

    // Off for every account with one opted-in binding: the generic REST path
    // has no identity read, so it cannot confirm the exception and refuses.
    aware(home.path())
        .args([
            "policy",
            "deny",
            "team-off",
            "--agent",
            "demo-rest",
            "--command",
            "*",
            "--except",
            &format!("trimble-connect={BINDING}"),
        ])
        .assert()
        .success();
    invoke(home.path(), "list-projects", None)
        .code(5)
        .stderr(predicate::str::contains("account-unverified"));
    assert_eq!(count(&server), 1);
}

#[test]
fn a_no_auth_command_of_a_credentialed_agent_is_still_governed() {
    let server = stub();
    let home = home_with_rest_agent(&format!("http://127.0.0.1:{}", server.port));
    aware(home.path())
        .args([
            "policy",
            "deny",
            "ping-off",
            "--agent",
            "demo-rest",
            "--command",
            "ping",
        ])
        .assert()
        .success();
    invoke(home.path(), "ping", None).code(5);
    assert_eq!(count(&server), 0);
}

#[test]
fn an_unreadable_policy_refuses_credentialed_dispatch_and_show_says_why() {
    let server = stub();
    let home = home_with_rest_agent(&format!("http://127.0.0.1:{}", server.port));
    std::fs::write(home.path().join("command-policy.json"), "{ broken").unwrap();
    invoke(home.path(), "list-projects", None)
        .code(5)
        .stderr(predicate::str::contains("policy-unreadable"));
    assert_eq!(count(&server), 0);
    let out = aware(home.path())
        .args(["--json", "policy", "show"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let v: serde_json::Value = serde_json::from_slice(&out).unwrap();
    assert_eq!(v["data"]["state"], "unreadable");
    // A merge into it is refused; a whole replace repairs it.
    aware(home.path())
        .args(["policy", "remove", "x"])
        .assert()
        .code(3)
        .stderr(predicate::str::contains("E_COMMAND_POLICY_UNREADABLE"));
    let doc = home.path().join("doc.json");
    std::fs::write(&doc, r#"{"schema":"aware.command-policy/v1","rules":[]}"#).unwrap();
    aware(home.path())
        .args(["policy", "set", &format!("@{}", doc.display())])
        .assert()
        .success();
    invoke(home.path(), "list-projects", None).success();
}

#[test]
fn policy_verbs_validate_and_round_trip_as_json() {
    let home = tempfile::tempdir().unwrap();
    // An exception must name a binding.
    aware(home.path())
        .args([
            "policy",
            "deny",
            "r",
            "--agent",
            "a",
            "--command",
            "c",
            "--except",
            "a",
        ])
        .assert()
        .code(3)
        .stderr(predicate::str::contains("E_COMMAND_POLICY_INVALID"));
    aware(home.path())
        .args([
            "--json",
            "policy",
            "deny",
            "r",
            "--agent",
            "a",
            "--command",
            "c",
            "--account",
            &format!("a.work={}", BINDING.to_ascii_uppercase()),
            "--note",
            "host note",
        ])
        .assert()
        .success();
    let out = aware(home.path())
        .args(["--json", "policy", "show"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let v: serde_json::Value = serde_json::from_slice(&out).unwrap();
    assert_eq!(v["data"]["state"], "readable");
    assert_eq!(
        v["data"]["rules"][0],
        serde_json::json!({"id": "r", "agent": "a", "command": "c",
            "accounts": [{"slot": "a.work", "binding": BINDING}], "note": "host note"})
    );
    let doc = home.path().join("bad.json");
    std::fs::write(
        &doc,
        r#"{"schema":"aware.command-policy/v1","rules":[{"id":"r"}]}"#,
    )
    .unwrap();
    aware(home.path())
        .args(["policy", "set", &format!("@{}", doc.display())])
        .assert()
        .code(3)
        .stderr(predicate::str::contains("E_COMMAND_POLICY_INVALID"));
}
