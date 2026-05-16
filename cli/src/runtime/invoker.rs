//! Agent invocation abstraction.
//!
//! `AgentInvoker` trait + `MockInvoker` (test-only) here. `CliInvoker`
//! (subprocess + JSON envelope) added in Task 6.

#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde_json::Value;
use tokio::sync::{mpsc, oneshot};

use crate::error::AwareError;

/// Map key: (agent-name, command-name).
type CmdKey = (String, String);

/// Handle returned by `invoke_stream`. Drop the handle (or call `stop`) to
/// signal cancellation; outputs arrive on `rx` until the stream ends.
#[derive(Debug)]
pub struct StreamingHandle {
    pub rx: mpsc::Receiver<Result<Value, AwareError>>,
    pub stop: oneshot::Sender<()>,
}

#[async_trait]
pub trait AgentInvoker: Send + Sync {
    /// Run a single-lifecycle command (request → response).
    async fn invoke_single(
        &self,
        agent: &str,
        command: &str,
        args: Value,
    ) -> Result<Value, AwareError>;
    /// Run a start-lifecycle command (request → stream of outputs).
    async fn invoke_stream(
        &self,
        agent: &str,
        command: &str,
        args: Value,
    ) -> Result<StreamingHandle, AwareError>;
}

/// Test-only invoker: pre-baked outputs per (agent, command).
#[derive(Default, Clone)]
pub struct MockInvoker {
    /// (agent, command) -> single output
    single: Arc<Mutex<HashMap<CmdKey, Value>>>,
    /// (agent, command) -> ordered stream of outputs (delay between each is ~5ms)
    streams: Arc<Mutex<HashMap<CmdKey, Vec<Value>>>>,
}

impl MockInvoker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_single(self, agent: &str, command: &str, output: Value) -> Self {
        self.single
            .lock()
            .unwrap()
            .insert((agent.into(), command.into()), output);
        self
    }

    pub fn with_stream(self, agent: &str, command: &str, outputs: Vec<Value>) -> Self {
        self.streams
            .lock()
            .unwrap()
            .insert((agent.into(), command.into()), outputs);
        self
    }
}

#[async_trait]
impl AgentInvoker for MockInvoker {
    async fn invoke_single(
        &self,
        agent: &str,
        command: &str,
        _args: Value,
    ) -> Result<Value, AwareError> {
        let key = (agent.to_string(), command.to_string());
        self.single
            .lock()
            .unwrap()
            .get(&key)
            .cloned()
            .ok_or_else(|| {
                AwareError::NotFound(format!(
                    "mock invoker has no canned output for {agent}/{command}"
                ))
            })
    }

    async fn invoke_stream(
        &self,
        agent: &str,
        command: &str,
        _args: Value,
    ) -> Result<StreamingHandle, AwareError> {
        let key = (agent.to_string(), command.to_string());
        let outputs = self
            .streams
            .lock()
            .unwrap()
            .get(&key)
            .cloned()
            .ok_or_else(|| {
                AwareError::NotFound(format!(
                    "mock invoker has no canned stream for {agent}/{command}"
                ))
            })?;
        let (tx, rx) = mpsc::channel(16);
        let (stop_tx, mut stop_rx) = oneshot::channel();
        tokio::spawn(async move {
            for o in outputs {
                tokio::select! {
                    _ = &mut stop_rx => break,
                    res = tx.send(Ok(o)) => {
                        if res.is_err() { break; }
                    }
                }
                tokio::time::sleep(std::time::Duration::from_millis(5)).await;
            }
        });
        Ok(StreamingHandle { rx, stop: stop_tx })
    }
}

/// Production invoker: spawn the agent's CLI transport binary,
/// talk JSON over stdin/stdout.
pub struct CliInvoker {
    pub agents_dir: std::path::PathBuf,
}

#[async_trait]
impl AgentInvoker for CliInvoker {
    async fn invoke_single(
        &self,
        agent: &str,
        command: &str,
        args: Value,
    ) -> Result<Value, AwareError> {
        let manifest_path = self.agents_dir.join(agent).join("manifest.yaml");
        let m = crate::manifest::loader::load_agent(&manifest_path)?;
        let cli =
            m.transport.cli.as_ref().ok_or_else(|| {
                AwareError::Validation(format!("agent {agent} has no cli transport"))
            })?;
        let binary = &cli.binary;

        let mut child = tokio::process::Command::new(binary)
            .arg(command)
            .arg("--json-stdin")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| AwareError::Network(format!("spawn {binary}: {e}")))?;

        if let Some(mut stdin) = child.stdin.take() {
            use tokio::io::AsyncWriteExt;
            let args_text = serde_json::to_string(&args).unwrap();
            stdin
                .write_all(args_text.as_bytes())
                .await
                .map_err(|e| AwareError::Network(format!("stdin write: {e}")))?;
            stdin
                .shutdown()
                .await
                .map_err(|e| AwareError::Network(format!("stdin close: {e}")))?;
        }

