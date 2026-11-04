//! Trimble Connect file upload / download (#200).
//!
//! TC files are multi-step, binary, cross-domain flows the single-call REST transport
//! cannot express (see `20-agents/aeco/construction/trimble-connect/skills/files.md`):
//!
//! - **upload** (3-step package upload): `POST files/fs/upload` (auth) to initiate →
//!   `PUT` the bytes to a pre-signed S3 URL (a *different* domain, **no** auth header) →
//!   `GET files/fs/upload?uploadId=…&wait=true` (auth) to complete.
//! - **download** (2-step): `GET files/fs/{id}/downloadurl` (auth) → `GET` the bytes from
//!   the pre-signed URL (**no** auth header).
//!
//! So `trimble-connect`'s `upload`/`download` commands are handled here, dispatched from
//! [`super::invoker`]'s REST path. Auth (with refresh, #198) and the base URL resolve
//! exactly as the generic REST path does. Binary travels as base64 over JSON.

use std::io::Read;
use std::time::Duration;

use base64::Engine;
use serde_json::{Value, json};

use crate::agent_resolution::AgentCatalogue;
use crate::error::AwareError;
use crate::runtime::invoker::{
    alias_credential_handle, percent_encode_path, resolve_rest_credential, rest_base_url,
};
use crate::runtime::trimble_region::{region_arg, resolve_core, with_region};

const TC_AGENT: &str = "trimble-connect";

/// HTTP agent with connect + read (inactivity) timeouts, so an unresponsive TC / S3
/// endpoint can't hang the blocking worker thread forever (review). `timeout_read` is a
/// per-read inactivity bound, so a steady large-file transfer never trips it.
pub(super) fn http_agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(15))
        .timeout_read(Duration::from_secs(120))
        .build()
}

/// `trimble-connect.upload` — 3-step package upload. Blocking HTTP runs off the reactor.
pub async fn upload(
    catalogue: impl Into<AgentCatalogue>,
    args: Value,
) -> Result<Value, AwareError> {
    let catalogue = catalogue.into();
    tokio::task::spawn_blocking(move || upload_blocking(&catalogue, &args))
        .await
        .map_err(|e| AwareError::Internal(format!("trimble upload task join: {e}")))?
}

/// `trimble-connect.download` — 2-step pre-signed download. Blocking HTTP off the reactor.
pub async fn download(
    catalogue: impl Into<AgentCatalogue>,
    args: Value,
) -> Result<Value, AwareError> {
    let catalogue = catalogue.into();
    tokio::task::spawn_blocking(move || download_blocking(&catalogue, &args))
        .await
        .map_err(|e| AwareError::Internal(format!("trimble download task join: {e}")))?
}

/// Resolve the (refreshed, #198) bearer token + trimmed base URL for trimble-connect.
/// The manifest is the run's resolved one (#626); `agents_dir()` only names
/// AWARE_HOME for the credential.
pub(super) fn auth_and_base(catalogue: &AgentCatalogue) -> Result<(String, String), AwareError> {
    auth_and_base_for(catalogue, None)
}

/// [`auth_and_base`] for `aware agent invoke --as <alias>` (floless.app#2466):
/// that slot and no other. A slot that is not connected refuses here, before
/// any request, exactly as the generic REST path does — never the default
/// account in its place.
pub(super) fn auth_and_base_for(
    catalogue: &AgentCatalogue,
    alias: Option<&str>,
) -> Result<(String, String), AwareError> {
    let manifest = catalogue.manifest(TC_AGENT)?;
    let mut auth = manifest.auth.clone().ok_or_else(|| {
        AwareError::Validation("trimble-connect: manifest has no `auth:` block".into())
    })?;
    if let Some(alias) = alias {
        auth.secret = alias_credential_handle(TC_AGENT, &auth, alias, catalogue.agents_dir())?;
    }
    let token = resolve_rest_credential(catalogue.agents_dir(), &auth).ok_or_else(|| {
        AwareError::Validation(
            "trimble-connect declares `auth` but the credential is missing — \
             run `aware connect trimble-connect`"
                .into(),
        )
    })?;
    let base = rest_base_url(catalogue, TC_AGENT).ok_or_else(|| {
        AwareError::Validation("trimble-connect: no `transport.rest.base`".into())
    })?;
    Ok((token, base.trim_end_matches('/').to_string()))
}

fn upload_blocking(catalogue: &AgentCatalogue, args: &Value) -> Result<Value, AwareError> {
    let region = region_arg(args)?;
    let (token, master) = auth_and_base(catalogue)?;
    let (base, location) = resolve_core(&master, &token, region, None)?;
    upload_to(&base, &token, args).map(|out| with_region(out, location.as_deref()))
}

fn upload_to(base: &str, token: &str, args: &Value) -> Result<Value, AwareError> {
    let token = token.to_string();
    let agent = http_agent();
    let folder_id = str_arg(args, "folder-id")?;
    let filename = str_arg(args, "filename")?;
    let bytes = extract_bytes(args.get("bytes"))?;

    // Step 1 — initiate. `parentId`/`parentType` are QUERY params; the body is just
    // name + a single empty content slot (per files.md; multipart returns 415).
    let initiate_url = format!(
        "{base}/files/fs/upload?parentId={}&parentType=FOLDER",
        percent_encode_path(folder_id)
    );
    let init = post_json(
        &agent,
        &initiate_url,
        &token,
        &json!({ "name": filename, "contents": [ {} ] }),
        "upload initiate",
    )?;
    // A content-identical file already present: TC returns DUPLICATE + the fileId.
    if init.get("status").and_then(|v| v.as_str()) == Some("DUPLICATE") {
        let file_id = init
            .get("fileId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AwareError::Network("upload DUPLICATE: no fileId".into()))?
            .to_string();
        // The Files API only guarantees fileId on DUPLICATE, but the result contract
        // promises a usable version-id — fetch it from the file metadata when the
        // initiate response doesn't carry it (#200 Codex).
        let version_id = match init.get("versionId").and_then(|v| v.as_str()) {
            Some(v) => v.to_string(),
            None => {
                let meta = get_json(
                    &agent,
                    &format!("{base}/files/{}", percent_encode_path(&file_id)),
                    Some(&token),
                    "duplicate file metadata",
                )?;
                meta.get("versionId")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        AwareError::Network("duplicate file metadata: no versionId".into())
                    })?
                    .to_string()
            }
        };
        return Ok(json!({
            "file-id": file_id,
            "version-id": version_id,
            "replaced": true,
            "status": "DUPLICATE",
        }));
    }
    let upload_id = init
        .get("uploadId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AwareError::Network("upload initiate: no uploadId".into()))?;
    let presigned = init
        .get("contents")
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("url"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| AwareError::Network("upload initiate: no pre-signed url".into()))?;

    // Step 2 — PUT bytes to the pre-signed S3 URL (different domain — NO auth header).
    // The same bytes to the same URL: safe to repeat.
    fetch(true, MAX_JSON_BYTES, "upload PUT", || {
        agent
            .put(presigned)
            .set("Content-Type", "application/octet-stream")
            .send_bytes(&bytes)
            .map_err(Box::new)
    })?;

    // Step 3 — complete.
    let complete_url = format!(
        "{base}/files/fs/upload?uploadId={}&wait=true",
        percent_encode_path(upload_id)
    );
    let complete = get_json(&agent, &complete_url, Some(&token), "upload complete")?;
    // Fail fast on an incomplete 2xx (e.g. a status-only response) rather than
    // reporting success with empty identifiers downstream nodes would persist (#200 Codex).
    let file_id = complete
        .get("fileId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AwareError::Network("upload complete: no fileId".into()))?;
    let version_id = complete
        .get("versionId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AwareError::Network("upload complete: no versionId".into()))?;
    Ok(json!({
        "file-id": file_id,
        "version-id": version_id,
        "replaced": false,
    }))
}

fn download_blocking(catalogue: &AgentCatalogue, args: &Value) -> Result<Value, AwareError> {
    let region = region_arg(args)?;
    let (token, master) = auth_and_base(catalogue)?;
    let (base, location) = resolve_core(&master, &token, region, None)?;
    download_from(&base, &token, args).map(|out| with_region(out, location.as_deref()))
}

fn download_from(base: &str, token: &str, args: &Value) -> Result<Value, AwareError> {
    let token = token.to_string();
    let agent = http_agent();
    let file_id = str_arg(args, "file-id")?;

    // Step 1 — pre-signed download URL (optionally for a specific version).
    let mut url_endpoint = format!(
        "{base}/files/fs/{}/downloadurl",
        percent_encode_path(file_id)
    );
    if let Some(ver) = args
        .get("version-id")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
    {
        url_endpoint.push_str(&format!("?versionId={}", percent_encode_path(ver)));
    }
    let url_json = get_json(&agent, &url_endpoint, Some(&token), "download url")?;
    let presigned = url_json
        .get("url")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AwareError::Network("download: no pre-signed url".into()))?;

    // Step 2 — GET the bytes from the pre-signed URL (NO auth header). A body
    // that breaks off mid-read repeats the GET, like a dropped request.
    let bytes = fetch(true, u64::MAX, "download GET", || {
        agent.get(presigned).call().map_err(Box::new)
    })?
    .body;
    Ok(json!({
        "bytes": base64::engine::general_purpose::STANDARD.encode(&bytes),
        "encoding": "base64",
        "size-kb": (bytes.len() as f64) / 1024.0,
    }))
}

// ── helpers ──────────────────────────────────────────────────────────────────

pub(super) fn str_arg<'a>(args: &'a Value, key: &str) -> Result<&'a str, AwareError> {
    args.get(key)
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            AwareError::Validation(format!("trimble-connect: missing required input `{key}`"))
        })
}

