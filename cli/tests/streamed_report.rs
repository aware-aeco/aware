mod common;
use assert_cmd::Command;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::io::Read;

struct PausedBody {
    remaining: usize,
    sent: std::sync::mpsc::Sender<()>,
    resume: std::sync::mpsc::Receiver<()>,
}

impl Read for PausedBody {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.remaining > 0 {
            let n = buf.len().min(self.remaining);
            buf[..n].fill(b'x');
            self.remaining -= n;
            if self.remaining == 0 {
                let _ = self.sent.send(());
            }
            return Ok(n);
        }
        let _ = self.resume.recv();
        Ok(0)
    }
}

#[test]
fn interrupted_live_report_cannot_prune_until_its_writer_is_dead() {
    let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
    let base = format!("http://{}", server.server_addr());
    let (first_tx, first_rx) = std::sync::mpsc::channel();
    let (resume_tx, resume_rx) = std::sync::mpsc::channel();
    let responder = std::thread::spawn(move || {
        let request = server.recv().unwrap();
        let content_type =
            tiny_http::Header::from_bytes(b"content-type", b"application/x-ndjson").unwrap();
        let body = PausedBody {
            remaining: 512 * 1024,
            sent: first_tx,
            resume: resume_rx,
        };
        let response = tiny_http::Response::new(
            tiny_http::StatusCode(200),
            vec![content_type],
            body,
            None,
            None,
        );
        let _ = request.respond(response);
    });
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("aware");
    let agent = home.join("agents/floless-workspace");
    std::fs::create_dir_all(&agent).unwrap();
    std::fs::write(
        agent.join("manifest.yaml"),
        format!(
            r#"agent: floless-workspace
version: 0.2.4
description: Test streamed report source
stateful: false
license: MIT
transport:
  rest:
    base: {base}
commands:
  read-model-complete:
    lifecycle: single
    mode: read
    description: Read model stream
    method: POST
    path: /stream
    response: artifact-stream
    inputs:
      body: {{type: object, in: body}}
      content-type: {{type: string, in: header}}
    outputs:
      type: single
      schema: {{status: int, headers: object, body: object}}
"#
        ),
    )
    .unwrap();
    let app = home.join("apps/report-crash");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("report-crash.flo"),
        r#"app: report-crash
version: 0.1.0
description: Test in-process report stream
requires:
  - floless-workspace@0.2.4
layout: linear
nodes:
  - id: model
    agent: floless-workspace
    command: read-model-complete
    mode: read
    config:
      content-type: application/json
      body: '{"selection":"test"}'
"#,
    )
    .unwrap();
    common::approve_installed_apps(&home);
    let mut writer = std::process::Command::new(assert_cmd::cargo::cargo_bin("aware"))
        .env("AWARE_HOME", &home)
        .env("AWARE_REPORT_RESERVATION_ID", "crash-reservation")
        .env("AWARE_REPORT_SOURCE_BYTES", "1000000")
        .env("AWARE_REPORT_RENDER_BYTES", "1000000")
        .args(["app", "run", "report-crash"])
        .spawn()
        .unwrap();
    first_rx
        .recv_timeout(std::time::Duration::from_secs(20))
        .unwrap();
    let marker: Value = serde_json::from_slice(
        &std::fs::read(home.join("logs/.report-reservations/crash-reservation.json")).unwrap(),
    )
    .unwrap();
    let run_id = marker["runId"].as_str().unwrap();
    let artifact_dir = home
        .join("logs/report-crash/default")
        .join(format!("{run_id}.artifacts"));
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while std::time::Instant::now() < deadline {
        if std::fs::read_dir(&artifact_dir).unwrap().any(|entry| {
            entry
                .ok()
                .and_then(|entry| entry.metadata().ok())
                .is_some_and(|meta| meta.len() > 0)
        }) {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    assert!(
        std::fs::read_dir(&artifact_dir).unwrap().any(|entry| {
            entry
                .ok()
                .and_then(|entry| entry.metadata().ok())
                .is_some_and(|meta| meta.len() > 0)
        }),
        "the live writer must have spooled real bytes before the crash"
    );
    let args = [
        "app",
        "artifact",
        "report-crash",
        "--run-id",
        run_id,
        "--prune",
        "--reservation-id",
        "crash-reservation",
    ];
    let live = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &home)
        .args(args)
        .output()
        .unwrap();
    assert!(
        !live.status.success(),
        "live writer must hold the kernel lock"
    );
    assert!(String::from_utf8_lossy(&live.stderr).contains("still active"));
    writer.kill().unwrap();
    writer.wait().unwrap();
    let _ = resume_tx.send(());
    responder.join().unwrap();
    let after = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &home)
        .args(args)
        .output()
        .unwrap();
    assert!(
        after.status.success(),
        "{}",
        String::from_utf8_lossy(&after.stderr)
    );
    let result: Value = serde_json::from_slice(&after.stdout).unwrap();
    assert!(result["bytes"].as_u64().unwrap() > 0);
    assert_eq!(std::fs::read_dir(&artifact_dir).unwrap().count(), 0);
}

