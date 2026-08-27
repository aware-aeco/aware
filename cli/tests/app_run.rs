mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn run_one_shot_app_with_no_installed_agents_fails_clearly() {
    // welded-to-tc references agents that aren't installed in this temp home, so
    // the run must not proceed. Since #308 that is caught by the missing-agent
    // pre-flight (`E_APP_AGENT_NOT_INSTALLED`) before any dispatch; previously it
    // got as far as spawning `aware-tekla` and failed there. Either way the
    // contract this test pins is the same: a non-zero exit, never a silent pass.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");

    // Install the welded-to-tc app from the repo .flo
    let app_src = tmp.path().join("welded-to-tc-src");
    std::fs::create_dir_all(&app_src).unwrap();
    let flo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("30-apps/_examples/welded-to-tc.app");
    std::fs::copy(&flo, app_src.join("welded-to-tc.app")).unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "install"])
        .arg(&app_src)
        .assert()
        .success();

    // Now `app run` — should fail with a clear error (binary not found or network)
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "run", "welded-to-tc"])
        .assert()
        .failure();
}

#[test]
fn missing_non_bridge_cli_binary_does_not_suggest_sidecar_install() {
    // #276: the "program not found" hint used to point every `aware-*` binary at
    // `aware sidecar install <id>`, but that command rejects agents not in the
    // BRIDGES table — a dead end. A non-bridge cli agent must get the
    // build-or-PATH hint instead.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");

    let agent_dir = aware.join("agents/ifc-inspector");
    std::fs::create_dir_all(&agent_dir).unwrap();
    std::fs::write(
        agent_dir.join("manifest.yaml"),
        r#"agent: ifc-inspector
version: 0.0.1
description: x
stateful: false
license: MIT
transport:
  cli:
    binary: aware-ifc-inspector
commands:
  inspect:
    lifecycle: single
    category: curated
    mode: read
    description: inspects an ifc
    outputs:
      type: single
      schema:
        summary: string
"#,
    )
    .unwrap();

    let app_dir = aware.join("apps/inspectapp");
    std::fs::create_dir_all(&app_dir).unwrap();
    std::fs::write(
        app_dir.join("inspectapp.flo"),
        r#"app: inspectapp
version: 0.0.1
description: x
nodes:
  - id: ins
    agent: ifc-inspector
    command: inspect
connections: []
requires: []
"#,
    )
    .unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "run", "inspectapp"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("no downloadable bridge"))
        .stderr(predicate::str::contains("sidecar install").not());
}

#[test]
fn simulate_runs_app_end_to_end_without_host() {
    // The #103 scenario: an app whose read-mode node would reach for a host
    // sidecar that isn't installed. A real run fails ("program not found");
    // `--simulate` must stub the read node and complete cleanly.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");

    // A curated read-command agent whose binary does not exist on PATH.
    let agent_dir = aware.join("agents/stub-reader");
    std::fs::create_dir_all(&agent_dir).unwrap();
    std::fs::write(
        agent_dir.join("manifest.yaml"),
        r#"agent: stub-reader
version: 0.0.1
description: x
stateful: false
license: MIT
transport:
  cli:
    binary: this-read-binary-does-not-exist
commands:
  report:
    lifecycle: single
    category: curated
    mode: read
    description: emits a report path
    outputs:
      type: single
      schema:
        path: string
        row-count: number
"#,
    )
    .unwrap();

    let app_dir = aware.join("apps/reportapp");
    std::fs::create_dir_all(&app_dir).unwrap();
    std::fs::write(
        app_dir.join("reportapp.flo"),
        r#"app: reportapp
version: 0.0.1
description: x
nodes:
  - id: rep
    agent: stub-reader
    command: report
connections: []
requires: []
"#,
    )
    .unwrap();

    // A real run fails: the read node spawns the missing host binary.
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "run", "reportapp"])
        .assert()
        .failure();

    // `--simulate` stubs the read node — no host contact — and succeeds.
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "run", "reportapp", "--simulate"])
        .assert()
        .success()
        .stdout(predicate::str::contains("simulate"))
        .stdout(predicate::str::contains("schema-shaped placeholder"));
}

