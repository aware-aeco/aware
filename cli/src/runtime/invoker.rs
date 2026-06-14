//! Agent invocation abstraction.
//!
//! `AgentInvoker` trait + `MockInvoker` (test-only) here. `CliInvoker`
//! (subprocess + JSON envelope) added in Task 6.

#![allow(dead_code)]

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use serde_json::Value;
use tokio::sync::{mpsc, oneshot};

use crate::error::AwareError;
use crate::manifest::App;

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

/// Resolve an agent's `transport.cli.binary` to the path to spawn.
///
/// Known host bridges live in `~/.aware/bridges/` — a persistent directory that
/// is NOT on PATH (#148), so they must be resolved to an absolute path. Any other
/// name (a real PATH command, or a legacy bridge still sitting next to `aware`)
/// falls through to the bare name, which the OS resolves via PATH.
fn resolve_cli_binary(binary: &str, bridges_dir: &std::path::Path) -> std::path::PathBuf {
    crate::commands::sidecar::find_bridge_by_binary(binary, bridges_dir)
        .unwrap_or_else(|| std::path::PathBuf::from(binary))
}

/// The persistent bridges directory (`<AWARE_HOME>/bridges`), derived from the
/// same env-driven source the rest of the CLI uses.
fn bridges_dir() -> std::path::PathBuf {
    crate::paths::Paths::from_env()
        .map(|p| p.aware_home.join("bridges"))
        .unwrap_or_default()
}

/// Production invoker: spawn the agent's CLI transport binary,
/// talk JSON over stdin/stdout.
pub struct CliInvoker {
    pub agents_dir: std::path::PathBuf,
}

impl CliInvoker {
    /// Spawn the agent's CLI transport binary for `command`, with stdin / stdout
    /// / stderr piped. Shared by `invoke_single` (request → one JSON response)
    /// and `invoke_stream` (request → newline-delimited JSON event stream).
    /// Returns the spawned child plus the resolved binary name (for labelling
    /// errors raised after the spawn).
    fn spawn_cli(
        &self,
        agent: &str,
        command: &str,
    ) -> Result<(tokio::process::Child, String), AwareError> {
        let manifest_path = self.agents_dir.join(agent).join("manifest.yaml");
        let m = crate::manifest::loader::load_agent(&manifest_path)?;
        let cli =
            m.transport.cli.as_ref().ok_or_else(|| {
                AwareError::Validation(format!("agent {agent} has no cli transport"))
            })?;
        let binary = cli.binary.clone();
        // Host bridges live in ~/.aware/bridges (off PATH); resolve to an absolute
        // path. Non-bridge binaries fall back to bare-name PATH resolution.
        let bridges = bridges_dir();
        let program = resolve_cli_binary(&binary, &bridges);
        // A managed bridge installed by a different CLI version may speak an older
        // protocol. We warn (rather than refuse) so compatible bridges keep working
        // and a CLI patch bump doesn't force a redownload; a truly incompatible
        // bridge will fail the run with a clear protocol error.
        if crate::commands::sidecar::managed_bridge_is_stale(
            &binary,
            &bridges,
            env!("CARGO_PKG_VERSION"),
        ) {
            let id = binary.strip_prefix("aware-").unwrap_or(&binary);
            eprintln!(
                "aware: warning: {binary} was installed by a different aware version; \
                 run `aware sidecar install {id}` to refresh if this run fails"
            );
        }

        let child = tokio::process::Command::new(&program)
            .arg(command)
            .arg("--json-stdin")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| {
                // When a host bridge binary is missing, surface an actionable hint.
                let hint = if e.kind() == std::io::ErrorKind::NotFound {
                    // Strip the exe suffix on Windows to get the bare binary name.
                    let bare = binary.strip_suffix(".exe").unwrap_or(&binary);
                    if let Some(id) = bare.strip_prefix("aware-") {
                        format!(
                            "\n  hint: run `aware sidecar install {id}` to download the bridge binary"
                        )
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };
                AwareError::Network(format!("spawn {binary}: {e}{hint}"))
            })?;
        Ok((child, binary))
    }
}

#[async_trait]
impl AgentInvoker for CliInvoker {
    async fn invoke_single(
        &self,
        agent: &str,
        command: &str,
        args: Value,
    ) -> Result<Value, AwareError> {
        let (mut child, _binary) = self.spawn_cli(agent, command)?;

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
        agent: &str,
        command: &str,
        args: Value,
    ) -> Result<StreamingHandle, AwareError> {
        let (mut child, binary) = self.spawn_cli(agent, command)?;
        let label = format!("{agent}/{command}");

        // Deliver the rendered command args once over stdin, then close it — on a
        // separate task so startup never blocks. A streaming source reads its
        // config (e.g. a `filter`) here, then emits events until stopped; but a
        // watcher that never reads stdin is also valid, and a large rendered
        // config could otherwise fill the stdin pipe and deadlock `write_all`
        // before the stdout pump is even running. Delivery is best-effort — the
        // event stream itself is the source of truth.
        if let Some(mut stdin) = child.stdin.take() {
            let args_text = serde_json::to_string(&args).unwrap_or_else(|_| "{}".into());
            let label_in = label.clone();
            tokio::spawn(async move {
                use tokio::io::AsyncWriteExt;
                if let Err(e) = stdin.write_all(args_text.as_bytes()).await {
                    eprintln!("aware: warning: {label_in} stdin write failed: {e}");
                }
                let _ = stdin.shutdown().await;
            });
        }

        let stdout = child.stdout.take().ok_or_else(|| {
            AwareError::Internal(format!("{binary}/{command}: child stdout unavailable"))
        })?;

        // Drain stderr concurrently so a chatty child can't fill the OS pipe
        // buffer and deadlock — blocked on a stderr write, it would stop
        // producing stdout and the run would hang. Only the tail is retained
        // (bounded, so an hours-long watcher can't grow memory without bound);
        // it labels a non-zero exit.
        let stderr = child.stderr.take();
        let stderr_drain = tokio::spawn(async move {
            // Keep only the last few KiB — enough to explain a crash.
            const TAIL_CAP: usize = 8 * 1024;
            let mut tail: Vec<u8> = Vec::new();
            if let Some(mut err) = stderr {
                use tokio::io::AsyncReadExt;
                let mut chunk = [0u8; 4096];
                loop {
                    match err.read(&mut chunk).await {
                        Ok(0) | Err(_) => break,
                        Ok(n) => {
                            tail.extend_from_slice(&chunk[..n]);
                            if tail.len() > TAIL_CAP {
                                tail.drain(..tail.len() - TAIL_CAP);
                            }
                        }
                    }
                }
            }
            String::from_utf8_lossy(&tail).into_owned()
        });

        let (tx, rx) = mpsc::channel::<Result<Value, AwareError>>(64);
        let (stop_tx, stop_rx) = oneshot::channel::<()>();

        // Drive the child's stdout on a background task. The returned handle's
        // `stop` cancels it; events arrive on `rx` until the stream ends.
        tokio::spawn(async move {
            let mut child = child;
            match pump_jsonl_stream(stdout, &tx, stop_rx).await {
                // Cancellation or a downstream drop / read error: terminate the
                // child so a long-running watcher doesn't outlive the run.
                StreamEnd::Stopped | StreamEnd::ReceiverGone | StreamEnd::ReadError => {
                    let _ = child.start_kill();
                    let _ = child.wait().await;
                    stderr_drain.abort();
                }
                // Natural EOF: reap, and surface a non-zero exit as a terminal
                // stream error so the run doesn't report success for a watcher
                // that crashed — `run_long_running` forwards this as a node error
                // and fails the run.
                StreamEnd::Eof => {
                    let status = child.wait().await;
                    let detail = stderr_drain.await.unwrap_or_default();
                    if let Ok(status) = status
                        && !status.success()
                    {
                        let _ = tx
                            .send(Err(AwareError::Network(format!(
                                "{label} exited {:?}: {}",
                                status.code(),
                                detail.trim()
                            ))))
                            .await;
                    }
                }
            }
        });

        Ok(StreamingHandle { rx, stop: stop_tx })
    }
}

/// How a streaming pump terminated — drives the spawner's cleanup decision.
#[derive(Debug, PartialEq, Eq)]
enum StreamEnd {
    /// `stop` fired — caller should kill the child.
    Stopped,
    /// The child closed stdout — caller should reap and check the exit code.
    Eof,
    /// The receiver was dropped (no one is consuming) — caller should kill.
    ReceiverGone,
    /// A read error on stdout — caller should kill.
    ReadError,
}

/// Read newline-delimited JSON from `reader`, forwarding each non-blank line as
/// an event (`Ok(Value)`, or `Err` for a malformed line) into `tx`, until EOF or
/// `stop` fires. Each stdout line is one event — the streaming analogue of the
/// single-shot transport's one-JSON-document-per-call contract.
async fn pump_jsonl_stream<R>(
    reader: R,
    tx: &mpsc::Sender<Result<Value, AwareError>>,
    mut stop: oneshot::Receiver<()>,
) -> StreamEnd
where
    R: tokio::io::AsyncRead + Unpin,
{
    use tokio::io::{AsyncBufReadExt, BufReader};
    let mut lines = BufReader::new(reader).lines();
    loop {
        tokio::select! {
            _ = &mut stop => return StreamEnd::Stopped,
            next = lines.next_line() => match next {
                Ok(Some(line)) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }
                    let event = serde_json::from_str::<Value>(trimmed).map_err(|e| {
                        AwareError::Validation(format!("stream line not JSON: {e}"))
                    });
                    if tx.send(event).await.is_err() {
                        return StreamEnd::ReceiverGone;
                    }
                }
                Ok(None) => return StreamEnd::Eof,
                Err(e) => {
                    let _ = tx
                        .send(Err(AwareError::Network(format!("stream read: {e}"))))
                        .await;
                    return StreamEnd::ReadError;
                }
            }
        }
    }
}

/// Production invoker for the `rest` transport: build an HTTP request from the
/// rendered command inputs and execute it via the bundled `ureq` client.
///
/// Handles two command shapes: the generic `http` agent, where the command name
/// *is* the HTTP method (`get`/`post`/…) and the inputs carry
/// `url` / `headers` / `query` / `body`; and built OpenAPI agents (#106), where
/// the command maps to a manifest `method` + `path` template and inputs are
/// routed to path / query / header / body by their declared `in:` location.
/// App-templated secrets
/// (`apikey: "{{ secrets.supabase }}"`) arrive already substituted in the
/// inputs. In addition, if the agent's manifest declares an `auth:` block, this
/// invoker loads the referenced credential and injects it automatically
/// (bearer/oauth2 → `Authorization: Bearer`; api-key → header/query), without
/// the app author hand-templating it — fill-if-absent, so explicit input wins
/// (#106).
pub struct RestInvoker {
    pub agents_dir: std::path::PathBuf,
}

#[async_trait]
impl AgentInvoker for RestInvoker {
    async fn invoke_single(
        &self,
        agent: &str,
        command: &str,
        args: Value,
    ) -> Result<Value, AwareError> {
        // Trimble Connect file ops are multi-step, binary, cross-domain flows the
        // single-call REST path can't express, so they're handled out-of-line (#200).
        if agent == "trimble-connect" {
            match command {
                "upload" => {
                    return crate::runtime::trimble_files::upload(self.agents_dir.clone(), args)
                        .await;
                }
                "download" => {
                    return crate::runtime::trimble_files::download(self.agents_dir.clone(), args)
                        .await;
                }
                _ => {}
            }
        }
        // Two shapes reach this transport:
        //  - built OpenAPI agent (#106): the command maps to a manifest `method`
        //    + `path` template, and inputs are routed by their declared `in:`
        //    location (path / query / header / body).
        //  - generic `http` agent: the command IS the HTTP method, and inputs
        //    carry `url` / `headers` / `query` / `body`.
        // A manifest mapping is preferred, so an operationId that kebab-cases to
        // an HTTP verb still routes to its mapped operation (Codex #106).
        let (method, url, mut headers, mut query, body) = if command_method(
            &self.agents_dir,
            agent,
            command,
        )
        .is_some()
        {
            build_operation_request(&self.agents_dir, agent, command, &args)?
        } else {
            let upper = command.to_ascii_uppercase();
            if !matches!(
                upper.as_str(),
                "GET" | "POST" | "PUT" | "PATCH" | "DELETE" | "HEAD"
            ) {
                return Err(AwareError::Validation(format!(
                    "rest transport: command {command:?} is not an HTTP method and has no manifest `method:` mapping"
                )));
            }
            let raw_url = args.get("url").and_then(|v| v.as_str()).ok_or_else(|| {
                AwareError::Validation(format!("{agent}/{command}: missing required input `url`"))
            })?;
            let url = resolve_url(rest_base_url(&self.agents_dir, agent).as_deref(), raw_url);
            (
                upper,
                url,
                collect_str_map(args.get("headers")),
                collect_str_map(args.get("query")),
                args.get("body").cloned(),
            )
        };
        // Inject declarative auth (#106): if the agent's manifest carries an
        // `auth:` block, load its credential and add it — bearer/oauth2 as
        // `Authorization: Bearer …`, api-key as a header / query / cookie. Only
        // fills when the caller hasn't already set that slot, so an explicit
        // `{{ secrets.x }}` in the app still wins. A declared-but-unprovisioned
        // credential is a fail-fast error (not a silent unauthenticated call).
        // Load the manifest once for the auth block + this command's `no-auth`
        // (public-endpoint) flag, which opts a command out of agent auth.
        if let Ok(m) =
            crate::manifest::loader::load_agent(&self.agents_dir.join(agent).join("manifest.yaml"))
        {
            let is_public = m.commands.get(command).is_some_and(|c| c.no_auth);
            if let Some(auth) = &m.auth
                && !is_public
            {
                // Credential resolution can refresh an OAuth token (keychain I/O +
                // a blocking token-endpoint POST), so run it off the async reactor —
                // same posture as the REST request below (#198 Codex).
                let auth_owned = auth.clone();
                let dir = self.agents_dir.clone();
                let cred =
                    tokio::task::spawn_blocking(move || resolve_rest_credential(&dir, &auth_owned))
                        .await
                        .map_err(|e| {
                            AwareError::Internal(format!("credential resolve task join: {e}"))
                        })?;
                let Some(cred) = cred else {
                    return Err(AwareError::Validation(format!(
                        "agent {agent} declares `auth` but credential {:?} is missing or unusable — \
                         provision it (`~/.aware/credentials/{}.json` or `aware connect`)",
                        auth.secret, auth.secret
                    )));
                };
                inject_auth(auth, &cred, &mut headers, &mut query);
            }
        }
        let send_body = matches!(method.as_str(), "POST" | "PUT" | "PATCH" | "DELETE")
            && body.as_ref().is_some_and(|b| !b.is_null());
        // Owned label for the error path — `agent`/`command` are borrows that
        // can't escape into the `'static` blocking closure.
        let label = format!("{agent}/{command}");

        // `ureq` is blocking; run it off the async runtime so we don't stall
        // the orchestrator's reactor.
        tokio::task::spawn_blocking(move || -> Result<Value, AwareError> {
            let mut req = ureq::request(&method, &url);
            for (k, v) in &headers {
                req = req.set(k, v);
            }
            for (k, v) in &query {
                req = req.query(k, v);
            }
            let outcome = if send_body {
                // `ureq` is built without the `json` feature, so serialize here.
                // Objects/arrays are sent as application/json (unless the caller
                // set their own Content-Type); a string body is sent verbatim.
                let has_ct = headers
                    .iter()
                    .any(|(k, _)| k.eq_ignore_ascii_case("content-type"));
                let payload = match body.unwrap() {
                    Value::String(s) => s,
                    other => {
                        if !has_ct {
                            req = req.set("Content-Type", "application/json");
                        }
                        other.to_string()
                    }
                };
                req.send_string(&payload)
            } else {
                req.call()
            };
            let response = match outcome {
                Ok(r) => r,
                // 4xx/5xx is a *completed* HTTP exchange — return it as a normal
                // response object (with its status) so the app can branch on it,
                // rather than aborting the run.
                Err(ureq::Error::Status(_code, r)) => r,
                Err(ureq::Error::Transport(t)) => {
                    return Err(AwareError::Network(format!("{label}: {t}")));
                }
            };
            Ok(shape_response(response))
        })
        .await
        .map_err(|e| AwareError::Internal(format!("rest task join: {e}")))?
    }

