//! The Gmail Knowledge-source reads (#838): `gmail.labels.list`,
//! `gmail.list-message-ids` and `gmail.get-message`. Plan:
//! `docs/superpowers/specs/2026-11-15-838-gmail-knowledge-reads-PLAN.md` (§4
//! is the review revision that governs).
//!
//! A child of the Gmail module, so every request goes through its [`Session`]
//! (one token, one deadline, the slot re-proven before each) and [`fetch`]
//! (no redirects, a body cap, the rate-limit reason). What only these reads
//! need lives here: completeness that is never guessed (`complete`,
//! `label-ids-complete`, `omitted`, `text-state`, `attachments-state`), the
//! listing's failure reasons, and a deterministic MIME walk that never reads
//! text from an attachment and never decodes, returns or fetches attachment
//! content.

use super::*;
use base64::Engine as _;
use mail_parser::MimeHeaders as _;
use std::collections::HashSet;

/// Raw answer caps, per request.
const LABELS_CAP: usize = 8 * 1024 * 1024;
const IDS_CAP: usize = 256 * 1024;
const MESSAGE_CAP: usize = 8 * 1024 * 1024;
const BODY_CAP: usize = 4 * 1024 * 1024;
const PROBE_CAP: usize = 64 * 1024;

/// Labels returned at most (Gmail's own per-account limit).
const MAX_LABELS_OUT: usize = 10_000;
const MAX_LABEL_NAME: usize = 256;
/// Label ids kept per message; past this `label-ids-complete` is false.
const MAX_MESSAGE_LABELS: usize = 500;
/// Text kept per message, in chars.
const MAX_TEXT: usize = 65_536;
/// Recipients kept per list (`to`, `cc`); the rest are counted.
const MAX_RECIPIENTS: usize = 100;
/// Attachments listed per message; the rest are counted.
const MAX_ATTACHMENTS: usize = 50;
const MAX_ATTACHMENT_NAME: usize = 1024;
const MAX_TYPE: usize = 256;
/// The MIME walk's bounds: nesting depth and parts visited.
const MAX_DEPTH: usize = 16;
const MAX_PARTS: usize = 256;
/// Body sections fetched through `attachments.get`, per message.
const MAX_BODY_FETCHES: usize = 4;
/// A body part's `attachmentId`: 1..2048 of `[A-Za-z0-9_-]`.
const MAX_ATTACHMENT_ID_BYTES: usize = 2048;
/// The label route the listing's failure probe reads.
const LABEL_ROUTE: &[&str] = &["gmail", "v1", "users", "me", "labels"];

// ── canonical forms ──────────────────────────────────────────────────────────

/// What every #838 row's canonical form shares: where the token goes, the
/// identity, the scopes, the budget, the failure rules and the inputs.
fn common(op: &Operation) -> Map<String, Json> {
    let Json::Object(map) = json!({
        "schema": OPERATION_SCHEMA,
        "canonical": "v2",
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
            "subject": "sub",
            "label": "email-when-verified",
        },
        "identityScopes": op.provider.identity_scopes(),
        "scopes": op.any_of_scopes,
        "requestScopes": op.request_scopes,
        "adminConsent": op.admin_consent,
        "bindsAccount": op.binds_account,
        "redirects": "refused",
        "budget": {"deadline": "one-for-all-requests", "partial": "never"},
        "slotCheck": "before-every-request",
        "rateLimit": {
            "statuses": [429],
            "forbiddenReasons": RATE_LIMIT_REASONS,
            "reason": "rate-limited",
            "retryAfter": {"detail": "retry-after-secs", "max": MAX_RETRY_AFTER_SECS},
        },
        "segments": Operation::route_json(op.route.segments),
        "query": op.fixed_query.iter().map(|(k, v)| json!([k, v])).collect::<Vec<_>>(),
        "projection": op.projection.id(),
        "limitInput": op.limit_input,
        "inputs": op.inputs_json(),
    }) else {
        unreachable!("a JSON object literal")
    };
    map
}