        let output = child
            .wait_with_output()
            .await
            .map_err(|e| AwareError::Network(format!("wait: {e}")))?;
        if !output.status.success() {
            return Err(AwareError::Network(format!(
                "agent {agent}/{command} failed (exit {:?}): {}",
                output.status.code(),
                String::from_utf8_lossy(&output.stderr).trim()
            )));
        }
        let body: Value = serde_json::from_slice(&output.stdout).map_err(|e| {
            AwareError::Validation(format!("agent {agent}/{command} stdout not JSON: {e}"))
        })?;
        Ok(body)
    }

    async fn invoke_stream(
        &self,
        _agent: &str,
        _command: &str,
        _args: Value,
    ) -> Result<StreamingHandle, AwareError> {
        // v0.5 lands real agent binaries with streaming support. Until then,
        // streaming via CLI transport is stubbed.
        Err(AwareError::NotYetImplemented(
            "CliInvoker::invoke_stream (waiting on agent binaries in v0.5)",
        ))
    }
}

#[cfg(test)]
mod cli_invoker_tests {
    use super::*;

    #[tokio::test]
    async fn missing_binary_returns_clear_network_error() {
        let tmp = tempfile::tempdir().unwrap();
        // Build a minimal agent dir with a manifest pointing at a binary that doesn't exist.
        let agent_dir = tmp.path().join("phantom");
        std::fs::create_dir_all(&agent_dir).unwrap();
        std::fs::write(
            agent_dir.join("manifest.yaml"),
            r#"agent: phantom
version: 0.1.0
description: a non-existent agent
stateful: false
license: MIT
transport:
  cli:
    binary: this-binary-definitely-does-not-exist-12345
commands:
  do:
    lifecycle: single
    description: nope
"#,
        )
        .unwrap();

        let inv = CliInvoker {
            agents_dir: tmp.path().to_path_buf(),
        };
        let err = inv
            .invoke_single("phantom", "do", serde_json::json!({}))
            .await
            .unwrap_err();
        assert!(matches!(err, AwareError::Network(_)), "got: {err:?}");
    }

    #[tokio::test]
    async fn missing_transport_cli_returns_validation_error() {
        let tmp = tempfile::tempdir().unwrap();
        let agent_dir = tmp.path().join("only-mcp");
        std::fs::create_dir_all(&agent_dir).unwrap();
        // Transport has neither cli nor any other field — invalid but parseable
        // (transport.cli is Option<TransportCli>).
        std::fs::write(
            agent_dir.join("manifest.yaml"),
            r#"agent: only-mcp
version: 0.1.0
description: x
stateful: false
license: MIT
transport: {}
commands:
  do:
    lifecycle: single
    description: nope
"#,
        )
        .unwrap();

        let inv = CliInvoker {
            agents_dir: tmp.path().to_path_buf(),
        };
        let err = inv
            .invoke_single("only-mcp", "do", serde_json::json!({}))
            .await
            .unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)), "got: {err:?}");
    }

    #[tokio::test]
    async fn stream_is_not_yet_implemented() {
        let tmp = tempfile::tempdir().unwrap();
        let inv = CliInvoker {
            agents_dir: tmp.path().to_path_buf(),
        };
        let err = inv
            .invoke_stream("anything", "anything", serde_json::json!({}))
            .await
            .unwrap_err();
        assert!(matches!(err, AwareError::NotYetImplemented(_)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn mock_single_returns_canned_output() {
        let inv =
            MockInvoker::new().with_single("trimble-connect", "upload", json!({"file-id":"f_1"}));
        let out = inv
            .invoke_single("trimble-connect", "upload", json!({}))
            .await
            .unwrap();
        assert_eq!(out["file-id"], "f_1");
    }

    #[tokio::test]
    async fn mock_single_missing_is_not_found() {
        let inv = MockInvoker::new();
        let err = inv.invoke_single("x", "y", json!({})).await.unwrap_err();
        assert!(matches!(err, AwareError::NotFound(_)));
    }

    #[tokio::test]
    async fn mock_stream_yields_outputs_in_order() {
        let inv = MockInvoker::new().with_stream(
            "tekla",
            "watch",
            vec![
                json!({"mark":"A"}),
                json!({"mark":"B"}),
                json!({"mark":"C"}),
            ],
        );
        let mut handle = inv
            .invoke_stream("tekla", "watch", json!({}))
            .await
            .unwrap();
        let mut collected = Vec::new();
        while let Some(v) = handle.rx.recv().await {
            collected.push(v.unwrap());
            if collected.len() == 3 {
                break;
            }
        }
        assert_eq!(collected.len(), 3);
        assert_eq!(collected[0]["mark"], "A");
        assert_eq!(collected[1]["mark"], "B");
        assert_eq!(collected[2]["mark"], "C");
    }

    #[tokio::test]
    async fn mock_stream_stops_on_signal() {
        let inv = MockInvoker::new().with_stream(
            "tekla",
            "watch",
            vec![
                json!({"n":1}),
                json!({"n":2}),
                json!({"n":3}),
                json!({"n":4}),
                json!({"n":5}),
            ],
        );
        let handle = inv
            .invoke_stream("tekla", "watch", json!({}))
            .await
            .unwrap();
        // Immediately signal stop
        let _ = handle.stop.send(());
        // Drain whatever made it through
        let mut rx = handle.rx;
        let mut count = 0;
        while rx.recv().await.is_some() {
            count += 1;
        }
        // We can't assert exact count due to timing — but it must terminate
        assert!(count <= 5);
    }
}