#[test]
fn simulate_runs_streaming_watcher_app_without_a_bridge() {
    // The #117-2 scenario: a `lifecycle: start` watcher source routes to the
    // long-running path. A real run fails because the watch bridge binary isn't
    // installed (`invoke_stream` now spawns it; #172); `--simulate` must stub the
    // stream and complete cleanly without contacting any host.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");

    // A stateful watcher agent → makes `is_long_running` true.
    let agent_dir = aware.join("agents/stub-watcher");
    std::fs::create_dir_all(&agent_dir).unwrap();
    std::fs::write(
        agent_dir.join("manifest.yaml"),
        r#"agent: stub-watcher
version: 0.0.1
description: x
stateful: true
license: MIT
transport:
  cli:
    binary: this-watch-binary-does-not-exist
commands:
  watch:
    lifecycle: start
    category: curated
    mode: read
    description: stream of marks
    outputs:
      type: stream
      schema:
        mark: string
"#,
    )
    .unwrap();

    let app_dir = aware.join("apps/watchapp");
    std::fs::create_dir_all(&app_dir).unwrap();
    std::fs::write(
        app_dir.join("watchapp.flo"),
        r#"app: watchapp
version: 0.0.1
description: x
nodes:
  - id: watch
    agent: stub-watcher
    command: watch
connections: []
requires: []
"#,
    )
    .unwrap();

    // A real run takes the long-running path and fails: invoke_stream tries to
    // spawn `this-watch-binary-does-not-exist`, which isn't on PATH (#172).
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .timeout(std::time::Duration::from_secs(30))
        .args(["app", "run", "watchapp"])
        .assert()
        .failure();

    // `--simulate` stubs the stream source (one placeholder event, then the
    // source closes) and the run completes — no invoke_stream.
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .timeout(std::time::Duration::from_secs(30))
        .args(["app", "run", "watchapp", "--simulate"])
        .assert()
        .success();
}

#[test]
fn simulated_reader_app_preserves_an_existing_reader_pidfile() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");

    let watcher_dir = aware.join("agents/stub-reader-watcher");
    std::fs::create_dir_all(&watcher_dir).unwrap();
    std::fs::write(
        watcher_dir.join("manifest.yaml"),
        r#"agent: stub-reader-watcher
version: 0.0.1
description: x
stateful: true
license: MIT
transport:
  cli:
    binary: this-watch-binary-does-not-exist
commands:
  watch:
    lifecycle: start
    category: curated
    mode: read
    description: stream of marks
    outputs:
      type: stream
      schema:
        mark: string
"#,
    )
    .unwrap();

    let reader_dir = aware.join("agents/model-reference-reader");
    std::fs::create_dir_all(&reader_dir).unwrap();
    std::fs::write(
        reader_dir.join("manifest.yaml"),
        r#"agent: model-reference-reader
version: 0.4.0
description: x
stateful: false
license: MIT
transport:
  cli:
    binary: this-reader-binary-does-not-exist
commands:
  read-model:
    lifecycle: single
    category: curated
    mode: read
    description: normalize model
    outputs:
      type: object
      schema:
        snapshotId: string
"#,
    )
    .unwrap();

    let app_dir = aware.join("apps/reader-watchapp");
    std::fs::create_dir_all(&app_dir).unwrap();
    std::fs::write(
        app_dir.join("reader-watchapp.flo"),
        r#"app: reader-watchapp
version: 0.0.1
description: x
nodes:
  - id: watch
    agent: stub-reader-watcher
    command: watch
  - id: read
    agent: model-reference-reader
    command: read-model
connections: []
requires: []
"#,
    )
    .unwrap();

    let instance_dir = app_dir.join("instances/default");
    std::fs::create_dir_all(&instance_dir).unwrap();
    let pidfile = instance_dir.join("pidfile.yaml");
    let existing_pidfile = b"pid: 4242\nstartedAt: preserved\n";
    std::fs::write(&pidfile, existing_pidfile).unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .timeout(std::time::Duration::from_secs(30))
        .args(["app", "run", "reader-watchapp", "--simulate"])
        .assert()
        .success();

    assert_eq!(std::fs::read(pidfile).unwrap(), existing_pidfile);
}

#[test]
fn run_renders_hyphenated_input_ref_in_dot_syntax() {
    // The #117-4 scenario: a node config references `{{ inputs.supabase-url }}`
    // (kebab input name, dot syntax). Before the template normalizer this failed
    // at render ("tried to use - operator"); now it renders end-to-end.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");

    let agent_dir = aware.join("agents/stub-writer");
    std::fs::create_dir_all(&agent_dir).unwrap();
    std::fs::write(
        agent_dir.join("manifest.yaml"),
        r#"agent: stub-writer
version: 0.0.1
description: x
stateful: false
license: MIT
transport:
  cli:
    binary: this-write-binary-does-not-exist
commands:
  push:
    lifecycle: single
    category: curated
    mode: write
    description: push data
"#,
    )
    .unwrap();

    let app_dir = aware.join("apps/supabaseapp");
    std::fs::create_dir_all(&app_dir).unwrap();
    std::fs::write(
        app_dir.join("supabaseapp.flo"),
        r#"app: supabaseapp
version: 0.0.1
description: x
nodes:
  - id: push
    agent: stub-writer
    command: push
    config:
      url: '{{ inputs.supabase-url }}/rest/v1/drawings'
    safety:
      transaction-group: notify
      snapshot: false
connections: []
requires: []
"#,
    )
    .unwrap();

    // `--simulate` renders the write node's config (for the would-write event),
    // exercising the hyphenated dot-path. Must succeed.
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args([
            "app",
            "run",
            "supabaseapp",
            "--simulate",
            "--input",
            "supabase-url=https://demo.supabase.co",
        ])
        .assert()
        .success();
}