fn with(mut map: Map<String, Json>, extra: Json) -> Json {
    if let Json::Object(extra) = extra {
        map.extend(extra);
    }
    Json::Object(map)
}

pub(super) fn labels_canonical(op: &Operation) -> Json {
    with(
        common(op),
        json!({
            "cap": LABELS_CAP,
            "labels": {
                "id": LABEL_FORMAT,
                "invalid": "omitted",
                "duplicate": "omitted",
                "max": MAX_LABELS_OUT,
                "fit": "whole-envelope-within-max-output-bytes",
                "name": {"max": MAX_LABEL_NAME, "cut": "truncated"},
                "type": ["system", "user"],
                "complete": "omitted-is-zero",
            },
        }),
    )
}

pub(super) fn ids_canonical(op: &Operation) -> Json {
    with(
        common(op),
        json!({
            "cap": IDS_CAP,
            "order": ["q", "labelIds", "maxResults", "pageToken", "includeSpamTrash"],
            "q": {
                "order": ["from", "text", "after", "before"],
                "separator": " ",
                "after": "floor-epoch-seconds-minus-1",
                "before": "ceil-epoch-seconds-plus-1",
                "window": "superset-post-filter-on-received-at",
                "required": {"anyOf": ["text", "from", "label"]},
            },
            "label": {"format": LABEL_FORMAT, "param": "labelIds", "refused": ["SPAM", "TRASH"]},
            "providerIds": {"format": ID_FORMAT, "invalid": "omitted", "duplicate": "omitted"},
            "localCut": "never",
            "nextPageToken": {
                "format": PAGE_TOKEN_FORMAT,
                "outsideFormat": "withheld",
                "followed": "never",
            },
            "failures": {
                "400-404": {
                    "probe": {"segments": LABEL_ROUTE, "when": "label-sent", "cap": PROBE_CAP},
                    "reasons": ["label-not-found", "page-token-invalid", "request-invalid"],
                },
            },
        }),
    )
}

pub(super) fn message_canonical(op: &Operation) -> Json {
    with(
        common(op),
        json!({
            "cap": MESSAGE_CAP,
            "notFound": "gone",
            "idMismatch": "fails",
            "labelIds": {"max": MAX_MESSAGE_LABELS, "format": LABEL_FORMAT, "complete": "flagged"},
            "mime": {
                "maxDepth": MAX_DEPTH,
                "maxParts": MAX_PARTS,
                "attachment": ["filename", "content-disposition-attachment", "message/rfc822", "non-text-leaf"],
                "attachmentSubtree": "never-entered",
                "alternative": "first-all-plain-else-first-any",
                "related": "first",
                "signed": "first",
                "mixed": "concatenate",
                "html": "html-text/1",
                "base64": "url-safe-padding-indifferent",
                "charset": "mail-parser",
            },
            "externalBody": {
                "segments": "message/attachments/{attachmentId}",
                "format": "[A-Za-z0-9_-]{1,2048}",
                "cap": BODY_CAP,
                "max": MAX_BODY_FETCHES,
                "notFoundOrTooLarge": "external-not-fetched",
                "otherFailure": "fails",
            },
            "caps": {
                "text": MAX_TEXT,
                "recipients": MAX_RECIPIENTS,
                "attachments": MAX_ATTACHMENTS,
                "attachmentName": MAX_ATTACHMENT_NAME,
                "type": MAX_TYPE,
                "subject": MAX_SUBJECT,
                "address": MAX_ADDRESS,
                "name": MAX_NAME,
                "headerBytes": MAX_HEADER_BYTES,
            },
            "attachmentBytes": "never-decoded-returned-or-fetched",
            "receivedAt": "internal-date-ms/1",
        }),
    )
}

// ── shared ───────────────────────────────────────────────────────────────────

fn parse(phase: &'static str, bytes: &[u8]) -> Result<Json, CallFailure> {
    serde_json::from_slice(bytes).map_err(|_| shape(phase, "response-shape"))
}

