//! `aware agent call-capabilities` and single-shot `aware agent call` (#618) —
//! reviewed, read-only, account-bound REST reads: Google Drive `list-files`,
//! Microsoft OneDrive `list-folder` (#667, #709 slice A) and the Microsoft
//! 365 reads of #709 slices B1 (`get-drive-item`, `outlook.mail.search`,
//! Planner plans and tasks, SharePoint lists and rows), C (Teams teams,
//! channels, channel messages, chats and chat messages) and B2 (Excel
//! worksheets and ranges, a file by its sharing link).
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
/// The tenant of every personal Microsoft account (Microsoft's ID token
/// reference: "the value is `9188040d-6c67-4c5b-b112-36a304b66dad`").
pub(crate) const MICROSOFT_CONSUMER_TENANT: &str = "9188040d-6c67-4c5b-b112-36a304b66dad";
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
/// Any one of these lists mail folders (`Mail.ReadBasic` is enough: a folder
/// carries no message content).
const MICROSOFT_MAIL_FOLDER_SCOPES: &[&str] = &["Mail.ReadBasic", "Mail.Read", "Mail.ReadWrite"];
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
/// Any one of these lists the teams a person belongs to.
const MICROSOFT_TEAMS_SCOPES: &[&str] = &[
    "Team.ReadBasic.All",
    "TeamSettings.Read.All",
    "TeamSettings.ReadWrite.All",
];
/// Any one of these lists a team's channels.
const MICROSOFT_CHANNELS_SCOPES: &[&str] = &[
    "Channel.ReadBasic.All",
    "ChannelSettings.Read.All",
    "ChannelSettings.ReadWrite.All",
];
/// Any one of these reads channel messages. Every one needs tenant admin
/// consent; AWARE says so and never asks for it on the person's behalf.
const MICROSOFT_CHANNEL_MESSAGES_SCOPES: &[&str] = &[
    "ChannelMessage.Read.All",
    "Group.Read.All",
    "Group.ReadWrite.All",
];
/// Any one of these lists the person's chats.
const MICROSOFT_CHATS_SCOPES: &[&str] = &["Chat.ReadBasic", "Chat.Read", "Chat.ReadWrite"];
/// Any one of these reads chat messages.
const MICROSOFT_CHAT_MESSAGES_SCOPES: &[&str] = &["Chat.Read", "Chat.ReadWrite"];

/// Excel workbook reads. Microsoft documents no read-only grant for them,
/// only `Files.ReadWrite`; AWARE still sends only a `GET`, never a body, a
/// `Workbook-Session-Id` or a `Prefer` header.
const MICROSOFT_WORKBOOK_SCOPES: &[&str] = &["Files.ReadWrite", "Files.ReadWrite.All"];

/// Opening a file by its sharing link (`/shares/{token}/driveItem`), with no
/// `Prefer` header, so a link is never redeemed into new access.
const MICROSOFT_SHARES_SCOPES: &[&str] = &[
    "Files.ReadWrite",
    "Files.ReadWrite.All",
    "Sites.ReadWrite.All",
];

const FILES_READ_WRITE: &str = "Files.ReadWrite";

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
    /// the projection where the endpoint documents no `$top`, or a `since`
    /// applied to the projection).
    Local,
    /// A `$filter` the code renders from a typed value
    /// (`<name> gt <timestamp>`), never caller-written OData.
    Filter,
    /// A delta cursor ([`DeltaCursor`]): with one, the request carries only
    /// its state token, never the first round's query.
    DeltaState,
    /// A page size sent as `Prefer: odata.maxpagesize=<n>` (on every request
    /// of a delta round, cursor requests included), never in the query.
    PreferPageSize,
}

impl Location {
    fn as_str(self) -> &'static str {
        match self {
            Location::Query => "query",
            Location::Path => "path",
            Location::Search => "search",
            Location::Local => "local",
            Location::Filter => "filter",
            Location::DeltaState => "delta-state",
            Location::PreferPageSize => "prefer-maxpagesize",
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
    /// A GUID, `8-4-4-4-12` hex (a SharePoint list id, a team id).
    Guid,
    /// A Teams thread id (a channel or a chat): `19:`, 1..200 of
    /// `[A-Za-z0-9_-]`, then one fixed suffix. `.` only inside the suffix, so
    /// a segment can never be `.` or `..`; no `/`, `%`, `?`, `#` or whitespace.
    TeamsThreadId,
    /// An Excel worksheet: its id (`{` GUID `}`) or its name — 1..31
    /// characters, none of `\ / ? * [ ] : '` or a control character, no
    /// leading or trailing space, never `.` or `..`. Sent as one path segment,
    /// percent-encoded by the URL builder.
    WorksheetRef,
    /// An A1 cell or rectangle (`B2`, `A1:D200`), at most `MAX_RANGE_CELLS`
    /// cells, normalised before it is rendered.
    ExcelA1Range,
    /// A SharePoint or OneDrive for Business sharing link, rendered as a Graph
    /// share token, never sent as given.
    SharingUrl,
    /// An Outlook folder or message id ([`is_outlook_id`]).
    OutlookId,
    /// An Outlook id or a well-known folder name (`msgfolderroot`, `inbox`, …),
    /// the name sent lower case.
    OutlookFolderRef,
}

/// The suffixes a Teams thread id may end with.
const TEAMS_THREAD_SUFFIXES: &[&str] = &[
    "@thread.tacv2",
    "@thread.v2",
    "@thread.skype",
    "@unq.gbl.spaces",
];

fn is_teams_thread_id(value: &str) -> bool {
    let Some(rest) = value.strip_prefix("19:") else {
        return false;
    };
    TEAMS_THREAD_SUFFIXES.iter().any(|suffix| {
        rest.strip_suffix(suffix).is_some_and(|body| {
            !body.is_empty()
                && body.len() <= 200
                && body
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-'))
        })
    })
}

/// `graph-utc-timestamp/1`: exactly `YYYY-MM-DDTHH:MM:SS`, an optional `.`
/// and 1..7 digits, then `Z` — a real UTC instant (no other offset, no
/// out-of-range field, no leap second).
fn utc_timestamp(value: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    let body = value.strip_suffix('Z')?;
    let (main, fraction) = match body.split_once('.') {
        Some((main, fraction)) => (main, Some(fraction)),
        None => (body, None),
    };
    let shape = main.len() == 19
        && main.bytes().enumerate().all(|(i, b)| match i {
            4 | 7 => b == b'-',
            10 => b == b'T',
            13 | 16 => b == b':',
            _ => b.is_ascii_digit(),
        });
    if !shape
        || fraction
            .is_some_and(|f| f.is_empty() || f.len() > 7 || !f.bytes().all(|b| b.is_ascii_digit()))
    {
        return None;
    }
    // chrono accepts second 60 (a leap second); a Graph instant never has one.
    let (_, seconds) = main.rsplit_once(':')?;
    if seconds.parse::<u32>().ok()? > 59 {
        return None;
    }
    let naive = chrono::NaiveDateTime::parse_from_str(body, "%Y-%m-%dT%H:%M:%S%.f").ok()?;
    Some(naive.and_utc())
}

/// The format id of a `since` input, part of the operation digest.
const UTC_TIMESTAMP_FORMAT: &str = "graph-utc-timestamp/1";

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

/// The most cells one `excel.range.read` may ask for.
const MAX_RANGE_CELLS: u64 = 5_000;

/// Excel's last column (`XFD`) and row.
const MAX_COLUMN: u32 = 16_384;
const MAX_ROW: u32 = 1_048_576;

/// A rectangle of cells, 1-based, first corner top-left.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RangeAsk {
    first_column: u32,
    first_row: u32,
    last_column: u32,
    last_row: u32,
}

impl RangeAsk {
    fn rows(self) -> u64 {
        u64::from(self.last_row - self.first_row) + 1
    }

    fn columns(self) -> u64 {
        u64::from(self.last_column - self.first_column) + 1
    }

    fn cells(self) -> u64 {
        self.rows() * self.columns()
    }

    /// The one spelling AWARE sends: upper case, top-left first, and a single
    /// cell without a second corner (`a1:a1` is `A1`).
    fn address(self) -> String {
        let first = format!("{}{}", column_name(self.first_column), self.first_row);
        if self.first_column == self.last_column && self.first_row == self.last_row {
            return first;
        }
        format!("{first}:{}{}", column_name(self.last_column), self.last_row)
    }
}

/// `1` → `A`, `28` → `AB`, `16384` → `XFD`.
fn column_name(mut column: u32) -> String {
    let mut letters = Vec::new();
    while column > 0 {
        let rem = (column - 1) % 26;
        letters.push(char::from(b'A' + u8::try_from(rem).unwrap_or(0)));
        column = (column - 1) / 26;
    }
    letters.iter().rev().collect()
}

/// One A1 cell: 1..3 letters (any case) then a row without a leading zero.
/// `$` before either part only when `absolute` (Graph writes it; a caller
/// may not).
fn a1_cell(text: &str, absolute: bool) -> Option<(u32, u32)> {
    let text = if absolute {
        text.strip_prefix('$').unwrap_or(text)
    } else {
        text
    };
    let letters = text.bytes().take_while(u8::is_ascii_alphabetic).count();
    if !(1..=3).contains(&letters) {
        return None;
    }
    let (column_text, rest) = text.split_at(letters);
    let row_text = if absolute {
        rest.strip_prefix('$').unwrap_or(rest)
    } else {
        rest
    };
    if row_text.is_empty()
        || row_text.len() > 7
        || row_text.starts_with('0')
        || !row_text.bytes().all(|b| b.is_ascii_digit())
    {
        return None;
    }
    let column = column_text.bytes().fold(0u32, |acc, b| {
        acc * 26 + u32::from(b.to_ascii_uppercase() - b'A') + 1
    });
    let row: u32 = row_text.parse().ok()?;
    (column <= MAX_COLUMN && row <= MAX_ROW).then_some((column, row))
}

/// An A1 cell or two-corner rectangle, corners in any order.
fn a1_rect(text: &str, absolute: bool) -> Option<RangeAsk> {
    let (a, b) = match text.split_once(':') {
        Some((a, b)) => (a1_cell(a, absolute)?, a1_cell(b, absolute)?),
        None => {
            let cell = a1_cell(text, absolute)?;
            (cell, cell)
        }
    };
    Some(RangeAsk {
        first_column: a.0.min(b.0),
        first_row: a.1.min(b.1),
        last_column: a.0.max(b.0),
        last_row: a.1.max(b.1),
    })
}

/// A caller's `range`: no `$`, no sheet, no whole row or column, at most
/// `MAX_RANGE_CELLS` cells.
fn a1_range(text: &str) -> Option<RangeAsk> {
    a1_rect(text, false).filter(|ask| ask.cells() <= MAX_RANGE_CELLS)
}

/// Characters Excel forbids in a sheet name, plus `'`, refused because the
/// escaping Graph applies to a quoted name segment is undocumented.
const WORKSHEET_FORBIDDEN: &[char] = &['\\', '/', '?', '*', '[', ']', ':', '\''];

fn is_worksheet_ref(value: &str) -> bool {
    if let Some(inner) = value.strip_prefix('{').and_then(|v| v.strip_suffix('}')) {
        return is_guid(inner);
    }
    let count = value.chars().count();
    (1..=31).contains(&count)
        && value != "."
        && value != ".."
        && !value.starts_with(' ')
        && !value.ends_with(' ')
        && !value
            .chars()
            .any(|c| c.is_control() || WORKSHEET_FORBIDDEN.contains(&c))
}

/// A work or school sharing link: `https://<tenant>.sharepoint.com/…` or
/// `https://<tenant>-my.sharepoint.com/…`, no userinfo, port or fragment, and
/// a path and query of printable ASCII only.
fn is_sharing_url(value: &str) -> bool {
    if value.len() > 2048 {
        return false;
    }
    let Some(rest) = value.strip_prefix("https://") else {
        return false;
    };
    let Some(split) = rest.find(['/', '?']) else {
        return false;
    };
    let (host, tail) = rest.split_at(split);
    let host = host.to_ascii_lowercase();
    let Some(label) = host.strip_suffix(".sharepoint.com") else {
        return false;
    };
    let label_ok = !label.is_empty()
        && label.len() <= 63
        && !label.starts_with('-')
        && !label.ends_with('-')
        && label
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-');
    label_ok
        && tail.starts_with('/')
        && tail.bytes().all(|b| {
            (0x21..=0x7e).contains(&b)
                && !matches!(
                    b,
                    b'"' | b'<' | b'>' | b'\\' | b'^' | b'`' | b'{' | b'|' | b'}' | b'#'
                )
        })
}

