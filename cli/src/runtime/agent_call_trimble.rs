//! Trimble Connect in the reviewed, account-bound call (#788): read-only
//! project discovery per region, BCF topics and their comments, and ToDos.
//!
//! A child of `agent_call`, so it shares the operation table, the identity
//! verification, the binding, the bounds and the failure envelope. What only
//! Trimble needs lives here:
//!
//! - **Hosts per region.** Every project lives in one region, and its data is
//!   served only by that region's hosts (`trimble_region.rs`). A call names the
//!   region (`region`, one of [`REGIONS`]); AWARE asks the public
//!   `GET {core}/regions` (no token, no redirect) for that region's `tc-api`
//!   and `topic-api`, and sends the token only to a Core host the Core
//!   allowlist holds or a Topics host the Topics allowlist holds — the two
//!   roles checked apart. Only the origin of a discovered URL is used; the
//!   path is code-owned.
//! - **Phases.** A call that names a project first checks it on its region's
//!   Core host (`GET {core}/tc/api/2.0/projects/{id}`), then makes the one item
//!   request. Every failure carries `details.phase` — `region`, `project` or
//!   `item` — so a consumer can tell "this account cannot open the project"
//!   from "this topic was not found".
//! - **Evidence, never invention.** A projection keeps a field's presence: a
//!   field Trimble did not send is absent, one it sent as `null` is `null`, one
//!   it sent in a shape AWARE does not pass on is named in `withheld`. A record
//!   without a usable id is dropped and counted in `omitted`. Revision fields
//!   (`modified_date`, `modifiedOn`) are passed through only when sent.
//! - **ToDo ranges as data.** `list-todos` returns the HTTP `status` and the
//!   parsed `Content-Range`; a 416 is an answer (the end of the list), not a
//!   failure.

use super::*;

/// The Trimble Connect locations a call may name, as `GET /regions` and every
/// project's `location` spell them (`skills/projects.md#regions`, verified
/// 2026-10-11). Published in the capability and in the digest.
pub(super) const REGIONS: &[&str] = &[
    "northAmerica",
    "europe",
    "unitedKingdom",
    "asia",
    "australia",
];

/// What a Trimble Connect principal's `namespace` says: a Trimble Identity
/// user as Trimble Connect's `/users/me` names it, in no directory.
pub(super) const PRINCIPAL_NAMESPACE: &str = "trimble-connect-user";

/// The most bytes read from Trimble for one item request; the caller's output
/// budget still bounds what it returns.
pub(super) const READ_CAP: usize = 8 * 1024 * 1024;
/// The public `/regions` answer.
const REGIONS_CAP: usize = 64 * 1024;
/// One project object (the project check).
const PROJECT_CAP: usize = 256 * 1024;
/// The largest `skip` a project listing takes.
pub(super) const MAX_PROJECT_SKIP: i64 = 100_000;
/// The widest ToDo page one `range` may ask for.
const MAX_RANGE_SPAN: u64 = 1_000;

/// The Core API path on a regional Core host; the Topics API is at the host's
/// root. A `/regions` URL with any other path is not used.
const CORE_PATH: &[&str] = &["tc", "api", "2.0"];

/// Text kept per long field (a description, a comment), in chars.
const MAX_LONG_TEXT: usize = 65_536;
/// Labels kept per topic; the rest are named in `truncated`.
const MAX_LABELS: usize = 200;
/// Assignees kept per ToDo.
const MAX_ASSIGNEES_KEPT: usize = 200;

/// Which of Trimble's two APIs an operation's item request goes to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Api {
    Core,
    Topics,
}

impl Api {
    fn of(op: &Operation) -> Self {
        match op.projection {
            Projection::TrimbleProjects | Projection::TrimbleTodos => Api::Core,
            _ => Api::Topics,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Api::Core => "core",
            Api::Topics => "topics",
        }
    }
}

/// The statuses an item request answers with as data.
fn accepted_statuses(op: &Operation) -> &'static [u16] {
    match op.projection {
        // A ranged ToDo read: a page, the whole list, an empty list, or the
        // end of the list (416 with `items */total`).
        Projection::TrimbleTodos => &[200, 204, 206, 416],
        Projection::TrimbleTopic => &[200],
        _ => &[200, 204, 206],
    }
}