    async fn invoke_stream(
        &self,
        _agent: &str,
        _command: &str,
        _args: Value,
    ) -> Result<StreamingHandle, AwareError> {
        Err(AwareError::Validation(
            "rest transport is request/response only and does not support streaming".into(),
        ))
    }
}

/// Read an optional `base` URL from an agent's `rest` transport block. The
/// generic `http` agent declares no base (callers pass absolute URLs); a
/// domain-specific rest agent may set one and pass relative paths.
pub(crate) fn rest_base_url(agents_dir: &std::path::Path, agent: &str) -> Option<String> {
    let m =
        crate::manifest::loader::load_agent(&agents_dir.join(agent).join("manifest.yaml")).ok()?;
    m.transport
        .rest
        .as_ref()?
        .get("base")?
        .as_str()
        .map(|s| s.to_string())
}

/// The HTTP method a manifest command maps to (a built OpenAPI operation), if
/// any. Used to prefer the operation executor over the generic url-based path.
fn command_method(agents_dir: &std::path::Path, agent: &str, command: &str) -> Option<String> {
    crate::manifest::loader::load_agent(&agents_dir.join(agent).join("manifest.yaml"))
        .ok()?
        .commands
        .get(command)?
        .method
        .clone()
}

/// Percent-encode an OpenAPI path-parameter value (RFC 3986 unreserved set):
/// everything except `A-Z a-z 0-9 - . _ ~` is `%`-escaped, so a value like
/// `a/b.txt` becomes `a%2Fb.txt` rather than splitting the route (#106).
pub(crate) fn percent_encode_path(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// Join an optional base URL with a (possibly relative) path. An absolute
/// `http(s)://` path is used as-is.
fn resolve_url(base: Option<&str>, raw: &str) -> String {
    match base {
        Some(b) if !raw.starts_with("http://") && !raw.starts_with("https://") => {
            format!(
                "{}/{}",
                b.trim_end_matches('/'),
                raw.trim_start_matches('/')
            )
        }
        _ => raw.to_string(),
    }
}

/// Stringify a scalar JSON value for use as a path / query / header value.
/// Objects and arrays (not valid in those positions) yield `None`.
fn value_to_str(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        Value::Number(_) | Value::Bool(_) => Some(v.to_string()),
        _ => None,
    }
}

/// `(method, url, headers, query, body)` — the parts of an HTTP request the REST
/// transport assembles before adding auth and dispatching it.
type RequestParts = (
    String,
    String,
    Vec<(String, String)>,
    Vec<(String, String)>,
    Option<Value>,
);

/// Build a request for a built OpenAPI operation command (#106): resolve the
/// manifest command's `method` + `path` template, fill `{name}` path segments,
/// and route each input to path / query / header / body by its declared `in:`
/// location (default query). The `in: body` input becomes the request body —
/// keyed by location, not name, so a query/header param named `body` is routed
/// correctly.
fn build_operation_request(
    agents_dir: &std::path::Path,
    agent: &str,
    command: &str,
    args: &Value,
) -> Result<RequestParts, AwareError> {
    let m = crate::manifest::loader::load_agent(&agents_dir.join(agent).join("manifest.yaml"))?;
    let cmd = m.commands.get(command).ok_or_else(|| {
        AwareError::Validation(format!("{agent}/{command}: command not found in manifest"))
    })?;
    let method = cmd
        .method
        .clone()
        .ok_or_else(|| {
            AwareError::Validation(format!(
                "rest transport: command {command:?} is not an HTTP method and the manifest declares no `method:` mapping"
            ))
        })?
        .to_ascii_uppercase();
    let mut path = cmd.path.clone().unwrap_or_default();
    let mut headers = Vec::new();
    let mut query = Vec::new();
    let mut cookies: Vec<String> = Vec::new();
    let mut body = None;
    if let Value::Object(provided) = args {
        for (name, val) in provided {
            // The `in: body` input is the request body (by location, not name).
            let loc = cmd
                .inputs
                .get(name.as_str())
                .and_then(|s| s.get("in"))
                .and_then(|v| v.as_str())
                .unwrap_or("query");
            if loc == "body" {
                body = Some(val.clone());
                continue;
            }
            // Expand the value into one or more string pieces — an array yields
            // multiple (a scalar, one). Empty (object/unstringifiable) is skipped.
            let pieces: Vec<String> = match val {
                Value::Array(items) => items.iter().filter_map(value_to_str).collect(),
                other => value_to_str(other).into_iter().collect(),
            };
            if pieces.is_empty() {
                continue;
            }
            match loc {
                // style=simple (default): comma-join, then encode the segment.
                "path" => {
                    let joined = pieces.join(",");
                    path = path.replace(&format!("{{{name}}}"), &percent_encode_path(&joined));
                }
                "header" => headers.push((name.clone(), pieces.join(","))),
                "cookie" => cookies.push(format!("{name}={}", pieces.join(","))),
                // style=form, explode=true (query default): repeat per item.
                _ => {
                    for piece in pieces {
                        query.push((name.clone(), piece));
                    }
                }
            }
        }
    }
    // An unfilled `{name}` means a required path parameter wasn't supplied —
    // fail clearly rather than dispatching a malformed URL (Codex #106).
    if let Some(start) = path.find('{') {
        let pname = path[start + 1..].split('}').next().unwrap_or("");
        return Err(AwareError::Validation(format!(
            "{agent}/{command}: missing required path parameter {{{pname}}} (path {path:?})"
        )));
    }
    // `in: cookie` parameters fold into a single `Cookie` header.
    if !cookies.is_empty() {
        headers.push(("Cookie".to_string(), cookies.join("; ")));
    }
    let url = resolve_url(rest_base_url(agents_dir, agent).as_deref(), &path);
    Ok((method, url, headers, query, body))
}

/// Resolve the credential string to inject for a REST `auth:` block.
///
/// For an `oauth2`/`bearer` scheme whose `secret` names a **registered** OAuth
/// integration (`config::for_integration` resolves it), refresh the token first:
/// [`crate::auth::refresh::ensure_fresh`] re-mints + re-stores it when within the
/// expiry buffer, so a long-lived session never sends a stale access token (#198).
/// Best-effort — if refresh fails (e.g. the refresh token itself expired), fall back
/// to the raw stored value, so a still-valid token is used and a genuinely-missing
/// one yields the caller's clear "provision it" error. For api-key schemes and
/// unregistered secrets (out-of-band tokens) the raw stored value is used unchanged.
pub(crate) fn resolve_rest_credential(
    agents_dir: &std::path::Path,
    auth: &crate::manifest::agent::AuthScheme,
) -> Option<String> {
    if matches!(auth.scheme.as_str(), "oauth2" | "bearer") {
        // The credential handle may be alias-qualified — `<integration>.<alias>`,
        // e.g. `google-workspace.personal` from `aware connect … --as personal`. The
        // registered-integration check + refresh must use the base integration and
        // pass the alias through (the keychain stores `<integration>.<alias>` accounts
        // and `ensure_fresh` takes an alias). A handle whose base is not a registered
        // integration (e.g. `my.api.key`) falls through to the raw stored value.
        let (integration, alias) = match auth.secret.split_once('.') {
            Some((base, alias)) => (base, Some(alias)),
            None => (auth.secret.as_str(), None),
        };
        if crate::auth::config::for_integration(integration).is_ok()
            && let Some(home) = agents_dir.parent()
            && let Ok(tok) = crate::auth::refresh::ensure_fresh(integration, alias, home)
        {
            return Some(tok.access_token);
        }
    }
    load_secret_value(agents_dir, &auth.secret)
        .as_ref()
        .and_then(secret_as_str)
}

/// Load a credential by handle, reusing the runtime secret loader (OS keychain,
/// then `<aware-home>/credentials/<id>.json`). `agents_dir` is `<home>/agents`,
/// so the credentials directory is its `credentials` sibling.
fn load_secret_value(agents_dir: &std::path::Path, id: &str) -> Option<Value> {
    let creds_dir = agents_dir.parent()?.join("credentials");
    let mut ctx = crate::runtime::template::RenderContext::default();
    crate::runtime::context::load_secret(&mut ctx, &creds_dir, id).ok()?;
    ctx.secrets.get(id).cloned()
}

/// Extract the usable credential string from a loaded secret: a raw string is
/// used verbatim; an object is probed for the common token/key fields.
fn secret_as_str(secret: &Value) -> Option<String> {
    match secret {
        Value::String(s) => Some(s.clone()),
        Value::Object(m) => [
            "token",
            "access_token",
            "key",
            "apikey",
            "api_key",
            "value",
            "secret",
        ]
        .iter()
        .find_map(|k| m.get(*k).and_then(|v| v.as_str()).map(|s| s.to_string())),
        _ => None,
    }
}

/// Inject the declared credential into the request: bearer/oauth2 as an
/// `Authorization: Bearer` header, api-key as a header / query / cookie. A slot
/// the caller already populated is left untouched (explicit input wins).
fn inject_auth(
    auth: &crate::manifest::agent::AuthScheme,
    cred: &str,
    headers: &mut Vec<(String, String)>,
    query: &mut Vec<(String, String)>,
) {
    match auth.scheme.as_str() {
        "bearer" | "oauth2"
            if !headers
                .iter()
                .any(|(k, _)| k.eq_ignore_ascii_case("authorization")) =>
        {
            headers.push(("Authorization".into(), format!("Bearer {cred}")));
        }
        "api-key" => {
            let name = auth.name.as_deref().unwrap_or("Authorization");
            match auth.location.as_deref() {
                Some("query") => {
                    if !query.iter().any(|(k, _)| k == name) {
                        query.push((name.to_string(), cred.to_string()));
                    }
                }
                // apiKey `in: cookie` → `Cookie: name=value`. Append to an
                // existing Cookie header (e.g. one built from cookie params)
                // instead of skipping it, unless that cookie name is already set.
                Some("cookie") => {
                    let pair = format!("{name}={cred}");
                    if let Some((_, existing)) = headers
                        .iter_mut()
                        .find(|(k, _)| k.eq_ignore_ascii_case("cookie"))
                    {
                        let already = existing
                            .split(';')
                            .any(|c| c.trim_start().starts_with(&format!("{name}=")));
                        if !already {
                            existing.push_str("; ");
                            existing.push_str(&pair);
                        }
                    } else {
                        headers.push(("Cookie".into(), pair));
                    }
                }
                // header (default).
                _ => {
                    if !headers.iter().any(|(k, _)| k.eq_ignore_ascii_case(name)) {
                        headers.push((name.to_string(), cred.to_string()));
                    }
                }
            }
        }
        _ => {}
    }
}

/// Flatten an optional `{ key: value }` input (headers / query) into ordered
/// string pairs. Non-string scalar values are stringified; nested objects and
/// arrays are skipped (a header value can't be a structure).
fn collect_str_map(v: Option<&Value>) -> Vec<(String, String)> {
    let mut out = Vec::new();
    if let Some(Value::Object(m)) = v {
        for (k, val) in m {
            match val {
                Value::String(s) => out.push((k.clone(), s.clone())),
                Value::Number(_) | Value::Bool(_) => out.push((k.clone(), val.to_string())),
                _ => {}
            }
        }
    }
    out
}

