//! `aware agent call-capabilities` and single-shot `aware agent call` (#618,
//! minimal slice) — one reviewed, read-only, account-bound REST read: Google
//! Drive `list-files`.
//!
//! Plan: `docs/superpowers/specs/2026-10-08-618-agent-call-slice-PLAN.md`. The
//! rules are the #617 probe's, applied to a caller-driven read:
//!
//! - **The operation is code-owned.** Method, origin, path, the literal field
//!   mask, the input → wire mapping, bounds, the scope allowlist and the output
//!   projection live in [`OPERATIONS`]. A manifest only has to *declare* the
//!   command runnable; it cannot change where the token goes or what is sent.
//! - **Exactly one account slot.** `integration[.alias]`, never another — a
//!   missing alias slot is "missing", never the default account.
//! - **The account is verified live.** Both verbs ask Google's OpenID userinfo
//!   endpoint who the slot's token belongs to; the binding id is derived from
//!   that answer, so a reconnect to another Google account changes it and a call
//!   bound to the old account is refused before Drive is touched.
//! - **Nothing raw leaves.** Failures are a code, AWARE's own fixed sentence and
//!   details of codes / statuses / field names only. Success carries only the
//!   projected file list. No token, header, response text or query value.
//! - **Bounded.** No redirects, a body cap, a deadline over the whole verb with a
//!   budget checked before every network phase.
//! - **Persistent writes, exactly:** one code-only line in `logs/agent-call.log`
//!   (the command layer), plus the credential maintenance the resolver already
//!   performs (an OAuth refresh stored back to the same slot; a generation
//!   written for a legacy credential).

use std::path::Path;
use std::time::{Duration, Instant};

use serde_json::{Map, Value as Json, json};

use crate::auth::keychain::StoredToken;
use crate::manifest::agent::{Agent, AgentStatus, Mode};

pub(crate) const CAPABILITY_SCHEMA: &str = "aware.agent-call-capability/v1";
pub(crate) const REQUEST_SCHEMA: &str = "aware.agent-call/v1";
pub(crate) const RECORD_SCHEMA: &str = "aware.agent-call-record/v1";
pub(crate) const RESULT_SCHEMA: &str = "aware.agent-call-result/v1";
const OPERATION_SCHEMA: &str = "aware.agent-call-operation/v1";
const BINDING_DOMAIN: &str = "aware.agent-call-binding/v1";

/// Largest request file `call` reads.
pub(crate) const MAX_REQUEST_BYTES: u64 = 64 * 1024;
/// Cap on the identity (userinfo) response.
const IDENTITY_CAP: usize = 64 * 1024;
/// Deadline bounds for both verbs (`--timeout-ms`, request `timeoutMs`).
pub(crate) const MIN_TIMEOUT_MS: u64 = 1_000;
pub(crate) const MAX_TIMEOUT_MS: u64 = 60_000;
pub(crate) const DEFAULT_TIMEOUT_MS: u64 = 15_000;
/// Request `maxOutputBytes` bounds.
const MIN_OUTPUT_BYTES: u64 = 1_024;
const MAX_OUTPUT_BYTES: u64 = 1024 * 1024;
/// Workflow-path bounds (no caller supplies them there).
const WORKFLOW_TIMEOUT: Duration = Duration::from_secs(30);
const WORKFLOW_OUTPUT_BYTES: usize = 1024 * 1024;
/// Largest opaque `owner` object, serialized.
const MAX_OWNER_BYTES: usize = 4 * 1024;
/// A token counts as fresh after a refresh conflict when it outlives this.
const FRESH_MARGIN_SECS: i64 = 60;

/// Google's OAuth token endpoint — the only place a google-workspace refresh
/// token may be sent from this module.
pub(crate) const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
/// Google's OpenID Connect userinfo endpoint — the code-owned identity read.
pub(crate) const GOOGLE_USERINFO_URL: &str = "https://openidconnect.googleapis.com/v1/userinfo";
/// The Drive scope AWARE asks for when a Google account is connected for Drive
/// (owner ruling 2026-10-08: it is the scope in the project's verification
/// submission). It could also read file content; this module never does — it
/// calls only `files.list` with the fixed metadata field mask.
pub(crate) const DRIVE_READONLY: &str = "https://www.googleapis.com/auth/drive.readonly";
/// The narrower metadata-only scope, also accepted.
pub(crate) const DRIVE_METADATA_READONLY: &str =
    "https://www.googleapis.com/auth/drive.metadata.readonly";
/// Scopes a google-workspace grant may hold beyond the mail set (`gmail.send`'s
/// least-privilege check) — exactly the Drive read scope(s) this module accepts.
/// One list for both checks, so a slot can never be Drive-callable while
/// silently losing mail.
pub(crate) const OPTIONAL_GOOGLE_SCOPES: &[&str] = &[DRIVE_READONLY, DRIVE_METADATA_READONLY];

// ── failure ──────────────────────────────────────────────────────────────────

/// A refused or failed call: a stable code, and details that hold only codes,
/// statuses, counts and AWARE's own field names.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CallFailure {
    pub(crate) code: &'static str,
    pub(crate) details: Map<String, Json>,
}

impl CallFailure {
    fn new(code: &'static str) -> Self {
        Self {
            code,
            details: Map::new(),
        }
    }

    fn reason(code: &'static str, reason: &'static str) -> Self {
        Self::new(code).with("reason", Json::String(reason.into()))
    }

    /// The request file itself is unusable (not `@<file>`, too large, …).
    pub(crate) fn request_reason(reason: &'static str) -> Self {
        Self::reason("E_CALL_REQUEST_INVALID", reason)
    }

    fn field(field: &'static str) -> Self {
        Self::new("E_CALL_REQUEST_INVALID").with("field", Json::String(field.into()))
    }

    fn input(input: &'static str) -> Self {
        Self::new("E_CALL_INPUT_INVALID").with("input", Json::String(input.into()))
    }

    fn with(mut self, key: &str, value: Json) -> Self {
        self.details.insert(key.into(), value);
        self
    }

    fn reason_str(&self) -> Option<&str> {
        self.details.get("reason").and_then(Json::as_str)
    }