/// Outlook's well-known folder names, accepted (any case, sent lower case)
/// only where a folder may be named rather than identified (`parent-id`).
const OUTLOOK_WELL_KNOWN_FOLDERS: &[&str] = &[
    "inbox",
    "sentitems",
    "drafts",
    "deleteditems",
    "archive",
    "junkemail",
    "msgfolderroot",
    "outbox",
    "clutter",
    "conversationhistory",
    "scheduled",
    "searchfolders",
    "recoverableitemsdeletions",
];

/// An Outlook id (a folder, a message, a conversation, an attachment), kept
/// byte for byte on the way in and on the way out: 1..512 bytes of base64url
/// `[A-Za-z0-9_-]`, then at most two `=`. No `.`, so `.`/`..` cannot form; no
/// `/`, `%`, `+`, `:` or whitespace.
fn is_outlook_id(value: &str) -> bool {
    let body = value.trim_end_matches('=');
    !body.is_empty()
        && value.len() <= 512
        && value.len() - body.len() <= 2
        && body
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-'))
}

/// The well-known folder `value` names, in the spelling AWARE sends.
fn outlook_folder_name(value: &str) -> Option<&'static str> {
    OUTLOOK_WELL_KNOWN_FOLDERS
        .iter()
        .copied()
        .find(|name| name.eq_ignore_ascii_case(value))
}

/// An Outlook id from Graph, when it is exact (see [`is_outlook_id`]).
fn outlook_id(value: Option<&Json>) -> Option<String> {
    value
        .and_then(Json::as_str)
        .filter(|s| is_outlook_id(s))
        .map(str::to_string)
}