#[test]
fn run_nonexistent_app_exits_7() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["app", "run", "nope"])
        .assert()
        .failure()
        .code(7);
}

#[test]
fn run_nonexistent_app_prints_error() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["app", "run", "does-not-exist"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn long_running_app_writes_and_removes_pidfile() {
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");

    // Install a stateful agent stub.
    let agent_dir = aware.join("agents/stub-stateful");
    std::fs::create_dir_all(&agent_dir).unwrap();
    std::fs::write(
        agent_dir.join("manifest.yaml"),
        r#"agent: stub-stateful
version: 0.0.1
description: x
stateful: true
license: MIT
transport:
  cli:
    binary: this-binary-does-not-exist-stateful
commands:
  watch:
    lifecycle: start
    description: nope
"#,
    )
    .unwrap();

    // Install an app that uses the stateful stub.
    let app_dir = aware.join("apps/stubapp");
    std::fs::create_dir_all(&app_dir).unwrap();
    std::fs::write(
        app_dir.join("stubapp.flo"),
        r#"app: stubapp
version: 0.0.1
description: x
nodes:
  - id: w
    agent: stub-stateful
    command: watch
connections: []
requires: []
"#,
    )
    .unwrap();

    // Try to run — will fail because the binary doesn't exist;
    // we just verify the pidfile is cleaned up afterwards.
    let _ = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "run", "stubapp"])
        .timeout(std::time::Duration::from_secs(5))
        .output();

    // After exit, pidfile must not exist (cleaned up regardless of error).
    let pidfile = aware.join("apps/stubapp/instances/default/pidfile.yaml");
    assert!(!pidfile.exists(), "pidfile leaked: {}", pidfile.display());
}

/// End-to-end: the real `http` agent (from the repo) executed through the
/// actual `aware` binary against a local mock server. Proves the manifest's
/// verb-named commands wire through DispatchInvoker → RestInvoker.
#[test]
fn http_agent_runs_get_end_to_end() {
    use std::io::{Read, Write};
    use std::net::TcpListener;

    // Local one-shot HTTP server.
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let (tx, rx) = std::sync::mpsc::channel();
    let server = std::thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            stream
                .set_read_timeout(Some(std::time::Duration::from_millis(300)))
                .unwrap();
            let mut data = Vec::new();
            let mut tmp = [0u8; 1024];
            loop {
                match stream.read(&mut tmp) {
                    Ok(0) => break,
                    Ok(n) => data.extend_from_slice(&tmp[..n]),
                    Err(_) => break,
                }
            }
            let _ = tx.send(String::from_utf8_lossy(&data).to_string());
            let body = r#"{"ok":true}"#;
            let resp = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            let _ = stream.write_all(resp.as_bytes());
        }
    });

    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");

    // Install the real http agent from the repo into AWARE_HOME/agents/http.
    let repo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    copy_dir(
        &repo.join("20-agents/_core/http"),
        &aware.join("agents/http"),
    )
    .unwrap();

    // A one-shot app with a single read-mode GET node. The dynamic mock-server
    // port is passed via `--input` (which also proves #117-1: the run-input
    // override no longer collides with the global `--config` flag).
    let app_dir = aware.join("apps/http-smoke");
    std::fs::create_dir_all(&app_dir).unwrap();
    std::fs::write(
        app_dir.join("http-smoke.flo"),
        r#"app: http-smoke
version: 0.0.1
description: x
nodes:
  - id: ping
    agent: http
    command: get
    config:
      url: "{{ inputs.target }}"
connections: []
requires: []
"#,
    )
    .unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "run", "http-smoke", "--input"])
        .arg(format!("target=http://127.0.0.1:{port}/ping"))
        .assert()
        .success();

    let req = rx.recv().unwrap();
    assert!(req.starts_with("GET /ping"), "server got: {req}");
    server.join().unwrap();
}