/// The answer must fit the caller's budget.
fn bounded(out: Json, cap: usize) -> Result<Json, CallFailure> {
    let size = serde_json::to_vec(&out).map_or(usize::MAX, |bytes| bytes.len());
    if size > cap {
        return Err(too_large(size, cap));
    }
    Ok(out)
}

fn size_of(value: &Json) -> usize {
    serde_json::to_vec(value).map_or(usize::MAX, |bytes| bytes.len())
}

// ── gmail.labels.list ────────────────────────────────────────────────────────

pub(super) fn read_labels(s: &Session<'_>, wire: &Wire, cap: usize) -> Result<Json, CallFailure> {
    let phase = "labels";
    let bytes = s
        .get(&wire.segments, &wire.query, LABELS_CAP)?
        .map_err(|f| failed(phase, f, LABELS_CAP))?;
    let body = parse(phase, &bytes)?;
    project_labels(&body, cap)?.ok_or_else(|| shape(phase, "response-shape"))
}

/// `labels.list`'s answer as the call returns it: labels in Gmail's order
/// while the whole answer fits `cap`; a label that is malformed, repeated or
/// does not fit is counted in `omitted`, and `complete` is `omitted == 0`.
/// `Ok(None)` when the answer is not an object, or `labels` is not an array;
/// `Err` when even an empty answer cannot fit.
pub(in crate::runtime::agent_call) fn project_labels(
    body: &Json,
    cap: usize,
) -> Result<Option<Json>, CallFailure> {
    let Some(body) = body.as_object() else {
        return Ok(None);
    };
    let list: &[Json] = match body.get("labels") {
        None | Some(Json::Null) => &[],
        Some(Json::Array(list)) => list,
        Some(_) => return Ok(None),
    };
    // The envelope at its largest: `false` and every label counted omitted.
    let base = size_of(&json!({"labels": [], "omitted": list.len(), "complete": false}));
    if base > cap {
        return Err(too_large(base, cap));
    }
    let mut size = base;
    let mut seen: HashSet<&str> = HashSet::new();
    let mut labels = Vec::new();
    let mut omitted = 0usize;
    for item in list {
        let Some(id) = item
            .get("id")
            .and_then(Json::as_str)
            .filter(|id| is_label_id(id) && !seen.contains(id))
        else {
            omitted += 1;
            continue;
        };
        seen.insert(id);
        let entry = label_entry(item, id);
        // A comma before every entry but the first: counted for all, so the
        // estimate never falls short.
        let grows = size_of(&entry) + 1;
        if labels.len() >= MAX_LABELS_OUT || size + grows > cap {
            omitted += 1;
            continue;
        }
        size += grows;
        labels.push(entry);
    }
    Ok(Some(json!({
        "labels": labels,
        "omitted": omitted,
        "complete": omitted == 0,
    })))
}

/// One label: its exact id, its name (cut and named in `truncated` past 256
/// chars, named there too when missing), its type when `system` or `user`.
fn label_entry(item: &Json, id: &str) -> Json {
    let mut entry = Entry::new(id.to_string());
    match item
        .get("name")
        .and_then(Json::as_str)
        .and_then(|n| capped(n, MAX_LABEL_NAME))
    {
        Some((name, cut)) => {
            entry.put("name", Json::String(name));
            if cut {
                entry.truncated.push("name");
            }
        }
        None => entry.truncated.push("name"),
    }
    match item.get("type").and_then(Json::as_str) {
        Some(kind @ ("system" | "user")) => entry.put("type", json!(kind)),
        _ => entry.truncated.push("type"),
    }
    entry.finish()
}

// ── gmail.list-message-ids ───────────────────────────────────────────────────