impl IdKind {
    fn as_str(self) -> &'static str {
        match self {
            IdKind::GraphOpaqueId => "graph-opaque-id",
            IdKind::SharePointSiteId => "sharepoint-site-id",
            IdKind::Guid => "guid",
            IdKind::TeamsThreadId => "teams-thread-id",
            IdKind::WorksheetRef => "worksheet-ref",
            IdKind::ExcelA1Range => "excel-a1-range/1",
            IdKind::SharingUrl => "sharing-url/1",
            IdKind::OutlookId => "outlook-id/1",
            IdKind::OutlookFolderRef => "outlook-folder-ref/1",
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
            IdKind::TeamsThreadId => is_teams_thread_id(value),
            IdKind::WorksheetRef => is_worksheet_ref(value),
            IdKind::ExcelA1Range => a1_range(value).is_some(),
            IdKind::SharingUrl => is_sharing_url(value),
            // A well-known name is base64url too; it identifies nothing here.
            IdKind::OutlookId => is_outlook_id(value) && outlook_folder_name(value).is_none(),
            IdKind::OutlookFolderRef => is_outlook_id(value),
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
    /// A `graph-utc-timestamp/1` value (`since`), filtered by Graph
    /// (`Location::Filter`) or by the projection (`Location::Local`).
    timestamp: bool,
    /// A timestamp's comparison in its `$filter` term; `None` is `gt` (a
    /// `since`), so every earlier row's canonical entry is unchanged.
    filter_op: Option<&'static str>,
    /// An `outlook-delta-cursor/1` value (`Location::DeltaState`).
    cursor: bool,
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
            timestamp: false,
            filter_op: None,
            cursor: false,
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
            timestamp: false,
            filter_op: None,
            cursor: false,
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
            timestamp: false,
            filter_op: None,
            cursor: false,
        }
    }

    /// A required path id of another grammar.
    const fn path_kind(source: &'static str, kind: IdKind) -> Self {
        let mut map = Self::path_id(source);
        map.id_kind = Some(kind);
        map
    }

    /// A required path input of another grammar and length.
    const fn path_long(source: &'static str, kind: IdKind, max_bytes: usize) -> Self {
        let mut map = Self::path_kind(source, kind);
        map.max_bytes = Some(max_bytes);
        map
    }

    /// A page bound applied by the code, never sent.
    const fn local_limit(source: &'static str, default: i64, max: i64) -> Self {
        let mut map = Self::query_limit(source, source, max);
        map.location = Location::Local;
        map.default = Some(default);
        map
    }

    /// A non-negative offset (`$skip`), default 0, always sent.
    const fn offset(source: &'static str, name: &'static str, max: i64) -> Self {
        let mut map = Self::query_limit(source, name, max);
        map.default = Some(0);
        map.minimum = Some(0);
        map
    }

    /// An optional delta cursor ([`DeltaCursor`]).
    const fn delta_cursor(source: &'static str) -> Self {
        let mut map = Self::query_string(source, source, MAX_CURSOR_BYTES);
        map.location = Location::DeltaState;
        map.cursor = true;
        map
    }

    /// A page size sent as `Prefer: odata.maxpagesize=<n>`.
    const fn prefer_page_size(source: &'static str, default: i64, max: i64) -> Self {
        let mut map = Self::query_limit(source, source, max);
        map.location = Location::PreferPageSize;
        map.default = Some(default);
        map
    }

    /// A `$top` page size with its own default.
    const fn top(source: &'static str, default: i64, max: i64) -> Self {
        let mut map = Self::query_limit(source, "$top", max);
        map.default = Some(default);
        map
    }

    /// A required `since`: rendered as `$filter=<name> gt <since>`
    /// (`Location::Filter`) or applied by the projection (`Location::Local`).
    const fn since(name: &'static str, location: Location) -> Self {
        let mut map = Self::query_string("since", name, 64);
        map.location = location;
        map.required = true;
        map.timestamp = true;
        map
    }

    /// A `graph-utc-timestamp/1` bound rendered as one term of the one
    /// `$filter` (`<name> <op> <value>`, terms joined by `and`): a window on a
    /// date, never caller-written OData.
    const fn time_bound(
        source: &'static str,
        name: &'static str,
        op: &'static str,
        required: bool,
    ) -> Self {
        let mut map = Self::query_string(source, name, 64);
        map.location = Location::Filter;
        map.required = required;
        map.timestamp = true;
        map.filter_op = Some(op);
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
    /// `range(address='<A1>')` from a normalised `ExcelA1Range` input; the
    /// address holds only `[A-Z0-9:]`, so the quote cannot be closed early.
    ExcelRange(&'static str),
    /// `u!` + unpadded base64url of a `SharingUrl` input: Graph's share token,
    /// computed in code.
    ShareToken(&'static str),
}

impl Seg {
    fn as_json(&self) -> Json {
        match self {
            Seg::Lit(text) => Json::String((*text).into()),
            Seg::Input(name) => Json::String(format!("{{{name}}}")),
            // Objects, so no earlier route's string form changes.
            Seg::ExcelRange(name) => json!({"render": "excel-range/1", "input": name}),
            Seg::ShareToken(name) => json!({"render": "share-token/1", "input": name}),
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
    TeamsTeams,
    TeamsChannels,
    TeamsChannelMessages,
    TeamsChats,
    TeamsChatMessages,
    ExcelWorksheets,
    ExcelRange,
    OutlookFolders,
    OutlookMessage,
    OutlookDelta,
    OutlookMessageIds,
}

/// What a projection may use besides the body: the page bound, a validated
/// `since`, the rectangle a range read asked for, and the `$skip` sent.
#[derive(Debug, Clone, Copy)]
struct ProjectionContext {
    limit: usize,
    skip: u64,
    since: Option<chrono::DateTime<chrono::Utc>>,
    range: Option<RangeAsk>,
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
            Projection::TeamsTeams => "teams/1",
            Projection::TeamsChannels => "channels/1",
            Projection::TeamsChannelMessages => "teams-channel-messages/1",
            Projection::TeamsChats => "chats/1",
            Projection::TeamsChatMessages => "teams-chat-messages/1",
            Projection::ExcelWorksheets => "excel-worksheets/1",
            Projection::ExcelRange => "excel-range/1",
            Projection::OutlookFolders => "outlook-folders/1",
            Projection::OutlookMessage => "outlook-message/1",
            Projection::OutlookDelta => "outlook-delta/1",
            Projection::OutlookMessageIds => "outlook-message-ids/1",
        }
    }

    fn apply(self, body: &Json, ctx: ProjectionContext) -> Option<Json> {
        let limit = ctx.limit;
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
            Projection::TeamsTeams => project_collection(body, limit, "teams", team_entry),
            Projection::TeamsChannels => project_collection(body, limit, "channels", channel_entry),
            Projection::TeamsChannelMessages => project_teams_messages(body, ctx, true),
            Projection::TeamsChats => project_collection(body, limit, "chats", chat_entry),
            Projection::TeamsChatMessages => project_teams_messages(body, ctx, false),
            Projection::ExcelWorksheets => {
                project_collection(body, limit, "worksheets", worksheet_entry)
            }
            Projection::ExcelRange => project_excel_range(body, ctx.range?),
            Projection::OutlookFolders => project_outlook_folders(body, ctx),
            Projection::OutlookMessage => project_outlook_message(body),
            Projection::OutlookMessageIds => {
                project_collection(body, usize::MAX, "messages", outlook_message_id_entry)
            }
            // Projected by `project_outlook_delta`, which needs the cursor
            // context and names its own failures (see `read_data`).
            Projection::OutlookDelta => None,
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
    /// Headers sent on the data request only (never on the identity read).
    fixed_headers: &'static [(&'static str, &'static str)],
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
    /// The most bytes read from the provider, when it differs from the
    /// caller's output budget (which still bounds the projected answer). Set
    /// only on rows that need it (#763); it enters the digest only there, and
    /// only those rows name which budget a too-large answer broke.
    read_cap: Option<usize>,
    /// The operation's cursor is bound to the verified account, so every entry
    /// point (call, workflow, invoke) verifies it before the data request.
    /// Set only on the delta read (#763); in the digest only there.
    binds_account: bool,
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

/// `$select` for a team's channels.
const CHANNELS_SELECT: &str = "id,displayName,description,membershipType,webUrl,createdDateTime";

/// `$select` for a workbook's worksheets.
const WORKSHEETS_SELECT: &str = "id,name,position,visibility";

/// `$select` for a range: raw values, their formatted text and their types —
/// never formulas or number formats.
const RANGE_SELECT: &str = "address,rowCount,columnCount,values,text,valueTypes";

/// `$select` for a folder's child folders.
const MAIL_FOLDERS_SELECT: &str =
    "id,displayName,parentFolderId,childFolderCount,totalItemCount,unreadItemCount";

/// `$select` for a folder's changes: headers and the preview, never a body
/// (a body can be hundreds of KB; inside a page it could make the page
/// unreadable and the cursor impossible to advance).
const MAIL_HEADERS_SELECT: &str = "id,internetMessageId,conversationId,subject,from,toRecipients,\
     ccRecipients,receivedDateTime,sentDateTime,lastModifiedDateTime,hasAttachments,bodyPreview,\
     webLink,isDraft";

/// `$select` for what a folder holds now: ids and the two dates a reader
/// pages and compares by, never a header or a body.
const MAIL_IDS_SELECT: &str = "id,receivedDateTime,lastModifiedDateTime,isDraft";

/// The format id of a delta cursor, part of the operation digest.
const CURSOR_FORMAT: &str = "outlook-delta-cursor/1";
/// What every delta cursor starts with.
const CURSOR_PREFIX: &str = "odc1.";
/// Largest cursor a caller may send.
const MAX_CURSOR_BYTES: usize = 16 * 1024;
/// Largest Graph state token AWARE carries.
const MAX_STATE_TOKEN_BYTES: usize = 8 * 1024;
/// Largest Graph error body AWARE reads to find its code.
const ERROR_BODY_CAP: usize = 16 * 1024;
/// Graph error codes that mean a delta state token is no longer usable.
const DEAD_CURSOR_CODES: &[&str] = &["syncStateNotFound", "syncStateInvalid", "resyncRequired"];

/// `$select` for one message: its headers, its folder and version, and both
/// bodies.
const GET_MESSAGE_SELECT: &str = "id,internetMessageId,conversationId,subject,from,toRecipients,\
     ccRecipients,receivedDateTime,sentDateTime,lastModifiedDateTime,hasAttachments,bodyPreview,\
     webLink,isDraft,parentFolderId,changeKey,body,uniqueBody";

/// Attachments by name, type and size, never `contentBytes`.
const GET_MESSAGE_EXPAND: &str = "attachments($select=id,name,contentType,size,isInline)";

/// Message ids that survive a move between folders, so a message read in a
/// folder's changes is the one `get-message` reads.
const OUTLOOK_IMMUTABLE_IDS: &[(&str, &str)] = &[("Prefer", "IdType=\"ImmutableId\"")];

/// The most bytes an Outlook message read takes from Graph; the caller's
/// output budget still bounds what it returns.
const OUTLOOK_READ_CAP: usize = 8 * 1024 * 1024;

/// The largest `$skip` a folder listing sends; a `next-skip` beyond it is not
/// offered (`limit-reached`).
const MAX_FOLDER_SKIP: i64 = 100_000;

/// Graph sends `systemEventMessage` (an evolvable enum member) only when asked.
const TEAMS_MESSAGE_HEADERS: &[(&str, &str)] = &[("Prefer", "include-unknown-enum-members")];

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
        fixed_headers: &[],
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
        read_cap: None,
        binds_account: false,
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
        fixed_headers: &[],
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
        read_cap: None,
        binds_account: false,
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
        fixed_headers: &[],
        inputs: &[InputMap::path_id("drive-id"), InputMap::path_id("item-id")],
        limit_input: None,
        any_of_scopes: MICROSOFT_FILES_SCOPES,
        request_scopes: &["User.Read", FILES_READ_ALL],
        access: "OneDrive read access",
        admin_consent: false,
        projection: Projection::GraphItem,
        canonical_v2: true,
        read_cap: None,
        binds_account: false,
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
        fixed_headers: &[],
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
        read_cap: None,
        binds_account: false,
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
        fixed_headers: &[],
        inputs: &[InputMap::local_limit("page-size", 100, 200)],
        limit_input: Some("page-size"),
        any_of_scopes: MICROSOFT_TASKS_SCOPES,
        request_scopes: &["User.Read", "Tasks.Read"],
        access: "Planner read access",
        admin_consent: false,
        projection: Projection::PlannerPlans,
        canonical_v2: true,
        read_cap: None,
        binds_account: false,
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
        fixed_headers: &[],
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
        read_cap: None,
        binds_account: false,
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
        fixed_headers: &[],
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
        read_cap: None,
        binds_account: false,
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
        fixed_headers: &[],
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
        read_cap: None,
        binds_account: false,
    },
    Operation {
        agent: "microsoft-365",
        command: "teams.team.list",
        integration: "microsoft-365",
        provider: Provider::Microsoft,
        route: Route {
            origin: MICROSOFT_GRAPH_ORIGIN,
            segments: &[Seg::Lit("v1.0"), Seg::Lit("me"), Seg::Lit("joinedTeams")],
            alternate: None,
        },
        // `joinedTeams` documents no query options.
        fixed_query: &[],
        fixed_headers: &[],
        inputs: &[InputMap::local_limit("page-size", 100, 200)],
        limit_input: Some("page-size"),
        any_of_scopes: MICROSOFT_TEAMS_SCOPES,
        request_scopes: &["User.Read", "Team.ReadBasic.All"],
        access: "Teams read access",
        admin_consent: false,
        projection: Projection::TeamsTeams,
        canonical_v2: true,
        read_cap: None,
        binds_account: false,
    },
    Operation {
        agent: "microsoft-365",
        command: "teams.channel.list",
        integration: "microsoft-365",
        provider: Provider::Microsoft,
        route: Route {
            origin: MICROSOFT_GRAPH_ORIGIN,
            segments: &[
                Seg::Lit("v1.0"),
                Seg::Lit("teams"),
                Seg::Input("team-id"),
                Seg::Lit("channels"),
            ],
            alternate: None,
        },
        fixed_query: &[("$select", CHANNELS_SELECT)],
        fixed_headers: &[],
        inputs: &[
            InputMap::path_kind("team-id", IdKind::Guid),
            InputMap::local_limit("page-size", 100, 200),
        ],
        limit_input: Some("page-size"),
        any_of_scopes: MICROSOFT_CHANNELS_SCOPES,
        request_scopes: &["User.Read", "Channel.ReadBasic.All"],
        access: "Teams channel read access",
        admin_consent: false,
        projection: Projection::TeamsChannels,
        canonical_v2: true,
        read_cap: None,
        binds_account: false,
    },
    Operation {
        agent: "microsoft-365",
        command: "teams.channel.list-messages",
        integration: "microsoft-365",
        provider: Provider::Microsoft,
        route: Route {
            origin: MICROSOFT_GRAPH_ORIGIN,
            segments: &[
                Seg::Lit("v1.0"),
                Seg::Lit("teams"),
                Seg::Input("team-id"),
                Seg::Lit("channels"),
                Seg::Input("channel-id"),
                Seg::Lit("messages"),
            ],
            alternate: None,
        },
        fixed_query: &[("$expand", "replies")],
        fixed_headers: TEAMS_MESSAGE_HEADERS,
        inputs: &[
            InputMap::path_kind("team-id", IdKind::Guid),
            InputMap::path_kind("channel-id", IdKind::TeamsThreadId),
            // The channel list documents no `$filter`: `since` is applied
            // to each thread's activity by the projection.
            InputMap::since("lastModifiedDateTime", Location::Local),
            InputMap::top("page-size", 20, 50),
        ],
        limit_input: Some("page-size"),
        any_of_scopes: MICROSOFT_CHANNEL_MESSAGES_SCOPES,
        request_scopes: &["User.Read", "ChannelMessage.Read.All"],
        access: "Teams channel message read access",
        admin_consent: true,
        projection: Projection::TeamsChannelMessages,
        canonical_v2: true,
        read_cap: None,
        binds_account: false,
    },
    Operation {
        agent: "microsoft-365",
        command: "teams.chat.list",
        integration: "microsoft-365",
        provider: Provider::Microsoft,
        route: Route {
            origin: MICROSOFT_GRAPH_ORIGIN,
            segments: &[Seg::Lit("v1.0"), Seg::Lit("me"), Seg::Lit("chats")],
            alternate: None,
        },
        fixed_query: &[("$orderby", "lastMessagePreview/createdDateTime desc")],
        fixed_headers: &[],
        inputs: &[InputMap::top("page-size", 25, 50)],
        limit_input: Some("page-size"),
        any_of_scopes: MICROSOFT_CHATS_SCOPES,
        request_scopes: &["User.Read", "Chat.ReadBasic"],
        access: "Teams chat list access",
        admin_consent: false,
        projection: Projection::TeamsChats,
        canonical_v2: true,
        read_cap: None,
        binds_account: false,
    },
    Operation {
        agent: "microsoft-365",
        command: "teams.chat.list-messages",
        integration: "microsoft-365",
        provider: Provider::Microsoft,
        route: Route {
            origin: MICROSOFT_GRAPH_ORIGIN,
            segments: &[
                Seg::Lit("v1.0"),
                Seg::Lit("chats"),
                Seg::Input("chat-id"),
                Seg::Lit("messages"),
            ],
            alternate: None,
        },
        // Graph requires `$orderby` on the filtered property, descending.
        fixed_query: &[("$orderby", "lastModifiedDateTime desc")],
        fixed_headers: TEAMS_MESSAGE_HEADERS,
        inputs: &[
            InputMap::path_kind("chat-id", IdKind::TeamsThreadId),
            InputMap::since("lastModifiedDateTime", Location::Filter),
            InputMap::top("page-size", 20, 50),
        ],
        limit_input: Some("page-size"),
        any_of_scopes: MICROSOFT_CHAT_MESSAGES_SCOPES,
        request_scopes: &["User.Read", "Chat.Read"],
        access: "Teams chat message read access",
        admin_consent: false,
        projection: Projection::TeamsChatMessages,
        canonical_v2: true,
        read_cap: None,
        binds_account: false,
    },
    Operation {
        agent: "microsoft-365",
        command: "outlook.mail.folder.list",
        integration: "microsoft-365",
        provider: Provider::Microsoft,
        route: Route {
            origin: MICROSOFT_GRAPH_ORIGIN,
            segments: &[
                Seg::Lit("v1.0"),
                Seg::Lit("me"),
                Seg::Lit("mailFolders"),
                Seg::Input("parent-id"),
                Seg::Lit("childFolders"),
            ],
            alternate: None,
        },
        fixed_query: &[("$select", MAIL_FOLDERS_SELECT)],
        fixed_headers: &[],
        inputs: &[
            InputMap::path_long("parent-id", IdKind::OutlookFolderRef, 512),
            InputMap::top("page-size", 50, 100),
            InputMap::offset("skip", "$skip", MAX_FOLDER_SKIP),
        ],
        limit_input: Some("page-size"),
        any_of_scopes: MICROSOFT_MAIL_FOLDER_SCOPES,
        request_scopes: &["User.Read", "Mail.Read"],
        access: "Outlook mail folder access",
        admin_consent: false,
        projection: Projection::OutlookFolders,
        canonical_v2: true,
        read_cap: None,
        binds_account: false,
    },
    Operation {
        agent: "microsoft-365",
        command: "outlook.mail.folder.list-messages",
        integration: "microsoft-365",
        provider: Provider::Microsoft,
        route: Route {
            origin: MICROSOFT_GRAPH_ORIGIN,
            segments: &[
                Seg::Lit("v1.0"),
                Seg::Lit("me"),
                Seg::Lit("mailFolders"),
                Seg::Input("folder-id"),
                Seg::Lit("messages"),
                Seg::Lit("delta"),
            ],
            alternate: None,
        },
        // First round only: a cursor request sends its state token alone,
        // which carries this selection.
        fixed_query: &[("$select", MAIL_HEADERS_SELECT)],
        fixed_headers: OUTLOOK_IMMUTABLE_IDS,
        inputs: &[
            InputMap::path_long("folder-id", IdKind::OutlookId, 512),
            InputMap::prefer_page_size("page-size", 25, 50),
            InputMap::delta_cursor("cursor"),
        ],
        limit_input: Some("page-size"),
        any_of_scopes: MICROSOFT_MAIL_SCOPES,
        request_scopes: &["User.Read", "Mail.Read"],
        access: "Outlook mail read access",
        admin_consent: false,
        projection: Projection::OutlookDelta,
        canonical_v2: true,
        read_cap: Some(OUTLOOK_READ_CAP),
        binds_account: true,
    },
    Operation {
        agent: "microsoft-365",
        command: "outlook.mail.folder.list-message-ids",
        integration: "microsoft-365",
        provider: Provider::Microsoft,
        route: Route {
            origin: MICROSOFT_GRAPH_ORIGIN,
            segments: &[
                Seg::Lit("v1.0"),
                Seg::Lit("me"),
                Seg::Lit("mailFolders"),
                Seg::Input("folder-id"),
                Seg::Lit("messages"),
            ],
            alternate: None,
        },
        // What is in the folder now, newest received first: a plain listing,
        // not the change stream, so a window on the received date is exact.
        // Graph requires the ordered property to lead the filter.
        fixed_query: &[
            ("$select", MAIL_IDS_SELECT),
            ("$orderby", "receivedDateTime desc"),
        ],
        fixed_headers: OUTLOOK_IMMUTABLE_IDS,
        inputs: &[
            InputMap::path_long("folder-id", IdKind::OutlookId, 512),
            InputMap::time_bound("received-after", "receivedDateTime", "ge", true),
            InputMap::time_bound("received-before", "receivedDateTime", "le", false),
            InputMap::top("page-size", 500, 1000),
        ],
        limit_input: Some("page-size"),
        any_of_scopes: MICROSOFT_MAIL_FOLDER_SCOPES,
        request_scopes: &["User.Read", "Mail.Read"],
        access: "Outlook mail folder access",
        admin_consent: false,
        projection: Projection::OutlookMessageIds,
        canonical_v2: true,
        read_cap: Some(OUTLOOK_READ_CAP),
        binds_account: false,
    },
    Operation {
        agent: "microsoft-365",
        command: "outlook.mail.get-message",
        integration: "microsoft-365",
        provider: Provider::Microsoft,
        route: Route {
            origin: MICROSOFT_GRAPH_ORIGIN,
            segments: &[
                Seg::Lit("v1.0"),
                Seg::Lit("me"),
                Seg::Lit("messages"),
                Seg::Input("id"),
            ],
            alternate: None,
        },
        fixed_query: &[
            ("$select", GET_MESSAGE_SELECT),
            ("$expand", GET_MESSAGE_EXPAND),
        ],
        fixed_headers: OUTLOOK_IMMUTABLE_IDS,
        inputs: &[InputMap::path_long("id", IdKind::OutlookId, 512)],
        limit_input: None,
        any_of_scopes: MICROSOFT_MAIL_SCOPES,
        request_scopes: &["User.Read", "Mail.Read"],
        access: "Outlook mail read access",
        admin_consent: false,
        projection: Projection::OutlookMessage,
        canonical_v2: true,
        read_cap: Some(OUTLOOK_READ_CAP),
        binds_account: false,
    },
    Operation {
        agent: "microsoft-365",
        command: "excel.worksheet.list",
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
                Seg::Lit("workbook"),
                Seg::Lit("worksheets"),
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
                    Seg::Lit("workbook"),
                    Seg::Lit("worksheets"),
                ],
            }),
        },
        fixed_query: &[("$select", WORKSHEETS_SELECT)],
        fixed_headers: &[],
        inputs: &[
            InputMap::path_id("drive-id"),
            InputMap::path_id("item-id"),
            InputMap::local_limit("page-size", 100, 200),
        ],
        limit_input: Some("page-size"),
        any_of_scopes: MICROSOFT_WORKBOOK_SCOPES,
        request_scopes: &["User.Read", FILES_READ_WRITE],
        access: "Excel workbook access",
        admin_consent: false,
        projection: Projection::ExcelWorksheets,
        canonical_v2: true,
        read_cap: None,
        binds_account: false,
    },
    Operation {
        agent: "microsoft-365",
        command: "excel.range.read",
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
                Seg::Lit("workbook"),
                Seg::Lit("worksheets"),
                Seg::Input("worksheet"),
                Seg::ExcelRange("range"),
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
                    Seg::Lit("workbook"),
                    Seg::Lit("worksheets"),
                    Seg::Input("worksheet"),
                    Seg::ExcelRange("range"),
                ],
            }),
        },
        fixed_query: &[("$select", RANGE_SELECT)],
        fixed_headers: &[],
        inputs: &[
            InputMap::path_id("drive-id"),
            InputMap::path_id("item-id"),
            InputMap::path_kind("worksheet", IdKind::WorksheetRef),
            InputMap::path_long("range", IdKind::ExcelA1Range, 32),
        ],
        limit_input: None,
        any_of_scopes: MICROSOFT_WORKBOOK_SCOPES,
        request_scopes: &["User.Read", FILES_READ_WRITE],
        access: "Excel workbook access",
        admin_consent: false,
        projection: Projection::ExcelRange,
        canonical_v2: true,
        read_cap: None,
        binds_account: false,
    },
    Operation {
        agent: "microsoft-365",
        command: "get-shared-item",
        integration: "microsoft-365",
        provider: Provider::Microsoft,
        route: Route {
            origin: MICROSOFT_GRAPH_ORIGIN,
            segments: &[
                Seg::Lit("v1.0"),
                Seg::Lit("shares"),
                Seg::ShareToken("sharing-url"),
                Seg::Lit("driveItem"),
            ],
            alternate: None,
        },
        fixed_query: &[("$select", GET_ITEM_SELECT)],
        fixed_headers: &[],
        inputs: &[InputMap::path_long("sharing-url", IdKind::SharingUrl, 2048)],
        limit_input: None,
        any_of_scopes: MICROSOFT_SHARES_SCOPES,
        request_scopes: &["User.Read", FILES_READ_WRITE],
        access: "access to files shared with you",
        admin_consent: false,
        projection: Projection::GraphItem,
        canonical_v2: true,
        read_cap: None,
        binds_account: false,
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
                        if kind == IdKind::ExcelA1Range {
                            map.insert("maxCells".into(), json!(MAX_RANGE_CELLS));
                        }
                    }
                    if let (Some(format), Json::Object(map)) = (input.search, &mut entry) {
                        map.insert("format".into(), Json::String(format.as_str().into()));
                    }
                    if let (true, Json::Object(map)) = (input.timestamp, &mut entry) {
                        map.insert("format".into(), Json::String(UTC_TIMESTAMP_FORMAT.into()));
                    }
                    if let (Some(op), Json::Object(map)) = (input.filter_op, &mut entry) {
                        map.insert("operator".into(), Json::String(op.into()));
                    }
                    if let (true, Json::Object(map)) = (input.cursor, &mut entry) {
                        map.insert("format".into(), Json::String(CURSOR_FORMAT.into()));
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
            // Only rows that send headers name them, so every earlier
            // canonical string (and digest) is unchanged.
            if let Some(cap) = self.read_cap {
                map.insert("readCap".into(), json!(cap));
            }
            if self.binds_account {
                map.insert("bindsAccount".into(), Json::Bool(true));
            }
            if !self.fixed_headers.is_empty() {
                map.insert(
                    "headers".into(),
                    self.fixed_headers
                        .iter()
                        .map(|(k, v)| json!([k, v]))
                        .collect::<Vec<_>>()
                        .into(),
                );
            }
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

/// The binding id for (integration, slot, subject[, tenant]): the first 16 bytes
/// of a domain-separated SHA-256, formatted as a version-8 RFC 4122 UUID.
///
/// `tenant` is the verified Microsoft tenant id (#764), so the same `oid` in a
/// different directory is a different binding. Google has none and passes
/// `None`, which hashes exactly the parts it always did — its binding ids are
/// unchanged.
pub(crate) fn binding_id(integration: &str, slot: &str, sub: &str, tenant: Option<&str>) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    for part in [BINDING_DOMAIN, integration, slot, sub]
        .into_iter()
        .chain(tenant)
    {
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
    /// The `$skip` sent (0 when the operation has none).
    skip: u64,
    /// A validated `since`, for the projection.
    since: Option<chrono::DateTime<chrono::Utc>>,
    /// The rectangle a range read asked for, for the projection.
    range: Option<RangeAsk>,
    /// Headers for the data request: the row's fixed ones, a rendered
    /// `Prefer: odata.maxpagesize` folded into its `Prefer` value.
    headers: Vec<(String, String)>,
    /// A delta cursor the caller sent, shape-checked; its account is checked
    /// once the account is known.
    cursor: Option<DeltaCursor>,
    /// The delta read's folder (its `folder-id`), which a cursor names.
    folder: Option<String>,
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
    let mut skip = 0u64;
    let mut cursor = None;
    let mut max_page_size = None;
    let mut since = None;
    let mut lower = None;
    let mut upper: Option<(&'static str, chrono::DateTime<chrono::Utc>)> = None;
    let mut range = None;
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
                if map.id_kind == Some(IdKind::ExcelA1Range) {
                    // Sent in its one normalised spelling; the projection
                    // checks the answer against the same rectangle.
                    let Some(ask) = a1_range(s) else {
                        return Err(CallFailure::input(map.source));
                    };
                    range = Some(ask);
                    path.push((map.source, ask.address()));
                    continue;
                }
                if map.id_kind == Some(IdKind::OutlookFolderRef)
                    && let Some(name) = outlook_folder_name(s)
                {
                    path.push((map.source, name.to_string()));
                    continue;
                }
                if let Some(format) = map.search {
                    let Some(terms) = format.terms(s) else {
                        return Err(CallFailure::input(map.source));
                    };
                    search.extend(terms);
                    continue;
                }
                if map.cursor {
                    cursor = Some(DeltaCursor::decode(s)?);
                    continue;
                }
                if map.timestamp {
                    let Some(instant) = utc_timestamp(s) else {
                        return Err(CallFailure::input(map.source));
                    };
                    match map.filter_op {
                        None => since = Some(instant),
                        Some("le" | "lt") => upper = Some((map.source, instant)),
                        Some(_) => lower = Some(instant),
                    }
                    if map.location == Location::Filter {
                        // Rendered by the code: an OData DateTimeOffset
                        // literal, unquoted, in one normalised form. Further
                        // terms join the first `$filter`, where it stands.
                        let literal = instant.to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true);
                        let term =
                            format!("{} {} {literal}", map.name, map.filter_op.unwrap_or("gt"));
                        match query.iter_mut().find(|(k, _)| k == "$filter") {
                            Some((_, filter)) => *filter = format!("{filter} and {term}"),
                            None => query.push(("$filter".to_string(), term)),
                        }
                    }
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
        if map.name == "$skip" {
            skip = text.as_deref().and_then(|t| t.parse().ok()).unwrap_or(0);
        }
        if map.location == Location::PreferPageSize {
            max_page_size = text.clone();
        }
        if let Some(text) = text {
            match map.location {
                Location::Query => query.push((map.name.to_string(), text)),
                Location::Path => path.push((map.source, text)),
                // Search terms, filters and cursors are rendered above; a
                // local input is never sent; a page-size preference goes in a
                // header.
                Location::Search
                | Location::Local
                | Location::Filter
                | Location::DeltaState
                | Location::PreferPageSize => {}
            }
        }
    }
    // An empty window would read as an empty folder, never as a mistake.
    if let (Some(lower), Some((source, upper))) = (lower, upper)
        && lower > upper
    {
        return Err(CallFailure::input(source));
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
    let folder = op
        .inputs
        .iter()
        .find(|m| m.location == Location::Path)
        .filter(|_| op.binds_account)
        .and_then(|m| path.iter().find(|(n, _)| *n == m.source))
        .map(|(_, v)| v.clone());
    if let Some(cursor) = &cursor {
        // The token carries the first round's query; nothing else is sent.
        if op.binds_account && folder.as_deref() != Some(cursor.folder.as_str()) {
            return Err(CallFailure::reason(
                "E_CALL_INPUT_INVALID",
                "cursor-context",
            ));
        }
        if cursor.op != op.sha256() {
            return Err(CallFailure::reason(
                "E_CALL_INPUT_INVALID",
                "cursor-context",
            ));
        }
        query = vec![(cursor.kind.param().to_string(), cursor.token.clone())];
    }
    let mut headers: Vec<(String, String)> = op
        .fixed_headers
        .iter()
        .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
        .collect();
    if let Some(n) = max_page_size {
        let wanted = format!("odata.maxpagesize={n}");
        match headers.iter_mut().find(|(k, _)| k == "Prefer") {
            // One `Prefer` value: a second header of the name would replace it.
            Some((_, value)) => *value = format!("{wanted}, {value}"),
            None => headers.push(("Prefer".to_string(), wanted)),
        }
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
            Seg::ExcelRange(name) => {
                let value = path_value(name)
                    .ok_or_else(|| CallFailure::reason("E_CALL_INPUT_INVALID", "path"))?;
                segments.push(format!("range(address='{value}')"));
            }
            Seg::ShareToken(name) => {
                use base64::Engine as _;
                let value = path_value(name)
                    .ok_or_else(|| CallFailure::reason("E_CALL_INPUT_INVALID", "path"))?;
                let token =
                    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(value.as_bytes());
                segments.push(format!("u!{token}"));
            }
        }
    }
    let limit = limit.unwrap_or(100);
    Ok(Wire {
        segments,
        query,
        headers,
        cursor,
        folder,
        limit,
        skip,
        since,
        range,
    })
}

// ── delta cursors ────────────────────────────────────────────────────────────

/// Which Graph state token a cursor carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CursorKind {
    /// `$skiptoken`: the round continues.
    Skip,
    /// `$deltatoken`: the round finished; the next round starts here.
    Delta,
}