    /// AWARE's own fixed sentence for the code — never text from a service,
    /// host, manifest or request.
    pub(crate) fn message(&self) -> &'static str {
        match self.code {
            "E_AGENT_NOT_INSTALLED" => "The agent is not installed.",
            "E_CALL_UNSUPPORTED" => "This agent command is not available through aware agent call.",
            "E_CALL_INVALID" => "The installed agent cannot be used for this call.",
            "E_CALL_ALIAS_INVALID" => "The requested account alias is not a valid alias.",
            "E_CALL_REQUEST_INVALID" => "The call request is not valid.",
            "E_CALL_INPUT_INVALID" => "An input of the call request is not valid.",
            "E_CALL_CHANGED" => {
                "The installed agent or operation is not the one the caller reviewed, so nothing was called."
            }
            "E_CALL_UNAVAILABLE" => "The installed agent does not offer this command yet.",
            "E_CALL_ORIGIN_NOT_ALLOWED" => {
                "The call would send a request to an origin that is not allowed for this credential."
            }
            "E_CREDENTIAL_MISSING" => "No usable credential is stored for this account.",
            "E_CREDENTIAL_EXPIRED" => {
                "The stored credential for this account has expired and could not be refreshed."
            }
            "E_CREDENTIAL_CHANGED" => {
                "The stored credential is not the one the caller expected (its generation changed)."
            }
            "E_CALL_SCOPE_MISSING" => {
                "The account was connected without Drive read access, so nothing was read."
            }
            "E_CALL_IDENTITY_FAILED" => {
                "Google did not confirm which account this credential belongs to."
            }
            "E_BINDING_CHANGED" => {
                "The stored credential now belongs to a different account than the one the caller bound."
            }
            "E_CALL_FAILED" => "The service answered the call with a failure.",
            "E_CALL_OUTPUT_TOO_LARGE" => "The service's answer exceeded the size limit.",
            "E_CALL_TIMEOUT" => "The call did not finish before the deadline.",
            _ => "The call failed.",
        }
    }

    /// Process exit status, on the `cli-spec.md` exit-code table.
    pub(crate) fn exit_code(&self) -> i32 {
        match self.code {
            "E_AGENT_NOT_INSTALLED" => 7,
            "E_CREDENTIAL_MISSING"
            | "E_CREDENTIAL_EXPIRED"
            | "E_CREDENTIAL_CHANGED"
            | "E_CALL_SCOPE_MISSING"
            | "E_BINDING_CHANGED" => 6,
            "E_CALL_FAILED"
            | "E_CALL_IDENTITY_FAILED"
            | "E_CALL_OUTPUT_TOO_LARGE"
            | "E_CALL_TIMEOUT" => 4,
            _ => 3,
        }
    }
}

// ── the code-owned operation table ───────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Scalar {
    String,
    Integer,
}

impl Scalar {
    fn as_str(self) -> &'static str {
        match self {
            Scalar::String => "string",
            Scalar::Integer => "integer",
        }
    }
}

/// One caller input and where it goes on the wire.
#[derive(Debug)]
struct InputMap {
    /// The input name the manifest command and the caller use.
    source: &'static str,
    /// The query-parameter name Google expects.
    name: &'static str,
    scalar: Scalar,
    default: Option<i64>,
    minimum: Option<i64>,
    maximum: Option<i64>,
    max_bytes: Option<usize>,
}

/// One reviewed operation. Everything that decides where a credential goes and
/// what is sent is here, in code.
#[derive(Debug)]
pub(crate) struct Operation {
    agent: &'static str,
    command: &'static str,
    integration: &'static str,
    url: &'static str,
    field_mask: &'static str,
    inputs: &'static [InputMap],
}

/// `fields` for `files.list`: `nextPageToken` must be in the mask or a first
/// page would read as complete; a page shorter than `pageSize` may still have
/// more.
const LIST_FILES_FIELDS: &str =
    "nextPageToken,incompleteSearch,files(id,name,mimeType,size,modifiedTime)";

static OPERATIONS: &[Operation] = &[Operation {
    agent: "google-workspace",
    command: "list-files",
    integration: "google-workspace",
    url: "https://www.googleapis.com/drive/v3/files",
    field_mask: LIST_FILES_FIELDS,
    inputs: &[
        InputMap {
            source: "query",
            name: "q",
            scalar: Scalar::String,
            default: None,
            minimum: None,
            maximum: None,
            max_bytes: Some(2048),
        },
        InputMap {
            source: "page-size",
            name: "pageSize",
            scalar: Scalar::Integer,
            default: Some(100),
            minimum: Some(1),
            maximum: Some(100),
            max_bytes: None,
        },
    ],
}];

pub(crate) fn operation(agent: &str, command: &str) -> Option<&'static Operation> {
    OPERATIONS
        .iter()
        .find(|op| op.agent == agent && op.command == command)
}

impl Operation {
    fn inputs_json(&self) -> Json {
        Json::Array(
            self.inputs
                .iter()
                .map(|input| {
                    json!({
                        "source": input.source,
                        "location": "query",
                        "name": input.name,
                        "scalar": input.scalar.as_str(),
                        "required": false,
                        "default": input.default,
                        "minimum": input.minimum,
                        "maximum": input.maximum,
                        "maxBytes": input.max_bytes,
                    })
                })
                .collect(),
        )
    }

    /// The canonical representation the operation digest is taken over.
    fn canonical(&self) -> Json {
        json!({
            "schema": OPERATION_SCHEMA,
            "agent": self.agent,
            "command": self.command,
            "integration": self.integration,
            "transport": "rest",
            "effect": "read",
            "method": "GET",
            "url": self.url,
            "fields": self.field_mask,
            "identity": GOOGLE_USERINFO_URL,
            "scopes": OPTIONAL_GOOGLE_SCOPES,
            "inputs": self.inputs_json(),
        })
    }

    pub(crate) fn sha256(&self) -> String {
        sha256_hex(canonical_string(&self.canonical()).as_bytes())
    }
}

// ── canonical JSON + digests ─────────────────────────────────────────────────

/// Recursively sort object keys (by UTF-16 code units, which for the ASCII keys
/// this module accepts is byte order — the order FloLess's `localeCompare`
/// produces for them) and serialize compactly, matching `JSON.stringify`.
pub(crate) fn canonical_string(value: &Json) -> String {
    fn sorted(value: &Json) -> Json {
        match value {
            Json::Object(map) => {
                let mut keys: Vec<&String> = map.keys().collect();
                keys.sort_by(|a, b| a.encode_utf16().cmp(b.encode_utf16()));
                let mut out = Map::new();
                for key in keys {
                    out.insert(key.clone(), sorted(&map[key]));
                }
                Json::Object(out)
            }
            Json::Array(items) => Json::Array(items.iter().map(sorted).collect()),
            other => other.clone(),
        }
    }
    sorted(value).to_string()
}

pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(bytes))
}

/// The binding id for (integration, slot, Google `sub`): the first 16 bytes of a
/// domain-separated SHA-256, formatted as a version-8 RFC 4122 UUID.
pub(crate) fn binding_id(integration: &str, slot: &str, sub: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    for part in [BINDING_DOMAIN, integration, slot, sub] {
        hasher.update(part.as_bytes());
        hasher.update([0u8]);
    }
    let digest = hasher.finalize();
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&digest[..16]);
    bytes[6] = (bytes[6] & 0x0f) | 0x80;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
    format!(
        "{}-{}-{}-{}-{}",
        &hex[0..8],
        &hex[8..12],
        &hex[12..16],
        &hex[16..20],
        &hex[20..32]
    )
}

