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
                let cred = load_secret_value(&self.agents_dir, &auth.secret)
                    .as_ref()
                    .and_then(secret_as_str);
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
fn rest_base_url(agents_dir: &std::path::Path, agent: &str) -> Option<String> {
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
fn percent_encode_path(s: &str) -> String {
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
        }
    }

    fn transport_kind(&self, agent: &str) -> Result<TransportKind, AwareError> {
        let m = crate::manifest::loader::load_agent(
            &self.agents_dir.join(agent).join("manifest.yaml"),
        )?;
        if m.transport.cli.is_some() {
            Ok(TransportKind::Cli)
        } else if m.transport.rest.is_some() {
            Ok(TransportKind::Rest)
        } else if m.transport.app.is_some() {
            Ok(TransportKind::App)
        } else {
            Err(AwareError::Validation(format!(
                "agent {agent} has no executable transport (need `cli`, `rest`, or `app`)"
            )))
        }
    }

    /// Resolve the backing app + validate the caller's routed inputs for an
    /// app-backed agent. Shared by the one-shot and streaming dispatch paths.
    fn resolve_exposed(
        &self,
        app_ctx: &AppTransportCtx,
        agent: &str,
        command: &str,
        args: &Value,
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
        // Type-check the caller's routed inputs against the declared contract.
        crate::manifest::expose::validate_exposed_inputs(command, exposed, args)?;
        Ok(app)
    }

    /// Build a leaf invoker for a nested app run — `app_ctx: None` forbids the
    /// nested app from composing yet another exposes-as-agent app (v0 rule).
    fn nested_leaf(&self) -> Arc<dyn AgentInvoker> {
        Arc::new(DispatchInvoker {
            agents_dir: self.agents_dir.clone(),
            app_ctx: None,
        })
    }

    async fn dispatch_app_single(
        &self,
        app_ctx: &AppTransportCtx,
        agent: &str,
        command: &str,
        args: Value,
    ) -> Result<Value, AwareError> {
        let app = self.resolve_exposed(app_ctx, agent, command, &args)?;
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
        args: Value,
    ) -> Result<StreamingHandle, AwareError> {
        let app = self.resolve_exposed(app_ctx, agent, command, &args)?;
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

enum TransportKind {
    Cli,
    Rest,
    App,
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