/// Shape a `ureq::Response` into the `{ status, headers, body }` value the
/// `http` agent's output-schema declares. The body is parsed as JSON when it
/// parses, else returned as a string.
fn shape_response(resp: ureq::Response) -> Value {
    let status = resp.status();
    let mut headers = serde_json::Map::new();
    for name in resp.headers_names() {
        if let Some(val) = resp.header(&name) {
            headers.insert(name, Value::String(val.to_string()));
        }
    }
    let text = resp.into_string().unwrap_or_default();
    let body = serde_json::from_str::<Value>(&text).unwrap_or(Value::String(text));
    serde_json::json!({
        "status": status,
        "headers": Value::Object(headers),
        "body": body,
    })
}

/// In-process handler for `builtin`-transport `_core` utilities (#201) — runtime
/// built-ins with no host binary to ship or install. Routed by agent id; serves
/// `html-report`'s generic renderer and the declarative-UI `ui` agent (#215).
/// `pub(crate)` so `aware agent invoke` (commands/agent.rs) drives the same
/// dispatch table outside a workflow.
pub(crate) struct BuiltinInvoker {
    /// On a `--dry-run` / `--simulate` run, the optional `output-path` file write is
    /// skipped so a preview never touches disk; the HTML and result shape are still
    /// returned so downstream nodes resolve.
    pub(crate) dry_run: bool,
}

#[async_trait]
impl AgentInvoker for BuiltinInvoker {
    async fn invoke_single(
        &self,
        agent: &str,
        command: &str,
        args: Value,
    ) -> Result<Value, AwareError> {
        match (agent, command) {
            ("html-report", "render") => render_html_report(args, self.dry_run),
            ("ui", "validate") => crate::render::ui::ui_validate(&args),
            ("ui", "catalog") => crate::render::ui::ui_catalog(&args),
            ("ui", "render") => crate::render::ui::ui_render(&args, self.dry_run),
            ("vision", "extract") => vision_extract(args, self.dry_run).await,
            ("viewer-3d", "render") => {
                crate::render::viewer_3d::viewer_3d_render(&args, self.dry_run)
            }
            _ => Err(AwareError::Validation(format!(
                "builtin transport: no handler for {agent}/{command}"
            ))),
        }
    }

    async fn invoke_stream(
        &self,
        agent: &str,
        command: &str,
        _args: Value,
    ) -> Result<StreamingHandle, AwareError> {
        Err(AwareError::Validation(format!(
            "builtin agent {agent} command {command:?} is single-shot and does not stream"
        )))
    }
}

/// `html-report.render` — generically format an arbitrary `data` payload into a
/// self-contained HTML document (#201). Emits the HTML under `html` (canonical — what
/// the report Viewer reads) and `htmlReport` (alias), plus `item-count`. When
/// `output-path` is given it also writes the file and echoes `output-path` + `bytes`.
fn render_html_report(args: Value, dry_run: bool) -> Result<Value, AwareError> {
    let title = args
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("Report");
    // The `data` input is the payload; if absent, render the whole args object so a
    // bare `render` over an upstream value still produces something useful.
    let data = args.get("data").unwrap_or(&args);
    // Optional table presentation: pick/order columns and sort rows, so a report can
    // drop noise (raw URLs, nested JSON blobs, internal ids) and lead with the latest.
    let opts = crate::render::html_report::TableOptions {
        columns: args.get("columns").and_then(|v| v.as_array()).map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect()
        }),
        sort_by: args
            .get("sort")
            .and_then(|v| v.as_str())
            .map(str::to_string),
        sort_desc: args
            .get("sort-desc")
            .and_then(|v| v.as_bool())
            .unwrap_or(false),
    };
    let html = crate::render::html_report::render_report_with(title, data, &opts);
    let count = crate::render::html_report::item_count(data) as u64;

    let mut out = serde_json::Map::new();
    out.insert("html".into(), Value::String(html.clone()));
    out.insert("htmlReport".into(), Value::String(html.clone()));
    out.insert("item-count".into(), Value::from(count));

    if let Some(path) = args
        .get("output-path")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        // Real run only: a preview (--dry-run / --simulate) returns the HTML + the
        // would-be path/size but never touches disk.
        if !dry_run {
            if let Some(parent) = std::path::Path::new(path).parent()
                && !parent.as_os_str().is_empty()
            {
                std::fs::create_dir_all(parent).map_err(|e| {
                    AwareError::Internal(format!("html-report: create {}: {e}", parent.display()))
                })?;
            }
            std::fs::write(path, html.as_bytes())
                .map_err(|e| AwareError::Internal(format!("html-report: write {path}: {e}")))?;
        }
        out.insert("output-path".into(), Value::String(path.to_string()));
        // `path` alias: existing apps reference the artifact under both names —
        // `{{ node.output-path }}` (e.g. an email attachment) and `{{ node.path }}`
        // (e.g. an engineering output seal). Both are declared in the manifest schema.
        out.insert("path".into(), Value::String(path.to_string()));
        out.insert("bytes".into(), Value::from(html.len() as u64));
    }

    Ok(Value::Object(out))
}

// ── vision.extract — the fenced runtime model extraction (RFC #223) ─────────────
// The validator (`validate::check_node_agents`) has already fenced WHICH nodes may
// reach this handler: only the curated, capability-flagged `vision.extract`. Here we
// add the §6 determinism core — a content-hash cache — so the extraction is a pure
// function with a cache: same (bytes, prompt, schema, model) → same JSON, replayed
// from disk with NO model call. Only first-sight of a new input calls the model.

fn vision_aware_home() -> std::path::PathBuf {
    std::env::var_os("AWARE_HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_default().join(".aware"))
}
fn vision_cache_dir() -> std::path::PathBuf {
    vision_aware_home().join("cache").join("vision")
}

/// sha256(bytes ‖ prompt ‖ schema ‖ model) — the §6 cache key.
fn vision_cache_key(bytes: &[u8], prompt: &str, schema: &Value, model: &str) -> String {
    use sha2::Digest;
    let mut h = sha2::Sha256::new();
    h.update(bytes);
    h.update(prompt.as_bytes());
    h.update(serde_json::to_string(schema).unwrap_or_default().as_bytes());
    h.update(model.as_bytes());
    format!("{:x}", h.finalize())
}

/// `vision.extract`: read the image/PDF at the `file` path and return schema-bound
/// JSON via the pinned model — content-hash cached. The `file` arrives as a path on
/// the edge (front doors store the picked artifact and template `{{ inputs.<x> }}`).
async fn vision_extract(args: Value, dry_run: bool) -> Result<Value, AwareError> {
    let s = |k: &str| args.get(k).and_then(|v| v.as_str()).map(str::to_string);
    let file = s("file").ok_or_else(|| {
        AwareError::Validation("vision.extract: `file` (image/PDF path) is required".into())
    })?;
    let prompt = s("prompt")
        .ok_or_else(|| AwareError::Validation("vision.extract: `prompt` is required".into()))?;
    let model = s("model").ok_or_else(|| {
        AwareError::Validation("vision.extract: `model` (a pinned model id) is required".into())
    })?;
    let schema = args.get("schema").cloned().unwrap_or(Value::Null);

    let bytes = std::fs::read(&file).map_err(|e| {
        AwareError::Validation(format!("vision.extract: cannot read file {file}: {e}"))
    })?;

    // §6 content-hash cache key: sha256(bytes ‖ prompt ‖ schema ‖ model).
    let key = vision_cache_key(&bytes, &prompt, &schema, &model);
    let cache_path = vision_cache_dir().join(format!("{key}.json"));

    // Cache HIT — deterministic replay, no model call.
    if let Ok(text) = std::fs::read_to_string(&cache_path)
        && let Ok(result) = serde_json::from_str::<Value>(&text)
    {
        return Ok(serde_json::json!({ "result": result, "cached": true, "model": model }));
    }
    if dry_run {
        return Ok(
            serde_json::json!({ "result": Value::Null, "cached": false, "dry-run": true, "model": model }),
        );
    }

    // MISS — call the pinned model once (blocking ureq → off the async reactor), store, return.
    let result = {
        let model2 = model.clone();
        tokio::task::spawn_blocking(move || call_vision_model(&bytes, &prompt, &schema, &model2))
            .await
            .map_err(|e| {
                AwareError::Internal(format!("vision.extract: blocking task failed: {e}"))
            })??
    };
    if let Some(dir) = cache_path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let _ = std::fs::write(
        &cache_path,
        serde_json::to_string(&result).unwrap_or_default(),
    );
    Ok(serde_json::json!({ "result": result, "cached": false, "model": model }))
}

/// Which model provider a `vision.extract` cache-miss calls. The credential file
/// `~/.aware/credentials/vision-model.json` is **optional**: AWARE runs inside an AI
/// terminal that already has an authenticated, subscription-billed CLI, so the common
/// case needs no API key at all (and avoids the ~10× metered-API marginal cost).
#[derive(Debug, PartialEq)]
enum VisionProvider {
    /// The local `claude` CLI in headless print mode (the zero-config default).
    ClaudeCli,
    /// The local `codex` CLI (`codex exec`), an explicit opt-in.
    CodexCli,
    /// The Anthropic Messages API — used when an `api_key` credential is configured.
    Anthropic { api_key: String, base_url: String },
}

/// Resolve the provider from the (optional) credential JSON. Pure + injectable so it
/// is unit-tested without touching the filesystem or PATH:
/// - `provider: claude|codex|anthropic` is honored explicitly.
/// - a bare `{"api_key":"…"}` (no `provider`) → Anthropic (back-compat).
/// - no file / no provider / no key → the local `claude` CLI if present, else `codex`.
fn resolve_vision_provider_from(
    cred: Option<&Value>,
    cred_path: &std::path::Path,
    cli_available: impl Fn(&str) -> bool,
) -> Result<VisionProvider, AwareError> {
    let provider = cred
        .and_then(|c| c.get("provider"))
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_ascii_lowercase());
    let api_key = cred
        .and_then(|c| c.get("api_key"))
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let base_url = cred
        .and_then(|c| c.get("base_url"))
        .and_then(|v| v.as_str())
        .unwrap_or("https://api.anthropic.com")
        .to_string();

    match provider.as_deref() {
        Some("claude" | "claude-cli" | "claude-code") => return Ok(VisionProvider::ClaudeCli),
        Some("codex" | "codex-cli") => return Ok(VisionProvider::CodexCli),
        Some("anthropic" | "api" | "anthropic-api") => {
            return match api_key {
                Some(k) => Ok(VisionProvider::Anthropic {
                    api_key: k,
                    base_url,
                }),
                None => Err(AwareError::Network(format!(
                    "vision.extract: provider `anthropic` needs an `api_key` in {}",
                    cred_path.display()
                ))),
            };
        }
        Some(other) => {
            return Err(AwareError::Validation(format!(
                "vision.extract: unknown provider `{other}` in {} — use `claude`, `codex`, or `anthropic`",
                cred_path.display()
            )));
        }
        None => {}
    }
    // No explicit provider: an api_key means "use the API" (back-compat); else the
    // local CLI the operator is already authenticated for.
    if let Some(k) = api_key {
        return Ok(VisionProvider::Anthropic {
            api_key: k,
            base_url,
        });
    }
    if cli_available("claude") {
        Ok(VisionProvider::ClaudeCli)
    } else if cli_available("codex") {
        Ok(VisionProvider::CodexCli)
    } else {
        Err(AwareError::Network(
            "vision.extract: no model provider available — install the `claude` CLI (recommended: \
             it uses your existing Claude subscription, no API key) or the `codex` CLI, or provision \
             ~/.aware/credentials/vision-model.json with {\"api_key\":\"…\"}. vision.extract calls a \
             model at run time (RFC #223)."
                .into(),
        ))
    }
}

/// Load the (optional) credential JSON. An **absent** file → `Ok(None)` (the zero-config
/// default path). A **present but unparseable** file → `Err` — a corrupt key file should
/// fail loudly, not silently switch the user to a different provider/billing.
fn load_vision_credential(path: &std::path::Path) -> Result<Option<Value>, AwareError> {
    match std::fs::read_to_string(path) {
        Ok(t) => {
            let v = serde_json::from_str(&t).map_err(|e| {
                AwareError::Network(format!(
                    "vision.extract: bad credential JSON in {}: {e}",
                    path.display()
                ))
            })?;
            Ok(Some(v))
        }
        Err(_) => Ok(None),
    }
}

fn resolve_vision_provider() -> Result<VisionProvider, AwareError> {
    let cred_path = vision_aware_home()
        .join("credentials")
        .join("vision-model.json");
    let cred = load_vision_credential(&cred_path)?;
    resolve_vision_provider_from(cred.as_ref(), &cred_path, |n| find_on_path(n).is_some())
}

/// Image/PDF sniff → (Anthropic media type, temp-file extension).
fn vision_media(bytes: &[u8]) -> (&'static str, &'static str) {
    if bytes.starts_with(b"\x89PNG") {
        ("image/png", ".png")
    } else if bytes.starts_with(&[0xff, 0xd8, 0xff]) {
        ("image/jpeg", ".jpg")
    } else if bytes.len() >= 12 && &bytes[8..12] == b"WEBP" {
        ("image/webp", ".webp")
    } else if bytes.starts_with(b"%PDF-") {
        ("application/pdf", ".pdf")
    } else {
        ("image/png", ".png")
    }
}

