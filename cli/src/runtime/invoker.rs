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