impl CursorKind {
    fn name(self) -> &'static str {
        match self {
            CursorKind::Skip => "skip",
            CursorKind::Delta => "delta",
        }
    }

    fn param(self) -> &'static str {
        match self {
            CursorKind::Skip => "$skiptoken",
            CursorKind::Delta => "$deltatoken",
        }
    }
}

/// `outlook-delta-cursor/1`: `odc1.` + unpadded base64url of the canonical
/// JSON `{binding, folder, kind, op, token, v: 1}`. It keeps streams apart (a
/// cursor of another folder, account or operation is refused, never mixed in);
/// it is not authenticated and is no security boundary: the route, scopes,
/// account check and projection are code-owned whatever token it carries.
#[derive(Debug, Clone, PartialEq, Eq)]
struct DeltaCursor {
    binding: String,
    folder: String,
    kind: CursorKind,
    op: String,
    token: String,
}

/// A state token AWARE carries: 1..8 KiB, no control character.
fn state_token(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_STATE_TOKEN_BYTES
        && !value.chars().any(char::is_control)
}

impl DeltaCursor {
    fn encode(&self) -> String {
        use base64::Engine as _;
        let canonical = canonical_string(&json!({
            "v": 1,
            "binding": self.binding,
            "folder": self.folder,
            "kind": self.kind.name(),
            "op": self.op,
            "token": self.token,
        }));
        format!(
            "{CURSOR_PREFIX}{}",
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(canonical.as_bytes())
        )
    }

    /// The cursor a caller sent, or `E_CALL_INPUT_INVALID input: cursor`.
    fn decode(text: &str) -> Result<Self, CallFailure> {
        use base64::Engine as _;
        let bad = || CallFailure::input("cursor");
        let body = text.strip_prefix(CURSOR_PREFIX).ok_or_else(bad)?;
        if body.is_empty()
            || !body
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-'))
        {
            return Err(bad());
        }
        let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(body)
            .map_err(|_| bad())?;
        let Ok(Json::Object(map)) = serde_json::from_slice::<Json>(&bytes) else {
            return Err(bad());
        };
        let mut keys: Vec<&str> = map.keys().map(String::as_str).collect();
        keys.sort_unstable();
        if keys != ["binding", "folder", "kind", "op", "token", "v"] || map["v"] != json!(1) {
            return Err(bad());
        }
        let text_of = |key: &str| map.get(key).and_then(Json::as_str).ok_or_else(bad);
        let kind = match text_of("kind")? {
            "skip" => CursorKind::Skip,
            "delta" => CursorKind::Delta,
            _ => return Err(bad()),
        };
        let (binding, folder, op, token) = (
            text_of("binding")?,
            text_of("folder")?,
            text_of("op")?,
            text_of("token")?,
        );
        if !is_uuid(binding) || !is_outlook_id(folder) || !is_sha256_hex(op) || !state_token(token)
        {
            return Err(bad());
        }
        Ok(Self {
            binding: binding.to_string(),
            folder: folder.to_string(),
            kind,
            op: op.to_string(),
            token: token.to_string(),
        })
    }
}