/// The schema-constrained extraction instruction shared by every provider.
fn vision_instruction(prompt: &str, schema: &Value) -> String {
    format!(
        "{prompt}\n\nReturn ONLY a JSON object conforming to this schema, with no prose or \
         markdown fences:\n{}",
        serde_json::to_string(schema).unwrap_or_default()
    )
}

/// Strip optional ``` fences and parse the model's reply as JSON.
fn parse_vision_json(text: &str) -> Result<Value, AwareError> {
    let cleaned = text
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();
    serde_json::from_str::<Value>(cleaned).map_err(|e| {
        AwareError::Validation(format!(
            "vision.extract: model did not return schema-conforming JSON: {e}"
        ))
    })
}

/// Resolve `name` to a directly-spawnable executable on PATH. On Windows this appends
/// the PATHEXT variants `Command::new` can actually launch (`.exe`/`.cmd`/`.bat`/`.com`)
/// — npm ships `codex` only as `.cmd`/`.ps1` shims, and the bare or `.ps1` forms are not
/// spawnable by CreateProcess. On Unix the bare name resolves.
fn find_on_path(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    let dirs: Vec<PathBuf> = std::env::split_paths(&path).collect();
    // If the caller already supplied an extension (e.g. "codex.cmd"), don't append a
    // second one — look the name up verbatim.
    let exts: &[&str] = if cfg!(windows) && std::path::Path::new(name).extension().is_none() {
        &[".exe", ".cmd", ".bat", ".com"]
    } else {
        &[""]
    };
    find_in_dirs(name, &dirs, exts)
}

fn find_in_dirs(name: &str, dirs: &[PathBuf], exts: &[&str]) -> Option<PathBuf> {
    for dir in dirs {
        for ext in exts {
            let cand = dir.join(format!("{name}{ext}"));
            if cand.is_file() {
                return Some(cand);
            }
        }
    }
    None
}

/// A unique temp path (pid + atomic seq) so concurrent extractions never collide.
fn vision_temp_path(ext: &str) -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let seq = SEQ.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("aware-vision-{}-{seq}{ext}", std::process::id()))
}

const VISION_CLI_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(240);