pub(super) fn read_message_ids(
    s: &Session<'_>,
    wire: &Wire,
    cap: usize,
) -> Result<Json, CallFailure> {
    let phase = "list";
    let bytes = match s.get(&wire.segments, &wire.query, IDS_CAP)? {
        Ok(bytes) => bytes,
        Err(Fail::Status {
            status: status @ (400 | 404),
            limited: false,
            ..
        }) => return Err(listing_failure(s, wire, status)?),
        Err(failure) => return Err(failed(phase, failure, IDS_CAP)),
    };
    let body = parse(phase, &bytes)?;
    let out = project_message_ids(&body).ok_or_else(|| shape(phase, "response-shape"))?;
    bounded(out, cap)
}

/// Why a listing answered 400 or 404, as evidence rather than proof: a label
/// that the probe says is gone is `label-not-found`; a page token sent (the
/// label, if any, existing now) is `page-token-invalid` — restart without the
/// token; anything else is `request-invalid`. The outer error is the probe's
/// own failure when it must be reported as such (a rate limit, the deadline,
/// a replaced slot).
fn listing_failure(s: &Session<'_>, wire: &Wire, status: u16) -> Result<CallFailure, CallFailure> {
    let token = wire.value("page-token").is_some();
    let reason = |reason: &'static str| {
        CallFailure::reason("E_CALL_FAILED", reason)
            .with("status", json!(status))
            .with("phase", json!("list"))
    };
    let Some(label) = wire.value("label") else {
        return Ok(reason(if token {
            "page-token-invalid"
        } else {
            "request-invalid"
        }));
    };
    let mut segments: Vec<String> = LABEL_ROUTE.iter().map(|s| (*s).to_string()).collect();
    segments.push(label.to_string());
    Ok(match s.get(&segments, &[], PROBE_CAP)? {
        Err(Fail::Status { status: 404, .. }) => reason("label-not-found"),
        Ok(_) if token => reason("page-token-invalid"),
        Err(failure @ (Fail::Status { limited: true, .. } | Fail::Timeout)) => {
            return Err(failed("label-probe", failure, PROBE_CAP));
        }
        _ => reason("request-invalid"),
    })
}

/// `messages.list`'s answer as the call returns it. Never cut: every entry
/// Gmail sent is kept or counted in `omitted` (a malformed or repeated id), so
/// the next page token never skips a message. A thread id that is missing or
/// malformed is named in that entry's `truncated`. `None` when the answer is
/// not an object, or `messages` is there and not an array.
pub(in crate::runtime::agent_call) fn project_message_ids(body: &Json) -> Option<Json> {
    let body = body.as_object()?;
    let entries: &[Json] = match body.get("messages") {
        None | Some(Json::Null) => &[],
        Some(Json::Array(entries)) => entries,
        Some(_) => return None,
    };
    let mut seen: HashSet<&str> = HashSet::new();
    let mut messages = Vec::with_capacity(entries.len());
    let mut omitted = 0usize;
    for item in entries {
        let Some(id) = item
            .get("id")
            .and_then(Json::as_str)
            .filter(|id| is_message_id(id) && seen.insert(id))
        else {
            omitted += 1;
            continue;
        };
        let mut entry = Entry::new(id.to_string());
        match item
            .get("threadId")
            .and_then(Json::as_str)
            .filter(|t| is_message_id(t))
        {
            Some(thread) => entry.put("thread-id", json!(thread)),
            None => entry.truncated.push("thread-id"),
        }
        messages.push(entry.finish());
    }
    let mut out = Map::new();
    out.insert("messages".into(), Json::Array(messages));
    let more = match body.get("nextPageToken") {
        None | Some(Json::Null) => false,
        Some(Json::String(token)) if token.is_empty() => false,
        Some(Json::String(token)) if is_page_token(token) => {
            out.insert("next-page-token".into(), json!(token));
            true
        }
        // A continuation that cannot be sent back: said, never dropped.
        Some(_) => {
            out.insert("next-page-token-withheld".into(), Json::Bool(true));
            true
        }
    };
    out.insert("more-available".into(), Json::Bool(more));
    out.insert("omitted".into(), json!(omitted));
    Some(Json::Object(out))
}