/// The state token of the one continuation link Graph sent. The link is never
/// followed and its path is never used: AWARE rebuilds the request from its
/// own route. Supported shape: a Graph `https` URL whose query is exactly one
/// pair, `$skiptoken` on a nextLink or `$deltatoken` on a deltaLink.
fn link_token(link: &str, kind: CursorKind) -> Result<String, CallFailure> {
    let unsupported = || CallFailure::reason("E_CALL_FAILED", "cursor-link-unsupported");
    let url = url::Url::parse(link).map_err(|_| unsupported())?;
    if url.scheme() != "https"
        || url.host_str() != Some("graph.microsoft.com")
        || url.port().is_some()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
    {
        return Err(unsupported());
    }
    let pairs: Vec<(String, String)> = url
        .query_pairs()
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();
    match pairs.as_slice() {
        [(name, token)] if name == kind.param() && state_token(token) => Ok(token.clone()),
        _ => Err(unsupported()),
    }
}

/// One folder's changes: every entry Graph sent, in Graph's order, never cut
/// (a dropped change would be lost when the caller advances its cursor).
/// An entry AWARE cannot identify is kept in place as `unidentifiable`, with
/// nothing from it, and counted; the cursor is still returned.
fn project_outlook_delta(
    body: &Json,
    op: &Operation,
    folder: &str,
    binding: &str,
) -> Result<Json, CallFailure> {
    let shape = || CallFailure::reason("E_CALL_FAILED", "response-shape");
    let entries = body
        .get("value")
        .and_then(Json::as_array)
        .ok_or_else(shape)?;
    let link = |key: &str| {
        body.get(key)
            .and_then(Json::as_str)
            .filter(|t| !t.is_empty())
    };
    let (kind, link) = match (link("@odata.nextLink"), link("@odata.deltaLink")) {
        (Some(next), None) => (CursorKind::Skip, next),
        (None, Some(delta)) => (CursorKind::Delta, delta),
        _ => return Err(shape()),
    };
    let token = link_token(link, kind)?;
    let mut changes = Vec::with_capacity(entries.len());
    let mut unidentifiable = 0usize;
    for (position, item) in entries.iter().enumerate() {
        let id = outlook_id(item.get("id"));
        let removed = item.get("@removed");
        let change = match (item.is_object(), id, removed) {
            (true, Some(id), Some(removed)) if removed.is_object() => {
                let mut change = Map::new();
                change.insert("change".into(), Json::String("removed".into()));
                change.insert("id".into(), Json::String(id));
                if let Some((reason, _)) = removed
                    .get("reason")
                    .and_then(Json::as_str)
                    .and_then(|r| capped(r, 64))
                {
                    change.insert("reason".into(), Json::String(reason));
                }
                Json::Object(change)
            }
            (true, Some(id), None) => {
                let mut entry = Entry::new(id.clone());
                outlook_headers(item, &mut entry);
                let mut fields = entry.finish();
                if let Json::Object(map) = &mut fields {
                    map.remove("id");
                }
                json!({"change": "upsert", "id": id, "fields": fields})
            }
            _ => {
                unidentifiable += 1;
                json!({"change": "unidentifiable", "position": position})
            }
        };
        changes.push(change);
    }
    let cursor = DeltaCursor {
        binding: binding.to_string(),
        folder: folder.to_string(),
        kind,
        op: op.sha256(),
        token,
    };
    Ok(json!({
        "changes": changes,
        "unidentifiable": unidentifiable,
        "cursor": cursor.encode(),
        "more-available": kind == CursorKind::Skip,
    }))
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
    if !usable_access_token(&fresh.access_token) {
        return Err(CallFailure::reason("E_CREDENTIAL_MISSING", "unusable"));
    }
    Ok(SlotToken {
        access_token: fresh.access_token,
        generation,
        scope: fresh.scope,
    })
}