/// Kill a child and (on Windows) its whole process tree — a `codex.cmd`/`.bat` shim
/// spawns the real CLI as a grandchild, and `claude` may spawn tool subprocesses, so
/// `Child::kill` alone can orphan them on timeout. Always reaps the immediate child.
fn kill_process_tree(child: &mut std::process::Child) {
    #[cfg(windows)]
    {
        let _ = std::process::Command::new("taskkill")
            .args(["/PID", &child.id().to_string(), "/T", "/F"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
    }
    let _ = child.kill();
    let _ = child.wait();
}

/// Spawn a CLI provider, draining stdout/stderr on threads (so a chatty child can't
/// deadlock on a full pipe) and killing it past `timeout`. Optionally feeds `stdin_data`
/// to the child — used to pass the prompt to `codex` out-of-band, since a multi-line
/// prompt in argv can't be escaped for a Windows `.cmd`/`.bat` shim. Returns trimmed stdout.
fn run_cli_capture(
    program: &std::path::Path,
    args: &[String],
    stdin_data: Option<&[u8]>,
    timeout: std::time::Duration,
    label: &str,
) -> Result<String, AwareError> {
    use std::io::Read;
    use std::process::Stdio;
    let mut child = std::process::Command::new(program)
        .args(args)
        .stdin(if stdin_data.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| AwareError::Network(format!("vision.extract: spawn {label}: {e}")))?;

    let mut stdout = child.stdout.take().expect("piped stdout");
    let mut stderr = child.stderr.take().expect("piped stderr");
    let h_out = std::thread::spawn(move || {
        let mut s = String::new();
        let _ = stdout.read_to_string(&mut s);
        s
    });
    let h_err = std::thread::spawn(move || {
        let mut s = String::new();
        let _ = stderr.read_to_string(&mut s);
        s
    });
    // Feed stdin from its OWN thread (after the output drains are running): a child that
    // stops reading, or a payload past the OS pipe buffer, then can't block us past the
    // watchdog — the wait loop below still governs the deadline. On kill, the broken pipe
    // ends this thread.
    let h_in = match (stdin_data, child.stdin.take()) {
        (Some(data), Some(mut si)) => {
            let owned = data.to_vec();
            Some(std::thread::spawn(move || {
                use std::io::Write;
                let _ = si.write_all(&owned);
                // drop `si` → closes stdin (EOF).
            }))
        }
        _ => None,
    };

    let deadline = std::time::Instant::now() + timeout;
    let status = loop {
        match child.try_wait() {
            Ok(Some(st)) => break st,
            Ok(None) => {
                if std::time::Instant::now() >= deadline {
                    kill_process_tree(&mut child);
                    return Err(AwareError::Network(format!(
                        "vision.extract: {label} timed out after {}s",
                        timeout.as_secs()
                    )));
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(e) => {
                return Err(AwareError::Network(format!(
                    "vision.extract: {label} wait failed: {e}"
                )));
            }
        }
    };
    let stdout_s = h_out.join().unwrap_or_default();
    let stderr_s = h_err.join().unwrap_or_default();
    if let Some(h) = h_in {
        let _ = h.join();
    }
    if !status.success() {
        let tail: String = stderr_s
            .chars()
            .rev()
            .take(500)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        return Err(AwareError::Network(format!(
            "vision.extract: {label} failed ({status}): {}",
            tail.trim()
        )));
    }
    Ok(stdout_s.trim().to_string())
}

/// Removes its temp path on drop, so a `?` early-return still cleans up.
struct ScratchFile(PathBuf);
impl ScratchFile {
    fn new(ext: &str) -> Self {
        ScratchFile(vision_temp_path(ext))
    }
    fn path(&self) -> &std::path::Path {
        &self.0
    }
}
impl Drop for ScratchFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// `claude` CLI provider: write the artifact to a temp file, have Claude read it
/// (its Read tool handles images **and** PDFs) and return schema-bound JSON. Passes
/// the pinned `model` id through `--model`, so the cache key's model matches what ran.
fn call_claude_cli(
    bytes: &[u8],
    prompt: &str,
    schema: &Value,
    model: &str,
) -> Result<Value, AwareError> {
    let program = find_on_path("claude").ok_or_else(|| {
        AwareError::Network(
            "vision.extract: provider `claude` selected but the `claude` CLI is not on PATH".into(),
        )
    })?;
    let (_, ext) = vision_media(bytes);
    let img = ScratchFile::new(ext);
    std::fs::write(img.path(), bytes)
        .map_err(|e| AwareError::Internal(format!("vision.extract: temp write: {e}")))?;
    let instruction = format!(
        "Read the file at {} using the Read tool — it is the image/PDF to extract from. {}",
        img.path().display(),
        vision_instruction(prompt, schema)
    );
    let mut args = vec![
        "-p".into(),
        instruction,
        "--allowedTools".into(),
        "Read".into(),
        "--permission-mode".into(),
        "acceptEdits".into(),
        "--output-format".into(),
        "text".into(),
    ];
    if !model.is_empty() {
        args.push("--model".into());
        args.push(model.to_string());
    }
    let out = run_cli_capture(&program, &args, None, VISION_CLI_TIMEOUT, "claude")?;
    parse_vision_json(&out)
}

/// `codex` CLI provider: attach the image with `-i`, take the final message from `-o`,
/// and constrain it via the schema embedded in the (stdin) prompt. We deliberately do
/// NOT use codex's `--output-schema`: it demands a strict JSON Schema (every object with
/// `additionalProperties:false` + all keys `required`), which arbitrary vision schemas
/// won't have — so we treat the schema as a textual constraint, exactly like the claude
/// and Anthropic paths. NOTE: `codex` runs its own configured model, so the pinned
/// (Anthropic) `model` id is informational here.
fn call_codex_cli(
    bytes: &[u8],
    prompt: &str,
    schema: &Value,
    _model: &str,
) -> Result<Value, AwareError> {
    let program = find_on_path("codex").ok_or_else(|| {
        AwareError::Network(
            "vision.extract: provider `codex` selected but the `codex` CLI is not on PATH".into(),
        )
    })?;
    let (_, ext) = vision_media(bytes);
    let img = ScratchFile::new(ext);
    let out_file = ScratchFile::new(".txt");
    std::fs::write(img.path(), bytes)
        .map_err(|e| AwareError::Internal(format!("vision.extract: temp write: {e}")))?;
    let args = vec![
        "exec".into(),
        "-i".into(),
        img.path().display().to_string(),
        "-o".into(),
        out_file.path().display().to_string(),
        "--skip-git-repo-check".into(),
        "--color".into(),
        "never".into(),
        // `-` is codex's explicit "read the prompt from stdin" form (vs relying on the
        // no-arg default). The prompt rides stdin, not argv, because codex resolves to a
        // `.cmd`/`.ps1` shim on Windows and a multi-line prompt in argv fails the `.cmd`
        // arg-escaping check.
        "-".into(),
    ];
    let instruction = vision_instruction(prompt, schema);
    run_cli_capture(
        &program,
        &args,
        Some(instruction.as_bytes()),
        VISION_CLI_TIMEOUT,
        "codex",
    )?;
    let result = std::fs::read_to_string(out_file.path()).map_err(|e| {
        AwareError::Network(format!("vision.extract: codex produced no output: {e}"))
    })?;
    parse_vision_json(&result)
}

/// Call the pinned multimodal model once (a cache miss). Dispatches to the resolved
/// provider — the local `claude`/`codex` CLI by default (no API key needed), or the
/// Anthropic Messages API when an `api_key` credential is configured. See
/// `resolve_vision_provider` + docs/superpowers/specs/2026-06-14-vision-extract-cli-provider.md.
fn call_vision_model(
    bytes: &[u8],
    prompt: &str,
    schema: &Value,
    model: &str,
) -> Result<Value, AwareError> {
    match resolve_vision_provider()? {
        VisionProvider::ClaudeCli => call_claude_cli(bytes, prompt, schema, model),
        VisionProvider::CodexCli => call_codex_cli(bytes, prompt, schema, model),
        VisionProvider::Anthropic { api_key, base_url } => {
            call_anthropic_api(bytes, prompt, schema, model, &api_key, &base_url)
        }
    }
}

/// The original Anthropic Messages API path (now one provider among several): send the
/// bytes + schema-constrained prompt to `{base}/v1/messages` and parse the JSON back.
fn call_anthropic_api(
    bytes: &[u8],
    prompt: &str,
    schema: &Value,
    model: &str,
    api_key: &str,
    base: &str,
) -> Result<Value, AwareError> {
    use base64::Engine as _;
    let (media, _) = vision_media(bytes);
    let block_type = if media == "application/pdf" {
        "document"
    } else {
        "image"
    };
    let data = base64::engine::general_purpose::STANDARD.encode(bytes);
    let instruction = vision_instruction(prompt, schema);
    let req = serde_json::json!({
        "model": model,
        "max_tokens": 4096,
        "messages": [{
            "role": "user",
            "content": [
                { "type": block_type, "source": { "type": "base64", "media_type": media, "data": data } },
                { "type": "text", "text": instruction }
            ]
        }]
    });
    let body_str = serde_json::to_string(&req)
        .map_err(|e| AwareError::Internal(format!("vision.extract: request encode: {e}")))?;
    let resp = match ureq::post(&format!("{base}/v1/messages"))
        .set("x-api-key", api_key)
        .set("anthropic-version", "2023-06-01")
        .set("content-type", "application/json")
        .send_string(&body_str)
    {
        Ok(r) => r,
        Err(ureq::Error::Status(code, _)) => {
            return Err(AwareError::Network(format!(
                "vision.extract: model returned HTTP {code}"
            )));
        }
        Err(e) => {
            return Err(AwareError::Network(format!(
                "vision.extract: model request failed: {e}"
            )));
        }
    };
    let text_body = resp.into_string().map_err(|e| {
        AwareError::Network(format!("vision.extract: unreadable model response: {e}"))
    })?;
    let parsed: Value = serde_json::from_str(&text_body).map_err(|e| {
        AwareError::Network(format!("vision.extract: non-JSON model response: {e}"))
    })?;
    let text = parsed
        .get("content")
        .and_then(|c| c.as_array())
        .and_then(|a| {
            a.iter()
                .find_map(|m| m.get("text").and_then(|t| t.as_str()))
        })
        .unwrap_or_default();
    parse_vision_json(text)
}

#[cfg(test)]
mod vision_provider_tests {
    use super::*;
    use serde_json::json;

    fn p() -> std::path::PathBuf {
        std::path::PathBuf::from("/x/credentials/vision-model.json")
    }

    #[test]
    fn no_credential_defaults_to_claude_cli() {
        let got = resolve_vision_provider_from(None, &p(), |n| n == "claude").unwrap();
        assert_eq!(got, VisionProvider::ClaudeCli);
    }

    #[test]
    fn no_credential_falls_back_to_codex_then_errors() {
        let codex = resolve_vision_provider_from(None, &p(), |n| n == "codex").unwrap();
        assert_eq!(codex, VisionProvider::CodexCli);
        // Neither CLI present and no api_key → a clear, actionable error (not a panic).
        assert!(resolve_vision_provider_from(None, &p(), |_| false).is_err());
    }

    #[test]
    fn bare_api_key_uses_anthropic_for_backcompat() {
        let c = json!({ "api_key": "sk-test" });
        // Even with no CLI on PATH, an explicit key keeps the original API behavior.
        let got = resolve_vision_provider_from(Some(&c), &p(), |_| false).unwrap();
        assert_eq!(
            got,
            VisionProvider::Anthropic {
                api_key: "sk-test".into(),
                base_url: "https://api.anthropic.com".into(),
            }
        );
    }

    #[test]
    fn explicit_claude_wins_over_a_present_api_key() {
        // provider:claude must NOT use the API even if a key is sitting in the file.
        let c = json!({ "provider": "claude", "api_key": "sk-test" });
        let got = resolve_vision_provider_from(Some(&c), &p(), |_| false).unwrap();
        assert_eq!(got, VisionProvider::ClaudeCli);
    }

    #[test]
    fn provider_name_is_case_and_alias_insensitive() {
        for name in ["codex", "CODEX", "codex-cli", " Codex "] {
            let c = json!({ "provider": name });
            assert_eq!(
                resolve_vision_provider_from(Some(&c), &p(), |_| false).unwrap(),
                VisionProvider::CodexCli,
                "provider {name:?}"
            );
        }
        for name in ["claude", "claude-cli", "claude-code"] {
            let c = json!({ "provider": name });
            assert_eq!(
                resolve_vision_provider_from(Some(&c), &p(), |_| false).unwrap(),
                VisionProvider::ClaudeCli,
                "provider {name:?}"
            );
        }
    }

    #[test]
    fn explicit_anthropic_requires_a_key_and_honors_base_url() {
        assert!(
            resolve_vision_provider_from(Some(&json!({ "provider": "anthropic" })), &p(), |_| true)
                .is_err()
        );
        let c =
            json!({ "provider": "anthropic", "api_key": "k", "base_url": "https://proxy.local" });
        assert_eq!(
            resolve_vision_provider_from(Some(&c), &p(), |_| false).unwrap(),
            VisionProvider::Anthropic {
                api_key: "k".into(),
                base_url: "https://proxy.local".into()
            }
        );
    }

    #[test]
    fn unknown_provider_is_a_clear_error() {
        let c = json!({ "provider": "gpt-4o" });
        assert!(resolve_vision_provider_from(Some(&c), &p(), |_| true).is_err());
    }

    #[test]
    fn find_in_dirs_resolves_by_extension_and_misses_cleanly() {
        let tmp = tempfile::tempdir().unwrap();
        let exts: &[&str] = if cfg!(windows) {
            &[".exe", ".cmd", ".bat", ".com"]
        } else {
            &[""]
        };
        let fname = if cfg!(windows) { "toolx.cmd" } else { "toolx" };
        std::fs::write(tmp.path().join(fname), b"shim").unwrap();
        let dirs = vec![tmp.path().to_path_buf()];
        assert!(find_in_dirs("toolx", &dirs, exts).is_some());
        assert!(find_in_dirs("absent", &dirs, exts).is_none());
    }

    #[test]
    fn vision_media_sniffs_image_and_pdf() {
        assert_eq!(vision_media(b"\x89PNG\r\n").0, "image/png");
        assert_eq!(vision_media(&[0xff, 0xd8, 0xff, 0xe0]).0, "image/jpeg");
        assert_eq!(vision_media(b"%PDF-1.7").0, "application/pdf");
        assert_eq!(vision_media(b"%PDF-1.7").1, ".pdf");
    }

    #[test]
    fn load_credential_absent_is_none_corrupt_is_error() {
        let tmp = tempfile::tempdir().unwrap();
        // Absent file → Ok(None): the zero-config default (use the local CLI).
        let missing = tmp.path().join("vision-model.json");
        assert_eq!(load_vision_credential(&missing).unwrap(), None);
        // Valid JSON → Ok(Some).
        let good = tmp.path().join("good.json");
        std::fs::write(&good, r#"{"provider":"claude"}"#).unwrap();
        assert!(load_vision_credential(&good).unwrap().is_some());
        // Present-but-corrupt → Err (a broken key file must not silently switch provider).
        let bad = tmp.path().join("bad.json");
        std::fs::write(&bad, "{not json").unwrap();
        assert!(load_vision_credential(&bad).is_err());
    }
}

/// The invoker the orchestrator uses in production. Routes each call to the
/// right transport invoker by inspecting the agent's manifest: a `cli`
/// transport spawns a subprocess; a `rest` transport makes an HTTP call; an
/// `app` transport (a synthesized agent backed by an `exposes-as-agent` app)
/// runs that app's node chain. This lets one app graph mix host-backed (cli),
/// web-API (rest), and app-backed agents.
pub struct DispatchInvoker {
    pub agents_dir: PathBuf,
    /// Set on the top-level invoker to enable app-backed-agent dispatch. `None`
    /// on a nested invoker — that disables app dispatch, enforcing the v0 rule
    /// that an exposed app cannot itself compose another exposes-as-agent app
    /// (app-spec § exposes-as-agent constraints).
    pub app_ctx: Option<AppTransportCtx>,
    /// `true` when this is a `--dry-run` / `--simulate` run. Carried independently
    /// of `app_ctx` (which is `None` on nested invokers) so built-ins suppress
    /// side effects (e.g. the html-report file write) even inside a nested
    /// exposes-as-agent app run.
    pub preview: bool,
}

/// Filesystem + run context a `DispatchInvoker` needs to dispatch an app-backed
/// agent (run an `exposes-as-agent` app as `agent: <app>`). `dry_run`/`simulate`
/// are inherited from the calling run so a nested app honors the same safety
/// posture.
#[derive(Clone)]
pub struct AppTransportCtx {
    pub apps_dir: PathBuf,
    pub logs_dir: PathBuf,
    pub credentials_dir: PathBuf,
    pub dry_run: bool,
    pub simulate: bool,
}

impl DispatchInvoker {
    /// Construct the top-level dispatch invoker, wiring app-backed-agent support
    /// from the given paths + run posture.
    pub fn new(paths: &crate::paths::Paths, dry_run: bool, simulate: bool) -> Self {
        Self {
            agents_dir: paths.agents_dir(),
            app_ctx: Some(AppTransportCtx {
                apps_dir: paths.apps_dir(),
                logs_dir: paths.logs_dir(),
                credentials_dir: paths.credentials_dir(),
                dry_run,
                simulate,
            }),
            preview: dry_run || simulate,
        }
    }

    fn transport_kind(&self, agent: &str) -> Result<TransportKind, AwareError> {
        let m = crate::manifest::loader::load_agent(
            &self.agents_dir.join(agent).join("manifest.yaml"),
        )?;
        effective_transport(&m, agent)
    }

    /// Resolve the backing app + validate the caller's routed inputs for an
    /// app-backed agent. Shared by the one-shot and streaming dispatch paths.
    fn resolve_exposed(
        &self,
        app_ctx: &AppTransportCtx,
        agent: &str,
        command: &str,
        args: &mut Value,
    ) -> Result<App, AwareError> {
        let manifest = crate::manifest::loader::load_agent(
            &self.agents_dir.join(agent).join("manifest.yaml"),
        )?;
        let backed_by = manifest
            .transport
            .app
            .as_ref()
            .map(|a| a.backed_by.clone())
            .unwrap_or_else(|| agent.to_string());

        let app_dir = app_ctx.apps_dir.join(&backed_by);
        if !app_dir.is_dir() {
            return Err(AwareError::NotFound(format!(
                "app-backed agent {agent}: backing app {backed_by} is not installed"
            )));
        }
        let manifest_path =
            crate::manifest::loader::find_app_manifest(&app_dir).ok_or_else(|| {
                AwareError::Validation(format!("backing app {backed_by} has no .flo/.app file"))
            })?;
        let app = crate::manifest::loader::load_app(&manifest_path)?;
        if !app.exposes_as_agent {
            return Err(AwareError::Validation(format!(
                "app {backed_by} is not declared exposes-as-agent"
            )));
        }
        let exposed = app.exposed_command(command).ok_or_else(|| {
            AwareError::Validation(format!(
                "app {backed_by} does not expose a command named {command:?}"
            ))
        })?;
        // Coerce + type-check the caller's routed inputs against the declared
        // contract (templating stringifies them; this restores declared types).
        crate::manifest::expose::validate_exposed_inputs(command, exposed, args)?;
        Ok(app)
    }

    /// Build a leaf invoker for a nested app run — `app_ctx: None` forbids the
    /// nested app from composing yet another exposes-as-agent app (v0 rule).
    fn nested_leaf(&self) -> Arc<dyn AgentInvoker> {
        Arc::new(DispatchInvoker {
            agents_dir: self.agents_dir.clone(),
            app_ctx: None,
            // Carry the preview posture into the nested run so its built-ins still
            // suppress side effects under --dry-run / --simulate (#201 Codex).
            preview: self.preview,
        })
    }

    async fn dispatch_app_single(
        &self,
        app_ctx: &AppTransportCtx,
        agent: &str,
        command: &str,
        mut args: Value,
    ) -> Result<Value, AwareError> {
        let app = self.resolve_exposed(app_ctx, agent, command, &mut args)?;
        let backed_by = app.app.clone();
        let run_id = crate::runtime::provenance::run_id_now();
        let log_path = crate::runtime::provenance::log_path_for(
            &app_ctx.logs_dir,
            &backed_by,
            "nested",
            &run_id,
        );
        let provenance = crate::runtime::provenance::ProvenanceWriter::open(&log_path).await?;
        crate::runtime::orchestrator::run_exposed_app_one_shot(
            app,
            args,
            self.agents_dir.clone(),
            self.nested_leaf(),
            provenance,
            run_id,
            "nested".to_string(),
            Some(app_ctx.credentials_dir.as_path()),
            app_ctx.dry_run,
            app_ctx.simulate,
        )
        .await
    }

    async fn dispatch_app_stream(
        &self,
        app_ctx: &AppTransportCtx,
        agent: &str,
        command: &str,
        mut args: Value,
    ) -> Result<StreamingHandle, AwareError> {
        let app = self.resolve_exposed(app_ctx, agent, command, &mut args)?;
        let backed_by = app.app.clone();
        let run_id = crate::runtime::provenance::run_id_now();
        let log_path = crate::runtime::provenance::log_path_for(
            &app_ctx.logs_dir,
            &backed_by,
            "nested",
            &run_id,
        );
        let provenance = crate::runtime::provenance::ProvenanceWriter::open(&log_path).await?;

        let (event_tx, event_rx) = mpsc::channel::<Result<Value, AwareError>>(64);
        let (stop_tx, stop_rx) = oneshot::channel::<()>();
        let (nested_stop_tx, nested_stop_rx) = crate::runtime::lifecycle::stop_channel();

        // Bridge the handle's stop (fired explicitly or on drop) to the nested
        // orchestrator's stop watch, so a parent `aware app stop` / Ctrl+C tears
        // the nested run down too.
        tokio::spawn(async move {
            let _ = stop_rx.await;
            let _ = nested_stop_tx.send(true);
        });

        let agents_dir = self.agents_dir.clone();
        let leaf = self.nested_leaf();
        let creds = app_ctx.credentials_dir.clone();
        let dry_run = app_ctx.dry_run;
        let simulate = app_ctx.simulate;
        tokio::spawn(async move {
            let res = crate::runtime::orchestrator::run_exposed_app_stream(
                app,
                args,
                agents_dir,
                leaf,
                provenance,
                run_id,
                "nested".to_string(),
                Some(creds.as_path()),
                dry_run,
                simulate,
                event_tx.clone(),
                nested_stop_rx,
            )
            .await;
            if let Err(e) = res {
                let _ = event_tx.send(Err(e)).await;
            }
            // event_tx dropped here → the caller's stream closes.
        });

        Ok(StreamingHandle {
            rx: event_rx,
            stop: stop_tx,
        })
    }

    /// Error returned when an app-backed agent is reached on a nested invoker
    /// (the v0 no-recursion rule). Shared by both invoke paths.
    fn nested_recursion_error(agent: &str) -> AwareError {
        AwareError::Validation(format!(
            "agent {agent} is an exposes-as-agent app, but an exposed app cannot itself \
             compose another exposes-as-agent app (v0 constraint, app-spec § exposes-as-agent)"
        ))
    }
}

/// The EFFECTIVE transport of an agent manifest. `pub(crate)` so callers
/// outside the orchestrator (e.g. `aware agent invoke`'s builtin-only guard)
/// resolve it through [`effective_transport`] — the same priority order
/// dispatch uses — instead of probing `transport.*` fields ad hoc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TransportKind {
    Cli,
    Rest,
    App,
    Builtin,
}

impl TransportKind {
    /// Lowercase manifest-key name (`cli` / `rest` / `app` / `builtin`) for
    /// error messages.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            TransportKind::Cli => "cli",
            TransportKind::Rest => "rest",
            TransportKind::App => "app",
            TransportKind::Builtin => "builtin",
        }
    }
}

/// Resolve which transport a manifest ACTUALLY dispatches on. This is the one
/// source of truth for the priority order (cli > rest > app > builtin): a
/// mixed-transport manifest (e.g. builtin + cli) runs as its highest-priority
/// transport, so every guard that asks "is this agent builtin?" must resolve
/// through here rather than checking `transport.builtin.is_some()` — otherwise
/// the guard and dispatch disagree (#215 review).
pub(crate) fn effective_transport(
    m: &crate::manifest::Agent,
    agent: &str,
) -> Result<TransportKind, AwareError> {
    if m.transport.cli.is_some() {
        Ok(TransportKind::Cli)
    } else if m.transport.rest.is_some() {
        Ok(TransportKind::Rest)
    } else if m.transport.app.is_some() {
        Ok(TransportKind::App)
    } else if m.transport.builtin.is_some() {
        Ok(TransportKind::Builtin)
    } else {
        Err(AwareError::Validation(format!(
            "agent {agent} has no executable transport (need `cli`, `rest`, `app`, or `builtin`)"
        )))
    }
}

#[async_trait]
impl AgentInvoker for DispatchInvoker {
    async fn invoke_single(
        &self,
        agent: &str,
        command: &str,
        args: Value,
    ) -> Result<Value, AwareError> {
        let dir = self.agents_dir.clone();
        match self.transport_kind(agent)? {
            TransportKind::Cli => {
                CliInvoker { agents_dir: dir }
                    .invoke_single(agent, command, args)
                    .await
            }
            TransportKind::Rest => {
                RestInvoker { agents_dir: dir }
                    .invoke_single(agent, command, args)
                    .await
            }
            TransportKind::App => match &self.app_ctx {
                Some(app_ctx) => {
                    self.dispatch_app_single(app_ctx, agent, command, args)
                        .await
                }
                None => Err(Self::nested_recursion_error(agent)),
            },
            TransportKind::Builtin => {
                BuiltinInvoker {
                    dry_run: self.preview,
                }
                .invoke_single(agent, command, args)
                .await
            }
        }
    }

    async fn invoke_stream(
        &self,
        agent: &str,
        command: &str,
        args: Value,
    ) -> Result<StreamingHandle, AwareError> {
        let dir = self.agents_dir.clone();
        match self.transport_kind(agent)? {
            TransportKind::Cli => {
                CliInvoker { agents_dir: dir }
                    .invoke_stream(agent, command, args)
                    .await
            }
            TransportKind::Rest => {
                RestInvoker { agents_dir: dir }
                    .invoke_stream(agent, command, args)
                    .await
            }
            TransportKind::App => match &self.app_ctx {
                Some(app_ctx) => {
                    self.dispatch_app_stream(app_ctx, agent, command, args)
                        .await
                }
                None => Err(Self::nested_recursion_error(agent)),
            },
            TransportKind::Builtin => {
                BuiltinInvoker {
                    dry_run: self.preview,
                }
                .invoke_stream(agent, command, args)
                .await
            }
        }
    }
}

#[cfg(test)]
mod cli_invoker_tests {
    use super::*;

    #[test]
    fn resolve_cli_binary_uses_installed_bridge_path() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("aware-tekla.exe"), b"fake").unwrap();
        let p = resolve_cli_binary("aware-tekla", tmp.path());
        assert!(p.is_absolute(), "expected absolute bridge path, got {p:?}");
        assert!(p.to_string_lossy().contains("aware-tekla"));
    }

    #[test]
    fn resolve_cli_binary_bridge_not_installed_falls_back_to_bare_name() {
        let tmp = tempfile::tempdir().unwrap();
        // Known bridge name but not present → bare name (legacy PATH / on-PATH).
        let p = resolve_cli_binary("aware-tekla", tmp.path());
        assert_eq!(p, std::path::PathBuf::from("aware-tekla"));
    }

    #[test]
    fn resolve_cli_binary_non_bridge_uses_bare_name() {
        let tmp = tempfile::tempdir().unwrap();
        let p = resolve_cli_binary("ripgrep", tmp.path());
        assert_eq!(p, std::path::PathBuf::from("ripgrep"));
    }

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
    async fn stream_missing_binary_returns_network_error() {
        // A stateful watch agent whose cli binary doesn't exist: invoke_stream
        // fails at spawn (same as invoke_single) rather than the old stub.
        let tmp = tempfile::tempdir().unwrap();
        let agent_dir = tmp.path().join("phantom-watcher");
        std::fs::create_dir_all(&agent_dir).unwrap();
        std::fs::write(
            agent_dir.join("manifest.yaml"),
            r#"agent: phantom-watcher
version: 0.1.0
description: a non-existent watcher
stateful: true
license: MIT
transport:
  cli:
    binary: this-watch-binary-definitely-does-not-exist-54321
commands:
  watch:
    lifecycle: start
    description: nope
    outputs:
      type: stream
      schema:
        mark: string
"#,
        )
        .unwrap();

        let inv = CliInvoker {
            agents_dir: tmp.path().to_path_buf(),
        };
        let err = inv
            .invoke_stream("phantom-watcher", "watch", serde_json::json!({}))
            .await
            .unwrap_err();
        assert!(matches!(err, AwareError::Network(_)), "got: {err:?}");
    }

    #[tokio::test]
    async fn stream_missing_transport_cli_returns_validation_error() {
        let tmp = tempfile::tempdir().unwrap();
        let agent_dir = tmp.path().join("no-cli");
        std::fs::create_dir_all(&agent_dir).unwrap();
        std::fs::write(
            agent_dir.join("manifest.yaml"),
            r#"agent: no-cli
version: 0.1.0
description: x
stateful: true
license: MIT
transport: {}
commands:
  watch:
    lifecycle: start
    description: nope
"#,
        )
        .unwrap();

        let inv = CliInvoker {
            agents_dir: tmp.path().to_path_buf(),
        };
        let err = inv
            .invoke_stream("no-cli", "watch", serde_json::json!({}))
            .await
            .unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)), "got: {err:?}");
    }
}