fn is_uuid(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 36
        && bytes.iter().enumerate().all(|(i, b)| match i {
            8 | 13 | 18 | 23 => *b == b'-',
            _ => b.is_ascii_hexdigit(),
        })
}

fn is_sha256_hex(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| i64::try_from(d.as_millis()).unwrap_or(i64::MAX))
        .unwrap_or(0)
}

/// Where the code-owned operation's requests go. Production uses only the
/// constants above; tests point it at local fixtures (and must widen the
/// allowlist to do so, which is exactly what the origin-pin test does not do).
#[derive(Debug, Clone)]
pub(crate) struct Endpoints {
    pub(crate) token_url: String,
    pub(crate) identity_url: String,
    /// `None` = the operation's own URL.
    pub(crate) files_url: Option<String>,
    pub(crate) allowed_origins: Vec<String>,
}

impl Endpoints {
    pub(crate) fn production(integration: &str) -> Self {
        let allowed_origins = crate::auth::config::for_integration(integration)
            .map(|config| {
                config
                    .call_origins()
                    .iter()
                    .map(|o| o.to_string())
                    .collect()
            })
            .unwrap_or_default();
        Self {
            token_url: GOOGLE_TOKEN_URL.to_string(),
            identity_url: GOOGLE_USERINFO_URL.to_string(),
            files_url: None,
            allowed_origins,
        }
    }

    fn files_url<'a>(&'a self, op: &'a Operation) -> &'a str {
        self.files_url.as_deref().unwrap_or(op.url)
    }

    /// Both request URLs must sit on an allowed origin — checked before any
    /// credential is read.
    fn check_origins(&self, op: &Operation) -> Result<(), CallFailure> {
        for url in [self.identity_url.as_str(), self.files_url(op)] {
            let allowed = crate::manifest::probe::origin_of(url)
                .is_some_and(|origin| self.allowed_origins.contains(&origin));
            if !allowed {
                return Err(CallFailure::new("E_CALL_ORIGIN_NOT_ALLOWED"));
            }
        }
        Ok(())
    }
}

// ── manifest checks ──────────────────────────────────────────────────────────

/// The installed manifest, with its digest.
struct Installed {
    agent: Agent,
    manifest_sha256: String,
}

/// Read the installed manifest's exact bytes; `expected_sha` (if any) is checked
/// before the bytes are even parsed.
fn read_installed(
    home: &Path,
    agent_id: &str,
    expected_sha: Option<&str>,
) -> Result<Installed, CallFailure> {
    let agents_dir = home.join("agents");
    let path = crate::manifest::loader::agent_manifest_path(&agents_dir, agent_id)
        .map_err(|_| CallFailure::new("E_AGENT_NOT_INSTALLED"))?;
    let bytes = std::fs::read(&path).map_err(|_| CallFailure::new("E_AGENT_NOT_INSTALLED"))?;
    let manifest_sha256 = crate::runtime::probe::manifest_sha256(&bytes);
    if let Some(expected) = expected_sha
        && expected != manifest_sha256
    {
        return Err(CallFailure::reason("E_CALL_CHANGED", "manifest"));
    }
    let text = String::from_utf8_lossy(&bytes);
    let agent: Agent = serde_yaml::from_str(text.trim_start_matches('\u{feff}'))
        .map_err(|_| CallFailure::reason("E_CALL_INVALID", "manifest-unreadable"))?;
    if agent.agent != agent_id {
        return Err(CallFailure::reason("E_CALL_INVALID", "agent-id-mismatch"));
    }
    if crate::validate::runtime_requirement_error(&agent, crate::validate::CURRENT_CLI_VERSION)
        .is_some()
    {
        return Err(CallFailure::reason("E_CALL_INVALID", "runtime-requirement"));
    }
    Ok(Installed {
        agent,
        manifest_sha256,
    })
}

/// The manifest's credential must be exactly the operation's OAuth integration.
/// `allow_baked_alias` is the workflow path's rule: there the manifest's
/// `secret` may name `<integration>.<alias>`, and that alias is the slot.
fn manifest_slot(
    agent: &Agent,
    op: &Operation,
    allow_baked_alias: bool,
) -> Result<Option<String>, CallFailure> {
    let auth = agent
        .auth
        .as_ref()
        .ok_or_else(|| CallFailure::reason("E_CALL_INVALID", "auth"))?;
    if auth.scheme != "oauth2" {
        return Err(CallFailure::reason("E_CALL_INVALID", "auth"));
    }
    if auth.secret == op.integration {
        return Ok(None);
    }
    if allow_baked_alias
        && let Some((base, alias)) = auth.secret.split_once('.')
        && base == op.integration
        && crate::commands::credential::is_valid_alias(alias)
    {
        return Ok(Some(alias.to_string()));
    }
    Err(CallFailure::reason("E_CALL_INVALID", "auth"))
}

/// The command is declared, runnable and read-only in the installed manifest.
fn command_runnable(agent: &Agent, op: &Operation) -> bool {
    if agent.status == AgentStatus::Planned {
        return false;
    }
    match agent.commands.get(op.command) {
        Some(command) => {
            command.status != AgentStatus::Planned
                && !command.mode_overridable
                && agent.mode_of(op.command, command) == Mode::Read
                && command.mode == Some(Mode::Read)
        }
        None => false,
    }
}

// ── inputs ───────────────────────────────────────────────────────────────────

/// Validate caller inputs against the operation's mapping (every key mapped,
/// right type, in bounds) and return the wire query in a fixed order.
fn wire_query(
    op: &Operation,
    inputs: &Map<String, Json>,
) -> Result<Vec<(String, String)>, CallFailure> {
    for key in inputs.keys() {
        if !op.inputs.iter().any(|m| m.source == key) {
            return Err(CallFailure::new("E_CALL_INPUT_INVALID")
                .with("reason", Json::String("unknown-input".into())));
        }
    }
    let mut query = Vec::new();
    for map in op.inputs {
        let value = inputs.get(map.source);
        let text = match (map.scalar, value) {
            (_, None) => map.default.map(|d| d.to_string()),
            (Scalar::String, Some(Json::String(s))) => {
                if s.contains('\0') || map.max_bytes.is_some_and(|max| s.len() > max) {
                    return Err(CallFailure::input(map.source));
                }
                Some(s.clone())
            }
            (Scalar::Integer, Some(Json::Number(n))) => {
                let Some(v) = n.as_i64() else {
                    return Err(CallFailure::input(map.source));
                };
                if map.minimum.is_some_and(|min| v < min) || map.maximum.is_some_and(|max| v > max)
                {
                    return Err(CallFailure::input(map.source));
                }
                Some(v.to_string())
            }
            _ => return Err(CallFailure::input(map.source)),
        };
        if let Some(text) = text {
            query.push((map.name.to_string(), text));
        }
    }
    query.push(("fields".to_string(), op.field_mask.to_string()));
    Ok(query)
}