/// A token AWARE may put in an `Authorization` header.
fn usable_access_token(token: &str) -> bool {
    !token.trim().is_empty()
        && crate::runtime::invoker::never_sendable_char(token).is_none()
        && crate::runtime::invoker::unsendable_in_header_char(token).is_none()
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
    /// A status >= 300 and, when the caller asked for it, a Graph error code
    /// from the allowlist it passed (never any other text).
    Status(u16, Option<&'static str>),
    Transport,
    Timeout,
    TooLarge,
}

/// One bounded GET: no redirects, a total deadline, a body cap; a status >= 300
/// is a failure carrying only that status. Blocking.
fn get_bounded(
    url: &str,
    query: &[(String, String)],
    headers: &[(&str, &str)],
    bearer: &str,
    timeout: Duration,
    cap: usize,
    error_codes: &[&'static str],
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
    for (name, value) in headers {
        request = request.set(name, value);
    }
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
        // Only an allowlisted `error.code` survives; a failed, malformed or
        // oversized error read keeps the status and yields no code.
        let code = if status >= 400 && !error_codes.is_empty() {
            let mut bytes = Vec::new();
            let read = response
                .into_reader()
                .take(ERROR_BODY_CAP as u64 + 1)
                .read_to_end(&mut bytes);
            if read.is_ok() && bytes.len() <= ERROR_BODY_CAP {
                serde_json::from_slice::<Json>(&bytes)
                    .ok()
                    .and_then(|body| body.get("error")?.get("code")?.as_str().map(str::to_string))
                    .and_then(|code| {
                        error_codes
                            .iter()
                            .copied()
                            .find(|known| known.eq_ignore_ascii_case(&code))
                    })
            } else {
                None
            }
        } else {
            None
        };
        return Err(HttpFailure::Status(status, code));
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
    /// Microsoft only: the verified tenant (directory) id, lowercase — see
    /// [`read_microsoft_tenant`]. Part of the binding. `None` for Google.
    pub(crate) tenant: Option<String>,
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
        &[],
        &token.access_token,
        timeout,
        IDENTITY_CAP,
        &[],
    )
    .map_err(|failure| match failure {
        HttpFailure::Status(status, _) => {
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
    Ok(Principal {
        sub,
        label,
        tenant: None,
    })
}

/// Graph `/organization`, on the identity read's own origin and version.
fn organization_url(identity_url: &str) -> Option<String> {
    identity_url
        .strip_suffix("/me")
        .map(|base| format!("{base}/organization"))
}

/// The one organisation `/organization` answered, when it answered exactly one
/// with a GUID id.
fn single_organization(bytes: &[u8]) -> Option<String> {
    let body: Json = serde_json::from_slice(bytes).ok()?;
    let [org] = body.get("value")?.as_array()?.as_slice() else {
        return None;
    };
    let id = org.get("id")?.as_str()?;
    is_guid(id).then(|| id.to_ascii_lowercase())
}

/// The verified Microsoft tenant id of the slot whose `token` `/me` just
/// confirmed as `sub` (#764), asked of Microsoft in the same verification.
/// Blocking.
///
/// 1. `GET /v1.0/organization?$select=id` with the same token (`User.Read`,
///    already required): for a work or school account, member or guest, Graph
///    answers the one directory the token was issued in — the directory whose
///    user object `/me` returned.
/// 2. Microsoft documents that read as unsupported for a personal account. When
///    it names no single organisation (an empty list, or a failure other than
///    the deadline), AWARE asks the token endpoint: one refresh-token grant
///    through the pinned Microsoft endpoint with `client_info=1`. From then on
///    `token` IS the refreshed access token — every later read of this
///    verification, and the data read, use it — so the tenant is always the
///    tenant of the token actually used. With it, `/me` must answer the same
///    `sub` and `/organization` is asked again; a single organisation there is
///    the tenant (a guest whose first read failed).
/// 3. Only then the consumer tenant, and only if every source agrees: the
///    endpoint's authority is `common` / `consumers` / the consumer GUID (a guest
///    signs in through its host directory's own authority), `client_info`'s
///    `utid` (the HOME tenant) is the consumer GUID, an ID token in the reply,
///    if any, says the same `tid`, and `sub` is not a directory object id (a
///    GUID) — a guest's object in a directory always is, so this can only
///    refuse, never accept. Anything else is unverified.
fn read_microsoft_tenant(
    home: &Path,
    integration: &str,
    alias: Option<&str>,
    endpoints: &Endpoints,
    token: &mut SlotToken,
    sub: &str,
    budget: &dyn Fn() -> Result<Duration, CallFailure>,
) -> Result<String, CallFailure> {
    let unverified = || CallFailure::reason("E_CALL_IDENTITY_FAILED", "tenant");
    let url = organization_url(&endpoints.identity_url).ok_or_else(unverified)?;
    let organization = |token: &SlotToken| -> Result<Option<String>, CallFailure> {
        match get_bounded(
            &url,
            &[("$select".to_string(), "id".to_string())],
            &[],
            &token.access_token,
            budget()?,
            IDENTITY_CAP,
            &[],
        ) {
            Ok(bytes) => Ok(single_organization(&bytes)),
            Err(HttpFailure::Timeout) => {
                Err(CallFailure::reason("E_CALL_IDENTITY_FAILED", "timeout"))
            }
            // Documented as unsupported for a personal account; the token
            // endpoint decides below, and accepts only the consumer tenant.
            Err(_) => Ok(None),
        }
    };
    if let Some(tenant) = organization(token)? {
        return Ok(tenant);
    }
    let config = crate::auth::config::for_integration(integration)
        .and_then(|config| config.with_profile(home, alias))
        .map_err(|_| CallFailure::reason("E_CREDENTIAL_EXPIRED", "oauth-config"))?;
    if !endpoints.token.allows(&config) {
        return Err(CallFailure::reason(
            "E_CREDENTIAL_EXPIRED",
            "token-endpoint",
        ));
    }
    let personal_authority = microsoft_authority(config.token_url()).is_some_and(|authority| {
        authority.eq_ignore_ascii_case("common")
            || authority.eq_ignore_ascii_case("consumers")
            || authority.eq_ignore_ascii_case(MICROSOFT_CONSUMER_TENANT)
    });
    if !personal_authority {
        return Err(unverified());
    }
    let refreshed =
        crate::auth::refresh::refresh_now_for_tenant(integration, alias, home, &config, budget()?)
            .map_err(|_| unverified())?;
    if refreshed.generation.as_deref() != Some(token.generation.as_str()) {
        return Err(CallFailure::reason("E_CREDENTIAL_CHANGED", "refreshed"));
    }
    if !usable_access_token(&refreshed.access_token) {
        return Err(unverified());
    }
    token.access_token = refreshed.access_token;
    if read_identity(endpoints, Provider::Microsoft, token, budget()?)?.sub != sub {
        return Err(unverified());
    }
    if let Some(tenant) = organization(token)? {
        return Ok(tenant);
    }
    let consumer = |tenant: &Option<String>| tenant.as_deref() == Some(MICROSOFT_CONSUMER_TENANT);
    let tenants = &refreshed.tenants;
    if consumer(&tenants.home)
        && (tenants.signed_in.is_none() || consumer(&tenants.signed_in))
        && !is_guid(sub)
    {
        Ok(MICROSOFT_CONSUMER_TENANT.to_string())
    } else {
        Err(unverified())
    }
}

/// The `<tenant>` of a `…/<tenant>/oauth2/v2.0/token` endpoint. The origin is
/// the token pin's business (production pins `login.microsoftonline.com`).
fn microsoft_authority(token_url: &str) -> Option<String> {
    let url = url::Url::parse(token_url).ok()?;
    let segments: Vec<&str> = url.path_segments()?.collect();
    match segments.as_slice() {
        [tenant, "oauth2", "v2.0", "token"] if !tenant.is_empty() => Some((*tenant).to_string()),
        _ => None,
    }
}

/// Who the slot's token belongs to, as the provider says now: the identity read
/// and, for Microsoft, the tenant. `budget` yields what remains of the verb's
/// deadline before each network phase. `token` may be replaced by a refreshed
/// access token of the same grant ([`read_microsoft_tenant`]); the caller must
/// use it from then on.
fn verify_principal(
    home: &Path,
    op: &Operation,
    alias: Option<&str>,
    endpoints: &Endpoints,
    token: &mut SlotToken,
    budget: &dyn Fn() -> Result<Duration, CallFailure>,
) -> Result<Principal, CallFailure> {
    let mut principal = read_identity(endpoints, op.provider, token, budget()?)?;
    if op.provider == Provider::Microsoft {
        principal.tenant = Some(read_microsoft_tenant(
            home,
            op.integration,
            alias,
            endpoints,
            token,
            &principal.sub,
            budget,
        )?);
    }
    Ok(principal)
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
    binding: Option<&str>,
) -> Result<Json, CallFailure> {
    // A delta cursor belongs to one verified account: checked before any data
    // request, and the account its successor is minted for.
    let binding = match (op.binds_account, binding) {
        (false, _) => None,
        (true, Some(binding)) => Some(binding.to_ascii_lowercase()),
        (true, None) => return Err(CallFailure::new("E_BINDING_CHANGED")),
    };
    if let (Some(cursor), Some(binding)) = (&wire.cursor, &binding)
        && cursor.binding != *binding
    {
        return Err(CallFailure::reason(
            "E_CALL_INPUT_INVALID",
            "cursor-context",
        ));
    }
    let delta = op.projection == Projection::OutlookDelta;
    let url = endpoints.data_url(op, wire)?;
    let headers: Vec<(&str, &str)> = wire
        .headers
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    let bytes = get_bounded(
        &url,
        &wire.query,
        &headers,
        &token.access_token,
        timeout,
        op.read_cap.unwrap_or(cap),
        if delta { DEAD_CURSOR_CODES } else { &[] },
    )
    .map_err(|failure| match failure {
        // A delta state token Graph no longer honours: start a new round.
        HttpFailure::Status(status, code)
            if delta && (status == 410 || (code.is_some() && matches!(status, 400 | 404))) =>
        {
            CallFailure::reason("E_CALL_FAILED", "cursor-expired").with("status", json!(status))
        }
        HttpFailure::Status(status, _) => {
            CallFailure::new("E_CALL_FAILED").with("status", json!(status))
        }
        HttpFailure::Timeout => CallFailure::new("E_CALL_TIMEOUT"),
        HttpFailure::TooLarge => {
            let failure = CallFailure::new("E_CALL_OUTPUT_TOO_LARGE");
            match op.read_cap {
                Some(_) => failure.with("reason", Json::String("raw-too-large".into())),
                None => failure,
            }
        }
        HttpFailure::Transport => CallFailure::reason("E_CALL_FAILED", "transport"),
    })?;
    let body: Json = serde_json::from_slice(&bytes)
        .map_err(|_| CallFailure::reason("E_CALL_FAILED", "response-shape"))?;
    let ctx = ProjectionContext {
        limit: wire.limit,
        skip: wire.skip,
        since: wire.since,
        range: wire.range,
    };
    let projected = match (&binding, &wire.folder) {
        (Some(binding), Some(folder)) if delta => {
            project_outlook_delta(&body, op, folder, binding)?
        }
        _ => op
            .projection
            .apply(&body, ctx)
            .ok_or_else(|| CallFailure::reason("E_CALL_FAILED", "response-shape"))?,
    };
    // The budget holds for what the caller receives, not only for what Graph
    // sent: a projection may add labels (truncated cells) to a near-cap answer.
    let size = serde_json::to_vec(&projected).map_or(usize::MAX, |bytes| bytes.len());
    if size > cap {
        let failure = CallFailure::new("E_CALL_OUTPUT_TOO_LARGE");
        return Err(match op.read_cap {
            // The caller may retry with a larger budget (up to the maximum).
            Some(_) => failure
                .with("reason", Json::String("output-too-large".into()))
                .with("outputBytes", json!(size)),
            None => failure,
        });
    }
    Ok(projected)
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

/// One worksheet: its braced-GUID id, name, position and visibility. A hidden
/// sheet is listed and labelled, never dropped.
fn worksheet_entry(item: &Json) -> Option<Json> {
    let id = item.get("id").and_then(Json::as_str)?;
    let inner = id.strip_prefix('{').and_then(|v| v.strip_suffix('}'))?;
    if !is_guid(inner) {
        return None;
    }
    let mut entry = Entry::new(id.to_string());
    entry.text("name", item.get("name"), 64);
    if let Some(position) = item.get("position").and_then(Json::as_u64) {
        entry.put("position", json!(position));
    }
    entry.text("visibility", item.get("visibility"), 32);
    Some(entry.finish())
}

/// The longest cell string returned whole.
const MAX_CELL_TEXT: usize = 2_048;

/// How many cut cells are listed by position.
const MAX_TRUNCATED_CELLS: usize = 100;

/// The value types Graph documents for a range cell.
const RANGE_VALUE_TYPES: &[&str] = &[
    "Empty", "String", "Integer", "Double", "Boolean", "Error", "Unknown",
];

/// A cell string, changed only by the length cut: every character Excel
/// returned (whitespace, control characters, an empty string) is kept.
fn cell_string(text: &str) -> (String, bool) {
    if text.chars().count() <= MAX_CELL_TEXT {
        return (text.to_string(), false);
    }
    (text.chars().take(MAX_CELL_TEXT).collect(), true)
}

/// `rows` arrays of exactly `columns` entries each.
fn matrix(value: Option<&Json>, rows: usize, columns: usize) -> Option<&Vec<Json>> {
    let outer = value?.as_array()?;
    (outer.len() == rows
        && outer
            .iter()
            .all(|row| row.as_array().is_some_and(|r| r.len() == columns)))
    .then_some(outer)
}

/// A range read (`excel.range.read`). `None` (response-shape) unless the
/// answer is exactly the rectangle asked for, with three well-formed matrices;
/// never a partial table.
fn project_excel_range(body: &Json, ask: RangeAsk) -> Option<Json> {
    let rows = body.get("rowCount")?.as_u64()?;
    let columns = body.get("columnCount")?.as_u64()?;
    if rows != ask.rows() || columns != ask.columns() || rows * columns > MAX_RANGE_CELLS {
        return None;
    }
    let address = body.get("address")?.as_str()?;
    let (_, corner) = address.rsplit_once('!')?;
    if a1_rect(corner, true)? != ask {
        return None;
    }
    let (rows, columns) = (usize::try_from(rows).ok()?, usize::try_from(columns).ok()?);
    let values = matrix(body.get("values"), rows, columns)?;
    let texts = matrix(body.get("text"), rows, columns)?;
    let types = matrix(body.get("valueTypes"), rows, columns)?;
    let mut out_values = Vec::with_capacity(rows);
    let mut out_texts = Vec::with_capacity(rows);
    let mut out_types = Vec::with_capacity(rows);
    let mut cut_cells = Vec::new();
    let mut cut_count = 0usize;
    for r in 0..rows {
        let mut value_row = Vec::with_capacity(columns);
        let mut text_row = Vec::with_capacity(columns);
        let mut type_row = Vec::with_capacity(columns);
        for c in 0..columns {
            let mut cut = false;
            let value = match &values[r].as_array()?[c] {
                Json::String(text) => {
                    let (kept, was_cut) = cell_string(text);
                    cut |= was_cut;
                    Json::String(kept)
                }
                number @ Json::Number(_) => number.clone(),
                flag @ Json::Bool(_) => flag.clone(),
                _ => return None,
            };
            let (text, was_cut) = cell_string(texts[r].as_array()?[c].as_str()?);
            cut |= was_cut;
            let kind = types[r].as_array()?[c].as_str()?;
            if !RANGE_VALUE_TYPES.contains(&kind) {
                return None;
            }
            if cut {
                cut_count += 1;
                if cut_cells.len() < MAX_TRUNCATED_CELLS {
                    cut_cells.push(json!([r, c]));
                }
            }
            value_row.push(value);
            text_row.push(Json::String(text));
            type_row.push(Json::String(kind.to_string()));
        }
        out_values.push(Json::Array(value_row));
        out_texts.push(Json::Array(text_row));
        out_types.push(Json::Array(type_row));
    }
    Some(json!({
        "address": address,
        "row-count": rows,
        "column-count": columns,
        "values": out_values,
        "text": out_texts,
        "value-types": out_types,
        "truncated-cells": cut_cells,
        "truncated-cells-omitted": cut_count - cut_cells.len(),
    }))
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

/// One team the person belongs to. Its id is its group's GUID; anything else
/// drops the entry (counted in `omitted`).
fn team_entry(item: &Json) -> Option<Json> {
    let id = exact_id(item.get("id")).filter(|id| is_guid(id))?;
    let mut entry = Entry::new(id);
    entry.text("name", item.get("displayName"), 1024);
    entry.text("description", item.get("description"), 1024);
    entry.text("web-url", item.get("webUrl"), 2048);
    Some(entry.finish())
}

/// One channel of a team. Its id must be a Teams thread id.
fn channel_entry(item: &Json) -> Option<Json> {
    let id = exact_id(item.get("id")).filter(|id| is_teams_thread_id(id))?;
    let mut entry = Entry::new(id);
    entry.text("name", item.get("displayName"), 1024);
    entry.text("description", item.get("description"), 1024);
    entry.text("membership-type", item.get("membershipType"), 64);
    entry.text("web-url", item.get("webUrl"), 2048);
    entry.text("created-at", item.get("createdDateTime"), 64);
    Some(entry.finish())
}

/// One chat. Members are not expanded (a follow-up, not a consent matter):
/// `chat-type` says what the chat is.
fn chat_entry(item: &Json) -> Option<Json> {
    let id = exact_id(item.get("id")).filter(|id| is_teams_thread_id(id))?;
    let mut entry = Entry::new(id);
    entry.text("topic", item.get("topic"), 1024);
    entry.text("chat-type", item.get("chatType"), 64);
    entry.text("created-at", item.get("createdDateTime"), 64);
    entry.text("last-updated-at", item.get("lastUpdatedDateTime"), 64);
    entry.text("web-url", item.get("webUrl"), 2048);
    Some(entry.finish())
}

/// A non-negative integer count, or nothing.
fn count(value: Option<&Json>) -> Option<Json> {
    value.and_then(Json::as_u64).map(Json::from)
}

/// One mail folder. Its id must be an exact Outlook id (else the entry is
/// dropped and counted); a malformed parent id is withheld, never cleaned.
fn outlook_folder_entry(item: &Json) -> Option<Json> {
    let mut entry = Entry::new(outlook_id(item.get("id"))?);
    entry.text("name", item.get("displayName"), 1024);
    match item.get("parentFolderId") {
        None | Some(Json::Null) => {}
        parent => match outlook_id(parent) {
            Some(parent) => entry.put("parent-id", Json::String(parent)),
            None => entry.put("parent-id-withheld", Json::Bool(true)),
        },
    }
    for (key, source) in [
        ("child-count", "childFolderCount"),
        ("total-count", "totalItemCount"),
        ("unread-count", "unreadItemCount"),
    ] {
        if let Some(n) = count(item.get(source)) {
            entry.put(key, n);
        }
    }
    Some(entry.finish())
}

/// One message of a folder listing (`folder.list-message-ids`): its id and
/// dates, each a real UTC instant or withheld (`<key>-withheld`) so a caller
/// can send it back as a window bound; absent stays absent, `null` stays
/// `null`. No local cut: every entry Graph sent is kept or counted, so a
/// listing never reads as shorter than it was. `None` drops a malformed id,
/// counted in `omitted`.
fn outlook_message_id_entry(item: &Json) -> Option<Json> {
    let mut entry = Entry::new(outlook_id(item.get("id"))?);
    for (key, source, withheld) in [
        ("received-at", "receivedDateTime", "received-at-withheld"),
        (
            "last-modified-at",
            "lastModifiedDateTime",
            "last-modified-at-withheld",
        ),
    ] {
        match item.get(source) {
            None => {}
            Some(Json::Null) => entry.put(key, Json::Null),
            Some(value) => match value.as_str().filter(|s| utc_timestamp(s).is_some()) {
                Some(text) => entry.put(key, Json::String(text.to_string())),
                None => entry.put(withheld, Json::Bool(true)),
            },
        }
    }
    match item.get("isDraft") {
        None => {}
        Some(Json::Bool(b)) => entry.put("draft", Json::Bool(*b)),
        Some(_) => entry.put("draft", Json::Null),
    }
    Some(entry.finish())
}

/// A folder's child folders, with the offset of the next page: the `$skip`
/// sent plus every entry Graph returned (omitted ones too), so a caller paging
/// by `next-skip` neither repeats nor skips a folder.
fn project_outlook_folders(body: &Json, ctx: ProjectionContext) -> Option<Json> {
    let returned = body.get("value")?.as_array()?.len() as u64;
    let mut out = project_collection(body, usize::MAX, "folders", outlook_folder_entry)?;
    let next = out.get("more-available").and_then(Json::as_bool) == Some(true);
    if let Json::Object(map) = &mut out
        && next
    {
        let offset = ctx.skip + returned;
        if offset <= MAX_FOLDER_SKIP as u64 {
            map.insert("next-skip".into(), json!(offset));
        } else {
            map.insert("limit-reached".into(), Json::Bool(true));
        }
    }
    Some(out)
}

/// Text kept per Outlook body (`text`, `unique-text`), in chars.
const MAX_MAIL_TEXT: usize = 65_536;
/// Recipients kept per list (`to`, `cc`); the rest are counted.
const MAX_RECIPIENTS: usize = 50;
/// Attachments kept per message; the rest are counted.
const MAX_MAIL_ATTACHMENTS: usize = 20;

/// An RFC 5322 `Message-ID` kept byte for byte: `<...>`, 3..998 bytes of
/// printable ASCII, no space.
fn internet_message_id(value: &str) -> bool {
    (3..=998).contains(&value.len())
        && value.starts_with('<')
        && value.ends_with('>')
        && value.bytes().all(|b| (0x21..=0x7e).contains(&b))
}

/// An Exchange `changeKey` kept byte for byte: 1..512 bytes of printable
/// ASCII, no space. Opaque: it says a message changed, never in which order.
fn change_key(value: &str) -> bool {
    (1..=512).contains(&value.len()) && value.bytes().all(|b| (0x21..=0x7e).contains(&b))
}

/// One recipient (`{emailAddress: {address, name}}`), and whether a field
/// was cut.
fn mail_party(value: &Json) -> Option<(Json, bool)> {
    let address = value.get("emailAddress")?;
    let mut party = Map::new();
    let mut cut = false;
    for (key, max) in [("address", 320), ("name", 256)] {
        if let Some((text, was_cut)) = address
            .get(key)
            .and_then(Json::as_str)
            .and_then(|s| capped(s, max))
        {
            party.insert(key.into(), Json::String(text));
            cut |= was_cut;
        }
    }
    Some((Json::Object(party), cut))
}

/// An exact id field: absent stays absent, `null` stays `null`, a malformed
/// value is withheld (`<key>-withheld`), never cleaned into another key.
fn exact_field(entry: &mut Entry, key: &'static str, value: Option<&Json>, ok: fn(&str) -> bool) {
    match value {
        None => {}
        Some(Json::Null) => entry.put(key, Json::Null),
        Some(value) => match value.as_str().filter(|s| ok(s)) {
            Some(text) => entry.put(key, Json::String(text.to_string())),
            None => entry.put(
                match key {
                    "internet-message-id" => "internet-message-id-withheld",
                    "conversation-id" => "conversation-id-withheld",
                    "parent-folder-id" => "parent-folder-id-withheld",
                    _ => "change-key-withheld",
                },
                Json::Bool(true),
            ),
        },
    }
}

/// The headers of an Outlook message (a folder change or `get-message`), with
/// presence kept: a property Graph did not send is absent, one it sent as
/// `null` is `null`, never an invented `false` or empty value.
fn outlook_headers(item: &Json, entry: &mut Entry) {
    exact_field(
        entry,
        "internet-message-id",
        item.get("internetMessageId"),
        internet_message_id,
    );
    exact_field(
        entry,
        "conversation-id",
        item.get("conversationId"),
        is_outlook_id,
    );
    if let Some(kind) = item.get("@odata.type").and_then(Json::as_str) {
        let kind = match kind {
            "#microsoft.graph.message" => "message",
            "#microsoft.graph.eventMessage"
            | "#microsoft.graph.eventMessageRequest"
            | "#microsoft.graph.eventMessageResponse" => "event-message",
            _ => "other",
        };
        entry.put("kind", Json::String(kind.into()));
    }
    for (key, source, max) in [
        ("subject", "subject", 1024),
        ("received-at", "receivedDateTime", 64),
        ("sent-at", "sentDateTime", 64),
        ("last-modified-at", "lastModifiedDateTime", 64),
        ("preview", "bodyPreview", 512),
        ("web-url", "webLink", 2048),
    ] {
        match item.get(source) {
            None => {}
            Some(Json::Null) => entry.put(key, Json::Null),
            value => entry.text(key, value, max),
        }
    }
    for (key, source) in [("draft", "isDraft"), ("has-attachments", "hasAttachments")] {
        match item.get(source) {
            None => {}
            Some(Json::Bool(b)) => entry.put(key, Json::Bool(*b)),
            Some(_) => entry.put(key, Json::Null),
        }
    }
    match item.get("from") {
        None => {}
        Some(Json::Null) => entry.put("from", Json::Null),
        Some(from) => {
            if let Some((party, cut)) = mail_party(from) {
                entry.put("from", party);
                if cut {
                    entry.truncated.push("from");
                }
            }
        }
    }
    let mut recipients_omitted = 0usize;
    let mut any_list = false;
    for (key, source) in [("to", "toRecipients"), ("cc", "ccRecipients")] {
        let Some(list) = item.get(source).and_then(Json::as_array) else {
            continue;
        };
        any_list = true;
        let mut kept = Vec::new();
        let mut cut = false;
        for recipient in list {
            if kept.len() >= MAX_RECIPIENTS {
                recipients_omitted += 1;
                continue;
            }
            match mail_party(recipient) {
                Some((party, was_cut)) => {
                    kept.push(party);
                    cut |= was_cut;
                }
                None => recipients_omitted += 1,
            }
        }
        entry.put(key, Json::Array(kept));
        if cut {
            entry.truncated.push(key);
        }
    }
    if any_list {
        entry.put("recipients-omitted", json!(recipients_omitted));
    }
}

/// An Outlook body as plain text, reduced in AWARE as Teams messages are;
/// `None` when Graph sent no body at all.
fn mail_body_text(body: Option<&Json>) -> Option<(String, bool)> {
    let body = body.filter(|b| b.is_object())?;
    let content = body.get("content").and_then(Json::as_str).unwrap_or("");
    let html = body
        .get("contentType")
        .and_then(Json::as_str)
        .is_some_and(|t| t.eq_ignore_ascii_case("html"));
    let plain = if html {
        html_text(content)
    } else {
        content.to_string()
    };
    Some(message_text(&plain, MAX_MAIL_TEXT).unwrap_or_default())
}

/// A message's attachments: names, types and sizes, never bytes. The state
/// is `complete` only when Graph sent the whole list and AWARE kept every
/// entry, `partial` otherwise, `unavailable` when Graph sent no list.
fn mail_attachments(item: &Json, entry: &mut Entry) {
    let Some(list) = item.get("attachments").and_then(Json::as_array) else {
        entry.put("attachments", Json::Array(Vec::new()));
        entry.put("attachments-omitted", json!(0));
        entry.put("attachments-state", Json::String("unavailable".into()));
        return;
    };
    let more = item
        .get("attachments@odata.nextLink")
        .and_then(Json::as_str)
        .is_some_and(|t| !t.is_empty());
    let mut out = Vec::new();
    let mut omitted = 0usize;
    for attachment in list {
        if out.len() >= MAX_MAIL_ATTACHMENTS {
            omitted += 1;
            continue;
        }
        let Some(id) = outlook_id(attachment.get("id")) else {
            omitted += 1;
            continue;
        };
        let mut a = Entry::new(id);
        let kind = match attachment.get("@odata.type").and_then(Json::as_str) {
            Some("#microsoft.graph.fileAttachment") => "file",
            Some("#microsoft.graph.itemAttachment") => "item",
            Some("#microsoft.graph.referenceAttachment") => "reference",
            _ => "other",
        };
        a.put("kind", Json::String(kind.into()));
        a.text("name", attachment.get("name"), 1024);
        a.text("content-type", attachment.get("contentType"), 256);
        if let Some(n) = count(attachment.get("size")) {
            a.put("size", n);
        }
        if let Some(inline) = attachment.get("isInline").and_then(Json::as_bool) {
            a.put("inline", Json::Bool(inline));
        }
        if ["contentBytes", "item"]
            .iter()
            .any(|key| attachment.get(*key).is_some_and(|v| !v.is_null()))
        {
            a.put("content-withheld", Json::Bool(true));
        }
        out.push(a.finish());
    }
    let state = if more || omitted > 0 {
        "partial"
    } else {
        "complete"
    };
    entry.put("attachments", Json::Array(out));
    entry.put("attachments-omitted", json!(omitted));
    entry.put("attachments-state", Json::String(state.into()));
}

/// One Outlook message (`get-message`). `None` when it is not an object with
/// an exact Outlook id.
fn project_outlook_message(body: &Json) -> Option<Json> {
    let mut entry = Entry::new(outlook_id(body.get("id"))?);
    outlook_headers(body, &mut entry);
    exact_field(
        &mut entry,
        "parent-folder-id",
        body.get("parentFolderId"),
        is_outlook_id,
    );
    exact_field(&mut entry, "change-key", body.get("changeKey"), change_key);
    for (key, source) in [("text", "body"), ("unique-text", "uniqueBody")] {
        if let Some((text, cut)) = mail_body_text(body.get(source)) {
            entry.put(key, Json::String(text));
            if cut {
                entry.truncated.push(key);
            }
        }
    }
    mail_attachments(body, &mut entry);
    Some(entry.finish())
}

/// Message text kept per message, in chars.
const MAX_MESSAGE_TEXT: usize = 4096;
/// Attachments kept per message; the rest are counted.
const MAX_ATTACHMENTS: usize = 20;
/// Replies kept per channel message; the rest are counted.
const MAX_REPLIES: usize = 50;

/// Elements whose whole subtree contributes no text.
const HTML_SKIPPED: &[&str] = &["script", "style", "template", "noscript", "head", "title"];
/// Elements that start and end a line.
const HTML_BLOCKS: &[&str] = &[
    "p",
    "div",
    "li",
    "tr",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "blockquote",
    "pre",
];

/// A Teams message body (HTML) as plain text: the text nodes in document
/// order (decoded once, by the parser), whitespace runs collapsed, `br` and
/// block boundaries as single newlines, `script`/`style`/… subtrees skipped.
/// The result is text to render as text: a decoded `<b>` is literal.
fn html_text(html: &str) -> String {
    let document = scraper::Html::parse_fragment(html);
    let mut out = String::new();
    let mut stack = vec![(document.tree.root(), false)];
    while let Some((node, leaving)) = stack.pop() {
        match node.value() {
            scraper::Node::Text(text) => {
                let mut last_space = out.ends_with([' ', '\n']);
                for c in text.chars() {
                    if c.is_whitespace() {
                        if !last_space {
                            out.push(' ');
                            last_space = true;
                        }
                    } else {
                        out.push(c);
                        last_space = false;
                    }
                }
            }
            scraper::Node::Element(element) => {
                let name = element.name();
                if HTML_SKIPPED.contains(&name) {
                    continue;
                }
                let block = HTML_BLOCKS.contains(&name);
                if leaving {
                    if block {
                        out.push('\n');
                    }
                    continue;
                }
                if name == "br" || block {
                    out.push('\n');
                }
                stack.push((node, true));
                let children: Vec<_> = node.children().collect();
                for child in children.into_iter().rev() {
                    stack.push((child, false));
                }
            }
            _ => {
                if !leaving {
                    let children: Vec<_> = node.children().collect();
                    for child in children.into_iter().rev() {
                        stack.push((child, false));
                    }
                }
            }
        }
    }
    out
}

/// Text that keeps its line breaks: runs of whitespace holding a newline
/// become one `\n`, other control characters (but `\t`) are removed, the
/// whole is trimmed and cut to `max` chars. `None` when nothing is left.
fn message_text(text: &str, max: usize) -> Option<(String, bool)> {
    let mut out = String::new();
    let mut pending: Option<char> = None;
    for c in text.chars() {
        if c == '\n' || c == '\r' {
            pending = Some('\n');
        } else if c == ' ' || c == '\t' {
            if pending.is_none() {
                pending = Some(c);
            }
        } else if c.is_control() {
            continue;
        } else {
            if let Some(p) = pending.take()
                && !out.is_empty()
            {
                out.push(p);
            }
            out.push(c);
        }
    }
    if out.is_empty() {
        return None;
    }
    if out.chars().count() <= max {
        return Some((out, false));
    }
    Some((out.chars().take(max).collect(), true))
}

/// Percent-decode once; `None` on a malformed escape.
fn percent_decoded(text: &str) -> Option<String> {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            let hex = std::str::from_utf8(bytes.get(i + 1..i + 3)?).ok()?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).ok()
}

/// An attachment's `contentUrl`, only when it is a plain SharePoint
/// reference: `https`, no userinfo, no port, no query, no fragment, a host of
/// exactly one label before `.sharepoint.com`, and no `hostedContents` in
/// the decoded path. Anything else is withheld — never stripped and returned.
fn attachment_url(raw: &str) -> Option<String> {
    if raw.len() > 2048 || raw.contains(['?', '#']) {
        return None;
    }
    let url = url::Url::parse(raw).ok()?;
    if url.scheme() != "https"
        || !url.username().is_empty()
        || url.password().is_some()
        || url.port().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return None;
    }
    let host = url.host_str()?.to_ascii_lowercase();
    let tenant = host.strip_suffix(".sharepoint.com")?;
    if tenant.is_empty()
        || tenant.len() > 63
        || !tenant
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-')
    {
        return None;
    }
    let path = percent_decoded(url.path())?;
    if path.contains('%') || path.to_ascii_lowercase().contains("hostedcontents") {
        return None;
    }
    Some(url.to_string())
}

/// A message's attachments: references only, never a payload or a fetch.
fn message_attachments(item: &Json) -> (Vec<Json>, usize) {
    let Some(list) = item.get("attachments").and_then(Json::as_array) else {
        return (Vec::new(), 0);
    };
    let mut out = Vec::new();
    let mut omitted = 0usize;
    for attachment in list {
        if out.len() >= MAX_ATTACHMENTS {
            omitted += 1;
            continue;
        }
        let Some(id) = exact_id(attachment.get("id")) else {
            omitted += 1;
            continue;
        };
        let mut entry = Entry::new(id);
        entry.text("name", attachment.get("name"), 1024);
        entry.text("content-type", attachment.get("contentType"), 256);
        match attachment.get("contentUrl").and_then(Json::as_str) {
            Some(raw) if !raw.is_empty() => match attachment_url(raw) {
                Some(url) => entry.put("url", Json::String(url)),
                None => entry.put("url-withheld", Json::Bool(true)),
            },
            _ => {}
        }
        let has_content = match attachment.get("content") {
            None | Some(Json::Null) => false,
            Some(Json::String(text)) => !text.is_empty(),
            Some(_) => true,
        };
        if has_content {
            entry.put("content-withheld", Json::Bool(true));
        }
        out.push(entry.finish());
    }
    (out, omitted)
}

/// The latest of a message's timestamps. `Err` when `createdDateTime` is
/// missing or unparsable, or `lastModifiedDateTime` is present and
/// unparsable — its activity is then unknown.
fn message_activity(item: &Json) -> Result<chrono::DateTime<chrono::Utc>, ()> {
    let parse = |key: &str| -> Result<Option<chrono::DateTime<chrono::Utc>>, ()> {
        match item.get(key) {
            None | Some(Json::Null) => Ok(None),
            Some(Json::String(text)) => chrono::DateTime::parse_from_rfc3339(text)
                .map(|t| Some(t.with_timezone(&chrono::Utc)))
                .map_err(|_| ()),
            Some(_) => Err(()),
        }
    };
    let created = parse("createdDateTime")?.ok_or(())?;
    Ok(match parse("lastModifiedDateTime")? {
        Some(modified) if modified > created => modified,
        _ => created,
    })
}

/// One Teams message (no replies). `None` when its id is malformed.
fn teams_message_entry(item: &Json) -> Option<Entry> {
    let mut entry = Entry::new(exact_id(item.get("id"))?);
    entry.text("created-at", item.get("createdDateTime"), 64);
    entry.text("last-modified-at", item.get("lastModifiedDateTime"), 64);
    entry.text("message-type", item.get("messageType"), 64);
    let from = item.get("from").filter(|f| f.is_object());
    let party = from.and_then(|f| {
        [("user", "user"), ("application", "application")]
            .into_iter()
            .find_map(|(key, kind)| f.get(key).filter(|p| p.is_object()).map(|p| (kind, p)))
    });
    let mut sender = Map::new();
    match party {
        Some((kind, who)) => match optional_id(who.get("id")) {
            Ok(id) => {
                sender.insert("kind".into(), Json::String(kind.into()));
                if let Some(id) = id {
                    sender.insert("id".into(), Json::String(id));
                }
                if let Some((name, cut)) = who
                    .get("displayName")
                    .and_then(Json::as_str)
                    .and_then(|s| capped(s, 256))
                {
                    sender.insert("display-name".into(), Json::String(name));
                    if cut {
                        entry.truncated.push("sender");
                    }
                }
            }
            Err(()) => {
                sender.insert("kind".into(), Json::String("unknown".into()));
                entry.put("sender-withheld", Json::Bool(true));
            }
        },
        None => {
            sender.insert("kind".into(), Json::String("unknown".into()));
        }
    }
    entry.put("sender", Json::Object(sender));
    let deleted = item.get("deletedDateTime").is_some_and(|d| !d.is_null());
    if deleted {
        entry.put("deleted", Json::Bool(true));
        entry.put("text", Json::String(String::new()));
    } else {
        let body = item.get("body");
        let content = body
            .and_then(|b| b.get("content"))
            .and_then(Json::as_str)
            .unwrap_or("");
        let html = body
            .and_then(|b| b.get("contentType"))
            .and_then(Json::as_str)
            .is_some_and(|t| t.eq_ignore_ascii_case("html"));
        let plain = if html {
            html_text(content)
        } else {
            content.to_string()
        };
        let (text, cut) = message_text(&plain, MAX_MESSAGE_TEXT).unwrap_or_default();
        entry.put("text", Json::String(text));
        if cut {
            entry.truncated.push("text");
        }
    }
    let (attachments, attachments_omitted) = message_attachments(item);
    entry.put("attachments", Json::Array(attachments));
    entry.put("attachments-omitted", json!(attachments_omitted));
    Some(entry)
}

/// Teams messages (`teams.channel.list-messages` with `replies`, or
/// `teams.chat.list-messages`). For the channel read a `since` keeps a thread
/// whose activity — the latest timestamp of its root and of every reply
/// received — is strictly after it; a thread whose activity cannot be known
/// (unseen replies, a bad timestamp) is kept with `activity-unknown`, never
/// declared old. `more-available` means only a top-level continuation or a
/// local cut of eligible entries.
fn project_teams_messages(body: &Json, ctx: ProjectionContext, channel: bool) -> Option<Json> {
    let entries = body.get("value")?.as_array()?;
    let mut out = Vec::new();
    let mut omitted = 0usize;
    let mut filtered = 0usize;
    let mut truncated = false;
    for item in entries {
        let Some(mut entry) = teams_message_entry(item) else {
            omitted += 1;
            continue;
        };
        let mut activity = message_activity(item);
        if channel {
            let received = item.get("replies").and_then(Json::as_array);
            let nested_more = item
                .get("replies@odata.nextLink")
                .and_then(Json::as_str)
                .is_some_and(|t| !t.is_empty());
            let mut replies = Vec::new();
            let mut replies_omitted = 0usize;
            let mut replies_cut = false;
            for reply in received.into_iter().flatten() {
                match message_activity(reply) {
                    Ok(at) => {
                        if let Ok(current) = activity
                            && at > current
                        {
                            activity = Ok(at);
                        }
                    }
                    Err(()) => activity = Err(()),
                }
                if replies.len() >= MAX_REPLIES {
                    replies_omitted += 1;
                    replies_cut = true;
                    continue;
                }
                match teams_message_entry(reply) {
                    Some(reply) => replies.push(reply.finish()),
                    None => replies_omitted += 1,
                }
            }
            entry.put("replies", Json::Array(replies));
            entry.put("replies-omitted", json!(replies_omitted));
            entry.put(
                "replies-more-available",
                Json::Bool(nested_more || replies_cut),
            );
            if let Some(since) = ctx.since {
                match activity {
                    Ok(at) if at > since => {}
                    // Unseen replies may be newer: kept, never declared old.
                    Err(()) => entry.put("activity-unknown", Json::Bool(true)),
                    Ok(_) if nested_more => entry.put("activity-unknown", Json::Bool(true)),
                    Ok(_) => {
                        filtered += 1;
                        continue;
                    }
                }
            }
        }
        if out.len() >= ctx.limit {
            truncated = true;
            continue;
        }
        out.push(entry.finish());
    }
    let next = body
        .get("@odata.nextLink")
        .and_then(Json::as_str)
        .is_some_and(|t| !t.is_empty());
    let mut result = Map::new();
    result.insert("messages".into(), Json::Array(out));
    result.insert("more-available".into(), Json::Bool(next || truncated));
    result.insert("omitted".into(), json!(omitted));
    if channel {
        result.insert("filtered-before-since".into(), json!(filtered));
    }
    Some(Json::Object(result))
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
            let mut token = load_slot(
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
            // Leave the outer deadline room to report: each identity read gets a
            // little less than what remains.
            let budget = || {
                remaining(timeout, started).map(|left| {
                    left.saturating_sub(Duration::from_millis(250))
                        .max(Duration::from_millis(100))
                })
            };
            let principal = verify_principal(
                &home,
                op,
                alias_owned.as_deref(),
                &endpoints,
                &mut token,
                &budget,
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
                "binding_id": binding_id(
                    op.integration,
                    &slot,
                    &principal.sub,
                    principal.tenant.as_deref(),
                ),
                "revision": 1,
                "credential_generation": token.generation,
                "principal": {
                    "integration": op.integration,
                    "stable_id": principal.sub,
                    "tenant_id": principal.tenant,
                },
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
    if let Some(cursor) = &wire.cursor
        && cursor.binding != request.binding_id.to_ascii_lowercase()
    {
        return Err(CallFailure::reason(
            "E_CALL_INPUT_INVALID",
            "cursor-context",
        ));
    }
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
        let mut token = load_slot(
            &home_owned,
            op.integration,
            alias.as_deref(),
            &endpoints,
            Some(&expected_generation),
        )?;
        if !token.has_identity_scopes(op) || !token.has_operation_scope(op) {
            return Err(CallFailure::new("E_CALL_SCOPE_MISSING"));
        }
        // The account AND (Microsoft) its tenant, verified again now: a binding
        // made in one directory never matches the same oid in another, and
        // there is no oid-only fallback.
        let budget = || remaining(timeout, started);
        let principal = verify_principal(
            &home_owned,
            op,
            alias.as_deref(),
            &endpoints,
            &mut token,
            &budget,
        )?;
        let slot = account_name(op.integration, alias.as_deref());
        if binding_id(
            op.integration,
            &slot,
            &principal.sub,
            principal.tenant.as_deref(),
        ) != expected_binding
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
        read_data(
            &endpoints,
            op,
            &token,
            &wire,
            left,
            cap,
            Some(&expected_binding),
        )
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

/// `aware agent invoke` running a reviewed operation (#745): the workflow
/// path above in every respect (code-owned request, scope check, replacement
/// boundary, projection, bounds, no redirects) except that `--as` picks the
/// credential slot. Without `--as` it is the manifest's own slot, as for a node.
pub(crate) async fn run_for_invoke(
    home: &Path,
    manifest: &Agent,
    command: &str,
    args: &Json,
    alias: Option<&str>,
) -> Result<Json, crate::error::AwareError> {
    run_reviewed(
        home,
        manifest,
        command,
        args,
        alias,
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
    run_reviewed(home, manifest, command, args, None, endpoints, timeout, cap).await
}

/// The reviewed-operation executor shared by a workflow node and `aware agent
/// invoke`. `alias` is `--as`; `None` keeps the manifest's own slot.
#[allow(clippy::too_many_arguments)]
pub(crate) async fn run_reviewed(
    home: &Path,
    manifest: &Agent,
    command: &str,
    args: &Json,
    alias: Option<&str>,
    endpoints: Option<&Endpoints>,
    timeout: Duration,
    cap: usize,
) -> Result<Json, CallFailure> {
    let started = Instant::now();
    let op = operation(&manifest.agent, command)
        .ok_or_else(|| CallFailure::new("E_CALL_UNSUPPORTED"))?;
    // The manifest must name this operation's integration either way, so `--as`
    // can only choose among that integration's own slots.
    let manifest_alias = manifest_slot(manifest, op, true)?;
    let alias = match alias {
        None => manifest_alias,
        Some(alias) if crate::commands::credential::is_valid_alias(alias) => {
            Some(alias.to_string())
        }
        Some(_) => return Err(CallFailure::field("alias")),
    };
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
        let mut token = load_slot(&home, op.integration, alias.as_deref(), &endpoints, None)?;
        if !token.has_operation_scope(op) || (op.binds_account && !token.has_identity_scopes(op)) {
            return Err(CallFailure::new("E_CALL_SCOPE_MISSING"));
        }
        // Rows whose cursor is bound to an account learn which account this
        // slot holds now; every other row keeps its no-identity path.
        let binding = if op.binds_account {
            let budget = || remaining(timeout, started);
            let principal =
                verify_principal(&home, op, alias.as_deref(), &endpoints, &mut token, &budget)?;
            let slot = account_name(op.integration, alias.as_deref());
            Some(binding_id(
                op.integration,
                &slot,
                &principal.sub,
                principal.tenant.as_deref(),
            ))
        } else {
            None
        };
        let left = remaining(timeout, started)?;
        check_not_replaced(&home, op.integration, alias.as_deref(), &token.generation)?;
        read_data(&endpoints, op, &token, &wire, left, cap, binding.as_deref())
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