/// Whether the operation checks its project before the item request.
fn checks_project(op: &Operation) -> bool {
    op.projection != Projection::TrimbleProjects
}

/// The canonical representation the operation digest is taken over: the
/// identity read, how hosts are resolved and which ones may receive the
/// token, the project check, the item route, the inputs and the projection.
pub(super) fn canonical(op: &Operation) -> Json {
    json!({
        "schema": OPERATION_SCHEMA,
        "agent": op.agent,
        "command": op.command,
        "integration": op.integration,
        "provider": op.provider.name(),
        "transport": "rest",
        "effect": "read",
        "method": "GET",
        "origin": op.route.origin,
        "identity": {
            "url": op.provider.identity_url(),
            "query": op.provider.identity_query().iter().map(|(k, v)| json!([k, v])).collect::<Vec<_>>(),
            "subject": "id",
            "tenant": null,
        },
        "identityScopes": op.provider.identity_scopes(),
        "scopes": op.any_of_scopes,
        "requestScopes": op.request_scopes,
        "adminConsent": op.admin_consent,
        "regions": {
            "lookup": ["tc", "api", "2.0", "regions"],
            "token": false,
            "locations": REGIONS,
            "coreHosts": super::super::trimble_region::CORE_HOSTS,
            "corePath": CORE_PATH,
            "topicsHosts": super::super::trimble_bcf::TOPICS_HOSTS,
            "topicsPath": [],
        },
        "api": Api::of(op).as_str(),
        "projectCheck": checks_project(op).then(|| json!({
            "api": "core",
            "segments": ["tc", "api", "2.0", "projects", "{project-id}"],
        })),
        "route": Operation::route_json(op.route.segments),
        "query": op.fixed_query.iter().map(|(k, v)| json!([k, v])).collect::<Vec<_>>(),
        "headers": op.fixed_headers.iter().map(|(k, v)| json!([k, v])).collect::<Vec<_>>(),
        "rangeHeader": (op.projection == Projection::TrimbleTodos).then_some("range"),
        "acceptedStatuses": accepted_statuses(op),
        "redirects": "refused",
        "projection": op.projection.id(),
        "readCap": op.read_cap,
        "limitInput": op.limit_input,
        "inputs": op.inputs_json(),
    })
}

// ── grammars ─────────────────────────────────────────────────────────────────

/// A ToDo `range`: `items=<start>-<end>`, zero-based and inclusive, decimal
/// without a leading zero, at most [`MAX_RANGE_SPAN`] items.
pub(super) fn items_range(value: &str) -> Option<(u64, u64)> {
    let (start, end) = value.strip_prefix("items=")?.split_once('-')?;
    let number = |text: &str| -> Option<u64> {
        let ok = !text.is_empty()
            && text.len() <= 12
            && text.bytes().all(|b| b.is_ascii_digit())
            && (text == "0" || !text.starts_with('0'));
        ok.then(|| text.parse().ok()).flatten()
    };
    let (start, end) = (number(start)?, number(end)?);
    (start <= end && end - start < MAX_RANGE_SPAN).then_some((start, end))
}

/// A `Content-Range` as Trimble sent it: `<unit> <start>-<end>/<total>` or
/// `<unit> */<total>`, the total `*` when unknown. Parsed for its shape only;
/// whether it is consistent with the request is the caller's to judge. `None`
/// when it is not of that shape.
pub(super) fn content_range(text: &str) -> Option<Json> {
    let number = |text: &str| -> Option<u64> {
        (!text.is_empty() && text.len() <= 15 && text.bytes().all(|b| b.is_ascii_digit()))
            .then(|| text.parse().ok())
            .flatten()
            .filter(|n| *n <= (1u64 << 53))
    };
    let (unit, rest) = text.trim().split_once(' ')?;
    if unit.is_empty() || unit.len() > 32 || !unit.bytes().all(|b| b.is_ascii_alphabetic()) {
        return None;
    }
    let (span, total) = rest.trim().split_once('/')?;
    let total = match total {
        "*" => Json::Null,
        n => json!(number(n)?),
    };
    let (start, end) = match span {
        "*" => (Json::Null, Json::Null),
        span => {
            let (s, e) = span.split_once('-')?;
            let (s, e) = (number(s)?, number(e)?);
            if s > e {
                return None;
            }
            (json!(s), json!(e))
        }
    };
    Some(json!({"unit": unit, "start": start, "end": end, "total": total}))
}