// ── gmail.get-message ────────────────────────────────────────────────────────

pub(super) fn read_message(s: &Session<'_>, wire: &Wire, cap: usize) -> Result<Json, CallFailure> {
    let phase = "message";
    let id = wire.value("id").ok_or_else(|| CallFailure::input("id"))?;
    let bytes = match s.get(&wire.segments, &wire.query, MESSAGE_CAP)? {
        Ok(bytes) => bytes,
        // Deleted, or never this account's: only the message read says gone.
        Err(Fail::Status { status: 404, .. }) => {
            return Err(CallFailure::reason("E_CALL_FAILED", "gone")
                .with("status", json!(404))
                .with("phase", json!(phase)));
        }
        Err(failure) => return Err(failed(phase, failure, MESSAGE_CAP)),
    };
    let item = parse(phase, &bytes)?;
    if !item.is_object() {
        return Err(shape(phase, "response-shape"));
    }
    if item.get("id").and_then(Json::as_str) != Some(id) {
        return Err(shape(phase, "id-mismatch"));
    }
    let mut fetch_body = |attachment_id: &str| -> Result<Option<String>, CallFailure> {
        if !is_attachment_id(attachment_id) {
            return Ok(None);
        }
        let mut segments = wire.segments.clone();
        segments.push("attachments".to_string());
        segments.push(attachment_id.to_string());
        match s.get(&segments, &[], BODY_CAP)? {
            Ok(bytes) => {
                let body = parse("body", &bytes)?;
                match body.get("data") {
                    Some(Json::String(data)) => Ok(Some(data.clone())),
                    None | Some(Json::Null) => Ok(Some(String::new())),
                    Some(_) => Err(shape("body", "response-shape")),
                }
            }
            // Gone since the message was read, or larger than its cap: that
            // section is not read, and the state says so.
            Err(Fail::Status { status: 404, .. } | Fail::TooLarge) => Ok(None),
            Err(failure) => Err(failed("body", failure, BODY_CAP)),
        }
    };
    let out = project_message(&item, id, &mut fetch_body)?;
    bounded(out, cap)
}

/// `[A-Za-z0-9_-]{1,2048}`: a body part's `attachmentId`, a URL segment.
fn is_attachment_id(value: &str) -> bool {
    (1..=MAX_ATTACHMENT_ID_BYTES).contains(&value.len())
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-'))
}

/// A body fetch: `Ok(Some(data))` the part's base64url data, `Ok(None)` not
/// fetched.
pub(in crate::runtime::agent_call) type FetchBody<'a> =
    dyn FnMut(&str) -> Result<Option<String>, CallFailure> + 'a;