fn page_size(query: &[(String, String)]) -> usize {
    query
        .iter()
        .find(|(k, _)| k == "pageSize")
        .and_then(|(_, v)| v.parse().ok())
        .unwrap_or(100)
}

// ── credentials ──────────────────────────────────────────────────────────────

/// The token a call will use, from exactly one slot.
#[derive(Debug, Clone)]
pub(crate) struct SlotToken {
    access_token: String,
    pub(crate) generation: String,
    scope: String,
}

impl SlotToken {
    fn has_drive_scope(&self) -> bool {
        crate::auth::keychain::normalized_scopes(&self.scope)
            .iter()
            .any(|s| OPTIONAL_GOOGLE_SCOPES.contains(&s.as_str()))
    }
}

fn account_name(integration: &str, alias: Option<&str>) -> String {
    match alias {
        Some(alias) => format!("{integration}.{alias}"),
        None => integration.to_string(),
    }
}

fn token_fresh(token: &StoredToken) -> bool {
    let now = crate::auth::unix_now_secs().unwrap_or(0);
    token.expires_at > now + FRESH_MARGIN_SECS
}

/// The slot's generation. A token whose generation could not be persisted (a
/// legacy credential whose metadata write failed) has none, and is never
/// treated as a bound, verified credential.
fn slot_generation(token: &StoredToken) -> Result<String, CallFailure> {
    token
        .generation
        .clone()
        .ok_or_else(|| CallFailure::reason("E_CREDENTIAL_CHANGED", "generation-unavailable"))
}

/// Load exactly the slot `integration[.alias]`, refresh it through the pinned
/// token endpoint, and check its generation before and after. Blocking.
fn load_slot(
    home: &Path,
    integration: &str,
    alias: Option<&str>,
    endpoints: &Endpoints,
    expected_generation: Option<&str>,
) -> Result<SlotToken, CallFailure> {
    // One configuration snapshot, validated and then used for the refresh, so a
    // profile rewritten in between can never receive the refresh token.
    let config = crate::auth::config::for_integration(integration)
        .and_then(|config| config.with_profile(home, alias))
        .map_err(|_| CallFailure::reason("E_CREDENTIAL_EXPIRED", "oauth-config"))?;
    if config.token_url() != endpoints.token_url {
        return Err(CallFailure::reason(
            "E_CREDENTIAL_EXPIRED",
            "token-endpoint",
        ));
    }
    let stored = crate::auth::keychain::load_token(integration, alias, home)
        .map_err(|_| CallFailure::reason("E_CREDENTIAL_MISSING", "unreadable"))?
        .ok_or_else(|| CallFailure::new("E_CREDENTIAL_MISSING"))?;
    let generation = slot_generation(&stored)?;
    if let Some(expected) = expected_generation
        && expected != generation
    {
        // Before any refresh: a credential the caller did not expect is never
        // refreshed, let alone sent.
        return Err(CallFailure::new("E_CREDENTIAL_CHANGED"));
    }
    let fresh =
        match crate::auth::refresh::ensure_fresh_with_config(integration, alias, home, &config) {
            Ok(token) => token,
            Err(_) => {
                // Another process may have refreshed this slot concurrently (the
                // compare-and-store lost). Its token is as good as ours when it
                // is fresh and still the same grant.
                match crate::auth::keychain::load_token(integration, alias, home) {
                    Ok(Some(token))
                        if token.generation.as_deref() == Some(generation.as_str())
                            && token_fresh(&token) =>
                    {
                        token
                    }
                    _ => return Err(CallFailure::new("E_CREDENTIAL_EXPIRED")),
                }
            }
        };
    if fresh.generation.as_deref() != Some(generation.as_str()) {
        return Err(CallFailure::reason("E_CREDENTIAL_CHANGED", "refreshed"));
    }
    if fresh.access_token.trim().is_empty()
        || crate::runtime::invoker::never_sendable_char(&fresh.access_token).is_some()
        || crate::runtime::invoker::unsendable_in_header_char(&fresh.access_token).is_some()
    {
        return Err(CallFailure::reason("E_CREDENTIAL_MISSING", "unusable"));
    }
    Ok(SlotToken {
        access_token: fresh.access_token,
        generation,
        scope: fresh.scope,
    })
}

/// Immediately before the Drive request: the slot must still hold the same
/// grant. A disconnect or reconnect since the identity read is refused.
fn check_not_replaced(
    home: &Path,
    integration: &str,
    alias: Option<&str>,
    generation: &str,
) -> Result<(), CallFailure> {
    match crate::auth::keychain::load_token(integration, alias, home) {
        Ok(Some(token)) if token.generation.as_deref() == Some(generation) => Ok(()),
        _ => Err(CallFailure::reason("E_CREDENTIAL_CHANGED", "replaced")),
    }
}

// ── bounded HTTP ─────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Eq)]
enum HttpFailure {
    Status(u16),
    Transport,
    Timeout,
    TooLarge,
}

/// One bounded GET: no redirects, a total deadline, a body cap; a status >= 300
/// is a failure carrying only that status. Blocking.
fn get_bounded(
    url: &str,
    query: &[(String, String)],
    bearer: &str,
    timeout: Duration,
    cap: usize,
) -> Result<Vec<u8>, HttpFailure> {
    use std::io::Read;
    let started = Instant::now();
    let agent = ureq::AgentBuilder::new()
        .redirects(0)
        .resolver(crate::http_body::BoundedDnsResolver::new(
            timeout.min(Duration::from_secs(10)),
        ))
        .timeout(timeout)
        .build();
    let mut request = agent
        .get(url)
        .set("Authorization", &format!("Bearer {bearer}"))
        .set("Accept", "application/json");
    for (name, value) in query {
        request = request.query(name, value);
    }
    let timed_out = |started: Instant| started.elapsed() + Duration::from_millis(50) >= timeout;
    let response = match request.call() {
        Ok(response) => response,
        Err(ureq::Error::Status(_, response)) => response,
        Err(ureq::Error::Transport(_)) => {
            return Err(if timed_out(started) {
                HttpFailure::Timeout
            } else {
                HttpFailure::Transport
            });
        }
    };
    let status = response.status();
    if status >= 300 {
        return Err(HttpFailure::Status(status));
    }
    let mut bytes = Vec::new();
    if response
        .into_reader()
        .take(cap as u64 + 1)
        .read_to_end(&mut bytes)
        .is_err()
    {
        return Err(if timed_out(started) {
            HttpFailure::Timeout
        } else {
            HttpFailure::Transport
        });
    }
    if bytes.len() > cap {
        return Err(HttpFailure::TooLarge);
    }
    Ok(bytes)
}

/// The account a token belongs to, as Google's userinfo endpoint says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Principal {
    pub(crate) sub: String,
    /// The email, only when Google marks it verified.
    pub(crate) label: Option<String>,
}

fn clean(text: &str, max_chars: usize) -> Option<String> {
    let clean: String = text.chars().filter(|c| !c.is_control()).collect();
    let clean = clean.trim();
    (!clean.is_empty() && clean.chars().count() <= max_chars).then(|| clean.to_string())
}