// ── bounded HTTP, with the status and headers kept ───────────────────────────

/// One answer: its status, the two headers a projection reads, and (for a
/// 2xx only) its body.
struct Raw {
    status: u16,
    content_range: Option<String>,
    next: Option<String>,
    body: Vec<u8>,
}

/// One bounded GET: no redirects (a 3xx comes back as its status, never
/// followed), a deadline, a body cap. The token is sent only when `bearer` is
/// given. A non-2xx answer's body is never read. Blocking.
fn fetch(
    url: &str,
    query: &[(String, String)],
    headers: &[(String, String)],
    bearer: Option<&str>,
    timeout: Duration,
    cap: usize,
) -> Result<Raw, HttpFailure> {
    use std::io::Read;
    let started = Instant::now();
    let agent = ureq::AgentBuilder::new()
        .redirects(0)
        .resolver(crate::http_body::BoundedDnsResolver::new(
            timeout.min(Duration::from_secs(10)),
        ))
        .timeout(timeout)
        .build();
    let mut request = agent.get(url).set("Accept", "application/json");
    if let Some(bearer) = bearer {
        request = request.set("Authorization", &format!("Bearer {bearer}"));
    }
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
    let header = |name: &str| response.header(name).map(str::to_string);
    let (content_range, next) = (header("Content-Range"), header("next"));
    if !(200..300).contains(&status) {
        return Ok(Raw {
            status,
            content_range,
            next,
            body: Vec::new(),
        });
    }
    let mut body = Vec::new();
    if response
        .into_reader()
        .take(cap as u64 + 1)
        .read_to_end(&mut body)
        .is_err()
    {
        return Err(if timed_out(started) {
            HttpFailure::Timeout
        } else {
            HttpFailure::Transport
        });
    }
    if body.len() > cap {
        return Err(HttpFailure::TooLarge);
    }
    Ok(Raw {
        status,
        content_range,
        next,
        body,
    })
}

/// A failed request, named by its phase. A 3xx is a refused redirect.
fn failed(phase: &'static str, failure: HttpFailure, cap: usize) -> CallFailure {
    let tagged = |f: CallFailure| f.with("phase", json!(phase));
    tagged(match failure {
        HttpFailure::Status(status, _) if (300..400).contains(&status) => {
            CallFailure::reason("E_CALL_FAILED", "redirect-refused").with("status", json!(status))
        }
        HttpFailure::Status(status, _) => {
            CallFailure::new("E_CALL_FAILED").with("status", json!(status))
        }
        HttpFailure::Timeout => CallFailure::new("E_CALL_TIMEOUT"),
        HttpFailure::TooLarge => CallFailure::reason("E_CALL_OUTPUT_TOO_LARGE", "raw-too-large")
            .with("limitBytes", json!(cap)),
        HttpFailure::Transport => CallFailure::reason("E_CALL_FAILED", "transport"),
    })
}

/// A shape failure in `phase`.
fn shape(phase: &'static str, reason: &'static str) -> CallFailure {
    CallFailure::reason("E_CALL_FAILED", reason).with("phase", json!(phase))
}

/// The answer of a request whose status is accepted, else its failure.
fn answer(
    phase: &'static str,
    outcome: Result<Raw, HttpFailure>,
    accepted: &[u16],
    cap: usize,
) -> Result<Raw, CallFailure> {
    let raw = outcome.map_err(|f| failed(phase, f, cap))?;
    if accepted.contains(&raw.status) {
        Ok(raw)
    } else {
        Err(failed(phase, HttpFailure::Status(raw.status, None), cap))
    }
}

// ── hosts ────────────────────────────────────────────────────────────────────

/// The origins one call uses: its region's Core host and, for a BCF read, its
/// Topics host.
struct Hosts {
    core: String,
    topics: Option<String>,
}