/// One `format=full` message as the call returns it. `id` is the requested
/// (and matched) id; `fetch` reads a body section stored behind an
/// `attachmentId`.
pub(in crate::runtime::agent_call) fn project_message(
    item: &Json,
    id: &str,
    fetch: &mut FetchBody<'_>,
) -> Result<Json, CallFailure> {
    let mut entry = Entry::new(id.to_string());
    match item
        .get("threadId")
        .and_then(Json::as_str)
        .filter(|t| is_message_id(t))
    {
        Some(thread) => entry.put("thread-id", json!(thread)),
        None => entry.truncated.push("thread-id"),
    }
    // Mandatory, and complete unless said otherwise: a caller never reads a
    // missing label as a removed one on incomplete evidence. An absent
    // `labelIds` is Google's JSON for an empty list.
    let (labels, complete) = match item.get("labelIds") {
        None | Some(Json::Null) => (Vec::new(), true),
        Some(Json::Array(list)) => {
            let kept: Vec<&str> = list
                .iter()
                .filter_map(Json::as_str)
                .filter(|l| is_label_id(l))
                .take(MAX_MESSAGE_LABELS)
                .collect();
            let complete = kept.len() == list.len();
            (kept, complete)
        }
        Some(_) => (Vec::new(), false),
    };
    entry.put("label-ids", json!(labels));
    entry.put("label-ids-complete", Json::Bool(complete));
    match item
        .get("internalDate")
        .and_then(Json::as_str)
        .and_then(received_at)
    {
        Some(at) => entry.put("received-at", json!(at)),
        None => entry.truncated.push("received-at"),
    }
    let headers = payload_headers(item);
    put_subject_and_from(&mut entry, headers);
    let mut omitted = 0usize;
    for (key, cc) in [("to", false), ("cc", true)] {
        let mut kept = Vec::new();
        let mut cut = false;
        match header_of(headers, if cc { "Cc" } else { "To" }) {
            None => {}
            Some(raw) => match raw.and_then(|raw| recipients(raw, cc)) {
                Some(list) => {
                    for mailbox in list {
                        if kept.len() >= MAX_RECIPIENTS {
                            omitted += 1;
                            continue;
                        }
                        let mut party = Map::new();
                        for (field, value, max) in [
                            ("address", mailbox.address, MAX_ADDRESS),
                            ("name", mailbox.name, MAX_NAME),
                        ] {
                            if let Some((text, was_cut)) =
                                value.as_deref().and_then(|s| capped(s, max))
                            {
                                party.insert(field.into(), Json::String(text));
                                cut |= was_cut;
                            }
                        }
                        if party.is_empty() {
                            omitted += 1;
                        } else {
                            kept.push(Json::Object(party));
                        }
                    }
                }
                None => cut = true,
            },
        }
        entry.put(key, Json::Array(kept));
        if cut {
            entry.truncated.push(key);
        }
    }
    entry.put("recipients-omitted", json!(omitted));

    let mut walk = Walk::default();
    let sections = match item.get("payload") {
        Some(payload) if payload.is_object() => visit(payload, 0, &mut walk),
        _ => Vec::new(),
    };
    let body = body_text(&sections, walk.cut, fetch)?;
    if let Some((text, source)) = body.text {
        entry.put("text", Json::String(text));
        entry.put("text-source", json!(source));
    }
    entry.put("text-state", json!(body.state));
    if body.cut {
        entry.truncated.push("text");
    }
    entry.put("attachments", Json::Array(walk.attachments));
    entry.put("attachments-omitted", json!(walk.attachments_omitted));
    let partial = walk.cut || walk.attachments_omitted > 0;
    entry.put(
        "attachments-state",
        json!(if partial { "partial" } else { "complete" }),
    );
    if walk.cut {
        entry.truncated.push("parts");
    }
    Ok(entry.finish())
}

// ── the MIME walk ────────────────────────────────────────────────────────────

/// One body section: a `text/plain` or `text/html` leaf that is no attachment.
struct Section<'a> {
    html: bool,
    charset: Option<String>,
    /// The inline base64url data, when Gmail sent it.
    data: Option<&'a str>,
    /// The `attachmentId` the data is stored behind, when it is not inline.
    external: Option<&'a str>,
}

#[derive(Default)]
struct Walk {
    attachments: Vec<Json>,
    attachments_omitted: usize,
    parts: usize,
    /// The walk stopped at the depth or part cap: what it did not reach may
    /// hold text or attachments.
    cut: bool,
}

impl Walk {
    /// List a part as an attachment: its name, type and size, never content.
    fn attach(&mut self, part: &Json, mime: &str) {
        if self.attachments.len() >= MAX_ATTACHMENTS {
            self.attachments_omitted += 1;
            return;
        }
        let mut out = Map::new();
        let mut truncated: Vec<&'static str> = Vec::new();
        for (key, value, max) in [
            (
                "name",
                part.get("filename").and_then(Json::as_str),
                MAX_ATTACHMENT_NAME,
            ),
            ("type", Some(mime), MAX_TYPE),
        ] {
            if let Some((text, cut)) = value.and_then(|v| capped(v, max)) {
                out.insert(key.into(), Json::String(text));
                if cut {
                    truncated.push(key);
                }
            }
        }
        if let Some(size) = part
            .get("body")
            .and_then(|b| b.get("size"))
            .and_then(Json::as_u64)
        {
            out.insert("size".into(), json!(size));
        }
        if !truncated.is_empty() {
            out.insert("truncated".into(), json!(truncated));
        }
        self.attachments.push(Json::Object(out));
    }
}

