//! `aware agent call-capabilities` and single-shot `aware agent call` (#618) —
//! reviewed, read-only, account-bound REST reads: Google Drive `list-files`,
//! Microsoft OneDrive `list-folder` (#667, #709 slice A) and the Microsoft
//! 365 reads of #709 slice B1 (`get-drive-item`, `outlook.mail.search`,
//! Planner plans and tasks, SharePoint lists and rows).
//!
//! Plans: `docs/superpowers/specs/2026-10-08-618-agent-call-slice-PLAN.md`
//! and `docs/superpowers/specs/2026-11-01-709-m365-transport-PLAN.md`. The
//! rules are the #617 probe's, applied to a caller-driven read:
//!
//! - **The operation is code-owned.** Method, origin, path, the literal field
//!   mask, the input → wire mapping, bounds, the scope allowlist and the output
//!   projection live in [`OPERATIONS`]. A manifest only has to *declare* the
//!   command runnable; it cannot change where the token goes or what is sent.
//! - **Exactly one account slot.** `integration[.alias]`, never another — a
//!   missing alias slot is "missing", never the default account.
//! - **The account is verified live.** Both verbs ask the provider who the
//!   slot's token belongs to (Google's OpenID userinfo, Microsoft Graph `/me`);
//!   the binding id is derived from that answer, so a reconnect to another
//!   account changes it and a call bound to the old account is refused before
//!   the data request.
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

/// Microsoft Graph — identity and every reviewed Microsoft data request.
pub(crate) const MICROSOFT_GRAPH_ORIGIN: &str = "https://graph.microsoft.com";
/// Graph `/me` — the code-owned Microsoft identity read.
pub(crate) const MICROSOFT_ME_URL: &str = "https://graph.microsoft.com/v1.0/me";
/// The only host a microsoft-365 refresh token may be sent to from this module.
const MICROSOFT_LOGIN_ORIGIN: &str = "https://login.microsoftonline.com";
/// `/me` needs `User.Read`; the operation scopes alone do not authorize it.
const MICROSOFT_IDENTITY_SCOPES: &[&str] = &["User.Read"];
/// The OneDrive scope AWARE names (owner ruling 2026-11-01): it lists shared and
/// SharePoint drives the person can already see, and needs no admin consent.
pub(crate) const FILES_READ_ALL: &str = "Files.Read.All";
/// Any one of these lets `list-folder` read folder metadata.
const MICROSOFT_FILES_SCOPES: &[&str] = &[
    "Files.Read",
    FILES_READ_ALL,
    "Files.ReadWrite",
    "Files.ReadWrite.All",
];
/// Any one of these lets `outlook.mail.search` read messages with their
/// preview (`Mail.ReadBasic` omits `bodyPreview`, so it is not enough).
const MICROSOFT_MAIL_SCOPES: &[&str] = &["Mail.Read", "Mail.ReadWrite"];
/// Any one of these reads Planner. The `Group.*` grants need admin consent;
/// AWARE never asks for them, but a person who holds one keeps working.
const MICROSOFT_TASKS_SCOPES: &[&str] = &[
    "Tasks.Read",
    "Tasks.ReadWrite",
    "Group.Read.All",
    "Group.ReadWrite.All",
];
/// Any one of these reads SharePoint lists (delegated: no admin consent).
const MICROSOFT_SITES_SCOPES: &[&str] = &["Sites.Read.All", "Sites.ReadWrite.All"];

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
                "The account was connected without the access this command needs, so nothing was read."
            }
            "E_CALL_IDENTITY_FAILED" => {
                "The service did not confirm which account this credential belongs to."
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

/// Where an input goes on the wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Location {
    Query,
    Path,
    /// A term of the one `$search` value the code renders (Graph mail search).
    Search,
    /// Never sent: validated, then used by the code (a page bound applied to
    /// the projection where the endpoint documents no `$top`).
    Local,
}

impl Location {
    fn as_str(self) -> &'static str {
        match self {
            Location::Query => "query",
            Location::Path => "path",
            Location::Search => "search",
            Location::Local => "local",
        }
    }
}

/// The grammar a path input must match before it becomes a URL segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IdKind {
    /// A Microsoft Graph drive or drive-item id: `b!…` base64url, hex, or
    /// `ABC!123`, and the literal `root`. No dot at all, so `.` and `..` can
    /// never form; no `/`, `%`, `:` or whitespace.
    GraphOpaqueId,
    /// A SharePoint site id: `root`, or `host,guid,guid` — DNS labels then two
    /// GUIDs. No label is empty, so no segment can be `.` or `..`.
    SharePointSiteId,
    /// A GUID, `8-4-4-4-12` hex (a SharePoint list id).
    Guid,
}

/// `8-4-4-4-12` hex digits.
fn is_guid(value: &str) -> bool {
    let groups: Vec<&str> = value.split('-').collect();
    groups.len() == 5
        && groups
            .iter()
            .zip([8, 4, 4, 4, 12])
            .all(|(g, n)| g.len() == n && g.bytes().all(|b| b.is_ascii_hexdigit()))
}

/// A DNS host name: 1..253 bytes of labels `[A-Za-z0-9-]{1,63}` joined by
/// single dots, no label starting or ending with `-`.
fn is_dns_host(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 253
        && value.split('.').all(|label| {
            !label.is_empty()
                && label.len() <= 63
                && !label.starts_with('-')
                && !label.ends_with('-')
                && label
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'-')
        })
}

impl IdKind {
    fn as_str(self) -> &'static str {
        match self {
            IdKind::GraphOpaqueId => "graph-opaque-id",
            IdKind::SharePointSiteId => "sharepoint-site-id",
            IdKind::Guid => "guid",
        }
    }

    fn accepts(self, value: &str) -> bool {
        match self {
            IdKind::GraphOpaqueId => {
                !value.is_empty()
                    && value.len() <= 256
                    && value
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'!' | b'_' | b'-'))
            }
            IdKind::SharePointSiteId => {
                if value == "root" {
                    return true;
                }
                let parts: Vec<&str> = value.split(',').collect();
                parts.len() == 3 && is_dns_host(parts[0]) && is_guid(parts[1]) && is_guid(parts[2])
            }
            IdKind::Guid => is_guid(value),
        }
    }
}

/// How a `Location::Search` input becomes `$search` terms: typed inputs
/// rendered by code, never caller-written KQL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SearchFormat {
    /// 1..16 plain words (letters, digits, `.`, `_`, `-`, `@`), none a KQL
    /// operator or starting with `-`; joined by spaces (KQL's implicit AND).
    Words,
    /// One mail address, rendered `from:<address>`.
    FromAddress,
}

/// KQL words that are operators, not search text.
const KQL_OPERATORS: &[&str] = &["AND", "OR", "NOT", "NEAR", "ONEAR"];