/// Decode the `bytes` input into raw file content. The convention (manifest + docs) is a
/// **base64 string**; whitespace and the URL-safe alphabet (padded or not) are tolerated,
/// but an invalid base64 string is rejected rather than silently uploaded as literal text
/// (review). A JSON array is taken as raw bytes (each element an integer 0-255).
pub(super) fn extract_bytes(v: Option<&Value>) -> Result<Vec<u8>, AwareError> {
    use base64::engine::general_purpose::{STANDARD, STANDARD_NO_PAD, URL_SAFE, URL_SAFE_NO_PAD};
    match v {
        Some(Value::String(s)) => {
            let cleaned: String = s.chars().filter(|c| !c.is_ascii_whitespace()).collect();
            STANDARD
                .decode(&cleaned)
                .or_else(|_| URL_SAFE.decode(&cleaned))
                .or_else(|_| STANDARD_NO_PAD.decode(&cleaned))
                .or_else(|_| URL_SAFE_NO_PAD.decode(&cleaned))
                .map_err(|_| {
                    AwareError::Validation(
                        "trimble-connect upload: `bytes` must be a base64 string".into(),
                    )
                })
        }
        // A byte array must be integers 0–255 — reject anything else rather than
        // silently dropping/truncating it into corrupted content (#200 Codex).
        Some(Value::Array(items)) => items
            .iter()
            .map(|n| {
                n.as_u64()
                    .filter(|&b| b <= 255)
                    .map(|b| b as u8)
                    .ok_or_else(|| {
                        AwareError::Validation(format!(
                            "trimble-connect upload: byte array element {n} is not an integer 0-255"
                        ))
                    })
            })
            .collect(),
        _ => Err(AwareError::Validation(
            "trimble-connect upload: missing required input `bytes`".into(),
        )),
    }
}

/// Attempts per TC request — the agent's rate-limit contract
/// (`skills/rate-limits.md`): immediately, then after 1 s, then after 2 s, or
/// after the response's `Retry-After` (capped at 30 s).
pub(super) const MAX_ATTEMPTS: u32 = 3;
const BACKOFF: Duration = Duration::from_secs(1);
const MAX_WAIT: Duration = Duration::from_secs(30);

/// A JSON response larger than this is refused rather than read into memory.
pub(super) const MAX_JSON_BYTES: u64 = 64 * 1024 * 1024;

/// One request's outcome as [`fetch`] takes it — ureq's error boxed, since
/// `ureq::Error` is large and every request closure returns one.
pub(super) type Sent = Result<ureq::Response, Box<ureq::Error>>;

/// A TC response read in full — status, headers and body — by [`fetch_any`].
pub(super) struct Fetched {
    status: u16,
    headers: Vec<(String, String)>,
    pub(super) body: Vec<u8>,
    attempts: u32,
}

impl Fetched {
    pub(super) fn status(&self) -> u16 {
        self.status
    }

    /// A response header, by case-insensitive name.
    pub(super) fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }

    /// The body as JSON; `None` for a 204 or an empty body.
    pub(super) fn json_opt(&self, what: &str) -> Result<Option<Value>, AwareError> {
        if self.status == 204 || self.body.iter().all(u8::is_ascii_whitespace) {
            return Ok(None);
        }
        serde_json::from_slice(&self.body)
            .map(Some)
            .map_err(|e| AwareError::Network(format!("{what}: bad JSON: {e}")))
    }

    /// The body as JSON, which must be there.
    pub(super) fn json(&self, what: &str) -> Result<Value, AwareError> {
        self.json_opt(what)?
            .ok_or_else(|| AwareError::Network(format!("{what}: TC answered with no content")))
    }

    /// This response as a failure: the step (with the attempt count), the
    /// status — and for a `429` the `Retry-After` TC asked for, so throttling is
    /// told apart from any other failure — and TC's own message.
    pub(super) fn failure(&self, what: &str) -> AwareError {
        let what = after_attempts(what, self.attempts);
        let body = String::from_utf8_lossy(&self.body);
        let code = self.status;
        let error = if (300..400).contains(&code) {
            AwareError::Network(format!(
                "{what}: HTTP {code} — redirects are not followed (location: {})",
                self.header("Location").unwrap_or("none")
            ))
        } else {
            AwareError::Network(match self.header("Retry-After").filter(|_| code == 429) {
                Some(after) => {
                    format!("{what}: HTTP {code} (rate limited; Retry-After: {after}): {body}")
                }
                None => format!("{what}: HTTP {code}: {body}"),
            })
        };
        direct::mark(error, direct::Outcome::Status(code))
    }
}