/// #402: a CLI agent can materialize a large response in the run-owned
/// artifact directory. The actual `aware app run` trace must contain only the
/// descriptor, and the CLI must retrieve the opaque id without loading the
/// payload into the trace path.
#[test]
fn large_cli_output_is_an_artifact_not_a_trace_record() {
    let tmp = tempfile::tempdir().unwrap();
    let Some(bin) = compile_helper(
        tmp.path(),
        "artifact_writer",
        r#"use std::{env, fs, io::{Read, Write}, path::Path};
fn main() {
    let mut stdin = String::new();
    let _ = std::io::stdin().read_to_string(&mut stdin);
    let dir = env::var("AWARE_ARTIFACT_DIR").expect("AWARE_ARTIFACT_DIR missing");
    fs::create_dir_all(&dir).unwrap();
    let path = Path::new(&dir).join("read-model.json");
    let mut file = fs::File::create(&path).unwrap();
    file.write_all(b"{\"objects\":[\"").unwrap();
    file.write_all(&vec![b'x'; 512 * 1024]).unwrap();
    file.write_all(b"\"]}").unwrap();
    println!("{{\"$aware-artifact\":{{\"id\":\"read-model.json\",\"mediaType\":\"application/json\",\"bytes\":{},\"items\":1}}}}", fs::metadata(path).unwrap().len());
}
"#,
    ) else {
        eprintln!("[skip] rustc not found on PATH; cannot build artifact helper");
        return;
    };

    let aware = tmp.path().join("aware");
    let agent_dir = aware.join("agents/artifact-reader");
    std::fs::create_dir_all(&agent_dir).unwrap();
    let binary = bin.to_string_lossy().replace('\\', "/");
    std::fs::write(
        agent_dir.join("manifest.yaml"),
        format!(
            r#"agent: artifact-reader
version: 0.0.1
description: x
stateful: false
license: MIT
transport:
  cli:
    binary: {binary}
commands:
  read:
    lifecycle: single
    category: curated
    mode: read
    description: emits an artifact
    outputs:
      type: single
"#
        ),
    )
    .unwrap();
    let app_dir = aware.join("apps/artifactapp");
    std::fs::create_dir_all(&app_dir).unwrap();
    std::fs::write(
        app_dir.join("artifactapp.flo"),
        r#"app: artifactapp
version: 0.0.1
description: x
nodes:
  - id: read
    agent: artifact-reader
    command: read
connections: []
requires: []
"#,
    )
    .unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "run", "artifactapp"])
        .assert()
        .success();

    let trace = find_first_jsonl(&aware.join("logs/artifactapp")).expect("no trace");
    let trace_body = std::fs::read_to_string(&trace).unwrap();
    assert!(
        trace_body.contains("$aware-artifact"),
        "trace: {trace_body}"
    );
    assert!(
        trace_body.len() < 10_000,
        "large artifact leaked into trace"
    );

    let output = tmp.path().join("retrieved.json");
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args([
            "app",
            "artifact",
            "artifactapp",
            "read-model.json",
            "--output",
        ])
        .arg(&output)
        .assert()
        .success();
    let copied = std::fs::read_to_string(&output).unwrap();
    assert!(copied.len() > 512 * 1024, "artifact was not copied in full");
    assert!(serde_json::from_str::<serde_json::Value>(&copied).is_ok());
}