impl SearchFormat {
    fn as_str(self) -> &'static str {
        match self {
            SearchFormat::Words => "graph-search-words/1",
            SearchFormat::FromAddress => "graph-search-from/1",
        }
    }

    /// The `$search` terms for a value, or `None` when it is outside the format.
    fn terms(self, value: &str) -> Option<Vec<String>> {
        match self {
            SearchFormat::Words => {
                let words: Vec<&str> = value.split(' ').filter(|w| !w.is_empty()).collect();
                let ok = !words.is_empty()
                    && words.len() <= 16
                    && words.iter().all(|word| {
                        word.chars().count() <= 64
                            // `-term` is KQL's NOT; inner hyphens are text.
                            && !word.starts_with('-')
                            && word
                                .chars()
                                .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '-' | '@'))
                            && !KQL_OPERATORS.iter().any(|op| op.eq_ignore_ascii_case(word))
                    });
                ok.then(|| words.iter().map(|w| (*w).to_string()).collect())
            }
            SearchFormat::FromAddress => {
                let (local, domain) = value.split_once('@')?;
                let ok = !local.is_empty()
                    && local.len() <= 64
                    && local.bytes().all(|b| {
                        b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'%' | b'+' | b'-')
                    })
                    && is_dns_host(domain);
                ok.then(|| vec![format!("from:{value}")])
            }
        }
    }
}

/// One caller input and where it goes on the wire.
#[derive(Debug, Clone)]
struct InputMap {
    /// The input name the manifest command and the caller use.
    source: &'static str,
    /// The wire name (query parameter); for a path input, the input name.
    name: &'static str,
    location: Location,
    required: bool,
    scalar: Scalar,
    default: Option<i64>,
    minimum: Option<i64>,
    maximum: Option<i64>,
    max_bytes: Option<usize>,
    /// Path inputs only: the grammar the value must match.
    id_kind: Option<IdKind>,
    /// Search inputs only: how the value becomes `$search` terms.
    search: Option<SearchFormat>,
}

impl InputMap {
    const fn query_string(source: &'static str, name: &'static str, max_bytes: usize) -> Self {
        Self {
            source,
            name,
            location: Location::Query,
            required: false,
            scalar: Scalar::String,
            default: None,
            minimum: None,
            maximum: None,
            max_bytes: Some(max_bytes),
            id_kind: None,
            search: None,
        }
    }

    const fn query_limit(source: &'static str, name: &'static str, max: i64) -> Self {
        Self {
            source,
            name,
            location: Location::Query,
            required: false,
            scalar: Scalar::Integer,
            default: Some(100),
            minimum: Some(1),
            maximum: Some(max),
            max_bytes: None,
            id_kind: None,
            search: None,
        }
    }

    const fn path_id(source: &'static str) -> Self {
        Self {
            source,
            name: source,
            location: Location::Path,
            required: true,
            scalar: Scalar::String,
            default: None,
            minimum: None,
            maximum: None,
            max_bytes: Some(256),
            id_kind: Some(IdKind::GraphOpaqueId),
            search: None,
        }
    }

    /// A required path id of another grammar.
    const fn path_kind(source: &'static str, kind: IdKind) -> Self {
        let mut map = Self::path_id(source);
        map.id_kind = Some(kind);
        map
    }

    /// A page bound applied by the code, never sent.
    const fn local_limit(source: &'static str, default: i64, max: i64) -> Self {
        let mut map = Self::query_limit(source, source, max);
        map.location = Location::Local;
        map.default = Some(default);
        map
    }

    /// A `$top` page size with its own default.
    const fn top(source: &'static str, default: i64, max: i64) -> Self {
        let mut map = Self::query_limit(source, "$top", max);
        map.default = Some(default);
        map
    }

    /// An optional typed `$search` term.
    const fn search(source: &'static str, format: SearchFormat, max_bytes: usize) -> Self {
        let mut map = Self::query_string(source, source, max_bytes);
        map.location = Location::Search;
        map.search = Some(format);
        map
    }
}

/// One URL path segment: fixed text, or a validated path input.
#[derive(Debug, Clone)]
enum Seg {
    Lit(&'static str),
    Input(&'static str),
}

impl Seg {
    fn as_json(&self) -> Json {
        match self {
            Seg::Lit(text) => Json::String((*text).into()),
            Seg::Input(name) => Json::String(format!("{{{name}}}")),
        }
    }
}

/// The request path: an origin plus fixed segments, assembled segment by
/// segment (never by string templating). `alternate` is a second fixed route
/// chosen when one input holds an exact literal (`drive-id: me`).
#[derive(Debug, Clone)]
struct Route {
    origin: &'static str,
    segments: &'static [Seg],
    alternate: Option<Alternate>,
}

#[derive(Debug, Clone)]
struct Alternate {
    input: &'static str,
    literal: &'static str,
    segments: &'static [Seg],
}

/// Who the operation's account belongs to: where its token may be refreshed,
/// how the account is identified, and how scopes compare.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Provider {
    Google,
    Microsoft,
}

impl Provider {
    fn identity_url(self) -> &'static str {
        match self {
            Provider::Google => GOOGLE_USERINFO_URL,
            Provider::Microsoft => MICROSOFT_ME_URL,
        }
    }

    fn identity_query(self) -> &'static [(&'static str, &'static str)] {
        match self {
            Provider::Google => &[],
            Provider::Microsoft => &[("$select", "id,userPrincipalName")],
        }
    }

    /// Scopes the identity read itself needs, all of them, on top of the
    /// operation's own.
    fn identity_scopes(self) -> &'static [&'static str] {
        match self {
            Provider::Google => &[],
            Provider::Microsoft => MICROSOFT_IDENTITY_SCOPES,
        }
    }

    /// Whether a granted scope is `wanted`. Google compares exactly. Microsoft
    /// returns Graph scopes as short names or as Graph resource URLs and in no
    /// fixed case, so the exact Graph resource prefix is dropped and case folded.
    fn scope_is(self, granted: &str, wanted: &str) -> bool {
        match self {
            Provider::Google => granted == wanted,
            Provider::Microsoft => {
                crate::auth::keychain::microsoft_scope_key(granted)
                    == crate::auth::keychain::microsoft_scope_key(wanted)
            }
        }
    }

    fn name(self) -> &'static str {
        match self {
            Provider::Google => "Google",
            Provider::Microsoft => "Microsoft",
        }
    }
}

/// Which projection turns the service's answer into the call's output. Its
/// name and version are part of the operation digest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Projection {
    DriveFiles,
    GraphChildren,
    GraphItem,
    GraphMessages,
    PlannerPlans,
    PlannerTasks,
    SharePointLists,
    SharePointItems,
}

impl Projection {
    fn id(self) -> &'static str {
        match self {
            Projection::DriveFiles => "drive-files/1",
            Projection::GraphChildren => "graph-children/1",
            Projection::GraphItem => "graph-item/1",
            Projection::GraphMessages => "graph-messages/1",
            Projection::PlannerPlans => "planner-plans/1",
            Projection::PlannerTasks => "planner-tasks/1",
            Projection::SharePointLists => "sp-lists/1",
            Projection::SharePointItems => "sp-items/1",
        }
    }

    fn apply(self, body: &Json, limit: usize) -> Option<Json> {
        match self {
            Projection::DriveFiles => project_files(body, limit),
            Projection::GraphChildren => project_children(body, limit),
            Projection::GraphItem => project_drive_item(body),
            Projection::GraphMessages => project_collection(body, limit, "messages", message_entry),
            Projection::PlannerPlans => project_collection(body, limit, "plans", plan_entry),
            Projection::PlannerTasks => project_collection(body, limit, "tasks", task_entry),
            Projection::SharePointLists => project_collection(body, limit, "lists", list_entry),
            Projection::SharePointItems => {
                project_collection(body, limit, "items", list_item_entry)
            }
        }
    }
}