/// The origin of a `/regions` URL, when it is a plain `http(s)` URL with no
/// credentials, query or fragment, whose path is exactly `path` (a trailing
/// `/` allowed). Its origin must then be in `allowed`.
fn regional_origin(
    raw: Option<&str>,
    path: &[&str],
    allowed: &[String],
) -> Result<String, CallFailure> {
    let not_allowed =
        || CallFailure::new("E_CALL_ORIGIN_NOT_ALLOWED").with("phase", json!("region"));
    let url = raw
        .and_then(|raw| url::Url::parse(raw).ok())
        .ok_or_else(|| shape("region", "region-shape"))?;
    let segments: Vec<&str> = url
        .path_segments()
        .map(|s| s.filter(|seg| !seg.is_empty()).collect())
        .unwrap_or_default();
    if !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || segments != path
    {
        return Err(not_allowed());
    }
    let origin = crate::manifest::probe::origin_of(url.as_str()).ok_or_else(not_allowed)?;
    if allowed.contains(&origin) {
        Ok(origin)
    } else {
        Err(not_allowed())
    }
}

/// The Core master the identity read is made on (`…/tc/api/2.0`).
fn core_master(endpoints: &Endpoints) -> Result<&str, CallFailure> {
    endpoints
        .identity_url
        .strip_suffix("/users/me")
        .ok_or_else(|| CallFailure::new("E_CALL_ORIGIN_NOT_ALLOWED").with("phase", json!("region")))
}

/// `region`'s hosts, from the public `GET {master}/regions` (sent without the
/// token), each checked against its own allowlist.
fn resolve(
    endpoints: &Endpoints,
    region: &str,
    api: Api,
    budget: &dyn Fn() -> Result<Duration, CallFailure>,
) -> Result<Hosts, CallFailure> {
    let phase = "region";
    let master = core_master(endpoints)?;
    let raw = answer(
        phase,
        fetch(
            &format!("{master}/regions"),
            &[],
            &[],
            None,
            budget().map_err(|f| f.with("phase", json!(phase)))?,
            REGIONS_CAP,
        ),
        &[200],
        REGIONS_CAP,
    )?;
    let regions: Json =
        serde_json::from_slice(&raw.body).map_err(|_| shape(phase, "response-shape"))?;
    let entries = regions
        .as_array()
        .ok_or_else(|| shape(phase, "response-shape"))?;
    let mut matching = entries
        .iter()
        .filter(|r| r.get("location").and_then(Json::as_str) == Some(region));
    let entry = matching
        .next()
        .ok_or_else(|| shape(phase, "region-unknown"))?;
    // Two entries for one location would leave the host ambiguous.
    if matching.next().is_some() {
        return Err(shape(phase, "region-ambiguous"));
    }
    let core = regional_origin(
        entry.get("tc-api").and_then(Json::as_str),
        CORE_PATH,
        &endpoints.allowed_origins,
    )?;
    let topics = match api {
        Api::Core => None,
        Api::Topics => Some(regional_origin(
            entry.get("topic-api").and_then(Json::as_str),
            &[],
            &endpoints.topics_origins,
        )?),
    };
    Ok(Hosts { core, topics })
}

/// `origin` + `segments`, each pushed (and percent-encoded) one by one.
fn url_of(origin: &str, segments: &[String]) -> Result<String, CallFailure> {
    let bad = || CallFailure::reason("E_CALL_INPUT_INVALID", "path");
    let mut url = url::Url::parse(origin).map_err(|_| bad())?;
    url.path_segments_mut()
        .map_err(|_| bad())?
        .clear()
        .extend(segments.iter());
    if url.path_segments().map(Iterator::count).unwrap_or(0) != segments.len() {
        return Err(bad());
    }
    Ok(url.to_string())
}

/// The project check: the project, on its region's Core host. A 404 or 403
/// here means this account cannot open the project in that region.
fn check_project(
    hosts: &Hosts,
    project: &str,
    token: &SlotToken,
    budget: &dyn Fn() -> Result<Duration, CallFailure>,
) -> Result<(), CallFailure> {
    let phase = "project";
    let mut segments: Vec<String> = CORE_PATH.iter().map(|s| (*s).to_string()).collect();
    segments.push("projects".into());
    segments.push(project.to_string());
    let raw = answer(
        phase,
        fetch(
            &url_of(&hosts.core, &segments)?,
            &[],
            &[],
            Some(&token.access_token),
            budget().map_err(|f| f.with("phase", json!(phase)))?,
            PROJECT_CAP,
        ),
        &[200],
        PROJECT_CAP,
    )?;
    let body: Json =
        serde_json::from_slice(&raw.body).map_err(|_| shape(phase, "response-shape"))?;
    // The answer must be the project asked for.
    match body.get("id").and_then(Json::as_str) {
        Some(id) if id == project => Ok(()),
        _ => Err(shape(phase, "project-mismatch")),
    }
}