/// #405: a long one-shot command must be able to hand a consumer usable geometry BEFORE it
/// finishes. The claim under test is the one the issue makes — first render without waiting for the
/// full artifact — so this drives the real `aware app run` as a live child and fetches an announced
/// segment while that child is still working, which is the only way to tell a progressive delivery
/// from a fast one.
#[test]
fn a_segment_is_retrievable_while_the_run_is_still_in_flight() {
    let tmp = tempfile::tempdir().unwrap();
    // The helper announces its first segment immediately and then keeps working for ~1.4s, so a
    // retrieval that succeeds during that window could not have come from a completed run.
    let Some(bin) = compile_helper(
        tmp.path(),
        "progressive_writer",
        r#"use std::{env, fs, io::{Read, Write}, path::Path, thread, time::Duration};
fn main() {
    let mut stdin = String::new();
    let _ = std::io::stdin().read_to_string(&mut stdin);
    let dir = env::var("AWARE_ARTIFACT_DIR").expect("AWARE_ARTIFACT_DIR missing");
    let channel = env::var("AWARE_PROGRESS_FILE").expect("AWARE_PROGRESS_FILE missing");
    fs::create_dir_all(&dir).unwrap();
    let mut log = fs::OpenOptions::new().create(true).append(true).open(&channel).unwrap();
    writeln!(log, "{{\"$aware-progress\":{{\"phase\":\"parse\",\"message\":\"opening model\"}}}}").unwrap();
    log.flush().unwrap();
    for seq in 1..=2u32 {
        let name = format!("seg-{seq}.json");
        let tmp_path = Path::new(&dir).join(format!("{name}.tmp"));
        let body = format!("{{\"seq\":{seq},\"objects\":[\"object-{seq}\"]}}");
        fs::write(&tmp_path, &body).unwrap();
        // Announce only after the rename: a half-written segment must never be advertised.
        let final_path = Path::new(&dir).join(&name);
        fs::rename(&tmp_path, &final_path).unwrap();
        let bytes = fs::metadata(&final_path).unwrap().len();
        writeln!(log, "{{\"$aware-progress\":{{\"phase\":\"batch\",\"done\":{seq},\"total\":2,\"artifact\":{{\"id\":\"{name}\",\"mediaType\":\"application/json\",\"bytes\":{bytes},\"items\":1,\"seq\":{seq}}}}}}}").unwrap();
        log.flush().unwrap();
        thread::sleep(Duration::from_millis(700));
    }
    // Noise on the channel is skipped, never mirrored: a producer's stray line must not become a
    // trace record, and must not stop the records around it from being one.
    writeln!(log, "loading geometry 100%").unwrap();
    writeln!(log, "{{\"$aware-progress\":{{\"phase\":\"complete\",\"done\":2,\"total\":2}}}}").unwrap();
    log.flush().unwrap();
    fs::write(Path::new(&dir).join("full.json"), b"{\"objects\":[\"object-1\",\"object-2\"]}").unwrap();
    println!("{{\"$aware-artifact\":{{\"id\":\"full.json\",\"mediaType\":\"application/json\",\"bytes\":36,\"items\":2}}}}");
}
"#,
    ) else {
        eprintln!("[skip] rustc not found on PATH; cannot build progressive helper");
        return;
    };

    let aware = tmp.path().join("aware");
    let agent_dir = aware.join("agents/progressive-reader");
    std::fs::create_dir_all(&agent_dir).unwrap();
    let binary = bin.to_string_lossy().replace('\\', "/");
    std::fs::write(
        agent_dir.join("manifest.yaml"),
        format!(
            r#"agent: progressive-reader
version: 0.0.1
description: x
stateful: false
license: MIT
transport:
  cli:
    binary: {binary}
commands:
  read:
    lifecycle: single
    category: curated
    mode: read
    description: emits segments while it works
    outputs:
      type: single
"#
        ),
    )
    .unwrap();
    let app_dir = aware.join("apps/progressiveapp");
    std::fs::create_dir_all(&app_dir).unwrap();
    std::fs::write(
        app_dir.join("progressiveapp.flo"),
        r#"app: progressiveapp
version: 0.0.1
description: x
nodes:
  - id: read
    agent: progressive-reader
    command: read
connections: []
requires: []
"#,
    )
    .unwrap();

    let exe = assert_cmd::cargo::cargo_bin("aware");
    let mut run = std::process::Command::new(&exe)
        .env("AWARE_HOME", &aware)
        .args(["app", "run", "progressiveapp"])
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("spawn aware app run");

    // Wait for the first announced segment to appear in the trace — the same thing a consumer
    // tailing `aware app logs --tail` waits for.
    let logs = aware.join("logs/progressiveapp");
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    let mut announced = None;
    while std::time::Instant::now() < deadline {
        if let Some(trace) = find_first_jsonl(&logs)
            && let Ok(body) = std::fs::read_to_string(&trace)
        {
            for line in body.lines() {
                let Ok(event) = serde_json::from_str::<serde_json::Value>(line) else {
                    continue;
                };
                if event["kind"] == "node-progress"
                    && let Some(id) = event["data"]["artifact"]["id"].as_str()
                {
                    announced = Some(id.to_string());
                    break;
                }
            }
        }
        if announced.is_some() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
    let Some(segment) = announced else {
        let trace = find_first_jsonl(&logs)
            .and_then(|p| std::fs::read_to_string(p).ok())
            .unwrap_or_else(|| "<no trace written>".into());
        panic!("no segment was announced while the node ran; trace:\n{trace}");
    };
    assert!(
        run.try_wait().unwrap().is_none(),
        "the run had already finished — this proves nothing about progressive delivery"
    );

    // Fetch it through the ordinary artifact contract, with the run still going.
    let fetched = tmp.path().join("first-batch.json");
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .args(["app", "artifact", "progressiveapp", &segment, "--output"])
        .arg(&fetched)
        .assert()
        .success();
    let body: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&fetched).unwrap()).unwrap();
    assert_eq!(body["seq"], 1, "the first segment must be the first batch");

    assert!(run.wait().unwrap().success(), "run failed");

    let trace = find_first_jsonl(&logs).expect("no trace");
    let events: Vec<serde_json::Value> = std::fs::read_to_string(&trace)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let phases: Vec<&str> = events
        .iter()
        .filter(|e| e["kind"] == "node-progress")
        .filter_map(|e| e["data"]["phase"].as_str())
        .collect();
    assert_eq!(
        phases,
        vec!["parse", "batch", "batch", "complete"],
        "phases must arrive in the order the producer wrote them, with the noise line skipped"
    );

    // Ordering is the proof that the records were flushed DURING the call: `node-output` is written
    // only after the invocation returns, so a runtime that buffered progress would put them after.
    let progress_at = events
        .iter()
        .position(|e| e["kind"] == "node-progress")
        .unwrap();
    let output_at = events
        .iter()
        .position(|e| e["kind"] == "node-output")
        .unwrap();
    assert!(
        progress_at < output_at,
        "progress must reach the trace before the node's single output"
    );
}