fn read_identity(
    endpoints: &Endpoints,
    token: &SlotToken,
    timeout: Duration,
) -> Result<Principal, CallFailure> {
    let bytes = get_bounded(
        &endpoints.identity_url,
        &[],
        &token.access_token,
        timeout,
        IDENTITY_CAP,
    )
    .map_err(|failure| match failure {
        HttpFailure::Status(status) => {
            CallFailure::new("E_CALL_IDENTITY_FAILED").with("status", json!(status))
        }
        HttpFailure::Timeout => CallFailure::reason("E_CALL_IDENTITY_FAILED", "timeout"),
        HttpFailure::TooLarge => CallFailure::reason("E_CALL_IDENTITY_FAILED", "too-large"),
        HttpFailure::Transport => CallFailure::reason("E_CALL_IDENTITY_FAILED", "transport"),
    })?;
    let body: Json = serde_json::from_slice(&bytes)
        .map_err(|_| CallFailure::reason("E_CALL_IDENTITY_FAILED", "response-shape"))?;
    let sub = body
        .get("sub")
        .and_then(Json::as_str)
        .filter(|s| !s.is_empty() && s.len() <= 255 && !s.chars().any(char::is_control))
        .ok_or_else(|| CallFailure::reason("E_CALL_IDENTITY_FAILED", "no-subject"))?
        .to_string();
    let label = if body.get("email_verified").and_then(Json::as_bool) == Some(true) {
        body.get("email")
            .and_then(Json::as_str)
            .and_then(|e| clean(e, 256))
    } else {
        None
    };
    Ok(Principal { sub, label })
}

/// Remaining budget, or `E_CALL_TIMEOUT` once it is spent — checked before every
/// network phase, so no phase starts after the deadline.
fn remaining(timeout: Duration, started: Instant) -> Result<Duration, CallFailure> {
    timeout
        .checked_sub(started.elapsed())
        .filter(|left| *left > Duration::from_millis(50))
        .ok_or_else(|| CallFailure::new("E_CALL_TIMEOUT"))
}

/// The Drive read and its projection. Blocking.
fn read_files(
    endpoints: &Endpoints,
    op: &Operation,
    token: &SlotToken,
    query: &[(String, String)],
    timeout: Duration,
    cap: usize,
) -> Result<Json, CallFailure> {
    let bytes = get_bounded(
        endpoints.files_url(op),
        query,
        &token.access_token,
        timeout,
        cap,
    )
    .map_err(|failure| match failure {
        HttpFailure::Status(status) => {
            CallFailure::new("E_CALL_FAILED").with("status", json!(status))
        }
        HttpFailure::Timeout => CallFailure::new("E_CALL_TIMEOUT"),
        HttpFailure::TooLarge => CallFailure::new("E_CALL_OUTPUT_TOO_LARGE"),
        HttpFailure::Transport => CallFailure::reason("E_CALL_FAILED", "transport"),
    })?;
    let body: Json = serde_json::from_slice(&bytes)
        .map_err(|_| CallFailure::reason("E_CALL_FAILED", "response-shape"))?;
    project_files(&body, page_size(query))
        .ok_or_else(|| CallFailure::reason("E_CALL_FAILED", "response-shape"))
}

/// Project a `files.list` response onto the allowlisted, bounded shape. `None`
/// when it is not an object with a `files` array.
pub(crate) fn project_files(body: &Json, limit: usize) -> Option<Json> {
    let files = body.get("files")?.as_array()?;
    let mut out = Vec::new();
    for file in files {
        if out.len() >= limit {
            break;
        }
        let Some(id) = file
            .get("id")
            .and_then(Json::as_str)
            .and_then(|s| clean(s, 256))
        else {
            continue;
        };
        let mut entry = Map::new();
        entry.insert("id".into(), Json::String(id));
        if let Some(name) = file
            .get("name")
            .and_then(Json::as_str)
            .and_then(|s| clean(s, 1024))
        {
            entry.insert("name".into(), Json::String(name));
        }
        if let Some(mime) = file
            .get("mimeType")
            .and_then(Json::as_str)
            .and_then(|s| clean(s, 256))
        {
            entry.insert("mime-type".into(), Json::String(mime));
        }
        // Drive sends `size` as an int64 string; Google-native docs have none.
        if let Some(size) = file
            .get("size")
            .and_then(Json::as_str)
            .and_then(|s| s.parse::<u64>().ok())
            .filter(|n| *n <= (1u64 << 53))
        {
            entry.insert("size".into(), json!(size));
        }
        if let Some(time) = file
            .get("modifiedTime")
            .and_then(Json::as_str)
            .and_then(|s| clean(s, 64))
        {
            entry.insert("modified-time".into(), Json::String(time));
        }
        out.push(Json::Object(entry));
    }
    let more = body
        .get("nextPageToken")
        .and_then(Json::as_str)
        .is_some_and(|t| !t.is_empty());
    let incomplete = body.get("incompleteSearch").and_then(Json::as_bool) == Some(true);
    Some(json!({
        "files": out,
        "more-available": more,
        "incomplete-search": incomplete,
    }))
}

/// Run blocking work off the reactor.
async fn blocking<T: Send + 'static>(
    work: impl FnOnce() -> Result<T, CallFailure> + Send + 'static,
) -> Result<T, CallFailure> {
    tokio::task::spawn_blocking(work)
        .await
        .map_err(|_| CallFailure::reason("E_CALL_FAILED", "internal"))?
}

// ── call-capabilities ────────────────────────────────────────────────────────

/// `aware agent call-capabilities <agent> <command> [--as <alias>]`. The whole
/// verb is bounded by `timeout`.
pub(crate) async fn capabilities(
    home: &Path,
    agent_id: &str,
    command: &str,
    alias: Option<&str>,
    timeout: Duration,
) -> Result<Json, CallFailure> {
    let op = operation(agent_id, command).ok_or_else(|| CallFailure::new("E_CALL_UNSUPPORTED"))?;
    capabilities_with(
        home,
        op,
        alias,
        timeout,
        &Endpoints::production(op.integration),
    )
    .await
}

pub(crate) async fn capabilities_with(
    home: &Path,
    op: &'static Operation,
    alias: Option<&str>,
    timeout: Duration,
    endpoints: &Endpoints,
) -> Result<Json, CallFailure> {
    let started = Instant::now();
    match tokio::time::timeout(
        timeout,
        capabilities_within(home, op, alias, timeout, started, endpoints),
    )
    .await
    {
        Ok(outcome) => outcome,
        Err(_) => Err(CallFailure::new("E_CALL_TIMEOUT")),
    }
}

