//! E2E coverage for `aware agent invoke` on a `rest` agent (#711), through the
//! real binary against a local stub HTTP server. Each test gets its own temp
//! AWARE_HOME; credentials are seeded as files (`AWARE_DISABLE_KEYRING`), the
//! same way `connect_token_import.rs` does.

use assert_cmd::Command;
use predicates::prelude::*;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

/// A stub HTTP server that answers every request and records
/// `"<METHOD> <PATH>\n<Authorization header or ->"` for each one.
struct Stub {
    port: u16,
    seen: Arc<Mutex<Vec<String>>>,
}

fn stub(redirect_to: Option<String>) -> Stub {
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
            let mut lines = text.lines();
            let request_line = lines.next().unwrap_or("").to_string();
            let auth = lines
                .find(|l| l.to_ascii_lowercase().starts_with("authorization:"))
                .unwrap_or("-")
                .to_string();
            let path = request_line.split(' ').nth(1).unwrap_or("").to_string();
            log.lock()
                .unwrap()
                .push(format!("{}\n{auth}", request_line.replace(" HTTP/1.1", "")));
            let resp = match (&redirect_to, path.starts_with("/redir")) {
                (Some(to), true) => format!(
                    "HTTP/1.1 302 Found\r\nLocation: {to}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                ),
                _ => {
                    let body = r#"[{"id":"p1","name":"Tower A"}]"#;
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                        body.len()
                    )
                }
            };
            let _ = stream.write_all(resp.as_bytes());
        }
    });
    Stub { port, seen }
}

fn seed(home: &std::path::Path, handle: &str, access: &str) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let creds = home.join("credentials");
    std::fs::create_dir_all(&creds).unwrap();
    std::fs::write(
        creds.join(format!("{handle}.json")),
        serde_json::json!({
            "access_token": access,
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

/// AWARE_HOME holding a `demo-rest` agent aimed at `base`, with the default
/// and a `team` credential slot.
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
    inputs:
      top:
        type: int
        in: query
        default: 100
        description: Page size.
  get-project:
    lifecycle: single
    description: One project.
    method: GET
    path: /projects/{{project-id}}
    inputs:
      project-id:
        type: string
        in: path
        description: Project id.
  redirect:
    lifecycle: single
    description: Answers with a redirect.
    method: GET
    path: /redir
"#
        ),
    )
    .unwrap();
    seed(tmp.path(), "trimble-connect", "default-token");
    seed(tmp.path(), "trimble-connect.team", "team-token");
    tmp
}

fn aware(home: &std::path::Path) -> Command {
    let mut cmd = Command::cargo_bin("aware").unwrap();
    cmd.env("AWARE_HOME", home)
        .env("AWARE_DISABLE_KEYRING", "1");
    cmd
}

fn first_json(stdout: &[u8]) -> serde_json::Value {
    serde_json::from_slice(stdout)
        .unwrap_or_else(|e| panic!("not JSON ({e}): {}", String::from_utf8_lossy(stdout)))
}

#[test]
fn invoke_rest_runs_the_command_and_prints_the_node_output() {
    let server = stub(None);
    let home = home_with_rest_agent(&format!("http://127.0.0.1:{}", server.port));
    let out = aware(home.path())
        .args([
            "--json",
            "agent",
            "invoke",
            "demo-rest",
            "list-projects",
            "--inputs",
            r#"{"top": 5}"#,
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("default-token").not())
        .get_output()
        .stdout
        .clone();
    let v = first_json(&out);
    assert_eq!(v["data"]["status"], 200);
    assert_eq!(v["data"]["body"][0]["name"], "Tower A");
    assert!(v["data"]["headers"].is_object());
    let seen = server.seen.lock().unwrap();
    assert_eq!(seen.len(), 1);
    assert!(seen[0].starts_with("GET /projects?top=5"), "{}", seen[0]);
    assert!(
        seen[0].ends_with("uthorization: Bearer default-token"),
        "{}",
        seen[0]
    );
}

#[test]
fn invoke_rest_as_alias_uses_that_credential_slot() {
    let server = stub(None);
    let home = home_with_rest_agent(&format!("http://127.0.0.1:{}", server.port));
    aware(home.path())
        .args([
            "agent",
            "invoke",
            "demo-rest",
            "list-projects",
            "--as",
            "team",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("team-token").not());
    let seen = server.seen.lock().unwrap();
    assert!(seen[0].ends_with("Bearer team-token"), "{}", seen[0]);
}

#[test]
fn invoke_rest_missing_alias_slot_is_its_own_error_and_sends_nothing() {
    let server = stub(None);
    let home = home_with_rest_agent(&format!("http://127.0.0.1:{}", server.port));
    aware(home.path())
        .args([
            "agent",
            "invoke",
            "demo-rest",
            "list-projects",
            "--as",
            "ghost",
        ])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("E_AGENT_INVOKE_ALIAS_MISSING"));
    assert!(server.seen.lock().unwrap().is_empty());
}

#[test]
fn invoke_rest_undeclared_input_is_refused_and_sends_nothing() {
    let server = stub(None);
    let home = home_with_rest_agent(&format!("http://127.0.0.1:{}", server.port));
    aware(home.path())
        .args([
            "agent",
            "invoke",
            "demo-rest",
            "list-projects",
            "--inputs",
            r#"{"top": 5, "evil": "x"}"#,
        ])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("E_AGENT_INVOKE_UNDECLARED_INPUT"))
        .stderr(predicate::str::contains("evil"));
    assert!(server.seen.lock().unwrap().is_empty());
}