#[cfg(test)]
mod stream_pump_tests {
    use super::*;
    use serde_json::json;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn pump_forwards_events_in_order_then_eof() {
        let (mut w, r) = tokio::io::duplex(256);
        let (tx, mut rx) = mpsc::channel(16);
        let (_stop_tx, stop_rx) = oneshot::channel();
        let pump = tokio::spawn(async move { pump_jsonl_stream(r, &tx, stop_rx).await });

        w.write_all(b"{\"mark\":\"A\"}\n{\"mark\":\"B\"}\n")
            .await
            .unwrap();
        w.shutdown().await.unwrap(); // EOF

        assert_eq!(rx.recv().await.unwrap().unwrap()["mark"], "A");
        assert_eq!(rx.recv().await.unwrap().unwrap()["mark"], "B");
        assert_eq!(pump.await.unwrap(), StreamEnd::Eof);
        // Channel closed once the pump returns and drops its borrow.
        assert!(rx.recv().await.is_none());
    }

    #[tokio::test]
    async fn pump_skips_blank_lines_and_flags_malformed() {
        let (mut w, r) = tokio::io::duplex(256);
        let (tx, mut rx) = mpsc::channel(16);
        let (_stop_tx, stop_rx) = oneshot::channel();
        let pump = tokio::spawn(async move { pump_jsonl_stream(r, &tx, stop_rx).await });

        w.write_all(b"\n{\"n\":1}\nnot-json\n").await.unwrap();
        w.shutdown().await.unwrap();

        assert_eq!(rx.recv().await.unwrap().unwrap()["n"], json!(1));
        assert!(matches!(
            rx.recv().await.unwrap(),
            Err(AwareError::Validation(_))
        ));
        assert_eq!(pump.await.unwrap(), StreamEnd::Eof);
    }

    #[tokio::test]
    async fn pump_stops_on_signal_while_stream_open() {
        // Writer never closes; only the stop signal can end the pump.
        let (mut w, r) = tokio::io::duplex(256);
        let (tx, mut rx) = mpsc::channel(16);
        let (stop_tx, stop_rx) = oneshot::channel();
        let pump = tokio::spawn(async move { pump_jsonl_stream(r, &tx, stop_rx).await });

        w.write_all(b"{\"n\":1}\n").await.unwrap();
        assert_eq!(rx.recv().await.unwrap().unwrap()["n"], json!(1));

        stop_tx.send(()).unwrap();
        assert_eq!(pump.await.unwrap(), StreamEnd::Stopped);
        drop(w); // keep the writer alive until the pump has stopped
    }

    #[tokio::test]
    async fn pump_reports_receiver_gone() {
        let (mut w, r) = tokio::io::duplex(256);
        let (tx, rx) = mpsc::channel::<Result<Value, AwareError>>(16);
        let (_stop_tx, stop_rx) = oneshot::channel();
        drop(rx); // no consumer
        let pump = tokio::spawn(async move { pump_jsonl_stream(r, &tx, stop_rx).await });

        w.write_all(b"{\"n\":1}\n").await.unwrap();
        assert_eq!(pump.await.unwrap(), StreamEnd::ReceiverGone);
        drop(w);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn vision_cache_key_is_content_addressed() {
        // RFC #223 §6: the key is sha256(bytes ‖ prompt ‖ schema ‖ model). Same inputs →
        // same key (deterministic replay); any field change → a different key (a fresh
        // input, prompt, schema, or model swap is a cache miss, never a stale hit).
        let schema = json!({ "type": "object" });
        let base = vision_cache_key(b"PNGBYTES", "extract rows", &schema, "claude-x");
        assert_eq!(base.len(), 64, "sha256 hex");
        assert_eq!(
            base,
            vision_cache_key(b"PNGBYTES", "extract rows", &schema, "claude-x"),
            "same inputs must give the same key"
        );
        assert_ne!(
            base,
            vision_cache_key(b"OTHER", "extract rows", &schema, "claude-x")
        );
        assert_ne!(
            base,
            vision_cache_key(b"PNGBYTES", "different", &schema, "claude-x")
        );
        assert_ne!(
            base,
            vision_cache_key(b"PNGBYTES", "extract rows", &schema, "claude-y")
        );
        assert_ne!(
            base,
            vision_cache_key(
                b"PNGBYTES",
                "extract rows",
                &json!({ "type": "array" }),
                "claude-x"
            ),
            "a schema change must miss"
        );
    }

    #[tokio::test]
    async fn vision_extract_missing_file_is_a_clear_error() {
        // A non-existent file path → a clear validation error, never a panic.
        let args = json!({ "file": "/no/such/file.png", "prompt": "x", "model": "claude-x", "schema": {} });
        let err = vision_extract(args, false).await.unwrap_err();
        assert!(format!("{err}").contains("cannot read file"), "got: {err}");
    }

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

#[cfg(test)]
mod rest_invoker_tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::sync::mpsc;

    /// Minimal one-shot HTTP/1.1 server (no extra deps). Accepts a single
    /// connection, captures the raw request, replies with `status` + JSON
    /// `body`, and ships the captured request back over the channel.
    fn mock_server(status: u16, body: &'static str) -> (u16, mpsc::Receiver<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
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
                        Err(_) => break, // read-timeout: request fully received
                    }
                }
                let _ = tx.send(String::from_utf8_lossy(&data).to_string());
                let resp = format!(
                    "HTTP/1.1 {status} X\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                );
                let _ = stream.write_all(resp.as_bytes());
            }
        });
        (port, rx)
    }

    fn http_agent_dir() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("http");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("manifest.yaml"),
            r#"agent: http
version: 0.1.0
description: generic http
stateful: false
license: MIT
transport:
  rest: {}
commands:
  get:
    lifecycle: single
    description: GET
  post:
    lifecycle: single
    mode: write
    description: POST