fn copy_dir(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)?.flatten() {
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&from, &to)?;
        } else {
            std::fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

/// End-to-end: a real subprocess emits a finite newline-delimited JSON event
/// stream; the stateful `watch` source routes to the long-running path;
/// `CliInvoker::invoke_stream` pumps the events; and the run ends cleanly when the
/// stream closes — proving the streaming transport works through the real `aware`
/// binary (#172), not just a mock.
#[test]
fn streaming_watcher_app_runs_end_to_end_via_invoke_stream() {
    let tmp = tempfile::tempdir().unwrap();
    // A streamer that drains stdin (where the CLI delivers the rendered args)
    // then emits three JSONL events and exits 0.
    let Some(bin) = compile_helper(
        tmp.path(),
        "streamer",
        r#"use std::io::{Read, Write};
fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_to_string(&mut buf);
    let out = std::io::stdout();
    let mut o = out.lock();
    for m in ["A", "B", "C"] {
        let _ = writeln!(o, "{{\"mark\":\"{}\"}}", m);
        let _ = o.flush();
    }
}
"#,
    ) else {
        eprintln!("[skip] rustc not found on PATH; cannot build the streaming sidecar helper");
        return;
    };

    let aware = tmp.path().join("aware");
    install_watcher_app(&aware, &bin);

    // A real (non-simulated) run: the long-running path consumes the stream and
    // completes when the source closes.
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .timeout(std::time::Duration::from_secs(30))
        .args(["app", "run", "watchapp"])
        .assert()
        .success();

    // The streamed events landed in the provenance trace.
    let trace = find_first_jsonl(&aware.join("logs/watchapp")).expect("no jsonl trace written");
    let body = std::fs::read_to_string(&trace).unwrap();
    assert!(
        body.contains("\"mark\":\"A\""),
        "trace missing first event:\n{body}"
    );
    assert!(
        body.contains("\"mark\":\"C\""),
        "trace missing last event:\n{body}"
    );
}

/// End-to-end: a streaming source that exits non-zero must fail the run (exit
/// non-zero) and record a node error — a crashed watcher cannot be reported as a
/// successful run (#172).
#[test]
fn streaming_watcher_exit_failure_fails_the_run() {
    let tmp = tempfile::tempdir().unwrap();
    // Emit one event, complain on stderr, then exit non-zero.
    let Some(bin) = compile_helper(
        tmp.path(),
        "crasher",
        r#"use std::io::{Read, Write};
fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_to_string(&mut buf);
    let _ = writeln!(std::io::stdout(), "{{\"mark\":\"A\"}}");
    let _ = std::io::stdout().flush();
    let _ = writeln!(std::io::stderr(), "watcher: lost connection to host");
    std::process::exit(3);
}
"#,
    ) else {
        eprintln!("[skip] rustc not found on PATH; cannot build the streaming sidecar helper");
        return;
    };

    let aware = tmp.path().join("aware");
    install_watcher_app(&aware, &bin);

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .timeout(std::time::Duration::from_secs(30))
        .args(["app", "run", "watchapp"])
        .assert()
        .failure();

    // The trace records the source failure as a node error.
    let trace = find_first_jsonl(&aware.join("logs/watchapp")).expect("no jsonl trace written");
    let body = std::fs::read_to_string(&trace).unwrap();
    assert!(
        body.contains("node-error"),
        "trace missing node-error for the crashed watcher:\n{body}"
    );
}

/// Install a stateful `watch` agent (cli binary = `bin`, absolute path → spawned
/// directly, bypassing bridge resolution) and an app whose source node is it.
fn install_watcher_app(aware: &std::path::Path, bin: &std::path::Path) {
    let agent_dir = aware.join("agents/stub-watcher");
    std::fs::create_dir_all(&agent_dir).unwrap();
    let bin_yaml = bin.to_string_lossy().replace('\\', "/");
    std::fs::write(
        agent_dir.join("manifest.yaml"),
        format!(
            r#"agent: stub-watcher
version: 0.0.1
description: emits a finite stream of marks
stateful: true
license: MIT
transport:
  cli:
    binary: {bin_yaml}
commands:
  watch:
    lifecycle: start
    category: curated
    mode: read
    description: stream of marks
    outputs:
      type: stream
      schema:
        mark: string
"#
        ),
    )
    .unwrap();

    let app_dir = aware.join("apps/watchapp");
    std::fs::create_dir_all(&app_dir).unwrap();
    std::fs::write(
        app_dir.join("watchapp.flo"),
        r#"app: watchapp
version: 0.0.1
description: x
nodes:
  - id: watch
    agent: stub-watcher
    command: watch
connections: []
requires: []
"#,
    )
    .unwrap();
}