/// One request through the retry contract, **its body included**: a response
/// counts only once its body has been read, so a connection that drops while
/// the body streams is retried exactly like one that drops before the headers,
/// from the same attempt budget — at most [`MAX_ATTEMPTS`] requests in all.
///
/// A `429` is retried whatever the method — TC did not process the request. A
/// `5xx`, a transport error or a body cut off mid-read is retried only when
/// `repeatable`: a GET, a PUT or a PATCH may be sent again, but a POST may
/// already have created something and a repeated DELETE would report the first
/// one's success as a 404, so those surface the fault instead. A body larger
/// than `limit` is refused, not retried.
///
/// Returns the final response whatever its status, so a caller can read a 404
/// as an answer; [`fetch`] turns a 4xx/5xx into an error.
pub(super) fn fetch_any(
    repeatable: bool,
    limit: u64,
    what: &str,
    mut once: impl FnMut() -> Sent,
) -> Result<Fetched, AwareError> {
    let wait = |attempt: u32, after: Option<Duration>| {
        let backoff = BACKOFF * 2u32.pow(attempt - 1);
        std::thread::sleep(after.unwrap_or(backoff).min(MAX_WAIT));
    };
    let mut attempt = 1;
    loop {
        let limit = direct::admit(limit)?;
        let resp = match once().map_err(|e| *e) {
            Ok(r) | Err(ureq::Error::Status(_, r)) => r,
            Err(ureq::Error::Transport(t)) => {
                if repeatable && attempt < MAX_ATTEMPTS {
                    wait(attempt, None);
                    attempt += 1;
                    continue;
                }
                return Err(AwareError::Network(format!(
                    "{}: {t}",
                    after_attempts(what, attempt)
                )));
            }
        };
        let status = resp.status();
        if (status == 429 || (repeatable && (500..=599).contains(&status)))
            && attempt < MAX_ATTEMPTS
        {
            wait(attempt, retry_after_of(&resp));
            attempt += 1;
            continue;
        }
        let headers = resp
            .headers_names()
            .into_iter()
            .filter_map(|n| resp.header(&n).map(|v| (n.clone(), v.to_string())))
            .collect();
        let mut body = Vec::new();
        match resp
            .into_reader()
            .take(limit.saturating_add(1))
            .read_to_end(&mut body)
        {
            Ok(_) if body.len() as u64 > limit => {
                return Err(direct::mark(
                    AwareError::Network(format!(
                        "{}: the response is larger than {limit} bytes",
                        after_attempts(what, attempt)
                    )),
                    direct::Outcome::TooLarge,
                ));
            }
            Ok(_) => {
                direct::spend(body.len() as u64)?;
                return Ok(Fetched {
                    status,
                    headers,
                    body,
                    attempts: attempt,
                });
            }
            // A body cut off mid-read still cost what arrived.
            Err(_) if repeatable && attempt < MAX_ATTEMPTS => {
                direct::spend(body.len() as u64)?;
                wait(attempt, None);
                attempt += 1;
            }
            Err(e) => {
                direct::spend(body.len() as u64)?;
                let error = AwareError::Network(format!(
                    "{}: read body: {e}",
                    after_attempts(what, attempt)
                ));
                // A failure's status arrived before its body broke off: keep it.
                return Err(if (200..300).contains(&status) {
                    error
                } else {
                    direct::mark(error, direct::Outcome::Status(status))
                });
            }
        }
    }
}

/// [`fetch_any`], with anything but a 2xx (after any retries) as an error. A
/// 3xx included: the no-redirect clients hand one back unfollowed, and its
/// body is not the answer that was asked for (floless.app#2466).
pub(super) fn fetch(
    repeatable: bool,
    limit: u64,
    what: &str,
    once: impl FnMut() -> Sent,
) -> Result<Fetched, AwareError> {
    let f = fetch_any(repeatable, limit, what, once)?;
    if !(200..300).contains(&f.status) {
        return Err(f.failure(what));
    }
    Ok(f)
}

/// The bounds and the failure boundary of a Trimble read run from `aware agent
/// invoke` (floless.app#2466). A workflow node runs under none of this.
///
/// Every request of one direct call goes through [`fetch_any`] on the one
/// blocking thread that runs it, so a thread-local budget bounds the whole
/// call — region discovery, every page of a BCF collection, retries and bodies
/// cut off mid-read included — without threading a limit through each helper:
/// at most [`MAX_REQUESTS`] requests and [`MAX_BYTES`] of response bodies.
/// Time is bounded by one absolute deadline, [`DEADLINE`] after the call
/// entered `invoke` ([`Call::starting_now`]), shared by everything below it:
/// [`bounded`] answers the caller `E_CALL_TIMEOUT` at that instant whatever a
/// request in flight is doing, and the blocking task — which a timed-out
/// caller cannot cancel — admits no request and accepts no answer after it.
/// Credential refresh and queueing for a blocking thread spend the same clock.
///
/// A failure leaves as AWARE's own sentence and code. Where it came from is
/// recorded as a typed [`Outcome`] at the point AWARE raised it ([`mark`]) —
/// never read back out of the error text, which can hold the service's words.
pub(crate) mod direct {
    use std::cell::RefCell;
    use std::time::{Duration, Instant};

    use crate::error::AwareError;

    pub(crate) const MAX_REQUESTS: u32 = 40;
    pub(crate) const MAX_BYTES: u64 = 8 * 1024 * 1024;
    pub(crate) const DEADLINE: Duration = Duration::from_secs(60);

    /// One direct call: its `--as` slot and its absolute deadline.
    #[derive(Debug, Clone)]
    pub(crate) struct Call {
        pub(crate) alias: Option<String>,
        pub(crate) deadline: Instant,
    }

    impl Call {
        /// The call `invoke` was asked for, its clock starting now.
        pub(crate) fn starting_now(call: &crate::runtime::invoker::DirectRestCall) -> Self {
            Self {
                alias: call.alias.clone(),
                deadline: Instant::now() + DEADLINE,
            }
        }
    }

