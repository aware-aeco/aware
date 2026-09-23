mod common;
use assert_cmd::Command;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

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
    if std::env::var_os("AWARE_TEST_EXACT_FLO_PATH").is_some() {
        println!("REPORT_OUTPUT {}", report["data"]);
    }
    if let Ok(target) = std::env::var("AWARE_TEST_BUNDLE_OUTPUT") {
        let run_id = report["data"]["bundle"]["runId"].as_str().unwrap();
        let id = report["data"]["bundle"]["id"].as_str().unwrap();
        std::fs::copy(logs.join(format!("{run_id}.artifacts")).join(id), target).unwrap();
    }
}