/// End-to-end through the REAL `aware-tekla` bridge (#176): the tekla agent's
/// `watch` command (`lifecycle: start`) routes to the long-running path;
/// `CliInvoker::invoke_stream` spawns `aware-tekla watch --json-stdin`; the
/// bridge's `watch` verb (in offline self-test mode, so no live Tekla is needed)
/// emits a `listening` signal then filtered `fired` JSONL events; and the run
/// ends cleanly when the synthetic stream closes. Proves the bridge half of the
/// event-trigger path that #173 (transport) and this issue (bridge verb)
/// together complete. Skips if dotnet/net48 is unavailable (Windows-only bridge),
/// mirroring how the rustc-helper streaming tests skip without rustc.
#[test]
fn tekla_watch_bridge_streams_events_end_to_end() {
    let Some(bridge) = build_tekla_bridge() else {
        eprintln!("[skip] dotnet/net48 unavailable; cannot build the aware-tekla bridge");
        return;
    };

    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");

    // Install a `tekla` agent whose cli transport points at the freshly built
    // bridge (absolute path → spawned directly). Only the `watch` command is
    // needed for this run.
    let agent_dir = aware.join("agents/tekla");
    std::fs::create_dir_all(&agent_dir).unwrap();
    let bin_yaml = bridge.to_string_lossy().replace('\\', "/");
    std::fs::write(
        agent_dir.join("manifest.yaml"),
        format!(
            r#"agent: tekla
version: 0.1.0
description: Tekla Structures bridge (test install)
stateful: true
license: Apache-2.0
transport:
  cli:
    binary: {bin_yaml}
commands:
  watch:
    lifecycle: start
    category: curated
    description: Subscribe to ModelObjectChanged events on the active Tekla model.
    inputs:
      filter:
        type: enum
        values: [all, welded, bolted, assembly]
        default: all
    outputs:
      type: stream
      schema:
        guid: string
        mark: string
        type: string
        change: string
"#
        ),
    )
    .unwrap();

    // A `tekla.watch` app. `self_test: true` in the node config flows through
    // render_config → the bridge's stdin JSON, driving the offline synthetic
    // stream (config passthrough preserves keys beyond declared inputs).
    let app_dir = aware.join("apps/watchapp");
    std::fs::create_dir_all(&app_dir).unwrap();
    std::fs::write(
        app_dir.join("watchapp.flo"),
        r#"app: watchapp
version: 0.0.1
description: x
nodes:
  - id: watch
    agent: tekla
    command: watch
    config:
      filter: welded
      self_test: true
connections: []
requires: []
"#,
    )
    .unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &aware)
        .timeout(std::time::Duration::from_secs(60))
        .args(["app", "run", "watchapp"])
        .assert()
        .success();

    // ≥1 event reached the provenance trace, and the `welded` filter was honored
    // (only the synthetic Weld change passes).
    let trace = find_first_jsonl(&aware.join("logs/watchapp")).expect("no jsonl trace written");
    let body = std::fs::read_to_string(&trace).unwrap();
    assert!(
        body.contains("\"type\":\"Weld\"") || body.contains("\\\"type\\\":\\\"Weld\\\""),
        "trace missing the filtered Weld event:\n{body}"
    );
}

/// Build the real `aware-tekla.exe` (net48) via `dotnet build`, returning its
/// path — or `None` if dotnet isn't on PATH or we're not on Windows (the bridge
/// targets net48, which only builds reliably on Windows). The caller skips when
/// `None`, mirroring `compile_helper`'s rustc gating.
fn build_tekla_bridge() -> Option<std::path::PathBuf> {
    if !cfg!(windows) {
        return None;
    }
    let dotnet = which_on_path("dotnet.exe").or_else(|| which_on_path("dotnet"))?;
    let repo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let csproj = repo.join("cli-tekla/cli-tekla.csproj");
    if !csproj.is_file() {
        return None;
    }
    let status = std::process::Command::new(&dotnet)
        .arg("build")
        .arg(&csproj)
        .args(["-c", "Debug", "--nologo", "-v", "quiet"])
        .status()
        .ok()?;
    if !status.success() {
        return None;
    }
    let exe = repo.join("cli-tekla/bin/Debug/net48/aware-tekla.exe");
    exe.is_file().then_some(exe)
}

