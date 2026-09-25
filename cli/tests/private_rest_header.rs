use std::io::{Read, Write};
use std::net::TcpListener;
use std::time::{Duration, Instant};

use assert_cmd::Command;

const SECRET: &str = "run-scoped-test-secret-123456789";

fn fixture(home: &std::path::Path, origin: &str, two_nodes: bool) {
    let agent_dir = home.join("agents/local-rest");
    let app_dir = home.join("apps/private-report");
    std::fs::create_dir_all(&agent_dir).unwrap();
    std::fs::create_dir_all(&app_dir).unwrap();
    std::fs::write(
        agent_dir.join("manifest.yaml"),
        format!(
            r#"agent: local-rest
version: 0.1.0
description: local REST fixture
stateful: false
license: MIT
transport:
  rest:
    base: {origin}
commands:
  read:
    lifecycle: single
    mode: read
    description: Read the report endpoint
    method: GET
    path: /private
    outputs:
      type: single
      schema:
        status: int
"#
        ),
    )
    .unwrap();
    let other = if two_nodes {
        "  - id: other\n    agent: local-rest\n    command: read\n    mode: read\n"
    } else {
        ""
    };
    std::fs::write(
        app_dir.join("private-report.flo"),
        format!("app: private-report\nversion: 0.1.0\ndescription: private header fixture\nrequires:\n  - local-rest@0.1.0\nnodes:\n{other}  - id: model\n    agent: local-rest\n    command: read\n    mode: read\nconnections: []\n"),
    )
    .unwrap();
}

fn descriptor(origin: &str) -> String {
    serde_json::json!({
        "nodeId": "model",
        "agent": "local-rest",
        "command": "read",
        "method": "GET",
        "origin": origin,
        "path": "/private",
        "headerName": "X-Private-Run",
    })
    .to_string()
}

fn compile(home: &std::path::Path) {
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["app", "compile"])
        .arg(home.join("apps/private-report/private-report.flo"))
        .assert()
        .success();
}

fn receive(listener: &TcpListener) -> Option<String> {
    let deadline = Instant::now() + Duration::from_secs(15);
    loop {
        match listener.accept() {
            Ok((mut stream, _)) => {
                stream
                    .set_read_timeout(Some(Duration::from_secs(2)))
                    .unwrap();
                let mut buf = [0u8; 16384];
                let n = stream.read(&mut buf).unwrap();
                let request = String::from_utf8_lossy(&buf[..n]).to_string();
                let body = r#"{"ok":true}"#;
                write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
                return Some(request);
            }
            Err(error)
                if error.kind() == std::io::ErrorKind::WouldBlock && Instant::now() < deadline =>
            {
                std::thread::sleep(Duration::from_millis(20));
            }
            Err(_) => return None,
        }
    }
}

#[test]
fn real_app_run_sends_private_header_only_to_the_bound_node() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let origin = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
    let server = std::thread::spawn(move || (receive(&listener), receive(&listener)));
    let home = tempfile::tempdir().unwrap();
    fixture(home.path(), &origin, true);
    compile(home.path());
    let result = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .env("AWARE_PRIVATE_REST_HEADER", descriptor(&origin))
        .env("AWARE_PRIVATE_REST_HEADER_VALUE", SECRET)
        .args(["app", "run", "private-report"])
        .output()
        .unwrap();
    let (first, second) = server.join().unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let first = first.expect("first request");
    let second = second.expect("second request");
    assert_eq!(
        usize::from(first.contains(SECRET)) + usize::from(second.contains(SECRET)),
        1,
        "private value must appear in exactly one request"
    );
    let trace = std::fs::read_dir(home.path().join("logs/private-report/default"))
        .unwrap()
        .filter_map(Result::ok)
        .find(|entry| entry.path().extension().is_some_and(|ext| ext == "jsonl"))
        .map(|entry| std::fs::read_to_string(entry.path()).unwrap())
        .unwrap();
    assert!(!trace.contains(SECRET));
    assert!(!String::from_utf8_lossy(&result.stdout).contains(SECRET));
    assert!(!String::from_utf8_lossy(&result.stderr).contains(SECRET));
}