#[test]
fn invoke_rest_path_input_is_one_percent_encoded_segment() {
    let server = stub(None);
    let home = home_with_rest_agent(&format!("http://127.0.0.1:{}", server.port));
    aware(home.path())
        .args([
            "agent",
            "invoke",
            "demo-rest",
            "get-project",
            "--inputs",
            r#"{"project-id": "a/b c?x=1"}"#,
        ])
        .assert()
        .success();
    let seen = server.seen.lock().unwrap();
    assert!(
        seen[0].starts_with("GET /projects/a%2Fb%20c%3Fx%3D1\n"),
        "{}",
        seen[0]
    );
}

#[test]
fn invoke_rest_dot_segment_path_input_is_refused() {
    let server = stub(None);
    let home = home_with_rest_agent(&format!("http://127.0.0.1:{}", server.port));
    aware(home.path())
        .args([
            "agent",
            "invoke",
            "demo-rest",
            "get-project",
            "--inputs",
            r#"{"project-id": ".."}"#,
        ])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("E_AGENT_INVOKE_PATH_SEGMENT"));
    // The request builder comma-joins an array into one segment, so `[".."]`
    // is `..` too.
    aware(home.path())
        .args([
            "agent",
            "invoke",
            "demo-rest",
            "get-project",
            "--inputs",
            r#"{"project-id": [".."]}"#,
        ])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("E_AGENT_INVOKE_PATH_SEGMENT"));
    assert!(server.seen.lock().unwrap().is_empty());
}

#[test]
fn invoke_rest_pinned_multi_step_flow_is_refused() {
    let server = stub(None);
    let home = home_with_rest_agent(&format!("http://127.0.0.1:{}", server.port));
    let dir = home.path().join("agents").join("trimble-connect");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("manifest.yaml"),
        format!(
            "agent: trimble-connect
version: 0.1.0
description: x
stateful: false
license: MIT
transport:
  rest:
    base: http://127.0.0.1:{}
auth: {{ scheme: oauth2, secret: trimble-connect }}
commands:
  upload: {{ lifecycle: single, description: x }}
",
            server.port
        ),
    )
    .unwrap();
    aware(home.path())
        .args(["agent", "invoke", "trimble-connect", "upload"])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("E_AGENT_INVOKE_PINNED_FLOW"));
    assert!(server.seen.lock().unwrap().is_empty());
}

#[test]
fn invoke_rest_redirect_to_another_host_is_not_followed() {
    let other = stub(None);
    // `localhost` is a different host from the request's `127.0.0.1`.
    let server = stub(Some(format!("http://localhost:{}/stolen", other.port)));
    let home = home_with_rest_agent(&format!("http://127.0.0.1:{}", server.port));
    let out = aware(home.path())
        .args(["--json", "agent", "invoke", "demo-rest", "redirect"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let v = first_json(&out);
    assert_eq!(v["data"]["status"], 302);
    assert_eq!(server.seen.lock().unwrap().len(), 1);
    assert!(
        other.seen.lock().unwrap().is_empty(),
        "the redirect target was contacted (token would have followed)"
    );
}

#[test]
fn describe_json_lists_each_commands_declared_inputs() {
    let home = home_with_rest_agent("http://127.0.0.1:9");
    let out = aware(home.path())
        .args(["--json", "agent", "describe", "demo-rest"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let v = first_json(&out);
    let cmds = v["data"]["commands"].as_array().unwrap();
    let find = |n: &str| cmds.iter().find(|c| c["name"] == n).unwrap();
    assert_eq!(
        find("list-projects")["inputs"],
        serde_json::json!([{
            "name": "top", "type": "int", "in": "query", "required": false,
            "default": 100, "description": "Page size."
        }])
    );
    let get = &find("get-project")["inputs"][0];
    assert_eq!(get["in"], "path");
    assert_eq!(get["required"], true);
    assert!(get["default"].is_null());
    assert_eq!(find("redirect")["inputs"], serde_json::json!([]));
}