/// Compile a single-file Rust helper into `dir`, returning its path — or `None`
/// if rustc isn't on PATH (the caller skips, mirroring the dotnet-sidecar test).
fn compile_helper(dir: &std::path::Path, name: &str, body: &str) -> Option<std::path::PathBuf> {
    let rustc = which_on_path(if cfg!(windows) { "rustc.exe" } else { "rustc" })?;
    let src = dir.join(format!("{name}.rs"));
    std::fs::write(&src, body).unwrap();
    let bin = dir.join(if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.to_string()
    });
    let status = std::process::Command::new(&rustc)
        .arg(&src)
        .arg("-o")
        .arg(&bin)
        .status()
        .expect("failed to invoke rustc");
    assert!(status.success(), "rustc failed to build the {name} helper");
    assert!(
        bin.is_file(),
        "{name} binary not produced at {}",
        bin.display()
    );
    Some(bin)
}

/// Minimal `which`: the first PATH entry containing `name`.
fn which_on_path(name: &str) -> Option<std::path::PathBuf> {
    std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths)
            .map(|p| p.join(name))
            .find(|p| p.is_file())
    })
}

/// First `*.jsonl` file under `dir` (searched recursively).
fn find_first_jsonl(dir: &std::path::Path) -> Option<std::path::PathBuf> {
    for entry in std::fs::read_dir(dir).ok()?.flatten() {
        let p = entry.path();
        // A run's artifact directory (`<run-id>.artifacts/`) sits beside its trace and may hold
        // JSONL of its own — the #405 progress channel is exactly that. It enumerates first, so a
        // blind recursion returns a producer's channel where the caller asked for the trace.
        if p.file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.ends_with(".artifacts"))
        {
            continue;
        }
        if p.is_dir() {
            if let Some(found) = find_first_jsonl(&p) {
                return Some(found);
            }
        } else if p.extension().and_then(|e| e.to_str()) == Some("jsonl") {
            return Some(p);
        }
    }
    None
}

#[test]
fn run_refuses_app_whose_agent_is_not_installed() {
    // #308: an app referencing an agent that does not exist installed + compiled
    // cleanly, then died at run with a bare `io: ... (os error 3)` naming neither
    // the app, the node, nor the agent. The pre-flight must name all three and
    // carry the remedy.
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("aware");
    let src = tmp.path().join("probe-src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(
        src.join("probe-unknown-agent.flo"),
        "app: probe-unknown-agent\nversion: 0.1.0\ndescription: |\n  probe\nrequires: []\n\
         nodes:\n  - id: n\n    agent: definitely-not-an-agent-xyz\n    command: render\n",
    )
    .unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &home)
        .args(["app", "install"])
        .arg(&src)
        .assert()
        .success()
        // Install still succeeds (an app may be installed before its agents, #170)
        // but must no longer be silent about the gap.
        .stderr(predicate::str::contains("W_APP_AGENT_NOT_INSTALLED"));

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &home)
        .args(["app", "run", "probe-unknown-agent"])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("E_APP_AGENT_NOT_INSTALLED")
                .and(predicate::str::contains("\"n\""))
                .and(predicate::str::contains("definitely-not-an-agent-xyz"))
                .and(predicate::str::contains(
                    "aware agent install definitely-not-an-agent-xyz",
                ))
                // The bare io error the issue reported must be gone.
                .and(predicate::str::contains("os error 3").not()),
        );
}

#[test]
fn simulate_still_runs_an_app_whose_agent_is_not_installed() {
    // #308 must not close the documented escape hatch: `--simulate` stubs every
    // node and contacts no binary, so it stays the way to check a composition
    // before its agents are installed.
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("aware");
    let src = tmp.path().join("probe-src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(
        src.join("probe-sim.flo"),
        "app: probe-sim\nversion: 0.1.0\ndescription: |\n  probe\nrequires: []\n\
         nodes:\n  - id: n\n    agent: definitely-not-an-agent-xyz\n    command: render\n    mode: read\n",
    )
    .unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &home)
        .args(["app", "install"])
        .arg(&src)
        .assert()
        .success();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &home)
        .args(["app", "run", "probe-sim", "--simulate"])
        .assert()
        .success();
}

#[test]
fn run_allows_a_frozen_node_whose_agent_is_not_installed() {
    // A frozen node emits its pinned output and never dispatches, so the #308
    // pre-flight must not require its agent to exist (app-spec § Frozen nodes).
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("aware");
    let src = tmp.path().join("frozen-src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(
        src.join("frozen-app.flo"),
        "app: frozen-app\nversion: 0.1.0\ndescription: |\n  frozen\nrequires: []\n\
         nodes:\n  - id: n\n    agent: definitely-not-an-agent-xyz\n    command: render\n\
         \x20   frozen:\n      ok: true\n",
    )
    .unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &home)
        .args(["app", "install"])
        .arg(&src)
        .assert()
        .success();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", &home)
        .args(["app", "run", "frozen-app"])
        .assert()
        .success();
}