/// A part's headers (`headers`), or none.
fn part_headers(part: &Json) -> &[Json] {
    part.get("headers")
        .and_then(Json::as_array)
        .map_or(&[], Vec::as_slice)
}

/// Whether a part is an attachment, which is listed and never entered: it has
/// a file name, says `Content-Disposition: attachment`, or is an enclosed
/// message. Any `filename` but the empty string counts — one of only spaces,
/// or a value that is not a string, still names a file — so a doubtful part
/// is pruned rather than read.
fn is_attachment(part: &Json, mime: &str) -> bool {
    let named = part
        .get("filename")
        .is_some_and(|f| f.as_str() != Some("") && !f.is_null());
    let disposed = header_of(part_headers(part), "Content-Disposition")
        .flatten()
        .and_then(|v| v.split(';').next())
        .is_some_and(|d| d.trim().eq_ignore_ascii_case("attachment"));
    named || disposed || mime == "message/rfc822"
}

/// The `charset` parameter of a part's `Content-Type`, as RFC 2045/2231 parse
/// it; `None` when there is none.
fn charset_of(part: &Json) -> Option<String> {
    let raw = header_of(part_headers(part), "Content-Type").flatten()?;
    let line = header_line("Content-Type", raw)?;
    let message = mail_parser::MessageParser::new()
        .header_content_type(mail_parser::HeaderName::ContentType)
        .parse_headers(&line)?;
    message
        .content_type()?
        .attribute("charset")
        .map(str::to_string)
}

/// The body sections a part contributes, in body order, listing every
/// attachment it holds on the way. Deterministic:
/// - an attachment ([`is_attachment`]) contributes nothing and its subtree is
///   never entered;
/// - `text/plain`/`text/html` is one section;
/// - `multipart/alternative` takes its first child whose sections are all
///   `text/plain`, else its first child with any section;
/// - `multipart/related` and `multipart/signed` take their first child (the
///   others are its inline resources or its signature);
/// - `multipart/mixed` and any other multipart concatenate their children;
/// - any other leaf (an inline image, a calendar part) is listed as an
///   attachment: text that a reader cannot render as text.
///
/// Every child is walked whatever is chosen, so every attachment is listed.
fn visit<'a>(part: &'a Json, depth: usize, walk: &mut Walk) -> Vec<Section<'a>> {
    walk.parts += 1;
    if walk.parts > MAX_PARTS || depth > MAX_DEPTH {
        walk.cut = true;
        return Vec::new();
    }
    let mime = part
        .get("mimeType")
        .and_then(Json::as_str)
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    if is_attachment(part, &mime) {
        walk.attach(part, &mime);
        return Vec::new();
    }
    if mime.starts_with("multipart/") {
        let children: &[Json] = part
            .get("parts")
            .and_then(Json::as_array)
            .map_or(&[], Vec::as_slice);
        let mut all: Vec<Vec<Section<'a>>> = children
            .iter()
            .map(|child| visit(child, depth + 1, walk))
            .collect();
        return match mime.as_str() {
            "multipart/alternative" => all
                .iter()
                .position(|s| !s.is_empty() && s.iter().all(|x| !x.html))
                .or_else(|| all.iter().position(|s| !s.is_empty()))
                .map(|i| all.swap_remove(i))
                .unwrap_or_default(),
            "multipart/related" | "multipart/signed" => {
                if all.is_empty() {
                    Vec::new()
                } else {
                    all.swap_remove(0)
                }
            }
            _ => all.into_iter().flatten().collect(),
        };
    }
    if mime == "text/plain" || mime == "text/html" {
        let body = part.get("body");
        return vec![Section {
            html: mime == "text/html",
            charset: charset_of(part),
            data: body.and_then(|b| b.get("data")).and_then(Json::as_str),
            external: body
                .and_then(|b| b.get("attachmentId"))
                .and_then(Json::as_str),
        }];
    }
    walk.attach(part, &mime);
    Vec::new()
}