// ── the read ─────────────────────────────────────────────────────────────────

/// Region, project, item, projection. Blocking.
///
/// `still_held` is the caller's slot check (`E_CREDENTIAL_CHANGED` once the
/// slot no longer holds the grant this token came from). It runs immediately
/// before every request that carries the token, because the region lookup and
/// the project check each leave a window in which the person can disconnect or
/// reconnect; a request already sent cannot be recalled, so the check is as
/// late as it can be.
pub(super) fn read(
    endpoints: &Endpoints,
    op: &Operation,
    token: &SlotToken,
    wire: &Wire,
    timeout: Duration,
    cap: usize,
    still_held: &dyn Fn() -> Result<(), CallFailure>,
) -> Result<Json, CallFailure> {
    let started = Instant::now();
    let budget = || remaining(timeout, started);
    let region = wire
        .value("region")
        .ok_or_else(|| CallFailure::input("region"))?;
    let api = Api::of(op);
    let hosts = resolve(endpoints, region, api, &budget)?;
    if checks_project(op) {
        let project = wire
            .value("project-id")
            .ok_or_else(|| CallFailure::input("project-id"))?;
        still_held()?;
        check_project(&hosts, project, token, &budget)?;
    }
    let phase = "item";
    let origin = match api {
        Api::Core => hosts.core.as_str(),
        Api::Topics => hosts
            .topics
            .as_deref()
            .ok_or_else(|| shape("region", "region-shape"))?,
    };
    let mut headers: Vec<(String, String)> = op
        .fixed_headers
        .iter()
        .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
        .collect();
    if op.projection == Projection::TrimbleTodos
        && let Some(range) = wire.value("range")
    {
        headers.push(("Range".into(), range.to_string()));
    }
    let read_cap = op.read_cap.unwrap_or(cap);
    still_held()?;
    let raw = answer(
        phase,
        fetch(
            &url_of(origin, &wire.segments)?,
            &wire.query,
            &headers,
            Some(&token.access_token),
            budget().map_err(|f| f.with("phase", json!(phase)))?,
            read_cap,
        ),
        accepted_statuses(op),
        read_cap,
    )?;
    let mut projected = match op.projection {
        Projection::TrimbleProjects => project_projects(&raw, wire)?,
        Projection::TrimbleTopics => project_page(&raw, topic_entry)?,
        Projection::TrimbleComments => project_page(&raw, comment_entry)?,
        Projection::TrimbleTopic => project_topic(&raw, wire)?,
        Projection::TrimbleTodos => project_todos(&raw)?,
        _ => return Err(CallFailure::new("E_CALL_UNSUPPORTED")),
    };
    if let Json::Object(map) = &mut projected {
        map.insert("region".into(), json!(region));
    }
    let size = serde_json::to_vec(&projected).map_or(usize::MAX, |bytes| bytes.len());
    if size > cap {
        return Err(
            CallFailure::reason("E_CALL_OUTPUT_TOO_LARGE", "output-too-large")
                .with("outputBytes", json!(size))
                .with("limitBytes", json!(cap))
                .with("phase", json!(phase)),
        );
    }
    Ok(projected)
}

/// A 2xx list body: a JSON array, or nothing (204 / an empty body).
fn list_body(raw: &Raw) -> Result<Vec<Json>, CallFailure> {
    if raw.status == 204 || raw.body.iter().all(u8::is_ascii_whitespace) {
        return Ok(Vec::new());
    }
    match serde_json::from_slice::<Json>(&raw.body) {
        Ok(Json::Array(items)) => Ok(items),
        _ => Err(shape("item", "response-shape")),
    }
}

// ── field projection ─────────────────────────────────────────────────────────

/// One projected record: its fields, the names of text fields cut to size
/// (`truncated`) and of fields Trimble sent in a shape AWARE does not pass on
/// (`withheld`).
struct Record {
    map: Map<String, Json>,
    truncated: Vec<&'static str>,
    withheld: Vec<&'static str>,
}