#[test]
fn report_reservation_owner_is_inspectable_before_dispatch_and_cannot_be_reused() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let origin = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
    let home = tempfile::tempdir().unwrap();
    fixture(home.path(), &origin, false);
    compile(home.path());
    let home_path = home.path().to_path_buf();
    let server = std::thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(15);
        let (mut stream, _) = loop {
            match listener.accept() {
                Ok(connection) => break connection,
                Err(error)
                    if error.kind() == std::io::ErrorKind::WouldBlock
                        && Instant::now() < deadline =>
                {
                    std::thread::sleep(Duration::from_millis(20));
                }
                Err(error) => panic!("report endpoint was not contacted: {error}"),
            }
        };
        let mut request = [0u8; 4096];
        let count = stream.read(&mut request).unwrap();
        assert!(String::from_utf8_lossy(&request[..count]).contains(SECRET));
        let owner = Command::cargo_bin("aware")
            .unwrap()
            .env("AWARE_HOME", &home_path)
            .args(["app", "artifact-reservation", "reservation-1"])
            .output()
            .unwrap();
        assert!(
            owner.status.success(),
            "{}",
            String::from_utf8_lossy(&owner.stderr)
        );
        let json: serde_json::Value = serde_json::from_slice(&owner.stdout).unwrap();
        assert_eq!(json["schemaVersion"], "aware.report-reservation/v1");
        assert_eq!(json["reservationId"], "reservation-1");
        assert_eq!(json["app"], "private-report");
        assert_eq!(json["instance"], "default");
        assert_eq!(json["artifactScope"]["runId"], json["runId"]);
        assert_eq!(json["artifactScope"]["app"], json["app"]);
        assert_eq!(json["artifactScope"]["instance"], json["instance"]);
        let body = r#"{"ok":true}"#;
        write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
        json
    });
    let first = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .env("AWARE_REPORT_RESERVATION_ID", "reservation-1")
        .env("AWARE_PRIVATE_REST_HEADER", descriptor(&origin))
        .env("AWARE_PRIVATE_REST_HEADER_VALUE", SECRET)
        .args(["app", "run", "private-report"])
        .output()
        .unwrap();
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    let owner = server.join().unwrap();
    assert!(String::from_utf8_lossy(&first.stdout).contains(owner["runId"].as_str().unwrap()));

    let reused = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .env("AWARE_REPORT_RESERVATION_ID", "reservation-1")
        .args(["app", "run", "private-report"])
        .output()
        .unwrap();
    assert_eq!(reused.status.code(), Some(8));
    assert!(String::from_utf8_lossy(&reused.stderr).contains("already used"));

    let missing = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .args(["app", "artifact-reservation", "never-used"])
        .output()
        .unwrap();
    assert_eq!(missing.status.code(), Some(7));

    let marker = home
        .path()
        .join("logs/.report-reservations/reservation-1.json");
    let mut mismatched: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&marker).unwrap()).unwrap();
    mismatched["artifactScope"]["runId"] = "another-run".into();
    std::fs::write(&marker, serde_json::to_vec(&mismatched).unwrap()).unwrap();
    let corrupt = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .args(["app", "artifact-reservation", "reservation-1"])
        .output()
        .unwrap();
    assert_eq!(corrupt.status.code(), Some(3));
    assert!(corrupt.stdout.is_empty());
}

#[test]
fn successful_graph_without_bound_request_fails_closed() {
    let home = tempfile::tempdir().unwrap();
    let origin = "http://127.0.0.1:9";
    fixture(home.path(), origin, false);
    compile(home.path());
    let wrong = serde_json::json!({
        "nodeId": "absent", "agent": "local-rest", "command": "read",
        "method": "GET", "origin": origin, "path": "/private",
        "headerName": "X-Private-Run",
    });
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .env("AWARE_PRIVATE_REST_HEADER", wrong.to_string())
        .env("AWARE_PRIVATE_REST_HEADER_VALUE", SECRET)
        .args(["app", "run", "private-report"])
        .assert()
        .failure();
}

#[test]
fn private_header_does_not_follow_a_redirect() {
    let source = TcpListener::bind("127.0.0.1:0").unwrap();
    let destination = TcpListener::bind("127.0.0.1:0").unwrap();
    destination.set_nonblocking(true).unwrap();
    let origin = format!("http://127.0.0.1:{}", source.local_addr().unwrap().port());
    let redirected = format!(
        "http://127.0.0.1:{}/stolen",
        destination.local_addr().unwrap().port()
    );
    let server = std::thread::spawn(move || {
        let (mut stream, _) = source.accept().unwrap();
        let mut buf = [0u8; 4096];
        let n = stream.read(&mut buf).unwrap();
        let request = String::from_utf8_lossy(&buf[..n]).to_string();
        write!(stream, "HTTP/1.1 302 Found\r\nLocation: {redirected}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").unwrap();
        request
    });
    let home = tempfile::tempdir().unwrap();
    fixture(home.path(), &origin, false);
    compile(home.path());
    let output = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home.path())
        .env("AWARE_PRIVATE_REST_HEADER", descriptor(&origin))
        .env("AWARE_PRIVATE_REST_HEADER_VALUE", SECRET)
        .args(["app", "run", "private-report"])
        .output()
        .unwrap();
    let request = server.join().unwrap();
    assert!(request.contains(SECRET));
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("private REST target redirected"));
    assert!(
        destination.accept().is_err(),
        "redirect destination was contacted"
    );
}

#[test]
fn malformed_or_oversized_private_environment_is_rejected_without_echo() {
    for descriptor in [
        "{}".to_string(),
        "{".repeat(2500),
        serde_json::json!({
            "nodeId":"model", "agent":"local-rest", "command":"read",
            "method":"GET", "origin":"http://127.0.0.1:1",
            "path":"/private", "headerName":"Authorization"
        })
        .to_string(),
    ] {
        let result = Command::cargo_bin("aware")
            .unwrap()
            .env("AWARE_PRIVATE_REST_HEADER", descriptor)
            .env("AWARE_PRIVATE_REST_HEADER_VALUE", SECRET)
            .arg("--version")
            .output()
            .unwrap();
        assert!(!result.status.success());
        assert!(!String::from_utf8_lossy(&result.stderr).contains(SECRET));
    }
}