async fn capabilities_within(
    home: &Path,
    op: &'static Operation,
    alias: Option<&str>,
    timeout: Duration,
    started: Instant,
    endpoints: &Endpoints,
) -> Result<Json, CallFailure> {
    if let Some(alias) = alias
        && !crate::commands::credential::is_valid_alias(alias)
    {
        return Err(CallFailure::new("E_CALL_ALIAS_INVALID"));
    }
    let installed = read_installed(home, op.agent, None)?;
    let auth_ok = manifest_slot(&installed.agent, op, false).is_ok();
    let runnable = command_runnable(&installed.agent, op);
    endpoints.check_origins(op)?;

    // The credential: exactly this slot, verified live.
    let alias_owned = alias.map(str::to_string);
    let credential = {
        let home = home.to_path_buf();
        let endpoints = endpoints.clone();
        let integration = op.integration;
        blocking(move || {
            remaining(timeout, started)?;
            let token = load_slot(&home, integration, alias_owned.as_deref(), &endpoints, None)?;
            let left = remaining(timeout, started)?;
            // Leave the outer deadline room to report: the identity read gets a
            // little less than what remains.
            let principal = read_identity(
                &endpoints,
                &token,
                left.saturating_sub(Duration::from_millis(250))
                    .max(Duration::from_millis(100)),
            )?;
            Ok((token, principal))
        })
        .await
    };
    let slot = account_name(op.integration, alias);
    let (credential_json, verified_scope, credential_code) = match credential {
        Ok((token, principal)) => (
            json!({
                "status": "verified",
                "integration": op.integration,
                "alias": alias,
                "binding_id": binding_id(op.integration, &slot, &principal.sub),
                "revision": 1,
                "credential_generation": token.generation,
                "principal": {"integration": op.integration, "stable_id": principal.sub},
                "presentation_label": principal.label,
                "verified_at": now_ms(),
            }),
            Some(token.has_drive_scope()),
            None,
        ),
        // The verb's own deadline is not an account problem: report it as such.
        Err(failure) if failure.code == "E_CALL_TIMEOUT" => return Err(failure),
        Err(failure)
            if failure.code == "E_CREDENTIAL_MISSING" && failure.reason_str().is_none() =>
        {
            (
                json!({"status": "missing", "integration": op.integration, "alias": alias}),
                None,
                Some("credential-missing"),
            )
        }
        Err(failure) => {
            let code = match (failure.code, failure.reason_str()) {
                ("E_CREDENTIAL_MISSING", _) => "credential-missing",
                ("E_CREDENTIAL_EXPIRED", _) => "credential-expired",
                ("E_CREDENTIAL_CHANGED", Some("generation-unavailable")) => {
                    "generation-unavailable"
                }
                _ => "identity-unverified",
            };
            (
                json!({"status": "unverified", "integration": op.integration, "alias": alias,
                       "revision": 0}),
                None,
                Some(code),
            )
        }
    };
    let unavailable_code = if !auth_ok {
        Some("auth-invalid")
    } else if !runnable {
        Some("command-planned")
    } else if let Some(code) = credential_code {
        Some(code)
    } else if verified_scope != Some(true) {
        Some("scope-missing")
    } else {
        None
    };
    Ok(json!({
        "schema": CAPABILITY_SCHEMA,
        "agent": op.agent,
        "command": op.command,
        "installedVersion": installed.agent.version,
        "manifestSha256": installed.manifest_sha256,
        "operationSha256": op.sha256(),
        "integration": op.integration,
        "alias": alias,
        "transport": "rest",
        "effect": "read",
        "cancellation": "none",
        "inputs": op.inputs_json(),
        "credential": credential_json,
        "available": unavailable_code.is_none(),
        "unavailableReason": unavailable_code.map(|code| unavailable_reason(code, alias)),
        "unavailableCode": unavailable_code,
    }))
}

/// AWARE's fixed sentence for each `unavailableCode`.
pub(crate) fn unavailable_reason(code: &str, alias: Option<&str>) -> String {
    match code {
        "auth-invalid" => {
            "The installed agent does not use the google-workspace sign-in for this command.".into()
        }
        "command-planned" => {
            "The installed google-workspace agent does not offer list-files yet; update the agent.".into()
        }
        "credential-missing" => "No Google account is connected in this slot.".into(),
        "credential-expired" => {
            "The Google sign-in for this slot has expired and could not be renewed; reconnect it.".into()
        }
        "generation-unavailable" => {
            "AWARE could not record which sign-in this slot holds; reconnect it.".into()
        }
        "identity-unverified" => {
            "Google did not confirm which account this slot belongs to; check the connection and try again.".into()
        }
        "scope-missing" => {
            let alias_arg = alias.map(|a| format!(" --as={a}")).unwrap_or_default();
            format!(
                "This Google account was connected without Drive read access. Reconnect it with `aware connect google-workspace{alias_arg} --oauth --scopes {DRIVE_READONLY}`."
            )
        }
        _ => "This operation is not available.".into(),
    }
}

// ── call ─────────────────────────────────────────────────────────────────────

/// A validated `aware.agent-call/v1` request.
#[derive(Debug, Clone)]
pub(crate) struct CallRequest {
    raw: Map<String, Json>,
    invocation_id: String,
    agent: String,
    command: String,
    expected_agent_version: String,
    expected_manifest_sha256: String,
    expected_operation_sha256: String,
    integration: String,
    alias: Option<String>,
    binding_id: String,
    binding_revision: u64,
    credential_generation: String,
    inputs: Map<String, Json>,
    inputs_sha256: String,
    timeout: Duration,
    max_output_bytes: usize,
}

const REQUEST_FIELDS: &[&str] = &[
    "schema",
    "invocationId",
    "agent",
    "command",
    "expectedAgentVersion",
    "expectedManifestSha256",
    "executionTransport",
    "operationSchemaVersion",
    "expectedOperationSha256",
    "integration",
    "alias",
    "bindingId",
    "bindingRevision",
    "credentialGeneration",
    "inputs",
    "inputsSha256",
    "owner",
    "connectionRevision",
    "approvalReceiptId",
    "timeoutMs",
    "maxOutputBytes",
];

fn string_field<'a>(
    map: &'a Map<String, Json>,
    field: &'static str,
    max: usize,
) -> Result<&'a str, CallFailure> {
    map.get(field)
        .and_then(Json::as_str)
        .filter(|s| !s.is_empty() && s.len() <= max && !s.chars().any(char::is_control))
        .ok_or_else(|| CallFailure::field(field))
}

fn integer_field(
    map: &Map<String, Json>,
    field: &'static str,
    min: u64,
    max: u64,
) -> Result<u64, CallFailure> {
    map.get(field)
        .and_then(Json::as_u64)
        .filter(|v| (min..=max).contains(v))
        .ok_or_else(|| CallFailure::field(field))
}

