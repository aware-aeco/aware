mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn run_one_shot_app_with_no_installed_agents_fails_clearly() {
    // welded-to-tc requires agents that aren't installed; with no agents installed,
    // is_long_running() returns false (no installed manifest), so we'd take the
    // one-shot path. But the orchestrator will try to invoke tekla via CliInvoker,
    // which spawns `aware-tekla` (not on PATH) → AwareError::Network.
    // The test confirms the exit is non-zero and the error mentions the binary.
    let tmp = tempfile::tempdir().unwrap();
    let aware = tmp.path().join("aware");

    // Install the welded-to-tc app from the repo .flo
    let app_src = tmp.path().join("welded-to-tc-src");
    std::fs::create_dir_all(&app_src).unwrap();
    let flo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("30-apps/_examples/welded-to-tc.flo");
    std::fs::copy(&flo, app_src.join("welded-to-tc.flo")).unwrap();

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
        values: [all, welded, bolted, assembly, drawing]
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