    /// What a marked failure was, in AWARE's terms.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) enum Outcome {
        /// The service answered with this status (not a 2xx).
        Status(u16),
        /// Every region answered 404 for the project.
        ProjectNotFound,
        /// A request, byte or response-size bound.
        TooLarge,
        Timeout,
    }

    struct Budget {
        requests: u32,
        bytes: u64,
        deadline: Instant,
        /// The errors AWARE raised, by their exact text, with what they were.
        marked: Vec<(String, Outcome)>,
    }

    thread_local! {
        static BUDGET: RefCell<Option<Budget>> = const { RefCell::new(None) };
    }

    /// Run `body` (a whole direct call, on its own blocking thread) under a
    /// fresh budget ending at `deadline`, then remove it, also when `body`
    /// panics. Its error, and an answer that arrived after the deadline, come
    /// back in AWARE's words.
    pub(crate) fn run<T>(
        deadline: Instant,
        body: impl FnOnce() -> Result<T, AwareError>,
    ) -> Result<T, AwareError> {
        struct Reset;
        impl Drop for Reset {
            fn drop(&mut self) {
                BUDGET.with(|b| *b.borrow_mut() = None);
            }
        }
        BUDGET.with(|b| {
            *b.borrow_mut() = Some(Budget {
                requests: 0,
                bytes: 0,
                deadline,
                marked: Vec::new(),
            })
        });
        let _reset = Reset;
        let result = body();
        let late = Instant::now() > deadline;
        match result {
            Ok(_) if late => Err(said(Outcome::Timeout)),
            Ok(value) => Ok(value),
            Err(error) => {
                let text = error.to_string();
                let outcome = BUDGET.with(|b| {
                    b.borrow().as_ref().and_then(|b| {
                        b.marked
                            .iter()
                            .rev()
                            .find(|(marked, _)| *marked == text)
                            .map(|(_, outcome)| *outcome)
                    })
                });
                Err(match outcome {
                    Some(outcome) => said(outcome),
                    None if late => said(Outcome::Timeout),
                    None => AwareError::Network(
                        "The service answered the call with a failure. [E_CALL_FAILED]".into(),
                    ),
                })
            }
        }
    }

    /// `call` (the blocking task of a direct read), answered by `deadline`:
    /// past it the caller gets `E_CALL_TIMEOUT`, and the task, which admits
    /// no request after the same deadline, is left to finish on its own.
    pub(crate) async fn bounded<T>(
        deadline: Instant,
        call: impl std::future::Future<Output = Result<T, AwareError>>,
    ) -> Result<T, AwareError> {
        tokio::time::timeout_at(tokio::time::Instant::from_std(deadline), call)
            .await
            .unwrap_or_else(|_| Err(said(Outcome::Timeout)))
    }

    /// Record what `error` was, inside a direct call; outside one (a workflow
    /// node) nothing is recorded. Returns `error` unchanged.
    pub(crate) fn mark(error: AwareError, outcome: Outcome) -> AwareError {
        BUDGET.with(|b| {
            if let Some(budget) = b.borrow_mut().as_mut() {
                budget.marked.push((error.to_string(), outcome));
            }
        });
        error
    }

    /// Admit one more request: refuse it past the request cap or the deadline,
    /// and shrink `limit` to what is left of the byte budget. No budget (a
    /// workflow node) admits everything unchanged.
    pub(super) fn admit(limit: u64) -> Result<u64, AwareError> {
        let refused = BUDGET.with(|b| {
            let mut b = b.borrow_mut();
            let budget = b.as_mut()?;
            if Instant::now() > budget.deadline {
                return Some(Outcome::Timeout);
            }
            if budget.requests >= MAX_REQUESTS || budget.bytes >= MAX_BYTES {
                return Some(Outcome::TooLarge);
            }
            budget.requests += 1;
            None
        });
        match refused {
            Some(outcome) => Err(mark(bound(outcome), outcome)),
            None => Ok(BUDGET.with(|b| {
                b.borrow()
                    .as_ref()
                    .map_or(limit, |b| limit.min(MAX_BYTES - b.bytes))
            })),
        }
    }

    /// Count response bytes against the budget — a whole body, or what
    /// arrived of one cut off — and refuse once it is spent past.
    pub(super) fn spend(bytes: u64) -> Result<(), AwareError> {
        let over = BUDGET.with(|b| {
            b.borrow_mut().as_mut().is_some_and(|budget| {
                budget.bytes = budget.bytes.saturating_add(bytes);
                budget.bytes > MAX_BYTES
            })
        });
        if over {
            return Err(mark(bound(Outcome::TooLarge), Outcome::TooLarge));
        }
        Ok(())
    }

    /// The budget's own refusal, as AWARE words it for a workflow's log.
    fn bound(outcome: Outcome) -> AwareError {
        AwareError::Network(match outcome {
            Outcome::Timeout => format!("the call took longer than {} s", DEADLINE.as_secs()),
            _ => format!("the call needed more than {MAX_REQUESTS} requests or {MAX_BYTES} bytes"),
        })
    }

    /// AWARE's sentence and code for `outcome`, the only failure text a direct
    /// read returns.
    fn said(outcome: Outcome) -> AwareError {
        AwareError::Network(match outcome {
            Outcome::TooLarge => {
                "The service's answer exceeded the size limit. [E_CALL_OUTPUT_TOO_LARGE]".into()
            }
            Outcome::Timeout => {
                "The call did not finish before the deadline. [E_CALL_TIMEOUT]".into()
            }
            Outcome::Status(code) => format!(
                "The service answered the call with a failure. [E_CALL_FAILED status={code}]"
            ),
            Outcome::ProjectNotFound => "The service answered the call with a failure. \
                 [E_CALL_FAILED status=404 reason=project-not-found]"
                .into(),
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn soon() -> Instant {
            Instant::now() + DEADLINE
        }

        /// The deadline is the one set when the call entered `invoke`: once it
        /// has passed — spent queueing, refreshing a credential or waiting out
        /// a Retry-After — the task sends nothing more and its answer is
        /// discarded.
        #[test]
        fn nothing_is_sent_or_accepted_after_the_deadline_set_at_entry() {
            let entered = Instant::now() + Duration::from_millis(20);
            std::thread::sleep(Duration::from_millis(40));
            let out = run(entered, || admit(10)).unwrap_err().to_string();
            assert!(out.ends_with("[E_CALL_TIMEOUT]"), "{out}");
            let late = run(entered, || Ok::<_, AwareError>(()))
                .unwrap_err()
                .to_string();
            assert!(late.ends_with("[E_CALL_TIMEOUT]"), "{late}");
        }

        #[test]
        fn a_failure_says_only_what_aware_recorded_never_what_the_service_said() {
            // A status AWARE recorded survives; the body it carried does not.
            let out = run::<()>(soon(), || {
                Err(mark(
                    AwareError::Network("list topics: HTTP 401: SECRET-BODY".into()),
                    Outcome::Status(401),
                ))
            })
            .unwrap_err()
            .to_string();
            assert!(out.ends_with("[E_CALL_FAILED status=401]"), "{out}");
            assert!(!out.contains("SECRET"), "{out}");
            // Service text that merely LOOKS like a status or a bound is not one:
            // nothing recorded it, so it is a plain failure.
            for forged in [
                "find item: unknown type \"HTTP 401\"",
                "find item: unknown type \"the call took longer than 60 s\"",
            ] {
                let out = run::<()>(soon(), || Err(AwareError::Network(forged.into())))
                    .unwrap_err()
                    .to_string();
                assert!(out.ends_with("[E_CALL_FAILED]"), "{forged}: {out}");
                assert!(!out.contains("HTTP") && !out.contains("took"), "{out}");
            }
        }

        #[test]
        fn the_budget_bounds_requests_and_bytes_and_is_gone_after_the_call() {
            let out = run(soon(), || {
                for _ in 0..MAX_REQUESTS {
                    admit(10)?;
                }
                admit(10)
            })
            .unwrap_err()
            .to_string();
            assert!(out.contains("E_CALL_OUTPUT_TOO_LARGE"), "{out}");
            run(soon(), || {
                assert_eq!(admit(u64::MAX).unwrap(), MAX_BYTES);
                spend(MAX_BYTES - 5).unwrap();
                assert_eq!(admit(u64::MAX).unwrap(), 5);
                // Bytes of a body cut off count too: past the budget refuses.
                assert!(spend(6).is_err());
                assert!(admit(1).is_err());
                Ok(())
            })
            .unwrap();
            // Outside a direct call (a workflow node) nothing is bounded or recorded.
            for _ in 0..(MAX_REQUESTS * 2) {
                assert_eq!(admit(u64::MAX).unwrap(), u64::MAX);
            }
            spend(u64::MAX).unwrap();
        }

        #[tokio::test]
        async fn a_call_still_running_at_the_limit_is_answered_with_a_timeout() {
            let out = bounded(Instant::now() + Duration::from_millis(50), async {
                tokio::time::sleep(Duration::from_secs(30)).await;
                Ok::<_, AwareError>(())
            })
            .await
            .unwrap_err()
            .to_string();
            assert!(out.ends_with("[E_CALL_TIMEOUT]"), "{out}");
        }
    }
}

/// `what`, saying how many attempts it took when there was more than one.
pub(super) fn after_attempts(what: &str, attempts: u32) -> String {
    if attempts > 1 {
        format!("{what} (after {attempts} attempts)")
    } else {
        what.to_string()
    }
}

/// A `Retry-After` given in seconds (TC's form); an HTTP date is not honoured
/// and the backoff applies instead.
fn retry_after_of(resp: &ureq::Response) -> Option<Duration> {
    resp.header("Retry-After")
        .and_then(|v| v.trim().parse::<u64>().ok())
        .map(Duration::from_secs)
}

pub(super) fn post_json(
    agent: &ureq::Agent,
    url: &str,
    token: &str,
    body: &Value,
    what: &str,
) -> Result<Value, AwareError> {
    fetch(false, MAX_JSON_BYTES, what, || {
        agent
            .post(url)
            .set("Authorization", &format!("Bearer {token}"))
            .set("Content-Type", "application/json")
            .send_string(&body.to_string())
            .map_err(Box::new)
    })?
    .json(what)
}

pub(super) fn get_json(
    agent: &ureq::Agent,
    url: &str,
    token: Option<&str>,
    what: &str,
) -> Result<Value, AwareError> {
    fetch(true, MAX_JSON_BYTES, what, || {
        let mut req = agent.get(url);
        if let Some(t) = token {
            req = req.set("Authorization", &format!("Bearer {t}"));
        }
        req.call().map_err(Box::new)
    })?
    .json(what)
}

/// Mock-server + temp-home helpers shared by this module's tests and
/// [`super::trimble_ops`]'s, so both exercise the same wire harness.
#[cfg(test)]
pub(crate) mod test_support {
    use std::net::TcpListener;
    use std::sync::mpsc;
    use std::time::Duration;

    use std::io::{Read as _, Write as _};

    /// Multi-request HTTP/1.1 mock. Its routes are built from the bound base URL (so a
    /// response can embed a pre-signed URL that points back at the mock), it serves
    /// `count` connections (ureq sends `Connection: close`, one request each), routes
    /// each by a substring of the request line, and ships the full captured request back.
    pub(crate) fn mock_routed<F>(count: usize, routes_fn: F) -> (String, mpsc::Receiver<String>)
    where
        F: FnOnce(&str) -> Vec<(&'static str, u16, String)> + Send + 'static,
    {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let base = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
        let base_for_thread = base.clone();
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let routes = routes_fn(&base_for_thread);
            for _ in 0..count {
                let Ok((mut stream, _)) = listener.accept() else {
                    break;
                };
                stream
                    .set_read_timeout(Some(Duration::from_millis(250)))
                    .ok();
                let mut data = Vec::new();
                let mut tmp = [0u8; 2048];
                while let Ok(n) = stream.read(&mut tmp) {
                    if n == 0 {
                        break;
                    }
                    data.extend_from_slice(&tmp[..n]);
                }
                let req = String::from_utf8_lossy(&data).to_string();
                let line = req.lines().next().unwrap_or("").to_string();
                let (status, body) = routes
                    .iter()
                    .find(|(pat, _, _)| line.contains(pat))
                    .map(|(_, s, b)| (*s, b.clone()))
                    .unwrap_or((404, "{}".to_string()));
                let resp = format!(
                    "HTTP/1.1 {status} X\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                );
                stream.write_all(resp.as_bytes()).ok();
                tx.send(req).ok();
            }
        });
        (base, rx)
    }

    /// Lay out an AWARE home with `manifest` installed as `trimble-connect`, and a
    /// stored credential when one is asked for. Split out of `mock_agents` so the
    /// `auth_and_base` refusals below can each drop exactly one of the three things
    /// `auth_and_base` requires and keep the other two intact.
    pub(crate) fn agents_with(manifest: &str, credential: Option<&str>) -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let agent = tmp.path().join("agents").join("trimble-connect");
        std::fs::create_dir_all(&agent).unwrap();
        std::fs::write(agent.join("manifest.yaml"), manifest).unwrap();
        if let Some(secret) = credential {
            let creds = tmp.path().join("credentials");
            std::fs::create_dir_all(&creds).unwrap();
            std::fs::write(
                creds.join(format!("{secret}.json")),
                r#"{"access_token":"TESTTOKEN"}"#,
            )
            .unwrap();
        }
        tmp
    }

    /// Install a trimble-connect manifest pointing at `base` with a NON-registered
    /// secret (so credential resolution is a hermetic raw file load — no real keychain
    /// or token-endpoint refresh) plus a stored token `TESTTOKEN`.
    pub(crate) fn mock_agents(base: &str) -> tempfile::TempDir {
        agents_with(
            &format!(
                "agent: trimble-connect\nversion: 0.1.0\ndescription: x\nstateful: false\n\
                 license: MIT\ntransport:\n  rest:\n    base: {base}/\nauth:\n  scheme: oauth2\n  \
                 secret: mock-tc-tok\ncommands:\n  upload:\n    lifecycle: single\n    description: x\n"
            ),
            Some("mock-tc-tok"),
        )
    }

    /// A syntactically valid base URL that nothing is listening on: bind a port
    /// to learn a free one, then drop the listener.
    ///
    /// At `a_required_input_that_is_missing_blank_or_unreadable_is_refused_before_any_request`
    /// this is load-bearing — a guard that leaked would reach the network and
    /// fail as `Network` rather than `Validation`, turning the test red. The
    /// `auth_and_base` refusals use it only for a well-formed base; they never
    /// reach the network under any mutation. The port is released before use, so
    /// this is a strong signal rather than a hermetic seal.
    pub(crate) fn dead_base() -> String {
        let l = TcpListener::bind("127.0.0.1:0").unwrap();
        let base = format!("http://127.0.0.1:{}", l.local_addr().unwrap().port());
        drop(l);
        base
    }

    /// The next request the mock captured, or a named failure. Never a blocking
    /// `recv`: every caller gives `mock_routed` a higher `count` than its flow
    /// will use, so the server thread stays parked in `accept()` holding the
    /// sender — a `recv` for a request that was never made would hang the test
    /// binary rather than fail it, and a hang in CI reads as an infra problem.
    pub(crate) fn next_request(rx: &mpsc::Receiver<String>) -> String {
        rx.recv_timeout(Duration::from_secs(20))
            .expect("mock server received no further request")
    }

    /// Drain whatever the mock captured, stopping when it goes quiet. Used where
    /// the assertion is about a request that must NOT have been made, so the
    /// count itself is the evidence.
    pub(crate) fn drain(rx: &mpsc::Receiver<String>) -> Vec<String> {
        let mut out = Vec::new();
        loop {
            match rx.recv_timeout(Duration::from_millis(750)) {
                Ok(req) => out.push(req),
                // Quiet, with the sender still alive: no more requests are
                // coming, which is the answer these assertions want.
                Err(mpsc::RecvTimeoutError::Timeout) => return out,
                // Disconnected means the mock thread ENDED — it panicked, or its
                // listener died. Folding that into "quiet" would let a test
                // asserting one request pass on a mock that served one and then
                // died, which is a pass for the wrong reason.
                Err(mpsc::RecvTimeoutError::Disconnected) => panic!(
                    "mock server ended after {} request(s); what follows would be \
                     an assertion about the harness, not about the code",
                    out.len()
                ),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::*;
    use super::*;

    /// A canned answer, as ureq hands it over: `Retry-After: 0` keeps the
    /// retry tests instant. `0` is a body that breaks off mid-read (it promises
    /// more bytes than it carries).
    fn answer(code: u16) -> Sent {
        let raw = if code == 0 {
            "HTTP/1.1 200 X\r\nContent-Length: 10\r\n\r\nabc".to_string()
        } else {
            format!("HTTP/1.1 {code} X\r\nRetry-After: 0\r\nContent-Length: 2\r\n\r\n{{}}")
        };
        let resp: ureq::Response = raw.parse().unwrap();
        if code >= 400 {
            Err(Box::new(ureq::Error::Status(code, resp)))
        } else {
            Ok(resp)
        }
    }

    /// Feed `fetch_any` the given answers in order (the last repeats) and
    /// report how many requests it made and how it ended.
    fn drive(repeatable: bool, script: &[u16]) -> (usize, Result<u16, String>) {
        let mut sent = 0usize;
        let res = fetch_any(repeatable, MAX_JSON_BYTES, "op", || {
            let code = script[sent.min(script.len() - 1)];
            sent += 1;
            answer(code)
        });
        (sent, res.map(|f| f.status()).map_err(|e| e.to_string()))
    }

    #[test]
    fn throttling_is_retried_for_every_method_up_to_three_attempts() {
        assert_eq!(drive(false, &[429, 201]), (2, Ok(201)));
        assert_eq!(drive(true, &[429, 429, 200]), (3, Ok(200)));
        assert_eq!(drive(true, &[429]), (3, Ok(429)), "gives up after three");
    }

    #[test]
    fn a_server_fault_or_a_cut_body_is_retried_only_where_repeating_is_safe() {
        assert_eq!(drive(true, &[503, 520, 200]), (3, Ok(200)));
        assert_eq!(
            drive(false, &[500, 201]),
            (1, Ok(500)),
            "a POST/DELETE is not repeated"
        );
        assert_eq!(
            drive(true, &[404, 200]),
            (1, Ok(404)),
            "a 4xx answer is final"
        );
        // A body that breaks off is retried like a dropped request …
        assert_eq!(drive(true, &[0, 200]), (2, Ok(200)));
        // … but never for a request that may already have taken effect.
        let (sent, end) = drive(false, &[0, 200]);
        assert_eq!(sent, 1);
        assert!(end.unwrap_err().contains("read body"));
    }

    /// A failure whose body breaks off on every attempt still reports the
    /// status its headers carried; a 2xx cut short has no failure status.
    #[test]
    fn a_failure_cut_off_mid_body_keeps_its_status() {
        let cut = |code: u16| -> Sent {
            let resp: ureq::Response =
                format!("HTTP/1.1 {code} X\r\nContent-Length: 10\r\n\r\nabc")
                    .parse()
                    .unwrap();
            if code >= 400 {
                Err(Box::new(ureq::Error::Status(code, resp)))
            } else {
                Ok(resp)
            }
        };
        let deadline = std::time::Instant::now() + direct::DEADLINE;
        let failed = direct::run(deadline, || {
            fetch_any(true, MAX_JSON_BYTES, "op", || cut(401)).map(|_| ())
        })
        .unwrap_err()
        .to_string();
        assert!(failed.ends_with("[E_CALL_FAILED status=401]"), "{failed}");
        let cut_ok = direct::run(deadline, || {
            fetch_any(true, MAX_JSON_BYTES, "op", || cut(200)).map(|_| ())
        })
        .unwrap_err()
        .to_string();
        assert!(cut_ok.ends_with("[E_CALL_FAILED]"), "{cut_ok}");
    }

    #[test]
    fn one_budget_covers_the_request_and_its_body() {
        // A fault before the headers and a cut body share the three attempts:
        // never more than three requests, whatever the mix.
        let (sent, end) = drive(true, &[503, 0, 0, 200]);
        assert_eq!(sent, 3);
        let err = end.unwrap_err();
        assert!(err.contains("op (after 3 attempts): read body"), "{err}");
    }

    #[test]
    fn a_final_failure_names_the_status_and_any_retry_after() {
        let f = fetch_any(true, MAX_JSON_BYTES, "op", || answer(429)).unwrap();
        let err = f.failure("op").to_string();
        assert!(
            err.contains("op (after 3 attempts): HTTP 429 (rate limited; Retry-After: 0)"),
            "{err}"
        );
        let f = fetch_any(true, MAX_JSON_BYTES, "op", || answer(404)).unwrap();
        assert_eq!(
            f.failure("op").to_string(),
            "network error: op: HTTP 404: {}"
        );
        let err = fetch(true, MAX_JSON_BYTES, "op", || answer(403))
            .err()
            .unwrap()
            .to_string();
        assert!(err.contains("op: HTTP 403"), "{err}");
    }

    #[test]
    fn a_body_over_the_limit_is_refused_not_read() {
        let err = fetch_any(true, 1, "op", || answer(200)).err().unwrap();
        assert!(err.to_string().contains("larger than 1 bytes"), "{err}");
    }

    #[test]
    fn the_attempt_count_reaches_the_error_only_when_there_was_a_retry() {
        assert_eq!(after_attempts("get file", 1), "get file");
        assert_eq!(after_attempts("get file", 3), "get file (after 3 attempts)");
    }

    #[test]
    fn extract_bytes_decodes_base64_string() {
        let b64 = base64::engine::general_purpose::STANDARD.encode(b"hello");
        let got = extract_bytes(Some(&Value::String(b64))).unwrap();
        assert_eq!(got, b"hello");
    }

    #[test]
    fn extract_bytes_invalid_base64_string_errors() {
        // Not valid base64 → rejected (NOT silently uploaded as literal text).
        assert!(extract_bytes(Some(&Value::String("!!!not base64".into()))).is_err());
    }

    #[test]
    fn extract_bytes_base64_tolerates_whitespace_and_url_safe() {
        // MIME-style line-wrapped base64 (with a newline) still decodes.
        let b64 = base64::engine::general_purpose::STANDARD.encode(b"a longer payload of bytes");
        let wrapped = format!("{}\n{}", &b64[..8], &b64[8..]);
        assert_eq!(
            extract_bytes(Some(&Value::String(wrapped))).unwrap(),
            b"a longer payload of bytes"
        );
        // URL-safe alphabet (no padding) also decodes.
        let url = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(b"\xff\xfe\x01");
        assert_eq!(
            extract_bytes(Some(&Value::String(url))).unwrap(),
            vec![0xff, 0xfe, 0x01]
        );
    }

    #[test]
    fn extract_bytes_array_is_raw_bytes() {
        let got = extract_bytes(Some(&json!([104, 105]))).unwrap();
        assert_eq!(got, b"hi");
    }

    #[test]
    fn extract_bytes_missing_errors() {
        assert!(extract_bytes(None).is_err());
    }

    #[tokio::test]
    async fn download_two_step_attaches_auth_then_fetches_bytes() {
        let (base, rx) = mock_routed(2, |b| {
            vec![
                ("downloadurl", 200, format!(r#"{{"url":"{b}/s3-get"}}"#)),
                ("/s3-get", 200, "FILEBYTES".to_string()),
            ]
        });
        let agents = mock_agents(&base);
        let out = download(
            agents.path().join("agents"),
            serde_json::json!({ "file-id": "f1" }),
        )
        .await
        .unwrap();
        assert_eq!(out["encoding"], "base64");
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(out["bytes"].as_str().unwrap())
            .unwrap();
        assert_eq!(decoded, b"FILEBYTES");
        // Step 1 (downloadurl) carries the bearer; step 2 (pre-signed S3) must NOT.
        let r1 = next_request(&rx);
        assert!(
            r1.contains("downloadurl") && r1.contains("Bearer TESTTOKEN"),
            "{r1}"
        );
        let r2 = next_request(&rx);
        assert!(
            r2.contains("/s3-get") && !r2.contains("Authorization"),
            "{r2}"
        );
    }

    #[tokio::test]
    async fn upload_three_step_orchestration() {
        let (base, rx) = mock_routed(3, |b| {
            vec![
                // complete (GET, request line carries `uploadId=`).
                (
                    "uploadId=",
                    200,
                    r#"{"fileId":"FID","versionId":"VID"}"#.to_string(),
                ),
                // initiate (POST).
                (
                    "POST /files/fs/upload",
                    200,
                    format!(
                        r#"{{"uploadId":"up1","status":"UPLOADABLE","contents":[{{"url":"{b}/s3-put"}}]}}"#
                    ),
                ),
                // pre-signed S3 PUT.
                ("/s3-put", 200, "{}".to_string()),
            ]
        });
        let agents = mock_agents(&base);
        let out = upload(
            agents.path().join("agents"),
            // bytes: base64("hello")
            serde_json::json!({ "folder-id": "fold1", "filename": "a.txt", "bytes": "aGVsbG8=" }),
        )
        .await
        .unwrap();
        assert_eq!(out["file-id"], "FID");
        assert_eq!(out["version-id"], "VID");
        let reqs: Vec<String> = (0..3).map(|_| next_request(&rx)).collect();
        // initiate carries the bearer + name; the S3 PUT carries the decoded bytes and NO auth.
        assert!(
            reqs.iter()
                .any(|r| r.contains("POST /files/fs/upload") && r.contains("Bearer TESTTOKEN"))
        );
        let put = reqs.iter().find(|r| r.contains("/s3-put")).unwrap();
        assert!(
            !put.contains("Authorization") && put.contains("hello"),
            "{put}"
        );
    }

    #[test]
    fn extract_bytes_rejects_out_of_range_or_non_int_array() {
        assert!(extract_bytes(Some(&json!([256]))).is_err());
        assert!(extract_bytes(Some(&json!([1, "x"]))).is_err());
        assert!(extract_bytes(Some(&json!([1, -1]))).is_err());
    }

    #[tokio::test]
    async fn upload_duplicate_fetches_version_from_metadata() {
        // DUPLICATE initiate carries only fileId → the handler fetches versionId from
        // the file metadata so the result stays usable.
        let (base, _rx) = mock_routed(2, |_b| {
            vec![
                (
                    "POST /files/fs/upload",
                    200,
                    r#"{"status":"DUPLICATE","fileId":"DUPFID"}"#.to_string(),
                ),
                (
                    "/files/DUPFID",
                    200,
                    r#"{"versionId":"DUPVER"}"#.to_string(),
                ),
            ]
        });
        let agents = mock_agents(&base);
        let out = upload(
            agents.path().join("agents"),
            serde_json::json!({ "folder-id": "f", "filename": "a.txt", "bytes": "aGk=" }),
        )
        .await
        .unwrap();
        assert_eq!(out["file-id"], "DUPFID");
        assert_eq!(out["version-id"], "DUPVER");
        assert_eq!(out["replaced"], true);
    }

    // ── transport refusals ──────────────────────────────────────────────────
    //
    // The `extract_bytes` tests above already cover one refusal — a `bytes`
    // input the runtime will not accept. What nothing covered is the transport
    // itself: TC answering 4xx, a 2xx that carries none of the identifiers the
    // next step consumes, and the `str_arg` guards on the flow arguments. Those
    // are the paths that decide whether a half-finished transfer is reported as
    // a success, and both flows persist their result downstream, so a refusal
    // that leaks through as `Ok` writes an identifier pointing at nothing.

    #[tokio::test]
    async fn a_required_input_that_is_missing_blank_or_unreadable_is_refused_before_any_request() {
        // `str_arg` rejects three shapes, and the blank one is the one worth
        // pinning: an empty `file-id` would otherwise build
        // `…/files/fs//downloadurl` and ask TC about a path with a hole in it.
        //
        // Each case asserts the MESSAGE, not the variant. `auth_and_base` runs
        // first and its three refusals are `Validation` too, so a variant-only
        // assertion would still hold if `str_arg` were deleted and something
        // upstream happened to fail instead — it would pass for the wrong reason.
        let agents = mock_agents(&dead_base());
        let dir = agents.path().join("agents");
        for (args, want) in [
            (json!({ "filename": "a.txt", "bytes": "aGk=" }), "folder-id"),
            (
                json!({ "folder-id": "", "filename": "a.txt", "bytes": "aGk=" }),
                "folder-id",
            ),
            (
                json!({ "folder-id": 7, "filename": "a.txt", "bytes": "aGk=" }),
                "folder-id",
            ),
            (json!({ "folder-id": "f", "bytes": "aGk=" }), "filename"),
            (
                json!({ "folder-id": "f", "filename": "", "bytes": "aGk=" }),
                "filename",
            ),
        ] {
            let err = upload(dir.clone(), args.clone()).await.unwrap_err();
            assert!(
                matches!(err, AwareError::Validation(_)),
                "upload {args}: {err}"
            );
            assert!(
                err.to_string()
                    .contains(&format!("missing required input `{want}`")),
                "upload {args} must name {want}: {err}"
            );
        }
        for args in [json!({}), json!({ "file-id": "" }), json!({ "file-id": 3 })] {
            let err = download(dir.clone(), args.clone()).await.unwrap_err();
            assert!(
                matches!(err, AwareError::Validation(_)),
                "download {args}: {err}"
            );
            assert!(
                err.to_string().contains("missing required input `file-id`"),
                "download {args} must name file-id: {err}"
            );
        }
    }

    #[tokio::test]
    async fn each_thing_the_transport_needs_is_named_when_it_is_the_one_that_is_missing() {
        // Three separate failures with three separate remedies. Reporting them
        // through one message would send an operator whose token expired off to
        // edit a manifest that is fine.
        let base = dead_base();
        let head = "agent: trimble-connect\nversion: 0.1.0\ndescription: x\nstateful: false\nlicense: MIT\n";
        let tail = "commands:\n  upload:\n    lifecycle: single\n    description: x\n";
        let auth = "auth:\n  scheme: oauth2\n  secret: mock-tc-tok\n";

        let no_auth = agents_with(
            &format!("{head}transport:\n  rest:\n    base: {base}/\n{tail}"),
            Some("mock-tc-tok"),
        );
        let err = download(no_auth.path().join("agents"), json!({ "file-id": "f" }))
            .await
            .unwrap_err();
        assert!(err.to_string().contains("no `auth:` block"), "{err}");

        let no_cred = agents_with(
            &format!("{head}transport:\n  rest:\n    base: {base}/\n{auth}{tail}"),
            None,
        );
        let err = download(no_cred.path().join("agents"), json!({ "file-id": "f" }))
            .await
            .unwrap_err();
        assert!(
            err.to_string().contains("aware connect trimble-connect"),
            "{err}"
        );

        // Two shapes reach the third refusal, and the likelier regression is the
        // second: an agent with no `rest:` transport at all, and one that has a
        // `rest:` block whose `base:` key went missing. `rest_base_url` answers
        // None to both, and both have to arrive here rather than as an empty base
        // that would build `/files/fs/…` against nothing.
        for transport in [
            "transport:\n  builtin: {}\n",
            "transport:\n  rest:\n    x: 1\n",
        ] {
            let no_base = agents_with(
                &format!("{head}{transport}{auth}{tail}"),
                Some("mock-tc-tok"),
            );
            let err = download(no_base.path().join("agents"), json!({ "file-id": "f" }))
                .await
                .unwrap_err();
            assert!(
                err.to_string().contains("no `transport.rest.base`"),
                "{transport:?}: {err}"
            );
        }
    }

    #[tokio::test]
    async fn a_non_2xx_is_reported_with_the_step_the_status_and_what_the_server_said() {
        // The body matters as much as the code: TC puts the reason in it, and a
        // 403 whose text is dropped is indistinguishable from a 403 for any other
        // reason. `ok_response` is the only place that reads a FAILURE body —
        // everywhere else a non-2xx has already been converted to an error.
        let (base, _rx) = mock_routed(1, |_b| {
            vec![(
                "downloadurl",
                403,
                r#"{"message":"token has expired"}"#.to_string(),
            )]
        });
        let agents = mock_agents(&base);
        let err = download(agents.path().join("agents"), json!({ "file-id": "f1" }))
            .await
            .unwrap_err();
        let text = err.to_string();
        assert!(text.contains("download url"), "names the step: {text}");
        assert!(text.contains("403"), "carries the status: {text}");
        assert!(text.contains("token has expired"), "quotes TC: {text}");
    }

    #[tokio::test]
    async fn a_2xx_that_is_not_json_fails_at_the_parse_rather_than_reading_as_empty() {
        // A proxy or a login wall answering 200 with HTML. Treating an unparseable
        // body as an empty object would push the failure one step downstream and
        // report it as "no pre-signed url" — a true statement about the wrong
        // thing, and one that hides the interception.
        let (base, _rx) = mock_routed(1, |_b| {
            vec![(
                "downloadurl",
                200,
                "<html><body>sign in</body></html>".to_string(),
            )]
        });
        let agents = mock_agents(&base);
        let err = download(agents.path().join("agents"), json!({ "file-id": "f1" }))
            .await
            .unwrap_err();
        let text = err.to_string();
        assert!(text.contains("download url"), "{text}");
        assert!(text.contains("bad JSON"), "{text}");
    }

    #[tokio::test]
    async fn a_failed_bytes_put_stops_before_the_completion_step() {
        // The worst outcome this module can produce: S3 refuses the bytes, the
        // completion step is asked anyway, and TC hands back a fileId for a file
        // whose content never arrived. `check_ok` is all that stands between the
        // two, and it is the one step whose result is otherwise discarded.
        //
        // "Stops before completing", not "aborts": nothing cancels the upload
        // session that step 1 opened, and this test does not pretend otherwise.
        let (base, rx) = mock_routed(3, |b| {
            vec![
                (
                    "uploadId=",
                    200,
                    r#"{"fileId":"FID","versionId":"VID"}"#.to_string(),
                ),
                (
                    "POST /files/fs/upload",
                    200,
                    format!(r#"{{"uploadId":"up1","contents":[{{"url":"{b}/s3-put"}}]}}"#),
                ),
                ("/s3-put", 403, "AccessDenied".to_string()),
            ]
        });
        let agents = mock_agents(&base);
        let err = upload(
            agents.path().join("agents"),
            json!({ "folder-id": "f", "filename": "a.txt", "bytes": "aGk=" }),
        )
        .await
        .unwrap_err();
        let text = err.to_string();
        assert!(
            text.contains("upload PUT") && text.contains("403"),
            "{text}"
        );
        // `unwrap_err` above already rules out a `check_ok` whose result is
        // discarded — the run would return Ok and panic there. What it cannot see
        // is a run that reports the PUT failure correctly but has already asked TC
        // to complete. The request log is the only witness that step 3 never went
        // out at all.
        let seen = drain(&rx);
        assert_eq!(seen.len(), 2, "expected initiate + PUT only: {seen:#?}");
        assert!(
            !seen.iter().any(|r| r.contains("uploadId=")),
            "completion must not be attempted: {seen:#?}"
        );
    }

    #[tokio::test]
    async fn an_initiate_that_names_no_upload_id_or_no_url_stops_there() {
        // Three 2xx initiate responses, each starving the next step: no
        // `uploadId`; no `contents` at all; and a `contents` slot that came back
        // exactly as step 1 sent it — present, but with no `url`. Defaulting any
        // of them to an empty string would PUT the bytes at `""` or complete an
        // upload that was never identified.
        for (body, want) in [
            (r#"{"status":"UPLOADABLE"}"#, "no uploadId"),
            (r#"{"uploadId":"up1"}"#, "no pre-signed url"),
            (r#"{"uploadId":"up1","contents":[{}]}"#, "no pre-signed url"),
        ] {
            let owned = body.to_string();
            let (base, rx) = mock_routed(2, move |_b| vec![("POST /files/fs/upload", 200, owned)]);
            let agents = mock_agents(&base);
            let err = upload(
                agents.path().join("agents"),
                json!({ "folder-id": "f", "filename": "a.txt", "bytes": "aGk=" }),
            )
            .await
            .unwrap_err();
            assert!(err.to_string().contains(want), "{body}: {err}");
            assert_eq!(drain(&rx).len(), 1, "{body}: nothing follows the initiate");
        }
    }

    #[tokio::test]
    async fn a_completion_missing_its_identifiers_is_a_failure_not_a_blank_success() {
        // The bytes are stored by this point, so the temptation is to call it a
        // win and hand back whatever came out. But `file-id` / `version-id` are
        // what downstream nodes persist, and an empty one is a reference to
        // nothing that only fails much later, somewhere else.
        for (body, want) in [
            ("{}", "no fileId"),
            (r#"{"versionId":"VID"}"#, "no fileId"),
            (r#"{"fileId":"FID"}"#, "no versionId"),
            (r#"{"fileId":"FID","versionId":null}"#, "no versionId"),
        ] {
            let owned = body.to_string();
            let (base, _rx) = mock_routed(3, move |b| {
                vec![
                    ("uploadId=", 200, owned),
                    (
                        "POST /files/fs/upload",
                        200,
                        format!(r#"{{"uploadId":"up1","contents":[{{"url":"{b}/s3-put"}}]}}"#),
                    ),
                    ("/s3-put", 200, "{}".to_string()),
                ]
            });
            let agents = mock_agents(&base);
            let err = upload(
                agents.path().join("agents"),
                json!({ "folder-id": "f", "filename": "a.txt", "bytes": "aGk=" }),
            )
            .await
            .unwrap_err();
            assert!(err.to_string().contains(want), "{body}: {err}");
        }
    }

    #[tokio::test]
    async fn a_duplicate_that_cannot_be_identified_is_refused_rather_than_returned_half_filled() {
        // `upload_duplicate_fetches_version_from_metadata` covers the recovery.
        // These are the two ways it runs out of road — and the DUPLICATE branch
        // returns early, so nothing further would have caught them.
        let (base, _rx) = mock_routed(1, |_b| {
            vec![(
                "POST /files/fs/upload",
                200,
                r#"{"status":"DUPLICATE"}"#.to_string(),
            )]
        });
        let agents = mock_agents(&base);
        let err = upload(
            agents.path().join("agents"),
            json!({ "folder-id": "f", "filename": "a.txt", "bytes": "aGk=" }),
        )
        .await
        .unwrap_err();
        assert!(err.to_string().contains("DUPLICATE: no fileId"), "{err}");

        let (base, _rx) = mock_routed(2, |_b| {
            vec![
                (
                    "POST /files/fs/upload",
                    200,
                    r#"{"status":"DUPLICATE","fileId":"DUPFID"}"#.to_string(),
                ),
                ("/files/DUPFID", 200, r#"{"name":"a.txt"}"#.to_string()),
            ]
        });
        let agents = mock_agents(&base);
        let err = upload(
            agents.path().join("agents"),
            json!({ "folder-id": "f", "filename": "a.txt", "bytes": "aGk=" }),
        )
        .await
        .unwrap_err();
        assert!(
            err.to_string()
                .contains("duplicate file metadata: no versionId"),
            "{err}"
        );
    }

    #[tokio::test]
    async fn a_download_url_response_without_a_url_stops_before_the_second_leg() {
        let (base, rx) = mock_routed(2, |_b| {
            vec![("downloadurl", 200, r#"{"expiresIn":60}"#.to_string())]
        });
        let agents = mock_agents(&base);
        let err = download(agents.path().join("agents"), json!({ "file-id": "f1" }))
            .await
            .unwrap_err();
        assert!(
            err.to_string().contains("download: no pre-signed url"),
            "{err}"
        );
        assert_eq!(drain(&rx).len(), 1, "no second leg without a url");
    }

    #[tokio::test]
    async fn a_version_pinned_download_asks_for_that_version_and_a_blank_pin_asks_for_none() {
        // `version-id` is optional, and the empty string is what a `{{ }}`
        // substitution over an unset input renders to. Appending `?versionId=`
        // with nothing after it asks TC for a version named "" instead of the
        // current one.
        for (pin, expect_query) in [
            (json!("v7"), true),
            (json!(""), false),
            (json!(null), false),
        ] {
            let (base, rx) = mock_routed(2, |b| {
                vec![
                    ("downloadurl", 200, format!(r#"{{"url":"{b}/s3-get"}}"#)),
                    ("/s3-get", 200, "BYTES".to_string()),
                ]
            });
            let agents = mock_agents(&base);
            download(
                agents.path().join("agents"),
                json!({ "file-id": "f1", "version-id": pin }),
            )
            .await
            .unwrap();
            let first = next_request(&rx);
            let line = first.lines().next().unwrap_or_default().to_string();
            assert_eq!(
                line.contains("versionId="),
                expect_query,
                "pin {pin}: {line}"
            );
            if expect_query {
                assert!(line.contains("versionId=v7"), "{line}");
            }
        }
    }

    #[tokio::test]
    async fn a_download_reports_the_size_of_what_it_actually_fetched_in_kib() {
        // The field is spelled `size-kb` but divided by 1024, and the agent's
        // `commands/download.md` documents it that way — the name says kB, the
        // contract says KiB, and the code follows the contract. 2048 bytes pins
        // that: exactly `2.0` under KiB, `2.048` under kB, with no float slop
        // either way.
        let (base, _rx) = mock_routed(2, |b| {
            vec![
                ("downloadurl", 200, format!(r#"{{"url":"{b}/s3-get"}}"#)),
                ("/s3-get", 200, "x".repeat(2048)),
            ]
        });
        let agents = mock_agents(&base);
        let out = download(agents.path().join("agents"), json!({ "file-id": "f1" }))
            .await
            .unwrap();
        assert_eq!(out["size-kb"], json!(2.0));
    }

    #[tokio::test]
    async fn a_fresh_upload_is_not_marked_as_a_replacement() {
        // `replaced` is a published output — the agent manifest declares it
        // ("true when TC returned DUPLICATE") and `welded-to-tc.app` forwards it
        // downstream in its exposed stream schema. Only the DUPLICATE path had a
        // test, so hard-coding either value passed.
        let (base, _rx) = mock_routed(3, |b| {
            vec![
                (
                    "uploadId=",
                    200,
                    r#"{"fileId":"FID","versionId":"VID"}"#.to_string(),
                ),
                (
                    "POST /files/fs/upload",
                    200,
                    format!(r#"{{"uploadId":"up1","contents":[{{"url":"{b}/s3-put"}}]}}"#),
                ),
                ("/s3-put", 200, "{}".to_string()),
            ]
        });
        let agents = mock_agents(&base);
        let out = upload(
            agents.path().join("agents"),
            json!({ "folder-id": "f", "filename": "a.txt", "bytes": "aGk=" }),
        )
        .await
        .unwrap();
        assert_eq!(out["replaced"], json!(false));
    }
}
