//! A script step reads the run file an `artifact-stream` step spooled, through the real
//! `aware app run`, in the exact shape FloLess's daily model check ships (floless.app#3593):
//! `artifacts: { model: '{{ read_model.body }}' }` with the step's whole output in `args`.
mod common;
use assert_cmd::Command;
use serde_json::Value;

fn install(home: &std::path::Path, base: &str) {
    let source = home.join("agents/model-source");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::write(
        source.join("manifest.yaml"),
        format!(
            r#"agent: model-source
version: 0.1.0
description: test record-stream source
stateful: false
license: MIT
transport:
  rest:
    base: {base}
commands:
  read-model-data:
    lifecycle: single
    mode: read
    description: Read model data as a record stream
    method: POST
    path: /model/data
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
    let script = home.join("agents/script");
    std::fs::create_dir_all(&script).unwrap();
    std::fs::copy(
        repo.join("20-agents/_core/script/manifest.yaml"),
        script.join("manifest.yaml"),
    )
    .unwrap();
    let app = home.join("apps/model-check");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("model-check.flo"),
        r#"app: model-check
version: 0.1.0
description: Count the objects a model read handed over
requires:
  - model-source@0.1.0
  - script@0.2.x
layout: linear
nodes:
  - id: read_model
    agent: model-source
    command: read-model-data
    mode: read
    config:
      content-type: application/json
      body: '{"selection":"latest"}'
  - id: check
    agent: script
    command: run
    inputs:
      language: javascript
      artifacts:
        model: '{{ read_model.body }}'
      args:
        read: '{{ read_model }}'
      code: |
        // Count the objects the read handed over, or say what the read answered instead.
        if (files.model === null) return { status: args.read.status, objects: null };
        return { status: args.read.status, objects: files.model.records.filter(r => r.rec === 'object').length };
connections:
  - { from: read_model, to: check }
"#,
    )
    .unwrap();
    common::approve_installed_apps(home);
}

fn run(home: &std::path::Path, reservation: &str) -> Vec<Value> {
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .env("AWARE_REPORT_RESERVATION_ID", reservation)
        .env("AWARE_REPORT_SOURCE_BYTES", "100000000")
        .args(["app", "run", "model-check"])
        .assert()
        .success();
    let logs = home.join("logs/model-check/default");
    let trace = std::fs::read_dir(&logs)
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .find(|p| p.extension().is_some_and(|e| e == "jsonl"))
        .unwrap();
    std::fs::read_to_string(trace)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

fn output<'a>(events: &'a [Value], node: &str) -> &'a Value {
    &events
        .iter()
        .find(|e| e["kind"] == "node-output" && e["node"] == node)
        .unwrap_or_else(|| panic!("no output for {node}: {events:?}"))["data"]
}

/// Past the 10 MB a step's answer can carry: 12 MB of records reach the script whole, the trace
/// keeps only the receipt, and the run's files can then be retired.
#[test]
fn a_script_reads_a_large_record_stream_the_read_step_spooled() {
    let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
    let base = format!("http://{}", server.server_addr());
    let objects = 120_000;
    let line = |i: usize| {
        format!(
            "{{\"rec\":\"object\",\"side\":\"current\",\"id\":\"part-{i:06}\",\"type\":\"PART\",\"name\":\"BEAM\",\"pad\":\"{}\"}}\n",
            "x".repeat(20)
        )
    };
    let mut stream = String::from("{\"rec\":\"header\"}\n");
    for i in 0..objects {
        stream.push_str(&line(i));
    }
    assert!(
        stream.len() > 11 * 1024 * 1024,
        "the stream must pass the 10 MB answer limit"
    );
    let bytes = stream.len();
    let responder = std::thread::spawn(move || {
        let request = server.recv().unwrap();
        let ct = tiny_http::Header::from_bytes(b"content-type", b"application/x-ndjson").unwrap();
        request
            .respond(tiny_http::Response::from_data(stream.into_bytes()).with_header(ct))
            .unwrap();
    });
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("aware");
    install(&home, &base);
    let events = run(&home, "check-reservation-1");
    responder.join().unwrap();

    let read = output(&events, "read_model");
    assert_eq!(read["status"], 200);
    assert_eq!(read["body"]["artifact"]["bytes"], bytes);
    let check = output(&events, "check");
    assert_eq!(check["result"]["objects"], objects);
    let receipt = check["files"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["name"] == "model")
        .unwrap();
    assert_eq!(receipt["sha256"], read["body"]["artifact"]["sha256"]);
    let trace_bytes: usize = events.iter().map(|e| e.to_string().len()).sum();
    assert!(
        trace_bytes < 100_000,
        "the trace keeps receipts, never the data ({trace_bytes} bytes)"
    );

    // A run made of a REST read and a script has no external writer, so its run files retire.
    let run_id = read["body"]["artifact"]["runId"].as_str().unwrap();
    let prune = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &home)
        .args([
            "app",
            "artifact",
            "model-check",
            "--run-id",
            run_id,
            "--prune",
            "--reservation-id",
            "check-reservation-1",
        ])
        .output()
        .unwrap();
    assert!(
        prune.status.success(),
        "{}",
        String::from_utf8_lossy(&prune.stderr)
    );
}

/// A read that answers 503 leaves the script a `null` run file and the read's own answer; the
/// run still ends, so the script can say what happened.
#[test]
fn a_failed_read_reaches_the_script_as_null_with_its_status() {
    let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
    let base = format!("http://{}", server.server_addr());
    let responder = std::thread::spawn(move || {
        let request = server.recv().unwrap();
        let ct = tiny_http::Header::from_bytes(b"content-type", b"application/json").unwrap();
        request
            .respond(
                tiny_http::Response::from_string(
                    r#"{"ok":false,"code":"model-data-busy","error":"FloLess is reading other models right now."}"#,
                )
                .with_status_code(503)
                .with_header(ct),
            )
            .unwrap();
    });
    let temp = tempfile::tempdir().unwrap();
    let home = temp.path().join("aware");
    install(&home, &base);
    let events = run(&home, "check-reservation-2");
    responder.join().unwrap();

    let check = output(&events, "check");
    assert_eq!(check["result"]["status"], 503);
    assert_eq!(check["result"]["objects"], Value::Null);
    let read = output(&events, "read_model");
    assert!(
        read["body"]["error"]
            .as_str()
            .is_some_and(|e| e.contains("model-data-busy")),
        "the bounded failure body keeps FloLess's sentence: {read}"
    );
}