/// How a field is passed on.
#[derive(Clone, Copy)]
enum Field {
    /// Display text: control characters removed, trimmed, cut to `max` chars.
    /// A string that cleans to nothing stays `""` (cleared), never absent.
    Text(usize),
    /// Long text keeping its line breaks (a description, a comment), cut to
    /// [`MAX_LONG_TEXT`] chars.
    Long,
    /// Kept byte for byte when it is at most `max` bytes with no control
    /// character (a date, a location): never reformatted, never parsed.
    Exact(usize),
    /// An id a caller sends back: [`exact_id`].
    Id,
    /// A list of display strings (`labels`).
    Labels,
    /// One person, or a list of them (`assigned_to`): text or a list of text.
    People,
    /// A whole number 0..=100 (`percentComplete`).
    Percent,
    /// A list of `{id, type?, name?}` (a ToDo's `assignees`).
    Assignees,
}

impl Record {
    fn new() -> Self {
        Self {
            map: Map::new(),
            truncated: Vec::new(),
            withheld: Vec::new(),
        }
    }

    /// `key` from `source`: absent stays absent, `null` stays `null`, a value
    /// AWARE does not pass on is named in `withheld`.
    fn field(&mut self, key: &'static str, source: Option<&Json>, kind: Field) {
        let Some(value) = source else {
            return;
        };
        if value.is_null() {
            self.map.insert(key.into(), Json::Null);
            return;
        }
        match project_field(value, kind) {
            Some((out, cut)) => {
                self.map.insert(key.into(), out);
                if cut {
                    self.truncated.push(key);
                }
            }
            None => self.withheld.push(key),
        }
    }

    fn finish(mut self) -> Json {
        if !self.truncated.is_empty() {
            self.map.insert("truncated".into(), json!(self.truncated));
        }
        if !self.withheld.is_empty() {
            self.map.insert("withheld".into(), json!(self.withheld));
        }
        Json::Object(self.map)
    }
}

/// Text with its line breaks kept (`\n`, `\r`, `\t`), other control
/// characters removed, cut to `max` chars. Never trimmed: what Trimble holds
/// is what is returned.
fn long_text(text: &str, max: usize) -> (String, bool) {
    let kept: String = text
        .chars()
        .filter(|c| !c.is_control() || matches!(c, '\n' | '\r' | '\t'))
        .collect();
    if kept.chars().count() <= max {
        (kept, false)
    } else {
        (kept.chars().take(max).collect(), true)
    }
}

/// A non-null value in `kind`'s shape, and whether it was cut. `None` when it
/// is not in that shape.
fn project_field(value: &Json, kind: Field) -> Option<(Json, bool)> {
    match kind {
        Field::Text(max) => {
            let text = value.as_str()?;
            Some(match capped(text, max) {
                Some((kept, cut)) => (Json::String(kept), cut),
                None => (Json::String(String::new()), false),
            })
        }
        Field::Long => {
            let (kept, cut) = long_text(value.as_str()?, MAX_LONG_TEXT);
            Some((Json::String(kept), cut))
        }
        Field::Exact(max) => value
            .as_str()
            .filter(|s| s.len() <= max && !s.chars().any(char::is_control))
            .map(|s| (Json::String(s.to_string()), false)),
        Field::Id => exact_id(Some(value)).map(|id| (Json::String(id), false)),
        Field::Labels => {
            let items = value.as_array()?;
            let mut out = Vec::new();
            let mut cut = items.len() > MAX_LABELS;
            for item in items.iter().take(MAX_LABELS) {
                let (text, was_cut) = capped(item.as_str()?, 256).unwrap_or_default();
                cut |= was_cut;
                out.push(Json::String(text));
            }
            Some((Json::Array(out), cut))
        }
        Field::People => match value {
            Json::String(text) => Some(match capped(text, 320) {
                Some((kept, cut)) => (Json::String(kept), cut),
                None => (Json::String(String::new()), false),
            }),
            Json::Array(items) => {
                let mut out = Vec::new();
                let mut cut = items.len() > MAX_ASSIGNEES_KEPT;
                for item in items.iter().take(MAX_ASSIGNEES_KEPT) {
                    let (text, was_cut) = capped(item.as_str()?, 320).unwrap_or_default();
                    cut |= was_cut;
                    out.push(Json::String(text));
                }
                Some((Json::Array(out), cut))
            }
            _ => None,
        },
        Field::Percent => value
            .as_u64()
            .filter(|n| *n <= 100)
            .map(|n| (json!(n), false)),
        Field::Assignees => {
            let items = value.as_array()?;
            let mut out = Vec::new();
            let mut cut = items.len() > MAX_ASSIGNEES_KEPT;
            for item in items.iter().take(MAX_ASSIGNEES_KEPT) {
                let item = item.as_object()?;
                let id = exact_id(item.get("id"))?;
                let mut person = Map::new();
                person.insert("id".into(), Json::String(id));
                for (key, max) in [("type", 32), ("name", 256)] {
                    match item.get(key) {
                        None => {}
                        Some(Json::Null) => {
                            person.insert(key.into(), Json::Null);
                        }
                        Some(v) => {
                            let (text, was_cut) = capped(v.as_str()?, max).unwrap_or_default();
                            cut |= was_cut;
                            person.insert(key.into(), Json::String(text));
                        }
                    }
                }
                out.push(Json::Object(person));
            }
            Some((Json::Array(out), cut))
        }
    }
}