/// The text of a message's body sections, and how complete it is.
struct BodyText {
    /// The text, and `text/plain`, `text/html` or `mixed`: what it was read
    /// from. `None` when no section was read.
    text: Option<(String, &'static str)>,
    /// The first that applies: `mime-cut`, `external-not-fetched`,
    /// `undecodable`, `truncated`, `none`, `complete`.
    state: &'static str,
    /// The text read was cut at `MAX_TEXT`, whatever `state` reports first:
    /// a cut is never hidden behind a higher-precedence state.
    cut: bool,
}

fn body_text(
    sections: &[Section<'_>],
    walk_cut: bool,
    fetch: &mut FetchBody<'_>,
) -> Result<BodyText, CallFailure> {
    let mut texts: Vec<String> = Vec::new();
    let (mut plain, mut html) = (false, false);
    let (mut not_fetched, mut undecodable) = (false, false);
    let mut fetches = 0usize;
    for section in sections {
        let data: String = match (section.data, section.external) {
            (Some(data), _) => data.to_string(),
            (None, Some(attachment_id)) => {
                if fetches >= MAX_BODY_FETCHES {
                    not_fetched = true;
                    continue;
                }
                fetches += 1;
                match fetch(attachment_id)? {
                    Some(data) => data,
                    None => {
                        not_fetched = true;
                        continue;
                    }
                }
            }
            // An empty part: nothing to read, nothing missing.
            (None, None) => String::new(),
        };
        let Some(text) = decode_base64url(&data)
            .and_then(|bytes| decode_charset(&bytes, section.charset.as_deref()))
        else {
            undecodable = true;
            continue;
        };
        if section.html {
            html = true;
            texts.push(html_text(&text));
        } else {
            plain = true;
            texts.push(text);
        }
    }
    let read = !texts.is_empty();
    let (text, cut) = message_text(&texts.join("\n\n"), MAX_TEXT).unwrap_or_default();
    let source = match (plain, html) {
        (true, false) => "text/plain",
        (false, true) => "text/html",
        _ => "mixed",
    };
    let state = if walk_cut {
        "mime-cut"
    } else if not_fetched {
        "external-not-fetched"
    } else if undecodable {
        "undecodable"
    } else if cut {
        "truncated"
    } else if sections.is_empty() {
        "none"
    } else {
        "complete"
    };
    Ok(BodyText {
        text: read.then_some((text, source)),
        state,
        cut,
    })
}

/// Gmail's base64url body data, with or without padding.
fn decode_base64url(data: &str) -> Option<Vec<u8>> {
    let engine = base64::engine::GeneralPurpose::new(
        &base64::alphabet::URL_SAFE,
        base64::engine::GeneralPurposeConfig::new()
            .with_decode_padding_mode(base64::engine::DecodePaddingMode::Indifferent),
    );
    engine.decode(data.trim()).ok()
}

/// A body part's bytes as text in its declared charset: none, UTF-8 or ASCII
/// read as UTF-8 (an invalid sequence replaced), any other through
/// mail-parser's decoders; `None` for a charset no decoder knows.
fn decode_charset(bytes: &[u8], charset: Option<&str>) -> Option<String> {
    let name = charset.map(|c| c.trim().to_ascii_lowercase());
    match name.as_deref() {
        None | Some("" | "utf-8" | "utf8" | "us-ascii" | "ascii") => {
            Some(String::from_utf8_lossy(bytes).into_owned())
        }
        Some(other) => mail_parser::decoders::charsets::map::charset_decoder(other.as_bytes())
            .map(|decode| decode(bytes)),
    }
}