/// One reviewed operation. Everything that decides where a credential goes and
/// what is sent is here, in code.
#[derive(Debug, Clone)]
pub(crate) struct Operation {
    agent: &'static str,
    command: &'static str,
    integration: &'static str,
    provider: Provider,
    route: Route,
    /// Query pairs sent on every request (a field mask, a `$select`).
    fixed_query: &'static [(&'static str, &'static str)],
    inputs: &'static [InputMap],
    /// The page-size input (its source name), which also bounds the projection.
    limit_input: Option<&'static str>,
    /// Any one of these grants the data request.
    any_of_scopes: &'static [&'static str],
    /// What AWARE tells a person to request (identity scopes included).
    request_scopes: &'static [&'static str],
    /// What a person lacks without those scopes, in a sentence.
    access: &'static str,
    /// The provider documents the delegated permission as needing tenant admin
    /// consent. Tenant policy may demand consent for anything; this says only
    /// what the provider requires.
    admin_consent: bool,
    projection: Projection,
    /// Digest form: `false` keeps the representation an operation shipped
    /// with (Google `list-files`, `list-folder`); `true` adds the page-bound
    /// input, so every Slice B row binds what its new input kinds do.
    canonical_v2: bool,
}

/// `fields` for `files.list`: `nextPageToken` must be in the mask or a first
/// page would read as complete; a page shorter than `pageSize` may still have
/// more.
const LIST_FILES_FIELDS: &str =
    "nextPageToken,incompleteSearch,files(id,name,mimeType,size,modifiedTime)";

/// `$select` for a Graph children listing. `parentReference` carries the drive
/// id a caller needs to open a child folder.
const LIST_FOLDER_SELECT: &str = "id,name,size,folder,file,lastModifiedDateTime,parentReference";

/// `$select` for one drive item: no `@microsoft.graph.downloadUrl` (a signed
/// URL never reaches a caller or a log).
const GET_ITEM_SELECT: &str =
    "id,name,size,folder,file,lastModifiedDateTime,parentReference,webUrl";

/// `$select` for a mail search: the preview, never the body.
const MAIL_SEARCH_SELECT: &str =
    "id,subject,from,receivedDateTime,bodyPreview,hasAttachments,webLink";

/// `$select` for a site's lists.
const SP_LISTS_SELECT: &str = "id,displayName,webUrl,list,lastModifiedDateTime";