/// The fields `list-topics` carries for a topic, and how each is passed on.
const TOPIC_FIELDS: &[(&str, Field)] = &[
    ("title", Field::Text(1024)),
    ("topic_type", Field::Text(256)),
    ("topic_status", Field::Text(256)),
    ("priority", Field::Text(256)),
    ("assigned_to", Field::People),
    ("due_date", Field::Exact(64)),
    ("labels", Field::Labels),
    ("creation_date", Field::Exact(64)),
    ("modified_date", Field::Exact(64)),
];

/// One topic of a listing: its `guid` (required) and every listed field.
fn topic_entry(item: &Json) -> Option<Json> {
    topic_record(item).map(Record::finish)
}

fn topic_record(item: &Json) -> Option<Record> {
    let guid = exact_id(item.get("guid"))?;
    let mut record = Record::new();
    record.map.insert("guid".into(), Json::String(guid));
    for (key, kind) in TOPIC_FIELDS {
        record.field(key, item.get(*key), *kind);
    }
    Some(record)
}

/// One comment: its `guid` (required), text, author and dates, and the links
/// it carries. `modified_date` only when Trimble sends one.
fn comment_entry(item: &Json) -> Option<Json> {
    let guid = exact_id(item.get("guid"))?;
    let mut record = Record::new();
    record.map.insert("guid".into(), Json::String(guid));
    for (key, kind) in [
        ("comment", Field::Long),
        ("author", Field::Text(320)),
        ("date", Field::Exact(64)),
        ("modified_date", Field::Exact(64)),
        ("modified_author", Field::Text(320)),
        ("viewpoint_guid", Field::Id),
        ("reply_to_comment_guid", Field::Id),
    ] {
        record.field(key, item.get(key), kind);
    }
    Some(record.finish())
}

/// One ToDo: its `id` (required), its fields, and the revision fields
/// (`createdOn`, `modifiedOn`) only when Trimble sends them.
fn todo_entry(item: &Json) -> Option<Json> {
    let id = exact_id(item.get("id"))?;
    let mut record = Record::new();
    record.map.insert("id".into(), Json::String(id));
    for (key, kind) in [
        ("label", Field::Text(256)),
        ("title", Field::Text(1024)),
        ("description", Field::Long),
        ("status", Field::Text(64)),
        ("priority", Field::Text(64)),
        ("dueDate", Field::Exact(64)),
        ("percentComplete", Field::Percent),
        ("assignees", Field::Assignees),
        ("createdOn", Field::Exact(64)),
        ("modifiedOn", Field::Exact(64)),
    ] {
        record.field(key, item.get(key), kind);
    }
    Some(record.finish())
}

/// One project of a region's listing: `{id, name, location}`.
fn project_entry(item: &Json) -> Option<Json> {
    let id = exact_id(item.get("id"))?;
    let mut record = Record::new();
    record.map.insert("id".into(), Json::String(id));
    record.field("name", item.get("name"), Field::Text(1024));
    record.field("location", item.get("location"), Field::Exact(64));
    Some(record.finish())
}