/// Parse and shape-validate a request file's bytes.
pub(crate) fn parse_request(bytes: &[u8]) -> Result<CallRequest, CallFailure> {
    let value: Json = serde_json::from_slice(bytes)
        .map_err(|_| CallFailure::reason("E_CALL_REQUEST_INVALID", "not-json"))?;
    let Json::Object(map) = value else {
        return Err(CallFailure::reason(
            "E_CALL_REQUEST_INVALID",
            "not-an-object",
        ));
    };
    if map.keys().any(|k| !REQUEST_FIELDS.contains(&k.as_str())) {
        return Err(CallFailure::reason(
            "E_CALL_REQUEST_INVALID",
            "unknown-field",
        ));
    }
    if map.get("schema").and_then(Json::as_str) != Some(REQUEST_SCHEMA) {
        return Err(CallFailure::field("schema"));
    }
    let invocation_id = string_field(&map, "invocationId", 36)?;
    if !is_uuid(invocation_id) {
        return Err(CallFailure::field("invocationId"));
    }
    let agent = string_field(&map, "agent", 128)?;
    if !crate::manifest::loader::is_safe_segment(agent) {
        return Err(CallFailure::field("agent"));
    }
    let command = string_field(&map, "command", 128)?;
    let expected_agent_version = string_field(&map, "expectedAgentVersion", 128)?;
    let hex = |field: &'static str| -> Result<String, CallFailure> {
        let value = string_field(&map, field, 64)?;
        if is_sha256_hex(value) {
            Ok(value.to_string())
        } else {
            Err(CallFailure::field(field))
        }
    };
    let expected_manifest_sha256 = hex("expectedManifestSha256")?;
    let expected_operation_sha256 = hex("expectedOperationSha256")?;
    let inputs_sha256 = hex("inputsSha256")?;
    if map.get("executionTransport").and_then(Json::as_str) != Some("rest") {
        return Err(CallFailure::field("executionTransport"));
    }
    if map.get("operationSchemaVersion").and_then(Json::as_u64) != Some(1) {
        return Err(CallFailure::field("operationSchemaVersion"));
    }
    let integration = string_field(&map, "integration", 128)?;
    let alias = match map.get("alias") {
        None | Some(Json::Null) => None,
        Some(Json::String(alias)) if crate::commands::credential::is_valid_alias(alias) => {
            Some(alias.clone())
        }
        _ => return Err(CallFailure::field("alias")),
    };
    let binding_id = string_field(&map, "bindingId", 36)?;
    if !is_uuid(binding_id) {
        return Err(CallFailure::field("bindingId"));
    }
    let binding_revision = integer_field(&map, "bindingRevision", 1, u64::from(u32::MAX))?;
    let credential_generation = string_field(&map, "credentialGeneration", 128)?;
    if !credential_generation
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-')
    {
        return Err(CallFailure::field("credentialGeneration"));
    }
    let Some(Json::Object(inputs)) = map.get("inputs") else {
        return Err(CallFailure::field("inputs"));
    };
    for value in inputs.values() {
        let scalar = match value {
            Json::String(_) | Json::Bool(_) => true,
            Json::Number(n) => n.as_i64().is_some_and(|v| v.unsigned_abs() < (1u64 << 53)),
            _ => false,
        };
        if !scalar {
            return Err(CallFailure::field("inputs"));
        }
    }
    match map.get("owner") {
        Some(owner @ Json::Object(_)) if owner.to_string().len() <= MAX_OWNER_BYTES => {}
        _ => return Err(CallFailure::field("owner")),
    }
    string_field(&map, "connectionRevision", 256)?;
    string_field(&map, "approvalReceiptId", 256)?;
    let timeout_ms = integer_field(&map, "timeoutMs", MIN_TIMEOUT_MS, MAX_TIMEOUT_MS)?;
    let max_output_bytes =
        integer_field(&map, "maxOutputBytes", MIN_OUTPUT_BYTES, MAX_OUTPUT_BYTES)?;
    Ok(CallRequest {
        invocation_id: invocation_id.to_string(),
        agent: agent.to_string(),
        command: command.to_string(),
        expected_agent_version: expected_agent_version.to_string(),
        expected_manifest_sha256,
        expected_operation_sha256,
        integration: integration.to_string(),
        alias,
        binding_id: binding_id.to_string(),
        binding_revision,
        credential_generation: credential_generation.to_string(),
        inputs: inputs.clone(),
        inputs_sha256,
        timeout: Duration::from_millis(timeout_ms),
        max_output_bytes: usize::try_from(max_output_bytes).unwrap_or(usize::MAX),
        raw: map,
    })
}

impl CallRequest {
    pub(crate) fn agent(&self) -> &str {
        &self.agent
    }

    pub(crate) fn command(&self) -> &str {
        &self.command
    }

    fn correlation(&self) -> Json {
        json!({
            "invocationId": self.invocation_id,
            "owner": self.raw.get("owner"),
            "connectionRevision": self.raw.get("connectionRevision"),
            "approvalReceiptId": self.raw.get("approvalReceiptId"),
        })
    }
}

/// `aware agent call @<request>` — the request's bytes have been read already.
pub(crate) async fn call(home: &Path, request: &CallRequest) -> Result<Json, CallFailure> {
    let op = operation(&request.agent, &request.command)
        .ok_or_else(|| CallFailure::new("E_CALL_UNSUPPORTED"))?;
    call_with(home, request, &Endpoints::production(op.integration)).await
}

pub(crate) async fn call_with(
    home: &Path,
    request: &CallRequest,
    endpoints: &Endpoints,
) -> Result<Json, CallFailure> {
    let started = Instant::now();
    let admitted_at = now_ms();
    match tokio::time::timeout(
        request.timeout,
        call_within(home, request, endpoints, started, admitted_at),
    )
    .await
    {
        Ok(outcome) => outcome,
        Err(_) => Err(CallFailure::new("E_CALL_TIMEOUT")),
    }
}