static OPERATIONS: &[Operation] = &[
    Operation {
        agent: "google-workspace",
        command: "list-files",
        integration: "google-workspace",
        provider: Provider::Google,
        route: Route {
            origin: "https://www.googleapis.com",
            segments: &[Seg::Lit("drive"), Seg::Lit("v3"), Seg::Lit("files")],
            alternate: None,
        },
        fixed_query: &[("fields", LIST_FILES_FIELDS)],
        inputs: &[
            InputMap::query_string("query", "q", 2048),
            InputMap::query_limit("page-size", "pageSize", 100),
        ],
        limit_input: Some("page-size"),
        any_of_scopes: OPTIONAL_GOOGLE_SCOPES,
        request_scopes: &[DRIVE_READONLY],
        access: "Drive read access",
        admin_consent: false,
        projection: Projection::DriveFiles,
        canonical_v2: false,
    },
    Operation {
        agent: "microsoft-365",
        command: "list-folder",
        integration: "microsoft-365",
        provider: Provider::Microsoft,
        route: Route {
            origin: MICROSOFT_GRAPH_ORIGIN,
            segments: &[
                Seg::Lit("v1.0"),
                Seg::Lit("drives"),
                Seg::Input("drive-id"),
                Seg::Lit("items"),
                Seg::Input("folder-id"),
                Seg::Lit("children"),
            ],
            alternate: Some(Alternate {
                input: "drive-id",
                literal: "me",
                segments: &[
                    Seg::Lit("v1.0"),
                    Seg::Lit("me"),
                    Seg::Lit("drive"),
                    Seg::Lit("items"),
                    Seg::Input("folder-id"),
                    Seg::Lit("children"),
                ],
            }),
        },
        fixed_query: &[("$select", LIST_FOLDER_SELECT)],
        inputs: &[
            InputMap::path_id("drive-id"),
            InputMap::path_id("folder-id"),
            InputMap::query_limit("page-size", "$top", 200),
        ],
        limit_input: Some("page-size"),
        any_of_scopes: MICROSOFT_FILES_SCOPES,
        request_scopes: &["User.Read", FILES_READ_ALL],
        access: "OneDrive read access",
        admin_consent: false,
        projection: Projection::GraphChildren,
        canonical_v2: false,
    },
    Operation {
        agent: "microsoft-365",
        command: "get-drive-item",
        integration: "microsoft-365",
        provider: Provider::Microsoft,
        route: Route {
            origin: MICROSOFT_GRAPH_ORIGIN,
            segments: &[
                Seg::Lit("v1.0"),
                Seg::Lit("drives"),
                Seg::Input("drive-id"),
                Seg::Lit("items"),
                Seg::Input("item-id"),
            ],
            alternate: Some(Alternate {
                input: "drive-id",
                literal: "me",
                segments: &[
                    Seg::Lit("v1.0"),
                    Seg::Lit("me"),
                    Seg::Lit("drive"),
                    Seg::Lit("items"),
                    Seg::Input("item-id"),
                ],
            }),
        },
        fixed_query: &[("$select", GET_ITEM_SELECT)],
        inputs: &[InputMap::path_id("drive-id"), InputMap::path_id("item-id")],
        limit_input: None,
        any_of_scopes: MICROSOFT_FILES_SCOPES,
        request_scopes: &["User.Read", FILES_READ_ALL],
        access: "OneDrive read access",
        admin_consent: false,
        projection: Projection::GraphItem,
        canonical_v2: true,
    },
    Operation {
        agent: "microsoft-365",
        command: "outlook.mail.search",
        integration: "microsoft-365",
        provider: Provider::Microsoft,
        route: Route {
            origin: MICROSOFT_GRAPH_ORIGIN,
            segments: &[Seg::Lit("v1.0"), Seg::Lit("me"), Seg::Lit("messages")],
            alternate: None,
        },
        fixed_query: &[("$select", MAIL_SEARCH_SELECT)],
        inputs: &[
            InputMap::search("text", SearchFormat::Words, 1024),
            InputMap::search("from", SearchFormat::FromAddress, 320),
            InputMap::top("page-size", 25, 50),
        ],
        limit_input: Some("page-size"),
        any_of_scopes: MICROSOFT_MAIL_SCOPES,
        request_scopes: &["User.Read", "Mail.Read"],
        access: "Outlook mail read access",
        admin_consent: false,
        projection: Projection::GraphMessages,
        canonical_v2: true,
    },
    Operation {
        agent: "microsoft-365",
        command: "planner.plan.list",
        integration: "microsoft-365",
        provider: Provider::Microsoft,
        route: Route {
            origin: MICROSOFT_GRAPH_ORIGIN,
            segments: &[
                Seg::Lit("v1.0"),
                Seg::Lit("me"),
                Seg::Lit("planner"),
                Seg::Lit("plans"),
            ],
            alternate: None,
        },
        // Planner documents no OData options: nothing is sent, the page is
        // bounded locally.
        fixed_query: &[],
        inputs: &[InputMap::local_limit("page-size", 100, 200)],
        limit_input: Some("page-size"),
        any_of_scopes: MICROSOFT_TASKS_SCOPES,
        request_scopes: &["User.Read", "Tasks.Read"],
        access: "Planner read access",
        admin_consent: false,
        projection: Projection::PlannerPlans,
        canonical_v2: true,
    },
    Operation {
        agent: "microsoft-365",
        command: "planner.task.list",
        integration: "microsoft-365",
        provider: Provider::Microsoft,
        route: Route {
            origin: MICROSOFT_GRAPH_ORIGIN,
            segments: &[
                Seg::Lit("v1.0"),
                Seg::Lit("planner"),
                Seg::Lit("plans"),
                Seg::Input("plan-id"),
                Seg::Lit("tasks"),
            ],
            alternate: None,
        },
        fixed_query: &[],
        inputs: &[
            InputMap::path_id("plan-id"),
            InputMap::local_limit("page-size", 100, 200),
        ],
        limit_input: Some("page-size"),
        any_of_scopes: MICROSOFT_TASKS_SCOPES,
        request_scopes: &["User.Read", "Tasks.Read"],
        access: "Planner read access",
        admin_consent: false,
        projection: Projection::PlannerTasks,
        canonical_v2: true,
    },
    Operation {
        agent: "microsoft-365",
        command: "sharepoint.list.list",
        integration: "microsoft-365",
        provider: Provider::Microsoft,
        route: Route {
            origin: MICROSOFT_GRAPH_ORIGIN,
            segments: &[
                Seg::Lit("v1.0"),
                Seg::Lit("sites"),
                Seg::Input("site-id"),
                Seg::Lit("lists"),
            ],
            alternate: None,
        },
        // `$top` is not documented for these SharePoint collections: the page
        // is bounded locally.
        fixed_query: &[("$select", SP_LISTS_SELECT)],
        inputs: &[
            InputMap::path_kind("site-id", IdKind::SharePointSiteId),
            InputMap::local_limit("page-size", 100, 200),
        ],
        limit_input: Some("page-size"),
        any_of_scopes: MICROSOFT_SITES_SCOPES,
        request_scopes: &["User.Read", "Sites.Read.All"],
        access: "SharePoint read access",
        admin_consent: false,
        projection: Projection::SharePointLists,
        canonical_v2: true,
    },
    Operation {
        agent: "microsoft-365",
        command: "sharepoint.list.read",
        integration: "microsoft-365",
        provider: Provider::Microsoft,
        route: Route {
            origin: MICROSOFT_GRAPH_ORIGIN,
            segments: &[
                Seg::Lit("v1.0"),
                Seg::Lit("sites"),
                Seg::Input("site-id"),
                Seg::Lit("lists"),
                Seg::Input("list-id"),
                Seg::Lit("items"),
            ],
            alternate: None,
        },
        fixed_query: &[("$expand", "fields")],
        inputs: &[
            InputMap::path_kind("site-id", IdKind::SharePointSiteId),
            InputMap::path_kind("list-id", IdKind::Guid),
            InputMap::local_limit("page-size", 100, 200),
        ],
        limit_input: Some("page-size"),
        any_of_scopes: MICROSOFT_SITES_SCOPES,
        request_scopes: &["User.Read", "Sites.Read.All"],
        access: "SharePoint read access",
        admin_consent: false,
        projection: Projection::SharePointItems,
        canonical_v2: true,
    },
];

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
                    let mut entry = json!({
                        "source": input.source,
                        "location": input.location.as_str(),
                        "name": input.name,
                        "scalar": input.scalar.as_str(),
                        "required": input.required,
                        "default": input.default,
                        "minimum": input.minimum,
                        "maximum": input.maximum,
                        "maxBytes": input.max_bytes,
                    });
                    // Only path inputs carry a grammar, so a query-only
                    // operation's entries (and its digest) are unchanged.
                    if let (Some(kind), Json::Object(map)) = (input.id_kind, &mut entry) {
                        map.insert("pattern".into(), Json::String(kind.as_str().into()));
                    }
                    if let (Some(format), Json::Object(map)) = (input.search, &mut entry) {
                        map.insert("format".into(), Json::String(format.as_str().into()));
                    }
                    entry
                })
                .collect(),
        )
    }

    fn route_json(segments: &[Seg]) -> Json {
        Json::Array(segments.iter().map(Seg::as_json).collect())
    }

    /// The canonical representation the operation digest is taken over.
    fn canonical(&self) -> Json {
        match self.provider {
            // Byte for byte the representation FloLess reviewed for #618; a
            // test pins its string and digest. Any change to this row changes
            // the digest, as it must.
            Provider::Google => json!({
                "schema": OPERATION_SCHEMA,
                "agent": self.agent,
                "command": self.command,
                "integration": self.integration,
                "transport": "rest",
                "effect": "read",
                "method": "GET",
                "url": self.single_url(),
                "fields": self.fixed_query.iter().find(|(k, _)| *k == "fields").map(|(_, v)| *v),
                "identity": self.provider.identity_url(),
                "scopes": self.any_of_scopes,
                "inputs": self.inputs_json(),
            }),
            Provider::Microsoft => self.microsoft_canonical(),
        }
    }

    fn microsoft_canonical(&self) -> Json {
        let mut canonical = json!({
            "schema": OPERATION_SCHEMA,
            "agent": self.agent,
            "command": self.command,
            "integration": self.integration,
            "provider": self.provider.name(),
            "transport": "rest",
            "effect": "read",
            "method": "GET",
            "origin": self.route.origin,
            "routes": {
                "default": Self::route_json(self.route.segments),
                "alternate": self.route.alternate.as_ref().map(|alt| json!({
                    "input": alt.input,
                    "literal": alt.literal,
                    "segments": Self::route_json(alt.segments),
                })),
            },
            "query": self.fixed_query.iter().map(|(k, v)| json!([k, v])).collect::<Vec<_>>(),
            "projection": self.projection.id(),
            "identity": {
                "url": self.provider.identity_url(),
                "query": self.provider.identity_query().iter().map(|(k, v)| json!([k, v])).collect::<Vec<_>>(),
            },
            "identityScopes": self.provider.identity_scopes(),
            "scopes": self.any_of_scopes,
            "requestScopes": self.request_scopes,
            "adminConsent": self.admin_consent,
            "inputs": self.inputs_json(),
        });
        if self.canonical_v2
            && let Json::Object(map) = &mut canonical
        {
            map.insert("canonical".into(), Json::String("v2".into()));
            map.insert("limitInput".into(), json!(self.limit_input));
        }
        canonical
    }

    /// The URL of an operation with one fixed route and no path input (the
    /// Google `list-files` digest names it).
    fn single_url(&self) -> String {
        let mut url = self.route.origin.to_string();
        for seg in self.route.segments {
            if let Seg::Lit(text) = seg {
                url.push('/');
                url.push_str(text);
            }
        }
        url
    }

    pub(crate) fn sha256(&self) -> String {
        sha256_hex(canonical_string(&self.canonical()).as_bytes())
    }

    /// The capability's `permissions`: what to request and how they combine.
    fn permissions_json(&self) -> Json {
        json!({
            "identityScopes": self.provider.identity_scopes(),
            "anyOfScopes": self.any_of_scopes,
            "requestScopes": self.request_scopes,
            "adminConsent": self.admin_consent,
        })
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
    // Grouped from the bytes, never by slicing the hex `str` at fixed offsets.
    let group = |from: usize, to: usize| -> String {
        bytes
            .iter()
            .take(to)
            .skip(from)
            .map(|b| format!("{b:02x}"))
            .collect()
    };
    format!(
        "{}-{}-{}-{}-{}",
        group(0, 4),
        group(4, 6),
        group(6, 8),
        group(8, 10),
        group(10, 16)
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

/// Where a slot's refresh token may be sent.
#[derive(Debug, Clone)]
pub(crate) enum TokenPin {
    /// Exactly this token endpoint (Google's; a fixture in tests).
    Exact(String),
    /// The bundled or tenant-rewritten Microsoft endpoint,
    /// `https://login.microsoftonline.com/<tenant>/oauth2/v2.0/token` — never a
    /// profile's `token_url` override, and a tenant only of `[A-Za-z0-9.-]`.
    MicrosoftLogin,
}

impl TokenPin {
    fn allows(&self, config: &crate::auth::config::IntegrationConfig) -> bool {
        match self {
            TokenPin::Exact(url) => config.token_url() == url,
            TokenPin::MicrosoftLogin => {
                config.token_endpoint_is_own() && is_microsoft_token_url(config.token_url())
            }
        }
    }
}

/// `https://login.microsoftonline.com/<tenant>/oauth2/v2.0/token`, where the
/// tenant is a GUID, a domain, or `common` / `organizations` / `consumers`.
pub(crate) fn is_microsoft_token_url(url: &str) -> bool {
    let Some(tenant) = url
        .strip_prefix(MICROSOFT_LOGIN_ORIGIN)
        .and_then(|rest| rest.strip_prefix('/'))
        .and_then(|rest| rest.strip_suffix("/oauth2/v2.0/token"))
    else {
        return false;
    };
    tenant.len() <= 256
        && tenant
            .bytes()
            .next()
            .is_some_and(|b| b.is_ascii_alphanumeric())
        && !tenant.contains("..")
        && tenant
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-'))
}

/// Where the code-owned operation's requests go. Production uses only the
/// constants above; tests point it at local fixtures (and must widen the
/// allowlist to do so, which is exactly what the origin-pin test does not do).
#[derive(Debug, Clone)]
pub(crate) struct Endpoints {
    pub(crate) token: TokenPin,
    pub(crate) identity_url: String,
    /// `None` = the operation's own origin. The path is always the operation's.
    pub(crate) data_origin: Option<String>,
    pub(crate) allowed_origins: Vec<String>,
}

impl Endpoints {
    pub(crate) fn production(op: &Operation) -> Self {
        let allowed_origins = crate::auth::config::for_integration(op.integration)
            .map(|config| {
                config
                    .call_origins()
                    .iter()
                    .map(|o| o.to_string())
                    .collect()
            })
            .unwrap_or_default();
        Self {
            token: match op.provider {
                Provider::Google => TokenPin::Exact(GOOGLE_TOKEN_URL.to_string()),
                Provider::Microsoft => TokenPin::MicrosoftLogin,
            },
            identity_url: op.provider.identity_url().to_string(),
            data_origin: None,
            allowed_origins,
        }
    }

    fn data_origin<'a>(&'a self, op: &'a Operation) -> &'a str {
        self.data_origin.as_deref().unwrap_or(op.route.origin)
    }

    /// Both request origins must be allowed — checked before any credential is
    /// read.
    fn check_origins(&self, op: &Operation) -> Result<(), CallFailure> {
        for url in [self.identity_url.as_str(), self.data_origin(op)] {
            let allowed = crate::manifest::probe::origin_of(url)
                .is_some_and(|origin| self.allowed_origins.contains(&origin));
            if !allowed {
                return Err(CallFailure::new("E_CALL_ORIGIN_NOT_ALLOWED"));
            }
        }
        Ok(())
    }

    /// The data request's URL: the allowed origin, then the wire's segments
    /// pushed one by one (each percent-encoded), never a templated string. The
    /// result must have exactly those segments.
    fn data_url(&self, op: &Operation, wire: &Wire) -> Result<String, CallFailure> {
        let shape = || CallFailure::reason("E_CALL_INPUT_INVALID", "path");
        let mut url = url::Url::parse(self.data_origin(op)).map_err(|_| shape())?;
        url.path_segments_mut()
            .map_err(|_| shape())?
            .clear()
            .extend(wire.segments.iter());
        let built = url.path_segments().map(Iterator::count).unwrap_or(0);
        if built != wire.segments.len() || url.query().is_some() || url.fragment().is_some() {
            return Err(shape());
        }
        Ok(url.to_string())
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

/// A validated request: the path segments and the query, in a fixed order.
#[derive(Debug, Clone)]
struct Wire {
    segments: Vec<String>,
    query: Vec<(String, String)>,
    /// The page size asked for, which also bounds the projection.
    limit: usize,
}

/// Validate caller inputs against the operation's mapping (every key mapped,
/// required ones present, right type, in bounds, path ids in their grammar) and
/// return the wire request.
fn wire(op: &Operation, inputs: &Map<String, Json>) -> Result<Wire, CallFailure> {
    for key in inputs.keys() {
        if !op.inputs.iter().any(|m| m.source == key) {
            return Err(CallFailure::new("E_CALL_INPUT_INVALID")
                .with("reason", Json::String("unknown-input".into())));
        }
    }
    let mut query = Vec::new();
    let mut path: Vec<(&'static str, String)> = Vec::new();
    let mut search: Vec<String> = Vec::new();
    let mut limit = None;
    for map in op.inputs {
        let value = inputs.get(map.source);
        let text = match (map.scalar, value) {
            (_, None) if map.required => return Err(CallFailure::input(map.source)),
            (_, None) => map.default.map(|d| d.to_string()),
            (Scalar::String, Some(Json::String(s))) => {
                if s.contains('\0') || map.max_bytes.is_some_and(|max| s.len() > max) {
                    return Err(CallFailure::input(map.source));
                }
                if map.id_kind.is_some_and(|kind| !kind.accepts(s)) {
                    return Err(CallFailure::input(map.source));
                }
                if let Some(format) = map.search {
                    let Some(terms) = format.terms(s) else {
                        return Err(CallFailure::input(map.source));
                    };
                    search.extend(terms);
                    continue;
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
        if Some(map.source) == op.limit_input {
            limit = text.as_deref().and_then(|t| t.parse::<usize>().ok());
        }
        if let Some(text) = text {
            match map.location {
                Location::Query => query.push((map.name.to_string(), text)),
                Location::Path => path.push((map.source, text)),
                // Search terms are collected above; a local input is never sent.
                Location::Search | Location::Local => {}
            }
        }
    }
    if op.inputs.iter().any(|m| m.location == Location::Search) {
        // At least one typed term; the value is quoted, as Graph requires.
        if search.is_empty() {
            let first = op
                .inputs
                .iter()
                .find(|m| m.location == Location::Search)
                .map(|m| m.source)
                .unwrap_or("query");
            return Err(CallFailure::input(first));
        }
        query.push(("$search".to_string(), format!("\"{}\"", search.join(" "))));
    }
    for (name, value) in op.fixed_query {
        query.push(((*name).to_string(), (*value).to_string()));
    }
    let path_value = |name: &str| path.iter().find(|(n, _)| *n == name).map(|(_, v)| v);
    let route = match &op.route.alternate {
        Some(alt) if path_value(alt.input).map(String::as_str) == Some(alt.literal) => alt.segments,
        _ => op.route.segments,
    };
    let mut segments = Vec::with_capacity(route.len());
    for seg in route {
        match seg {
            Seg::Lit(text) => segments.push((*text).to_string()),
            Seg::Input(name) => {
                let value = path_value(name)
                    .ok_or_else(|| CallFailure::reason("E_CALL_INPUT_INVALID", "path"))?;
                segments.push(value.clone());
            }
        }
    }
    let limit = limit.unwrap_or(100);
    Ok(Wire {
        segments,
        query,
        limit,
    })
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
    fn holds(&self, provider: Provider, wanted: &str) -> bool {
        crate::auth::keychain::normalized_scopes(&self.scope)
            .iter()
            .any(|granted| provider.scope_is(granted, wanted))
    }

    /// Every scope the identity read needs (none for Google).
    fn has_identity_scopes(&self, op: &Operation) -> bool {
        op.provider
            .identity_scopes()
            .iter()
            .all(|wanted| self.holds(op.provider, wanted))
    }

    /// At least one of the operation's own scopes.
    fn has_operation_scope(&self, op: &Operation) -> bool {
        op.any_of_scopes
            .iter()
            .any(|wanted| self.holds(op.provider, wanted))
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
    if !endpoints.token.allows(&config) {
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

/// The account a token belongs to, as the provider's identity read says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Principal {
    /// Google's `sub`, or Graph `/me`'s `id` — exact and opaque, never rewritten.
    pub(crate) sub: String,
    /// Presentation only: Google's email when it is verified, or Microsoft's
    /// `userPrincipalName`. Never part of the binding.
    pub(crate) label: Option<String>,
}

fn clean(text: &str, max_chars: usize) -> Option<String> {
    let clean: String = text.chars().filter(|c| !c.is_control()).collect();
    let clean = clean.trim();
    (!clean.is_empty() && clean.chars().count() <= max_chars).then(|| clean.to_string())
}

fn read_identity(
    endpoints: &Endpoints,
    provider: Provider,
    token: &SlotToken,
    timeout: Duration,
) -> Result<Principal, CallFailure> {
    let query: Vec<(String, String)> = provider
        .identity_query()
        .iter()
        .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
        .collect();
    let bytes = get_bounded(
        &endpoints.identity_url,
        &query,
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
    let subject_field = match provider {
        Provider::Google => "sub",
        Provider::Microsoft => "id",
    };
    let sub = body
        .get(subject_field)
        .and_then(Json::as_str)
        .filter(|s| !s.is_empty() && s.len() <= 255 && !s.chars().any(char::is_control))
        .ok_or_else(|| CallFailure::reason("E_CALL_IDENTITY_FAILED", "no-subject"))?
        .to_string();
    let label = match provider {
        Provider::Google if body.get("email_verified").and_then(Json::as_bool) == Some(true) => {
            body.get("email")
                .and_then(Json::as_str)
                .and_then(|e| clean(e, 256))
        }
        Provider::Google => None,
        Provider::Microsoft => body
            .get("userPrincipalName")
            .and_then(Json::as_str)
            .and_then(|e| clean(e, 256)),
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

/// The data read and its projection. Blocking.
fn read_data(
    endpoints: &Endpoints,
    op: &Operation,
    token: &SlotToken,
    wire: &Wire,
    timeout: Duration,
    cap: usize,
) -> Result<Json, CallFailure> {
    let url = endpoints.data_url(op, wire)?;
    let bytes =
        get_bounded(&url, &wire.query, &token.access_token, timeout, cap).map_err(|failure| {
            match failure {
                HttpFailure::Status(status) => {
                    CallFailure::new("E_CALL_FAILED").with("status", json!(status))
                }
                HttpFailure::Timeout => CallFailure::new("E_CALL_TIMEOUT"),
                HttpFailure::TooLarge => CallFailure::new("E_CALL_OUTPUT_TOO_LARGE"),
                HttpFailure::Transport => CallFailure::reason("E_CALL_FAILED", "transport"),
            }
        })?;
    let body: Json = serde_json::from_slice(&bytes)
        .map_err(|_| CallFailure::reason("E_CALL_FAILED", "response-shape"))?;
    op.projection
        .apply(&body, wire.limit)
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

/// An id a caller will send back as a lookup key: kept byte for byte when it is
/// non-empty, at most 256 bytes and free of control characters — never cleaned
/// into a different key.
fn exact_id(value: Option<&Json>) -> Option<String> {
    value
        .and_then(Json::as_str)
        .filter(|s| !s.is_empty() && s.len() <= 256 && !s.chars().any(char::is_control))
        .map(str::to_string)
}

/// Project a Graph `driveItem` collection (`value`) onto the allowlisted,
/// bounded shape. `None` when it is not an object with a `value` array.
///
/// Never claims a complete listing it did not return: `more-available` is set
/// by `@odata.nextLink` **or** by entries cut at `limit`, and `omitted` counts
/// entries dropped because an id was malformed.
pub(crate) fn project_children(body: &Json, limit: usize) -> Option<Json> {
    let entries = body.get("value")?.as_array()?;
    let mut out = Vec::new();
    let mut omitted = 0usize;
    let mut truncated = false;
    for item in entries {
        if out.len() >= limit {
            truncated = true;
            break;
        }
        let Some(id) = exact_id(item.get("id")) else {
            omitted += 1;
            continue;
        };
        let drive = item.get("parentReference").and_then(|p| p.get("driveId"));
        let drive_id = match drive {
            None | Some(Json::Null) => None,
            Some(value) => match exact_id(Some(value)) {
                Some(drive_id) => Some(drive_id),
                None => {
                    omitted += 1;
                    continue;
                }
            },
        };
        let mut entry = Map::new();
        entry.insert("id".into(), Json::String(id));
        if let Some(drive_id) = drive_id {
            entry.insert("drive-id".into(), Json::String(drive_id));
        }
        if let Some(name) = item
            .get("name")
            .and_then(Json::as_str)
            .and_then(|s| clean(s, 1024))
        {
            entry.insert("name".into(), Json::String(name));
        }
        let is_folder = item.get("folder").is_some_and(Json::is_object);
        entry.insert("is-folder".into(), Json::Bool(is_folder));
        if let Some(size) = item
            .get("size")
            .and_then(Json::as_u64)
            .filter(|n| *n <= (1u64 << 53))
        {
            entry.insert("size".into(), json!(size));
        }
        if let Some(mime) = item
            .get("file")
            .and_then(|f| f.get("mimeType"))
            .and_then(Json::as_str)
            .and_then(|s| clean(s, 256))
        {
            entry.insert("mime-type".into(), Json::String(mime));
        }
        if let Some(time) = item
            .get("lastModifiedDateTime")
            .and_then(Json::as_str)
            .and_then(|s| clean(s, 64))
        {
            entry.insert("modified-time".into(), Json::String(time));
        }
        out.push(Json::Object(entry));
    }
    let next = body
        .get("@odata.nextLink")
        .and_then(Json::as_str)
        .is_some_and(|t| !t.is_empty());
    Some(json!({
        "items": out,
        "more-available": next || truncated,
        "omitted": omitted,
    }))
}

/// Display text, never dropped for its length: control characters removed,
/// trimmed and cut to `max` chars. `None` when nothing is left; the flag says
/// it was cut.
fn capped(text: &str, max: usize) -> Option<(String, bool)> {
    let clean: String = text.chars().filter(|c| !c.is_control()).collect();
    let clean = clean.trim();
    if clean.is_empty() {
        return None;
    }
    if clean.chars().count() <= max {
        return Some((clean.to_string(), false));
    }
    Some((clean.chars().take(max).collect(), true))
}

/// One projected entry: its fields, and the names of the text fields that
/// were cut (listed as `truncated`, so nothing is shortened silently).
struct Entry {
    map: Map<String, Json>,
    truncated: Vec<&'static str>,
}

impl Entry {
    fn new(id: String) -> Self {
        let mut map = Map::new();
        map.insert("id".into(), Json::String(id));
        Self {
            map,
            truncated: Vec::new(),
        }
    }

    fn text(&mut self, key: &'static str, value: Option<&Json>, max: usize) {
        if let Some((text, cut)) = value.and_then(Json::as_str).and_then(|s| capped(s, max)) {
            self.map.insert(key.into(), Json::String(text));
            if cut {
                self.truncated.push(key);
            }
        }
    }

    fn put(&mut self, key: &'static str, value: Json) {
        self.map.insert(key.into(), value);
    }

    fn finish(mut self) -> Json {
        if !self.truncated.is_empty() {
            self.map.insert("truncated".into(), json!(self.truncated));
        }
        Json::Object(self.map)
    }
}

/// An id that, when present, must be exact. `Ok(None)` when absent or null,
/// `Err(())` when present but malformed (the caller drops the entry).
fn optional_id(value: Option<&Json>) -> Result<Option<String>, ()> {
    match value {
        None | Some(Json::Null) => Ok(None),
        Some(value) => exact_id(Some(value)).map(Some).ok_or(()),
    }
}

/// Project a Graph collection (`value`) onto `key: [...]`, bounded by
/// `limit`. `None` when it is not an object with a `value` array.
///
/// `more-available` means exactly: Graph sent `@odata.nextLink` (never
/// followed) or AWARE cut the page. `omitted` counts entries dropped because
/// an id was malformed.
fn project_collection(
    body: &Json,
    limit: usize,
    key: &str,
    entry: fn(&Json) -> Option<Json>,
) -> Option<Json> {
    let entries = body.get("value")?.as_array()?;
    let mut out = Vec::new();
    let mut omitted = 0usize;
    let mut truncated = false;
    for item in entries {
        if out.len() >= limit {
            truncated = true;
            break;
        }
        match entry(item) {
            Some(projected) => out.push(projected),
            None => omitted += 1,
        }
    }
    let next = body
        .get("@odata.nextLink")
        .and_then(Json::as_str)
        .is_some_and(|t| !t.is_empty());
    let mut result = Map::new();
    result.insert(key.into(), Json::Array(out));
    result.insert("more-available".into(), Json::Bool(next || truncated));
    result.insert("omitted".into(), json!(omitted));
    Some(Json::Object(result))
}

/// One Graph `driveItem` (`get-drive-item`). `None` when it is not an object
/// with an exact id. A download URL is never projected, even if sent.
pub(crate) fn project_drive_item(body: &Json) -> Option<Json> {
    let id = exact_id(body.get("id"))?;
    let drive_id = optional_id(body.get("parentReference").and_then(|p| p.get("driveId"))).ok()?;
    let mut entry = Entry::new(id);
    if let Some(drive_id) = drive_id {
        entry.put("drive-id", Json::String(drive_id));
    }
    entry.text("name", body.get("name"), 1024);
    entry.put(
        "is-folder",
        Json::Bool(body.get("folder").is_some_and(Json::is_object)),
    );
    if let Some(size) = body
        .get("size")
        .and_then(Json::as_u64)
        .filter(|n| *n <= (1u64 << 53))
    {
        entry.put("size", json!(size));
    }
    entry.text(
        "mime-type",
        body.get("file").and_then(|f| f.get("mimeType")),
        256,
    );
    entry.text("modified-time", body.get("lastModifiedDateTime"), 64);
    entry.text("web-url", body.get("webUrl"), 2048);
    Some(entry.finish())
}

/// One message of a mail search: headers and the preview, never the body.
fn message_entry(item: &Json) -> Option<Json> {
    let mut entry = Entry::new(exact_id(item.get("id"))?);
    entry.text("subject", item.get("subject"), 1024);
    if let Some(address) = item.get("from").and_then(|f| f.get("emailAddress")) {
        let mut from = Map::new();
        let mut cut = false;
        for (key, source, max) in [("address", "address", 320), ("name", "name", 256)] {
            if let Some((text, was_cut)) = address
                .get(source)
                .and_then(Json::as_str)
                .and_then(|s| capped(s, max))
            {
                from.insert(key.into(), Json::String(text));
                cut |= was_cut;
            }
        }
        if !from.is_empty() {
            entry.put("from", Json::Object(from));
        }
        if cut {
            entry.truncated.push("from");
        }
    }
    entry.text("received-at", item.get("receivedDateTime"), 64);
    entry.text("preview", item.get("bodyPreview"), 512);
    entry.put(
        "has-attachments",
        Json::Bool(item.get("hasAttachments").and_then(Json::as_bool) == Some(true)),
    );
    entry.text("web-url", item.get("webLink"), 2048);
    Some(entry.finish())
}

/// One Planner plan, with its typed container (never the deprecated `owner`).
fn plan_entry(item: &Json) -> Option<Json> {
    let mut entry = Entry::new(exact_id(item.get("id"))?);
    entry.text("title", item.get("title"), 1024);
    if let Some(container) = item.get("container").filter(|c| c.is_object()) {
        let id = optional_id(container.get("containerId")).ok()?;
        let mut out = Map::new();
        if let Some(id) = id {
            out.insert("id".into(), Json::String(id));
        }
        if let Some((kind, _)) = container
            .get("type")
            .and_then(Json::as_str)
            .and_then(|s| capped(s, 64))
        {
            out.insert("type".into(), Json::String(kind));
        }
        if !out.is_empty() {
            entry.put("container", Json::Object(out));
        }
    }
    entry.text("created-at", item.get("createdDateTime"), 64);
    Some(entry.finish())
}

/// Assignees kept per task; the rest are counted, never silently dropped.
const MAX_ASSIGNEES: usize = 50;

/// One Planner task. Assignees are the user ids keying `assignments`.
fn task_entry(item: &Json) -> Option<Json> {
    let mut entry = Entry::new(exact_id(item.get("id"))?);
    entry.text("title", item.get("title"), 1024);
    if let Some(bucket) = optional_id(item.get("bucketId")).ok()? {
        entry.put("bucket-id", Json::String(bucket));
    }
    if let Some(percent) = item
        .get("percentComplete")
        .and_then(Json::as_u64)
        .filter(|n| *n <= 100)
    {
        entry.put("percent-complete", json!(percent));
    }
    entry.text("due-date", item.get("dueDateTime"), 64);
    entry.text("completed-at", item.get("completedDateTime"), 64);
    let mut assigned = Vec::new();
    let mut assigned_omitted = 0usize;
    if let Some(assignments) = item.get("assignments").and_then(Json::as_object) {
        for key in assignments.keys() {
            match exact_id(Some(&Json::String(key.clone()))) {
                Some(user) if assigned.len() < MAX_ASSIGNEES => assigned.push(Json::String(user)),
                _ => assigned_omitted += 1,
            }
        }
    }
    entry.put("assigned-to", Json::Array(assigned));
    entry.put("assigned-to-omitted", json!(assigned_omitted));
    Some(entry.finish())
}

/// One SharePoint list of a site.
fn list_entry(item: &Json) -> Option<Json> {
    let mut entry = Entry::new(exact_id(item.get("id"))?);
    entry.text("name", item.get("displayName"), 1024);
    entry.text("web-url", item.get("webUrl"), 2048);
    entry.text(
        "template",
        item.get("list").and_then(|l| l.get("template")),
        64,
    );
    entry.text("modified-time", item.get("lastModifiedDateTime"), 64);
    Some(entry.finish())
}

/// Columns kept per list item; values in an array; chars of a text value.
const MAX_FIELDS: usize = 200;
const MAX_FIELD_ARRAY: usize = 50;
const MAX_FIELD_TEXT: usize = 2048;

/// A field value AWARE passes on: a scalar (text cut to `MAX_FIELD_TEXT`, the
/// flag says so). `None` for anything else.
fn field_scalar(value: &Json) -> Option<(Json, bool)> {
    match value {
        Json::String(text) => {
            let clean: String = text.chars().filter(|c| !c.is_control()).collect();
            if clean.chars().count() <= MAX_FIELD_TEXT {
                Some((Json::String(clean), false))
            } else {
                Some((
                    Json::String(clean.chars().take(MAX_FIELD_TEXT).collect()),
                    true,
                ))
            }
        }
        Json::Number(_) | Json::Bool(_) | Json::Null => Some((value.clone(), false)),
        _ => None,
    }
}

/// One SharePoint list item: its id and its scalar column values, keyed by
/// SharePoint's internal column names. OData annotations (`@…` keys) are not
/// columns. Every other value AWARE does not pass on is counted in
/// `skipped-fields`; every cut text value is named in `truncated-fields`.
fn list_item_entry(item: &Json) -> Option<Json> {
    let mut entry = Entry::new(exact_id(item.get("id"))?);
    let mut fields = Map::new();
    let mut skipped = 0usize;
    let mut cut: Vec<String> = Vec::new();
    let source = item.get("fields").and_then(Json::as_object);
    if let Some(source) = source {
        for (key, value) in source {
            if key.starts_with('@') {
                continue;
            }
            if key.is_empty() || key.len() > 128 || key.chars().any(char::is_control) {
                skipped += 1;
                continue;
            }
            if fields.len() >= MAX_FIELDS {
                skipped += 1;
                continue;
            }
            let kept = match value {
                Json::Array(values) if values.len() <= MAX_FIELD_ARRAY => {
                    let scalars: Option<Vec<(Json, bool)>> =
                        values.iter().map(field_scalar).collect();
                    scalars.map(|scalars| {
                        let was_cut = scalars.iter().any(|(_, c)| *c);
                        (
                            Json::Array(scalars.into_iter().map(|(v, _)| v).collect()),
                            was_cut,
                        )
                    })
                }
                Json::Array(_) => None,
                other => field_scalar(other),
            };
            match kept {
                Some((value, was_cut)) => {
                    if was_cut {
                        cut.push(key.clone());
                    }
                    fields.insert(key.clone(), value);
                }
                None => skipped += 1,
            }
        }
    }
    entry.put("fields", Json::Object(fields));
    entry.put("skipped-fields", json!(skipped));
    if !cut.is_empty() {
        entry.put("truncated-fields", json!(cut));
    }
    if source.is_none() {
        entry.put("fields-missing", Json::Bool(true));
    }
    Some(entry.finish())
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
    capabilities_with(home, op, alias, timeout, &Endpoints::production(op)).await
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
        blocking(move || {
            remaining(timeout, started)?;
            let token = load_slot(
                &home,
                op.integration,
                alias_owned.as_deref(),
                &endpoints,
                None,
            )?;
            // The identity read's own scope first: without it `/me` would fail
            // and the person would be told the account is unverified instead of
            // what to reconnect with. Nothing is sent.
            if !token.has_identity_scopes(op) {
                return Err(CallFailure::reason("E_CALL_SCOPE_MISSING", "identity"));
            }
            let left = remaining(timeout, started)?;
            // Leave the outer deadline room to report: the identity read gets a
            // little less than what remains.
            let principal = read_identity(
                &endpoints,
                op.provider,
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
            Some(token.has_operation_scope(op)),
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
                ("E_CALL_SCOPE_MISSING", _) => "scope-missing",
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
        "permissions": op.permissions_json(),
        "credential": credential_json,
        "available": unavailable_code.is_none(),
        "unavailableReason": unavailable_code.map(|code| unavailable_reason(op, code, alias)),
        "unavailableCode": unavailable_code,
    }))
}

/// AWARE's fixed sentence for each `unavailableCode`.
pub(crate) fn unavailable_reason(op: &Operation, code: &str, alias: Option<&str>) -> String {
    let provider = op.provider.name();
    match code {
        "auth-invalid" => format!(
            "The installed agent does not use the {} sign-in for this command.",
            op.integration
        ),
        "command-planned" => format!(
            "The installed {} agent does not offer {} yet; update the agent.",
            op.agent, op.command
        ),
        "credential-missing" => format!("No {provider} account is connected in this slot."),
        "credential-expired" => format!(
            "The {provider} sign-in for this slot has expired and could not be renewed; reconnect it."
        ),
        "generation-unavailable" => {
            "AWARE could not record which sign-in this slot holds; reconnect it.".into()
        }
        "identity-unverified" => format!(
            "{provider} did not confirm which account this slot belongs to; check the connection and try again."
        ),
        "scope-missing" => {
            let alias_arg = alias.map(|a| format!(" --as={a}")).unwrap_or_default();
            format!(
                "This {provider} account was connected without {}. Reconnect it with `aware connect {}{alias_arg} --oauth --scopes {}`.",
                op.access,
                op.integration,
                op.request_scopes.join(","),
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
    call_with(home, request, &Endpoints::production(op)).await
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
    let wire = wire(op, &request.inputs)?;
    let digest = sha256_hex(canonical_string(&Json::Object(request.inputs.clone())).as_bytes());
    if digest != request.inputs_sha256 {
        return Err(CallFailure::field("inputsSha256"));
    }
    // 6. Origins, before any credential is read.
    endpoints.check_origins(op)?;

    // 7-9. Credential, scopes, identity, replacement boundary, data — blocking.
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
        if !token.has_identity_scopes(op) || !token.has_operation_scope(op) {
            return Err(CallFailure::new("E_CALL_SCOPE_MISSING"));
        }
        let principal = read_identity(
            &endpoints,
            op.provider,
            &token,
            remaining(timeout, started)?,
        )?;
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
        read_data(&endpoints, op, &token, &wire, left, cap)
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
/// manifest's own slot (a baked `<integration>.<alias>` honoured), fixed
/// bounds, no identity step (a workflow has no binding to check, so only the
/// operation's own scope is required), and the same replacement boundary
/// before the data request.
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
    let wire = wire(op, &inputs)?;
    let endpoints = endpoints
        .cloned()
        .unwrap_or_else(|| Endpoints::production(op));
    endpoints.check_origins(op)?;
    let home = home.to_path_buf();
    let work = blocking(move || {
        remaining(timeout, started)?;
        let token = load_slot(&home, op.integration, alias.as_deref(), &endpoints, None)?;
        if !token.has_operation_scope(op) {
            return Err(CallFailure::new("E_CALL_SCOPE_MISSING"));
        }
        let left = remaining(timeout, started)?;
        check_not_replaced(&home, op.integration, alias.as_deref(), &token.generation)?;
        read_data(&endpoints, op, &token, &wire, left, cap)
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