/// Every entry of a list, projected; unusable ones counted.
fn entries(items: &[Json], entry: fn(&Json) -> Option<Json>) -> (Vec<Json>, usize) {
    let mut out = Vec::with_capacity(items.len());
    let mut omitted = 0usize;
    for item in items {
        match entry(item) {
            Some(projected) => out.push(projected),
            None => omitted += 1,
        }
    }
    (out, omitted)
}

/// One page of a BCF list (`list-topics`, `list-comments`): every entry, and
/// the `skiptoken` of Trimble's `next` header (never followed). A `next`
/// header carrying no usable token fails the page rather than reading as the
/// last one.
fn project_page(raw: &Raw, entry: fn(&Json) -> Option<Json>) -> Result<Json, CallFailure> {
    let (items, omitted) = entries(&list_body(raw)?, entry);
    let next = match raw.next.as_deref().map(str::trim).filter(|n| !n.is_empty()) {
        None => None,
        Some(link) => Some(
            super::super::trimble_bcf::skiptoken_of(link)
                .filter(|t| IdKind::TrimbleSkiptoken.accepts(t))
                .ok_or_else(|| shape("item", "next-link-unsupported"))?,
        ),
    };
    Ok(json!({
        "items": items,
        "next-skiptoken": next,
        "more-available": next.is_some(),
        "omitted": omitted,
    }))
}

/// One topic (`get-topic`): its `guid` and every field the listing carries,
/// plus `description`. It must be the topic asked for.
fn project_topic(raw: &Raw, wire: &Wire) -> Result<Json, CallFailure> {
    let body: Json =
        serde_json::from_slice(&raw.body).map_err(|_| shape("item", "response-shape"))?;
    let mut record = topic_record(&body).ok_or_else(|| shape("item", "response-shape"))?;
    let asked = wire.value("topic-id").unwrap_or_default();
    let same = record
        .map
        .get("guid")
        .and_then(Json::as_str)
        .is_some_and(|guid| guid.eq_ignore_ascii_case(asked));
    if !same {
        return Err(shape("item", "topic-mismatch"));
    }
    record.field("description", body.get("description"), Field::Long);
    Ok(json!({ "topic": record.finish() }))
}

/// A region's projects: every entry, and the offset of the next page. Core
/// sends no continuation: a page as long as `top` may be followed by more (it
/// may also be the last), so `more-available` is set and `next-skip` offered;
/// a shorter page is the last. `next-skip` counts every entry returned,
/// omitted ones too, so nothing is skipped.
fn project_projects(raw: &Raw, wire: &Wire) -> Result<Json, CallFailure> {
    let items = list_body(raw)?;
    let returned = items.len() as u64;
    let (projects, omitted) = entries(&items, project_entry);
    let number = |key: &str| {
        wire.value(key)
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0)
    };
    let (top, skip) = (number("top"), number("skip"));
    let more = raw.status == 206 || (top > 0 && returned >= top);
    let next = skip + returned;
    let (next_skip, limit_reached) = match more {
        false => (Json::Null, false),
        true if next <= MAX_PROJECT_SKIP as u64 => (json!(next), false),
        true => (Json::Null, true),
    };
    let mut out = json!({
        "projects": projects,
        "more-available": more,
        "next-skip": next_skip,
        "omitted": omitted,
        "regions": REGIONS,
    });
    if limit_reached && let Json::Object(map) = &mut out {
        map.insert("limit-reached".into(), Json::Bool(true));
    }
    Ok(out)
}

/// A ToDo read: the HTTP `status`, the `Content-Range` parsed (`range`, or
/// `null` when none was sent; `content-range-unparsed: true` when one was sent
/// in another shape), and the items. A 416 carries no items: with
/// `items */total` it is the end of the list.
fn project_todos(raw: &Raw) -> Result<Json, CallFailure> {
    let items = match raw.status {
        416 => Vec::new(),
        _ => list_body(raw)?,
    };
    let (todos, omitted) = entries(&items, todo_entry);
    let parsed = raw.content_range.as_deref().map(content_range);
    let mut out = json!({
        "status": raw.status,
        "partial": raw.status == 206,
        "range": parsed.clone().flatten(),
        "items": todos,
        "omitted": omitted,
    });
    if matches!(parsed, Some(None))
        && let Json::Object(map) = &mut out
    {
        map.insert("content-range-unparsed".into(), Json::Bool(true));
    }
    Ok(out)
}