"#,
        )
        .unwrap();
        tmp
    }

    #[tokio::test]
    async fn post_builds_request_and_shapes_response() {
        let (port, rx) = mock_server(200, r#"{"ok":true,"id":42}"#);
        let tmp = http_agent_dir();
        let inv = RestInvoker {
            agents_dir: tmp.path().to_path_buf(),
        };
        let out = inv
            .invoke_single(
                "http",
                "post",
                serde_json::json!({
                    "url": format!("http://127.0.0.1:{port}/rest/v1/projects"),
                    "headers": { "apikey": "sek-ret", "Prefer": "resolution=merge-duplicates" },
                    "body": { "name": "Tower A", "status": "active" }
                }),
            )
            .await
            .unwrap();

        // Response shape: status + parsed JSON body.
        assert_eq!(out["status"], serde_json::json!(200));
        assert_eq!(out["body"]["ok"], serde_json::json!(true));
        assert_eq!(out["body"]["id"], serde_json::json!(42));

        // Request the server actually received.
        let req = rx.recv().unwrap();
        assert!(req.starts_with("POST /rest/v1/projects"), "req line: {req}");
        assert!(req.contains("apikey: sek-ret"), "missing header in: {req}");
        assert!(
            req.contains("Content-Type: application/json"),
            "json body should default Content-Type; got: {req}"
        );
        assert!(
            req.contains("\"name\":\"Tower A\""),
            "missing body in: {req}"
        );
    }

    #[tokio::test]
    async fn query_params_are_appended() {
        let (port, rx) = mock_server(200, r#"{"rows":[]}"#);
        let tmp = http_agent_dir();
        let inv = RestInvoker {
            agents_dir: tmp.path().to_path_buf(),
        };
        let _ = inv
            .invoke_single(
                "http",
                "get",
                serde_json::json!({
                    "url": format!("http://127.0.0.1:{port}/rest/v1/drawings"),
                    "query": { "select": "id,status", "project": "eq.42" }
                }),
            )
            .await
            .unwrap();
        let req = rx.recv().unwrap();
        assert!(req.contains("select=id%2Cstatus"), "req: {req}");
        assert!(req.contains("project=eq.42"), "req: {req}");
    }

    #[tokio::test]
    async fn non_2xx_is_returned_as_response_not_error() {
        let (port, _rx) = mock_server(404, r#"{"message":"not found"}"#);
        let tmp = http_agent_dir();
        let inv = RestInvoker {
            agents_dir: tmp.path().to_path_buf(),
        };
        let out = inv
            .invoke_single(
                "http",
                "get",
                serde_json::json!({ "url": format!("http://127.0.0.1:{port}/missing") }),
            )
            .await
            .unwrap();
        assert_eq!(out["status"], serde_json::json!(404));
        assert_eq!(out["body"]["message"], serde_json::json!("not found"));
    }

    #[tokio::test]
    async fn transport_failure_is_network_error() {
        // Bind then drop to obtain a port nothing is listening on.
        let port = {
            let l = TcpListener::bind("127.0.0.1:0").unwrap();
            l.local_addr().unwrap().port()
        };
        let tmp = http_agent_dir();
        let inv = RestInvoker {
            agents_dir: tmp.path().to_path_buf(),
        };
        let err = inv
            .invoke_single(
                "http",
                "get",
                serde_json::json!({ "url": format!("http://127.0.0.1:{port}/") }),
            )
            .await
            .unwrap_err();
        assert!(matches!(err, AwareError::Network(_)), "got: {err:?}");
    }

    #[tokio::test]
    async fn command_that_is_not_a_method_is_rejected() {
        let tmp = http_agent_dir();
        let inv = RestInvoker {
            agents_dir: tmp.path().to_path_buf(),
        };
        let err = inv
            .invoke_single(
                "http",
                "frobnicate",
                serde_json::json!({ "url": "http://x" }),
            )
            .await
            .unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)), "got: {err:?}");
    }

    #[tokio::test]
    async fn dispatch_routes_rest_agent_to_http() {
        let (port, rx) = mock_server(200, r#"{"ok":true}"#);
        let tmp = http_agent_dir();
        let inv = DispatchInvoker {
            agents_dir: tmp.path().to_path_buf(),
            app_ctx: None,
            preview: false,
        };
        let out = inv
            .invoke_single(
                "http",
                "get",
                serde_json::json!({ "url": format!("http://127.0.0.1:{port}/ping") }),
            )
            .await
            .unwrap();
        assert_eq!(out["status"], serde_json::json!(200));
        let req = rx.recv().unwrap();
        assert!(req.starts_with("GET /ping"), "req: {req}");
    }

    #[tokio::test]
    async fn rest_injects_declared_apikey_header_from_credential() {
        // End-to-end: a rest agent declaring an `auth:` block + a stored
        // credential → the transport injects the header without the caller
        // templating it (#106). agents_dir is `<home>/agents`, so the
        // credentials file lives in the `<home>/credentials` sibling.
        let (port, rx) = mock_server(200, r#"{"ok":true}"#);
        let home = tempfile::tempdir().unwrap();
        let agents = home.path().join("agents");
        let dir = agents.join("secured");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("manifest.yaml"),
            r#"agent: secured
version: 0.1.0
description: secured api
stateful: false
license: MIT
transport:
  rest: {}
auth:
  scheme: api-key
  in: header
  name: X-API-Key
  secret: secured
requires:
  secrets:
    - secured
commands:
  get:
    lifecycle: single
    description: GET
"#,
        )
        .unwrap();
        let creds = home.path().join("credentials");
        std::fs::create_dir_all(&creds).unwrap();
        std::fs::write(creds.join("secured.json"), r#"{"key":"sk-live-123"}"#).unwrap();

        let inv = RestInvoker { agents_dir: agents };
        let out = inv
            .invoke_single(
                "secured",
                "get",
                serde_json::json!({ "url": format!("http://127.0.0.1:{port}/ping") }),
            )
            .await
            .unwrap();
        assert_eq!(out["status"], serde_json::json!(200));
        let req = rx.recv().unwrap();
        assert!(
            req.contains("X-API-Key: sk-live-123"),
            "injected api-key header missing: {req}"
        );
    }

    #[test]
    fn resolve_rest_credential_api_key_uses_raw_value_no_refresh() {
        use crate::manifest::agent::AuthScheme;
        // api-key scheme → never attempts OAuth refresh; the raw stored value is used.
        let tmp = tempfile::tempdir().unwrap();
        let agents = tmp.path().join("agents");
        let creds = tmp.path().join("credentials");
        std::fs::create_dir_all(&agents).unwrap();
        std::fs::create_dir_all(&creds).unwrap();
        std::fs::write(creds.join("svc-key.json"), r#"{"access_token":"raw-key"}"#).unwrap();
        let auth = AuthScheme {
            scheme: "api-key".into(),
            location: None,
            name: None,
            secret: "svc-key".into(),
        };
        assert_eq!(
            resolve_rest_credential(&agents, &auth).as_deref(),
            Some("raw-key")
        );
    }

    #[test]
    fn resolve_rest_credential_unregistered_oauth_falls_back_to_raw() {
        use crate::manifest::agent::AuthScheme;
        // oauth2 scheme but the secret is NOT a registered integration → raw value
        // (there is no refresh path for an out-of-band token).
        let tmp = tempfile::tempdir().unwrap();
        let agents = tmp.path().join("agents");
        let creds = tmp.path().join("credentials");
        std::fs::create_dir_all(&agents).unwrap();
        std::fs::create_dir_all(&creds).unwrap();
        std::fs::write(
            creds.join("some-byo-api.json"),
            r#"{"access_token":"byo-tok"}"#,
        )
        .unwrap();
        let auth = AuthScheme {
            scheme: "oauth2".into(),
            location: None,
            name: None,
            secret: "some-byo-api".into(),
        };
        assert_eq!(
            resolve_rest_credential(&agents, &auth).as_deref(),
            Some("byo-tok")
        );
    }

    #[test]
    fn resolve_rest_credential_dotted_non_integration_secret_uses_raw() {
        use crate::manifest::agent::AuthScheme;
        // A dotted handle whose base isn't a registered integration (e.g. `my.api.key`)
        // is NOT treated as an aliased OAuth credential — it falls back to the raw value.
        let tmp = tempfile::tempdir().unwrap();
        let agents = tmp.path().join("agents");
        let creds = tmp.path().join("credentials");
        std::fs::create_dir_all(&agents).unwrap();
        std::fs::create_dir_all(&creds).unwrap();
        std::fs::write(
            creds.join("my.api.key.json"),
            r#"{"access_token":"dotted"}"#,
        )
        .unwrap();
        let auth = AuthScheme {
            scheme: "oauth2".into(),
            location: None,
            name: None,
            secret: "my.api.key".into(),
        };
        assert_eq!(
            resolve_rest_credential(&agents, &auth).as_deref(),
            Some("dotted")
        );
    }

    #[test]
    fn inject_auth_covers_bearer_apikey_and_respects_explicit() {
        use crate::manifest::agent::AuthScheme;
        let bearer = AuthScheme {
            scheme: "bearer".into(),
            location: None,
            name: None,
            secret: "s".into(),
        };
        let mut h = Vec::new();
        let mut q = Vec::new();
        inject_auth(&bearer, "tok", &mut h, &mut q);
        assert_eq!(h, vec![("Authorization".into(), "Bearer tok".into())]);

        let apikey_query = AuthScheme {
            scheme: "api-key".into(),
            location: Some("query".into()),
            name: Some("apikey".into()),
            secret: "s".into(),
        };
        let mut h2 = Vec::new();
        let mut q2 = Vec::new();
        inject_auth(&apikey_query, "raw", &mut h2, &mut q2);
        assert_eq!(q2, vec![("apikey".into(), "raw".into())]);

        // An explicit Authorization the caller set is never overridden.
        let mut h3 = vec![("Authorization".to_string(), "Bearer explicit".to_string())];
        let mut q3 = Vec::new();
        inject_auth(&bearer, "other", &mut h3, &mut q3);
        assert_eq!(h3[0].1, "Bearer explicit");

        // apiKey `in: cookie` → `Cookie: name=value` header.
        let apikey_cookie = AuthScheme {
            scheme: "api-key".into(),
            location: Some("cookie".into()),
            name: Some("session".into()),
            secret: "s".into(),
        };
        let mut h4 = Vec::new();
        let mut q4 = Vec::new();
        inject_auth(&apikey_cookie, "sid-9", &mut h4, &mut q4);
        assert_eq!(h4, vec![("Cookie".into(), "session=sid-9".into())]);
    }

    #[test]
    fn percent_encode_path_escapes_reserved_chars() {
        assert_eq!(percent_encode_path("a/b.txt"), "a%2Fb.txt");
        assert_eq!(percent_encode_path("x y?z"), "x%20y%3Fz");
        // Unreserved set passes through unchanged.
        assert_eq!(percent_encode_path("Pet-1_2.3~ok"), "Pet-1_2.3~ok");
    }

    #[test]
    fn secret_as_str_handles_string_and_objects() {
        assert_eq!(secret_as_str(&serde_json::json!("raw")), Some("raw".into()));
        assert_eq!(
            secret_as_str(&serde_json::json!({"token":"t"})),
            Some("t".into())
        );
        assert_eq!(
            secret_as_str(&serde_json::json!({"apikey":"k"})),
            Some("k".into())
        );
        assert_eq!(secret_as_str(&serde_json::json!({"other":"x"})), None);
    }

    #[tokio::test]
    async fn built_openapi_get_executes_with_path_query_and_auth() {
        // A built OpenAPI agent: command `get-pet` maps to `GET /pets/{petId}`,
        // `petId` is a path param, `verbose` a query param, and the agent
        // declares api-key auth. The transport substitutes the path, appends the
        // query, and injects the credential — end to end (#106).
        let (port, rx) = mock_server(200, r#"{"id":42,"name":"Rex"}"#);
        let home = tempfile::tempdir().unwrap();
        let agents = home.path().join("agents");
        let dir = agents.join("petstore");
        std::fs::create_dir_all(&dir).unwrap();
        let manifest = r#"agent: petstore
version: 0.1.0
description: petstore
stateful: false
license: MIT
transport:
  rest:
    base: http://127.0.0.1:PORT
auth:
  scheme: api-key
  in: header
  name: X-API-Key
  secret: petstore
requires:
  secrets:
    - petstore
commands:
  get-pet:
    lifecycle: single
    description: "GET /pets/{petId}"
    method: GET
    path: /pets/{petId}
    inputs:
      petId:
        type: integer
        in: path
        required: true
      verbose:
        type: boolean
        in: query
"#
        .replace("PORT", &port.to_string());
        std::fs::write(dir.join("manifest.yaml"), manifest).unwrap();
        let creds = home.path().join("credentials");
        std::fs::create_dir_all(&creds).unwrap();
        std::fs::write(creds.join("petstore.json"), r#"{"key":"sk-abc"}"#).unwrap();

        let inv = RestInvoker { agents_dir: agents };
        let out = inv
            .invoke_single(
                "petstore",
                "get-pet",
                serde_json::json!({ "petId": 42, "verbose": true }),
            )
            .await
            .unwrap();
        assert_eq!(out["status"], serde_json::json!(200));
        assert_eq!(out["body"]["name"], serde_json::json!("Rex"));
        let req = rx.recv().unwrap();
        assert!(
            req.starts_with("GET /pets/42"),
            "path param not substituted: {req}"
        );
        assert!(req.contains("verbose=true"), "query param missing: {req}");
        assert!(
            req.contains("X-API-Key: sk-abc"),
            "auth not injected: {req}"
        );
    }

    #[tokio::test]
    async fn built_openapi_post_sends_body_with_bearer_auth() {
        let (port, rx) = mock_server(201, r#"{"ok":true}"#);
        let home = tempfile::tempdir().unwrap();
        let agents = home.path().join("agents");
        let dir = agents.join("petstore");
        std::fs::create_dir_all(&dir).unwrap();
        let manifest = r#"agent: petstore
version: 0.1.0
description: petstore
stateful: false
license: MIT
transport:
  rest:
    base: http://127.0.0.1:PORT
auth:
  scheme: bearer
  secret: petstore
requires:
  secrets:
    - petstore
commands:
  add-pet:
    lifecycle: single
    mode: write
    description: "POST /pets"
    method: POST
    path: /pets
    inputs:
      body:
        type: Pet
        in: body
"#
        .replace("PORT", &port.to_string());
        std::fs::write(dir.join("manifest.yaml"), manifest).unwrap();
        let creds = home.path().join("credentials");
        std::fs::create_dir_all(&creds).unwrap();
        std::fs::write(creds.join("petstore.json"), r#"{"token":"jwt-xyz"}"#).unwrap();

        let inv = RestInvoker { agents_dir: agents };
        let out = inv
            .invoke_single(
                "petstore",
                "add-pet",
                serde_json::json!({ "body": { "name": "Rex" } }),
            )
            .await
            .unwrap();
        assert_eq!(out["status"], serde_json::json!(201));
        let req = rx.recv().unwrap();
        assert!(req.starts_with("POST /pets"), "method/path wrong: {req}");
        assert!(req.contains("\"name\":\"Rex\""), "body missing: {req}");
        assert!(
            req.contains("Authorization: Bearer jwt-xyz"),
            "bearer auth not injected: {req}"
        );
    }

    #[tokio::test]
    async fn operation_command_named_like_a_verb_uses_its_mapping() {
        // An operationId that kebab-cases to `get` must still route via its
        // manifest method/path mapping, not the generic url-based path that
        // would demand a `url` input (Codex #106).
        let (port, rx) = mock_server(200, r#"{"ok":true}"#);
        let home = tempfile::tempdir().unwrap();
        let agents = home.path().join("agents");
        let dir = agents.join("svc");
        std::fs::create_dir_all(&dir).unwrap();
        let manifest = r#"agent: svc
version: 0.1.0
description: svc
stateful: false
license: MIT
transport:
  rest:
    base: http://127.0.0.1:PORT
commands:
  get:
    lifecycle: single
    description: "GET /health"
    method: GET
    path: /health
"#
        .replace("PORT", &port.to_string());
        std::fs::write(dir.join("manifest.yaml"), manifest).unwrap();

        let inv = RestInvoker { agents_dir: agents };
        // No `url` input — the mapping must drive the request.
        let out = inv
            .invoke_single("svc", "get", serde_json::json!({}))
            .await
            .unwrap();
        assert_eq!(out["status"], serde_json::json!(200));
        let req = rx.recv().unwrap();
        assert!(
            req.starts_with("GET /health"),
            "verb-named command should use its mapping, not the url path: {req}"
        );
    }

    #[tokio::test]
    async fn operation_cookie_param_folds_into_cookie_header() {
        // An `in: cookie` operation parameter must be sent as a `Cookie` header,
        // not appended to the query string (Codex #106).
        let (port, rx) = mock_server(200, r#"{"ok":true}"#);
        let home = tempfile::tempdir().unwrap();
        let agents = home.path().join("agents");
        let dir = agents.join("svc");
        std::fs::create_dir_all(&dir).unwrap();
        let manifest = r#"agent: svc
version: 0.1.0
description: svc
stateful: false
license: MIT
transport:
  rest:
    base: http://127.0.0.1:PORT
commands:
  get-thing:
    lifecycle: single
    description: "GET /thing"
    method: GET
    path: /thing
    inputs:
      sid:
        type: string
        in: cookie
"#
        .replace("PORT", &port.to_string());
        std::fs::write(dir.join("manifest.yaml"), manifest).unwrap();

        let inv = RestInvoker { agents_dir: agents };
        let _ = inv
            .invoke_single("svc", "get-thing", serde_json::json!({ "sid": "abc-9" }))
            .await
            .unwrap();
        let req = rx.recv().unwrap();
        assert!(
            req.contains("Cookie: sid=abc-9"),
            "cookie param should be a Cookie header: {req}"
        );
        assert!(
            req.starts_with("GET /thing "),
            "cookie param must not leak into the query string: {req}"
        );
    }

    #[tokio::test]
    async fn cookie_auth_appends_to_operation_cookie_param() {
        // apiKey `in: cookie` auth + an operation cookie param must coexist in
        // one `Cookie` header — auth appends rather than being skipped (#106).
        let (port, rx) = mock_server(200, r#"{"ok":true}"#);
        let home = tempfile::tempdir().unwrap();
        let agents = home.path().join("agents");
        let dir = agents.join("svc");
        std::fs::create_dir_all(&dir).unwrap();
        let manifest = r#"agent: svc
version: 0.1.0
description: svc
stateful: false
license: MIT
transport:
  rest:
    base: http://127.0.0.1:PORT
auth:
  scheme: api-key
  in: cookie
  name: token
  secret: svc
requires:
  secrets:
    - svc
commands:
  get-thing:
    lifecycle: single
    description: "GET /thing"
    method: GET
    path: /thing
    inputs:
      sid:
        type: string
        in: cookie
"#
        .replace("PORT", &port.to_string());
        std::fs::write(dir.join("manifest.yaml"), manifest).unwrap();
        let creds = home.path().join("credentials");
        std::fs::create_dir_all(&creds).unwrap();
        std::fs::write(creds.join("svc.json"), r#"{"key":"tok-9"}"#).unwrap();

        let inv = RestInvoker { agents_dir: agents };
        let _ = inv
            .invoke_single("svc", "get-thing", serde_json::json!({ "sid": "abc" }))
            .await
            .unwrap();
        let req = rx.recv().unwrap();
        assert!(req.contains("sid=abc"), "cookie param missing: {req}");
        assert!(req.contains("token=tok-9"), "auth cookie missing: {req}");
    }

    #[tokio::test]
    async fn declared_auth_with_missing_credential_errors_clearly() {
        // An agent that declares `auth:` but whose credential isn't provisioned
        // must fail fast with an actionable error, not send an unauthenticated
        // request and get a remote 401 (#106).
        let home = tempfile::tempdir().unwrap();
        let agents = home.path().join("agents");
        let dir = agents.join("secured");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("manifest.yaml"),
            r#"agent: secured
version: 0.1.0
description: x
stateful: false
license: MIT
transport:
  rest: {}
auth:
  scheme: bearer
  secret: secured
requires:
  secrets:
    - secured
commands:
  get:
    lifecycle: single
    description: GET
"#,
        )
        .unwrap();
        // Deliberately no credentials/secured.json.
        let inv = RestInvoker { agents_dir: agents };
        let err = inv
            .invoke_single(
                "secured",
                "get",
                serde_json::json!({ "url": "http://127.0.0.1:1/x" }),
            )
            .await
            .unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)), "got: {err:?}");
        assert!(
            format!("{err}").contains("credential"),
            "error should name the missing credential: {err}"
        );
    }

    #[tokio::test]
    async fn operation_array_query_param_repeats() {
        // An `in: query` array parameter expands to repeated entries
        // (form/explode default), e.g. `tags=a&tags=b` (#106).
        let (port, rx) = mock_server(200, r#"{"ok":true}"#);
        let home = tempfile::tempdir().unwrap();
        let agents = home.path().join("agents");
        let dir = agents.join("svc");
        std::fs::create_dir_all(&dir).unwrap();
        let manifest = r#"agent: svc
version: 0.1.0
description: svc
stateful: false
license: MIT
transport:
  rest:
    base: http://127.0.0.1:PORT
commands:
  list:
    lifecycle: single
    description: "GET /pets"
    method: GET
    path: /pets
    inputs:
      tags:
        type: array<string>
        in: query
"#
        .replace("PORT", &port.to_string());
        std::fs::write(dir.join("manifest.yaml"), manifest).unwrap();

        let inv = RestInvoker { agents_dir: agents };
        let _ = inv
            .invoke_single("svc", "list", serde_json::json!({ "tags": ["a", "b"] }))
            .await
            .unwrap();
        let req = rx.recv().unwrap();
        assert!(req.contains("tags=a"), "first array item missing: {req}");
        assert!(req.contains("tags=b"), "second array item missing: {req}");
    }

    #[tokio::test]
    async fn no_auth_command_skips_auth_even_without_credential() {
        // A `no-auth` (public) command on an auth-declaring agent must neither
        // inject the credential nor fail-fast when it's unprovisioned (#106).
        let (port, rx) = mock_server(200, r#"{"ok":true}"#);
        let home = tempfile::tempdir().unwrap();
        let agents = home.path().join("agents");
        let dir = agents.join("svc");
        std::fs::create_dir_all(&dir).unwrap();
        let manifest = r#"agent: svc
version: 0.1.0
description: svc
stateful: false
license: MIT
transport:
  rest:
    base: http://127.0.0.1:PORT
auth:
  scheme: bearer
  secret: svc
requires:
  secrets:
    - svc
commands:
  get-health:
    lifecycle: single
    description: "GET /health"
    method: GET
    path: /health
    no-auth: true
"#
        .replace("PORT", &port.to_string());
        std::fs::write(dir.join("manifest.yaml"), manifest).unwrap();
        // Deliberately no credentials/svc.json.

        let inv = RestInvoker { agents_dir: agents };
        let out = inv
            .invoke_single("svc", "get-health", serde_json::json!({}))
            .await
            .unwrap();
        assert_eq!(out["status"], serde_json::json!(200));
        let req = rx.recv().unwrap();
        assert!(
            !req.contains("Authorization"),
            "no-auth command must not inject auth: {req}"
        );
    }

    #[tokio::test]
    async fn missing_required_path_param_errors() {
        // Omitting a `{petId}` path parameter must be a clear validation error,
        // not a malformed URL with `{petId}` left in it (Codex #106).
        let home = tempfile::tempdir().unwrap();
        let agents = home.path().join("agents");
        let dir = agents.join("svc");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("manifest.yaml"),
            r#"agent: svc
version: 0.1.0
description: svc
stateful: false
license: MIT
transport:
  rest:
    base: http://127.0.0.1:1
commands:
  get-pet:
    lifecycle: single
    description: "GET /pets/{petId}"
    method: GET
    path: /pets/{petId}
    inputs:
      petId:
        type: integer
        in: path
        required: true
"#,
        )
        .unwrap();
        let inv = RestInvoker { agents_dir: agents };
        let err = inv
            .invoke_single("svc", "get-pet", serde_json::json!({}))
            .await
            .unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)), "got: {err:?}");
        assert!(
            format!("{err}").contains("petId"),
            "error should name the missing path param: {err}"
        );
    }

    #[tokio::test]
    async fn query_param_named_body_is_routed_by_location() {
        // A query parameter literally named `body` (declared `in: query`) must be
        // sent as a query param, not dropped or treated as the request body (#106).
        let (port, rx) = mock_server(200, r#"{"ok":true}"#);
        let home = tempfile::tempdir().unwrap();
        let agents = home.path().join("agents");
        let dir = agents.join("svc");
        std::fs::create_dir_all(&dir).unwrap();
        let manifest = r#"agent: svc
version: 0.1.0
description: svc
stateful: false
license: MIT
transport:
  rest:
    base: http://127.0.0.1:PORT
commands:
  search:
    lifecycle: single
    description: "GET /search"
    method: GET
    path: /search
    inputs:
      body:
        type: string
        in: query
"#
        .replace("PORT", &port.to_string());
        std::fs::write(dir.join("manifest.yaml"), manifest).unwrap();

        let inv = RestInvoker { agents_dir: agents };
        let _ = inv
            .invoke_single("svc", "search", serde_json::json!({ "body": "hello" }))
            .await
            .unwrap();
        let req = rx.recv().unwrap();
        assert!(
            req.starts_with("GET /search?body=hello"),
            "query param named `body` should be in the query string: {req}"
        );
    }
}

#[cfg(test)]
mod builtin_invoker_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn render_returns_html_aliases_and_count() {
        let out =
            render_html_report(json!({ "title": "T", "data": [{ "id": 1 }] }), false).unwrap();
        let html = out["html"].as_str().unwrap();
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("<title>T</title>"));
        // `htmlReport` mirrors `html` (styling contract); count is the array length.
        assert_eq!(out["htmlReport"], out["html"]);
        assert_eq!(out["item-count"], json!(1));
        // No output-path → pure formatter, no file keys.
        assert!(out.get("output-path").is_none());
    }

    #[test]
    fn render_dry_run_echoes_path_but_skips_write() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("r.html");
        let ps = path.to_string_lossy().to_string();
        let out =
            render_html_report(json!({ "data": [], "output-path": ps.clone() }), true).unwrap();
        assert_eq!(out["output-path"].as_str().unwrap(), ps);
        assert!(out["bytes"].as_u64().unwrap() > 0, "would-be size reported");
        assert!(!path.exists(), "a dry run must not write the file");
    }

    #[test]
    fn render_real_run_writes_the_file_and_creates_parents() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("sub").join("r.html"); // parent dir is created
        let ps = path.to_string_lossy().to_string();
        let out =
            render_html_report(json!({ "data": [{ "a": 1 }], "output-path": ps }), false).unwrap();
        assert!(path.exists(), "a real run writes the file");
        let written = std::fs::read_to_string(&path).unwrap();
        assert_eq!(written, out["html"].as_str().unwrap());
    }

    #[tokio::test]
    async fn builtin_stream_is_rejected() {
        let err = BuiltinInvoker { dry_run: false }
            .invoke_stream("html-report", "render", json!({}))
            .await
            .unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }

    #[tokio::test]
    async fn unknown_builtin_command_errors() {
        let err = BuiltinInvoker { dry_run: false }
            .invoke_single("html-report", "nope", json!({}))
            .await
            .unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }

    #[test]
    fn effective_transport_prioritizes_cli_over_builtin_on_mixed_manifests() {
        // A crafted MIXED-transport manifest (builtin + cli) dispatches as `cli`
        // (the priority order in `effective_transport`), so any builtin-only
        // guard resolving through it must see Cli, not Builtin (#215 review).
        let m: crate::manifest::Agent = serde_yaml::from_str(
            "agent: mixed\nversion: 0.1.0\ndescription: x\nstateful: false\n\
             license: MIT\ntransport:\n  builtin: {}\n  cli:\n    binary: evil\n\
             commands:\n  go: { lifecycle: single, description: x }\n",
        )
        .unwrap();
        assert_eq!(
            effective_transport(&m, "mixed").unwrap(),
            TransportKind::Cli
        );

        // Builtin-only manifests still resolve to Builtin…
        let m: crate::manifest::Agent = serde_yaml::from_str(
            "agent: pure\nversion: 0.1.0\ndescription: x\nstateful: false\n\
             license: MIT\ntransport:\n  builtin: {}\n\
             commands:\n  go: { lifecycle: single, description: x }\n",
        )
        .unwrap();
        assert_eq!(
            effective_transport(&m, "pure").unwrap(),
            TransportKind::Builtin
        );

        // …and a manifest with no executable transport is a clear error.
        let m: crate::manifest::Agent = serde_yaml::from_str(
            "agent: none\nversion: 0.1.0\ndescription: x\nstateful: false\n\
             license: MIT\ntransport: {}\n\
             commands:\n  go: { lifecycle: single, description: x }\n",
        )
        .unwrap();
        let err = effective_transport(&m, "none").unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)), "got: {err:?}");
        assert!(format!("{err}").contains("no executable transport"));
    }

    #[tokio::test]
    async fn dispatch_builtin_render_honors_preview_no_write() {
        // A preview run (--dry-run / --simulate) must not write the output-path file
        // even for a builtin reached through dispatch — including a nested invoker
        // where `app_ctx` is None (the #201 Codex finding). `preview` carries the
        // posture independently of `app_ctx`.
        let tmp = tempfile::tempdir().unwrap();
        let agents = tmp.path().join("agents");
        let dir = agents.join("html-report");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("manifest.yaml"),
            "agent: html-report\nversion: 0.2.0\ndescription: x\nstateful: false\n\
             license: MIT\ntransport:\n  builtin: {}\ncommands:\n  render:\n    \
             lifecycle: single\n    description: x\n",
        )
        .unwrap();
        let out = tmp.path().join("r.html");
        let inv = DispatchInvoker {
            agents_dir: agents,
            app_ctx: None, // nested-invoker shape
            preview: true,
        };
        let res = inv
            .invoke_single(
                "html-report",
                "render",
                json!({ "data": [{ "a": 1 }], "output-path": out.to_string_lossy() }),
            )
            .await
            .unwrap();
        assert!(res["html"].as_str().unwrap().contains("<!DOCTYPE html>"));
        assert!(
            !out.exists(),
            "preview run must not write the file via dispatch"
        );
    }
}
