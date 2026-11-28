use std::{
    process::Command,
    time::{Duration, Instant},
};

use super::supervision::{WorkerError, supervise};

fn js(source: &str) -> Command {
    let output = Command::new("where.exe").arg("node.exe").output().unwrap();
    assert!(
        output.status.success(),
        "Node 24.14.0 must be on PATH for supervisor tests"
    );
    let path = String::from_utf8(output.stdout).unwrap();
    let mut command = Command::new(path.lines().next().unwrap());
    command.args(["-e", source]);
    command
}

#[tokio::test]
async fn ordinary_child_exit() {
    let worker = js("process.stdin.pipe(process.stdout)");
    let output = supervise(worker, b"reply", Duration::from_secs(5))
        .await
        .unwrap();
    assert_eq!(output, b"reply");
}

async fn deadline(source: &str, request: &[u8]) {
    let start = Instant::now();
    let result = supervise(js(source), request, Duration::from_millis(700)).await;
    assert_eq!(result, Err(WorkerError::Timeout));
    assert!(start.elapsed() < Duration::from_secs(5));
}

#[tokio::test]
async fn blocked_stdin() {
    deadline("setInterval(() => {}, 1000)", &vec![0; 1024 * 1024]).await;
}

#[tokio::test]
async fn held_open_pipe() {
    let directory = tempfile::tempdir().unwrap();
    let ready = directory.path().join("descendant-ready");
    let mut worker = js(r#"
const fs = require('node:fs');
const ready = process.argv[1];
// Detached skips libuv's parent-owned job. Windows still inherits the supervisor's job.
const child = require('node:child_process').spawn(process.execPath, [
  '-e', "process.stdout.write('held', () => require('node:fs').writeFileSync(process.argv[1], 'ready')); setInterval(() => {}, 1000)",
  ready,
], {detached: true, windowsHide: true, stdio: ['ignore', 'inherit', 'inherit']});
child.on('error', () => process.exit(6));
setInterval(() => { if (fs.existsSync(ready)) process.exit(0); }, 10);
"#);
    worker.arg(&ready);
    let start = Instant::now();
    assert_eq!(
        supervise(worker, b"", Duration::from_secs(2)).await,
        Err(WorkerError::Timeout)
    );
    assert_eq!(std::fs::read(ready).unwrap(), b"ready");
    assert!(start.elapsed() < Duration::from_secs(6));
}

#[tokio::test]
async fn oversized_stdout() {
    let worker =
        js("process.stdout.write(Buffer.alloc(32 * 1024 * 1024 + 1)); setInterval(() => {}, 1000)");
    assert_eq!(
        supervise(worker, b"", Duration::from_secs(5)).await,
        Err(WorkerError::OutputLimit)
    );
}

#[tokio::test]
async fn oversized_stderr() {
    let worker =
        js("process.stderr.write(Buffer.alloc(64 * 1024 + 1)); setInterval(() => {}, 1000)");
    assert_eq!(
        supervise(worker, b"", Duration::from_secs(5)).await,
        Err(WorkerError::OutputLimit)
    );
}

#[tokio::test]
async fn nonzero_child_after_valid_reply() {
    let worker = js("process.stdout.write('{}', () => process.exit(7));");
    assert_eq!(
        supervise(worker, b"", Duration::from_secs(5)).await,
        Err(WorkerError::Exit)
    );
}