#[test]
fn shipped_report_topology_resolves_model_body_through_status_gate() {
    let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
    let base = format!("http://{}", server.server_addr());
    let header = json!({"type":"header","schemaVersion":"floless.complete-model-report/v1",
        "provider":"tekla","projectUuid":"p","revisionId":"r","manifestRoot":"m",
        "sourceSha256":"s","expected":{"entities":0,"properties":0,"relationships":0},
        "receipts":[
            {"role":"entities","digest":"a".repeat(64),"bytes":0,"items":0,"ordinal":0},
            {"role":"properties","digest":"b".repeat(64),"bytes":0,"items":0,"ordinal":0},
            {"role":"relationships","digest":"c".repeat(64),"bytes":0,"items":0,"ordinal":0}]});
    let provenance = json!({"type":"model-provenance","providerFingerprint":"fp",
        "sourceSha256":"s","manifestRoot":"m","projectUuid":"p","revisionId":"r"});
    let mut stream = format!("{header}\n{provenance}\n").into_bytes();
    let mut hash = Sha256::new();
    hash.update(&stream);
    let terminal = json!({"type":"terminal","observed":{"entities":0,"properties":0,"relationships":0},
        "priorBytes":stream.len(),"priorSha256":format!("{:x}",hash.finalize())});
    stream.extend_from_slice(format!("{terminal}\n").as_bytes());
    let responder = std::thread::spawn(move || {
        let request = server.recv().unwrap();
        assert_eq!(request.method(), &tiny_http::Method::Post);
        assert_eq!(request.url(), "/api/agent/workspace/model/report-stream");
        let ct = tiny_http::Header::from_bytes(b"content-type", b"application/x-ndjson").unwrap();
        request
            .respond(tiny_http::Response::from_data(stream).with_header(ct))
            .unwrap();
    });

    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("aware");
    let agent = home.join("agents/floless-workspace");
    std::fs::create_dir_all(&agent).unwrap();
    std::fs::write(
        agent.join("manifest.yaml"),
        format!(
            r#"agent: floless-workspace
version: 0.2.4
description: test report bridge
stateful: false
license: MIT
transport:
  rest:
    base: {base}
commands:
  read-model-complete:
    lifecycle: single
    mode: read
    description: Read complete approved model
    method: POST
    path: /api/agent/workspace/model/report-stream
    response: artifact-stream
    inputs:
      body: {{type: object, in: body}}
      content-type: {{type: string, in: header}}
    outputs:
      type: single
      schema: {{status: int, headers: object, body: object}}
"#
        ),
    )
    .unwrap();
    let repo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let report_agent = home.join("agents/html-report-stream");
    std::fs::create_dir_all(&report_agent).unwrap();
    std::fs::copy(
        repo.join("20-agents/_core/html-report-stream/manifest.yaml"),
        report_agent.join("manifest.yaml"),
    )
    .unwrap();
    let app = home.join("apps/tekla-model-report");
    std::fs::create_dir_all(&app).unwrap();
    let embedded_workflow = r#"app: tekla-model-report
version: 0.1.0
description: Complete approved Tekla model report
inputs:
  model_revision:
    type: string
    default: ""
requires:
  - floless-workspace@0.2.4
  - html-report-stream@0.1.x
layout: linear
nodes:
  - id: model
    agent: floless-workspace
    command: read-model-complete
    mode: read
    config:
      content-type: application/json
      body: |
        {"selection":{{ inputs.model_revision }}}
  - id: gate
    inline:
      kind: predicate
      description: Continue only when the complete approved Tekla model read succeeded.
      code: "e.status == 200"
  - id: report
    agent: html-report-stream
    command: render-stream
    mode: read
    config:
      title: Tekla Parser model properties
      data: '{{ model.body }}'
      columns: [Group, Property, Value, Unit, Provenance]
connections:
  - { from: model, to: gate }
  - { from: gate, to: report }
"#;
    let workflow = match std::env::var("AWARE_TEST_EXACT_FLO_PATH") {
        Ok(path) => std::fs::read_to_string(path).unwrap(),
        Err(_) => embedded_workflow.to_string(),
    };
    std::fs::write(app.join("tekla-model-report.flo"), workflow).unwrap();
    common::approve_installed_apps(&home);
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &home)
        .env("AWARE_REPORT_RESERVATION_ID", "test-reservation-1")
        .env("AWARE_REPORT_SOURCE_BYTES", "1000000")
        .env("AWARE_REPORT_RENDER_BYTES", "2000000")
        .args([
            "app",
            "run",
            "tekla-model-report",
            "--input",
            "model_revision={\"projectUuid\":\"p\"}",
        ])
        .assert()
        .success();
    responder.join().unwrap();
    let logs = home.join("logs/tekla-model-report/default");
    let trace_file = std::fs::read_dir(&logs)
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .find(|p| p.extension().is_some_and(|e| e == "jsonl"))
        .unwrap();
    let trace = std::fs::read_to_string(trace_file).unwrap();
    assert!(trace.len() < 20_000 && !trace.contains("<html"));
    let events: Vec<Value> = trace
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let model = events
        .iter()
        .find(|e| e["kind"] == "node-output" && e["node"] == "model")
        .unwrap();
    assert_eq!(model["data"]["status"], 200);
    assert_eq!(
        model["data"]["body"]["artifact"]["schemaVersion"],
        "aware.artifact-ref/v1"
    );
    let report = events
        .iter()
        .find(|e| e["kind"] == "node-output" && e["node"] == "report")
        .unwrap();
    assert_eq!(report["data"]["complete"], true);
    assert_eq!(report["data"]["objectCount"], 0);
    assert_eq!(
        report["data"]["bundle"]["contentType"],
        "application/vnd.aware.html-report.bundle.v1"
    );
    assert_eq!(report["data"]["bundle"]["app"], "tekla-model-report");
    assert!(report["data"]["bundle"]["runId"].as_str().is_some());
    let run_id = report["data"]["bundle"]["runId"].as_str().unwrap();
    let artifact_dir = logs.join(format!("{run_id}.artifacts"));
    if let Ok(target) = std::env::var("AWARE_TEST_BUNDLE_OUTPUT") {
        let id = report["data"]["bundle"]["id"].as_str().unwrap();
        std::fs::copy(artifact_dir.join(id), target).unwrap();
    }
    let before = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &home)
        .args([
            "app",
            "artifact",
            "tekla-model-report",
            "--run-id",
            run_id,
            "--usage",
        ])
        .output()
        .unwrap();
    assert!(
        before.status.success(),
        "{}",
        String::from_utf8_lossy(&before.stderr)
    );
    let before: Value = serde_json::from_slice(&before.stdout).unwrap();
    assert!(before["bytes"].as_u64().unwrap() > 0);
    let sibling = logs.join("unrelated-run.artifacts");
    std::fs::create_dir(&sibling).unwrap();
    std::fs::write(sibling.join("keep"), b"unrelated").unwrap();
    let wrong = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &home)
        .args([
            "app",
            "artifact",
            "tekla-model-report",
            "--run-id",
            run_id,
            "--prune",
            "--reservation-id",
            "some-other-reservation",
        ])
        .output()
        .unwrap();
    assert!(!wrong.status.success());
    assert!(artifact_dir.read_dir().unwrap().next().is_some());
    let foreign_directory = artifact_dir.join("foreign-directory");
    std::fs::create_dir(&foreign_directory).unwrap();
    let malformed = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &home)
        .args([
            "app",
            "artifact",
            "tekla-model-report",
            "--run-id",
            run_id,
            "--prune",
            "--reservation-id",
            "test-reservation-1",
        ])
        .output()
        .unwrap();
    assert!(!malformed.status.success());
    assert!(String::from_utf8_lossy(&malformed.stderr).contains("non-file"));
    std::fs::remove_dir(foreign_directory).unwrap();
    let pruned = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &home)
        .args([
            "app",
            "artifact",
            "tekla-model-report",
            "--run-id",
            run_id,
            "--prune",
            "--reservation-id",
            "test-reservation-1",
        ])
        .output()
        .unwrap();
    assert!(
        pruned.status.success(),
        "{}",
        String::from_utf8_lossy(&pruned.stderr)
    );
    let pruned: Value = serde_json::from_slice(&pruned.stdout).unwrap();
    assert_eq!(pruned["pruned"], true);
    assert_eq!(pruned["bytes"], before["bytes"]);
    assert!(
        artifact_dir.is_dir(),
        "empty artifact directory remains as a tombstone"
    );
    let after = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &home)
        .args([
            "app",
            "artifact",
            "tekla-model-report",
            "--run-id",
            run_id,
            "--usage",
        ])
        .output()
        .unwrap();
    assert!(after.status.success());
    let after: Value = serde_json::from_slice(&after.stdout).unwrap();
    assert_eq!(after["bytes"], 0);
    assert_eq!(after["files"], 0);
    let again = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &home)
        .args([
            "app",
            "artifact",
            "tekla-model-report",
            "--run-id",
            run_id,
            "--prune",
            "--reservation-id",
            "test-reservation-1",
        ])
        .output()
        .unwrap();
    assert!(again.status.success());
    let again: Value = serde_json::from_slice(&again.stdout).unwrap();
    assert_eq!(again["bytes"], 0);
    assert_eq!(std::fs::read(sibling.join("keep")).unwrap(), b"unrelated");
    if std::env::var_os("AWARE_TEST_EXACT_FLO_PATH").is_some() {
        println!("REPORT_OUTPUT {}", report["data"]);
    }
}