async fn call_within(
    home: &Path,
    request: &CallRequest,
    endpoints: &Endpoints,
    started: Instant,
    admitted_at: i64,
) -> Result<Json, CallFailure> {
    // 2. A reviewed operation, for the integration it belongs to.
    let op = operation(&request.agent, &request.command)
        .ok_or_else(|| CallFailure::new("E_CALL_UNSUPPORTED"))?;
    if request.integration != op.integration {
        return Err(CallFailure::new("E_CALL_UNSUPPORTED"));
    }
    // 3. The installed agent is the one the caller reviewed.
    let installed = read_installed(home, op.agent, Some(&request.expected_manifest_sha256))?;
    if installed.agent.version != request.expected_agent_version {
        return Err(CallFailure::reason("E_CALL_CHANGED", "version"));
    }
    manifest_slot(&installed.agent, op, false)?;
    if !command_runnable(&installed.agent, op) {
        return Err(CallFailure::reason("E_CALL_UNAVAILABLE", "command-planned"));
    }
    // 4. The operation is the one the caller reviewed.
    if op.sha256() != request.expected_operation_sha256 {
        return Err(CallFailure::reason("E_CALL_CHANGED", "operation"));
    }
    // 5. Inputs: mapped and in bounds first (so key order can never matter),
    //    then the digest the caller computed over them.
    let query = wire_query(op, &request.inputs)?;
    let digest = sha256_hex(canonical_string(&Json::Object(request.inputs.clone())).as_bytes());
    if digest != request.inputs_sha256 {
        return Err(CallFailure::field("inputsSha256"));
    }
    // 6. Origins, before any credential is read.
    endpoints.check_origins(op)?;

    // 7-9. Credential, identity, replacement boundary, Drive — blocking.
    let home_owned = home.to_path_buf();
    let endpoints = endpoints.clone();
    let alias = request.alias.clone();
    let expected_generation = request.credential_generation.clone();
    let expected_binding = request.binding_id.to_ascii_lowercase();
    let binding_revision = request.binding_revision;
    let timeout = request.timeout;
    let cap = request.max_output_bytes;
    let body = blocking(move || {
        remaining(timeout, started)?;
        let token = load_slot(
            &home_owned,
            op.integration,
            alias.as_deref(),
            &endpoints,
            Some(&expected_generation),
        )?;
        if !token.has_drive_scope() {
            return Err(CallFailure::new("E_CALL_SCOPE_MISSING"));
        }
        let principal = read_identity(&endpoints, &token, remaining(timeout, started)?)?;
        let slot = account_name(op.integration, alias.as_deref());
        if binding_id(op.integration, &slot, &principal.sub) != expected_binding
            || binding_revision != 1
        {
            return Err(CallFailure::new("E_BINDING_CHANGED"));
        }
        let left = remaining(timeout, started)?;
        check_not_replaced(
            &home_owned,
            op.integration,
            alias.as_deref(),
            &token.generation,
        )?;
        read_files(&endpoints, op, &token, &query, left, cap)
    })
    .await?;

    let request_digest =
        sha256_hex(canonical_string(&Json::Object(request.raw.clone())).as_bytes());
    let correlation = sha256_hex(canonical_string(&request.correlation()).as_bytes());
    let updated_at = now_ms();
    Ok(json!({
        "schema": RECORD_SCHEMA,
        "invocationId": request.invocation_id,
        "requestDigest": request_digest,
        "correlationSha256": correlation,
        "bindingId": request.binding_id,
        "requestedBindingRevision": request.binding_revision,
        "credentialGeneration": request.credential_generation,
        "resolvedBindingRevision": 1,
        "state": "completed",
        "admittedAt": admitted_at,
        "updatedAt": updated_at,
        "cancellationRequested": false,
        "cancelledAfterDispatch": false,
        "httpStatus": 200,
        "outcome": "ok",
        "payloadPresent": true,
        "resultExpired": false,
        "result": {
            "schema": RESULT_SCHEMA,
            "invocationId": request.invocation_id,
            "requestDigest": request_digest,
            "correlationSha256": correlation,
            "bindingId": request.binding_id,
            "bindingRevision": request.binding_revision,
            "resolvedBindingRevision": 1,
            "credentialGeneration": request.credential_generation,
            "httpStatus": 200,
            "outcome": "ok",
            "body": body,
            "cancelledAfterDispatch": false,
        },
    }))
}

// ── workflow path ────────────────────────────────────────────────────────────

/// Whether `RestInvoker` must hand `(agent, command)` to [`run_for_workflow`].
pub(crate) fn handles_workflow(agent: &str, command: &str) -> bool {
    operation(agent, command).is_some()
}

/// A workflow node running a reviewed operation: the same executor, the
/// manifest's own slot (a baked `google-workspace.<alias>` honoured), fixed
/// bounds, no identity step (a workflow has no binding to check).
pub(crate) async fn run_for_workflow(
    home: &Path,
    manifest: &Agent,
    command: &str,
    args: &Json,
) -> Result<Json, crate::error::AwareError> {
    run_for_workflow_with(
        home,
        manifest,
        command,
        args,
        None,
        WORKFLOW_TIMEOUT,
        WORKFLOW_OUTPUT_BYTES,
    )
    .await
    .map_err(workflow_error)
}

fn workflow_error(failure: CallFailure) -> crate::error::AwareError {
    let mut detail = failure.code.to_string();
    for (key, value) in &failure.details {
        detail.push_str(&format!(" {key}={value}"));
    }
    crate::error::AwareError::Validation(format!("{} [{detail}]", failure.message()))
}

pub(crate) async fn run_for_workflow_with(
    home: &Path,
    manifest: &Agent,
    command: &str,
    args: &Json,
    endpoints: Option<&Endpoints>,
    timeout: Duration,
    cap: usize,
) -> Result<Json, CallFailure> {
    let started = Instant::now();
    let op = operation(&manifest.agent, command)
        .ok_or_else(|| CallFailure::new("E_CALL_UNSUPPORTED"))?;
    let alias = manifest_slot(manifest, op, true)?;
    if !command_runnable(manifest, op) {
        return Err(CallFailure::reason("E_CALL_UNAVAILABLE", "command-planned"));
    }
    let inputs = match args {
        Json::Object(map) => map.clone(),
        Json::Null => Map::new(),
        _ => return Err(CallFailure::field("inputs")),
    };
    let query = wire_query(op, &inputs)?;
    let endpoints = endpoints
        .cloned()
        .unwrap_or_else(|| Endpoints::production(op.integration));
    endpoints.check_origins(op)?;
    let home = home.to_path_buf();
    let work = blocking(move || {
        remaining(timeout, started)?;
        let token = load_slot(&home, op.integration, alias.as_deref(), &endpoints, None)?;
        if !token.has_drive_scope() {
            return Err(CallFailure::new("E_CALL_SCOPE_MISSING"));
        }
        let left = remaining(timeout, started)?;
        read_files(&endpoints, op, &token, &query, left, cap)
    });
    match tokio::time::timeout(timeout, work).await {
        Ok(outcome) => outcome,
        Err(_) => Err(CallFailure::new("E_CALL_TIMEOUT")),
    }
}

// ── log ──────────────────────────────────────────────────────────────────────

/// Append the code-only log line for one invocation. Best-effort.
pub(crate) fn append_log_line(
    logs_dir: &Path,
    verb: &str,
    agent: &str,
    command: &str,
    outcome: &str,
) {
    use std::io::Write;
    let safe = |id: &str| -> String {
        if crate::manifest::loader::is_safe_segment(id) {
            id.to_string()
        } else {
            "-".to_string()
        }
    };
    let line = format!(
        "{} agent-call {verb} {} {} {outcome}\n",
        crate::time::now_iso(),
        safe(agent),
        if operation(agent, command).is_some() {
            command.to_string()
        } else {
            "-".to_string()
        },
    );
    if std::fs::create_dir_all(logs_dir).is_err() {
        return;
    }
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(logs_dir.join("agent-call.log"))
    {
        let _ = file.write_all(line.as_bytes());
    }
}

#[cfg(test)]
#[path = "agent_call_tests.rs"]
mod tests;
