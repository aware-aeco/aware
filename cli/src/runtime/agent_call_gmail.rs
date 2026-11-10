//! Gmail search in the reviewed, account-bound call (`google-workspace
//! gmail.search`): the Gmail counterpart of microsoft-365's
//! `outlook.mail.search`. Plan:
//! `docs/superpowers/specs/2026-11-10-gmail-search-PLAN.md` (§7 is the
//! review revision that governs).
//!
//! A child of `agent_call`, so it shares the operation table, the identity
//! verification, the binding, the bounds and the failure envelope. What only
//! Gmail needs lives here:
//!
//! - **Its own grammar.** `text` and `from` are typed inputs rendered into one
//!   Gmail `q` by code (`gmail-search-words/1`, `gmail-from/1`); `label` is an
//!   opaque id sent as `labelIds`, never inside `q`. Graph's `$search` grammar
//!   and rendering are never used for a Gmail row, and this one never for a
//!   Graph row.
//! - **Two phases, one account, one budget.** `messages.list` names the
//!   matching ids; then one `messages.get` with `format=metadata` and three
//!   named headers per id, in Gmail's order. One captured token and one
//!   verified account serve both phases; the slot is re-proven held before
//!   every request; one decreasing deadline covers them all. A timeout, a
//!   replaced account or any failure but a metadata 404 discards what was read
//!   and fails the call — there is never a partial success. A request already
//!   sent cannot be recalled.
//! - **Provider ids are inputs too.** Every id the list returns must match
//!   [`ID_FORMAT`] before it becomes a URL segment (through the segment
//!   builder, never string formatting); a bad or repeated one is dropped and
//!   counted in `omitted` before any metadata request, and a metadata answer
//!   for another id fails the call.
//! - **No body.** Only Gmail's own snippet is returned as the preview.

use super::*;

/// Gmail's API origin — the only host the token goes to for this read.
pub(super) const ORIGIN: &str = "https://gmail.googleapis.com";
/// The largest page a caller may ask for — and so the most metadata reads.
pub(super) const MAX_PAGE: i64 = 50;
/// Metadata attempts per call, 404s included: never more than a page.
pub(super) const MAX_METADATA_READS: usize = MAX_PAGE as usize;

/// The format id of a `label` input, part of the operation digest.
pub(super) const LABEL_FORMAT: &str = "gmail-label/1";
const WORDS_FORMAT: &str = "gmail-search-words/1";
const FROM_FORMAT: &str = "gmail-from/1";
/// How `q` is rendered: the `from:` term first, then the words, single spaces.
const QUERY_RENDER: &str = "gmail-q/1";
/// The grammar of a message or thread id Gmail returns.
const ID_FORMAT: &str = "gmail-id/1";
const MAX_ID_BYTES: usize = 64;

const MAX_WORDS: usize = 16;
const MAX_WORD_CHARS: usize = 64;
/// Gmail search words that are operators, not text (case-insensitive).
const OPERATOR_WORDS: &[&str] = &["OR", "AND", "AROUND", "NOT"];
/// At least one of these inputs must be given.
const REQUIRED_ANY_OF: &[&str] = &["text", "from", "label"];

/// The most bytes read for the list, and for each metadata answer.
const LIST_CAP: usize = 64 * 1024;
const METADATA_CAP: usize = 64 * 1024;
/// What each metadata read asks for: headers only, three of them, in order.
const METADATA_QUERY: &[(&str, &str)] = &[
    ("format", "metadata"),
    ("metadataHeaders", "From"),
    ("metadataHeaders", "Subject"),
    ("metadataHeaders", "Date"),
];

/// Caps per message, in chars (the `outlook.mail.search` caps).
const MAX_SUBJECT: usize = 1024;
const MAX_ADDRESS: usize = 320;
const MAX_NAME: usize = 256;
const MAX_PREVIEW: usize = 512;
/// Label ids kept per message; the rest are named in `truncated`.
const MAX_LABELS: usize = 100;
/// The largest header value parsed; a longer one is withheld.
const MAX_HEADER_BYTES: usize = 16 * 1024;
/// The web link's base: it opens in the browser session holding the account
/// it names (`authuser=<verified email>`), never `u/0`.
const WEB_URL_BASE: &str = "https://mail.google.com/mail/";

// ── grammars ─────────────────────────────────────────────────────────────────

/// How a `GmailQuery` input becomes terms of `q`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Format {
    /// 1..16 plain words (Unicode letters and digits, `.`, `_`, `-`, `@`),
    /// each at most 64 chars, none starting with `-` or a Gmail operator word.
    Words,
    /// One mail address, rendered `from:<address>`.
    From,
}

impl Format {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Format::Words => WORDS_FORMAT,
            Format::From => FROM_FORMAT,
        }
    }

    /// The `q` terms for a value, or `None` when it is outside the format.
    pub(super) fn terms(self, value: &str) -> Option<Vec<String>> {
        match self {
            Format::Words => {
                let words: Vec<&str> = value.split(' ').filter(|w| !w.is_empty()).collect();
                let ok = !words.is_empty()
                    && words.len() <= MAX_WORDS
                    && words.iter().all(|word| {
                        word.chars().count() <= MAX_WORD_CHARS
                            // `-word` excludes in Gmail; inner hyphens are text.
                            && !word.starts_with('-')
                            && word
                                .chars()
                                .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '-' | '@'))
                            && !OPERATOR_WORDS.iter().any(|op| op.eq_ignore_ascii_case(word))
                    });
                ok.then(|| words.iter().map(|w| (*w).to_string()).collect())
            }
            Format::From => {
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

/// The system labels a search may never name. Spam and Trash are never
/// searched: `includeSpamTrash=false` is fixed, and an explicit label must not
/// be a way around it.
const REFUSED_LABELS: &[&str] = &["SPAM", "TRASH"];

/// `gmail-label/1` as an input: a label id that is not Spam or Trash.
pub(super) fn is_label_input(value: &str) -> bool {
    is_label_id(value)
        && !REFUSED_LABELS
            .iter()
            .any(|label| label.eq_ignore_ascii_case(value))
}

/// `gmail-label/1`: 1..64 of `[A-Za-z0-9_-]`, an opaque id.
pub(super) fn is_label_id(value: &str) -> bool {
    (1..=64).contains(&value.len())
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-'))
}

/// `gmail-id/1`: 1..64 of `[A-Za-z0-9]` — a message or thread id.
fn is_message_id(value: &str) -> bool {
    (1..=MAX_ID_BYTES).contains(&value.len()) && value.bytes().all(|b| b.is_ascii_alphanumeric())
}

/// Render `q` from the typed terms (`from:` first, then the words) and put it
/// first in the query. With only a label, `q` is omitted; with nothing at all,
/// the call is refused before any request.
pub(super) fn render_query(
    query: &mut Vec<(String, String)>,
    terms: &[(Format, Vec<String>)],
) -> Result<(), CallFailure> {
    let of = |format: Format| {
        terms
            .iter()
            .filter(move |(f, _)| *f == format)
            .flat_map(|(_, t)| t.iter().cloned())
    };
    let rendered: Vec<String> = of(Format::From).chain(of(Format::Words)).collect();
    if rendered.is_empty() {
        if query.iter().any(|(name, _)| name == "labelIds") {
            return Ok(());
        }
        return Err(CallFailure::input("text"));
    }
    query.insert(0, ("q".to_string(), rendered.join(" ")));
    Ok(())
}

// ── canonical form ───────────────────────────────────────────────────────────

/// The Google v2 canonical form: everything that decides where the token goes,
/// what each of the two phases sends, how ids and inputs are checked, every
/// bound and the projection.
pub(super) fn canonical(op: &Operation) -> Json {
    let list = Operation::route_json(op.route.segments);
    let mut get = list.clone();
    if let Json::Array(segments) = &mut get {
        segments.push(json!({"render": "provider-id", "format": ID_FORMAT}));
    }
    json!({
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
        "list": {
            "segments": list,
            "query": op.fixed_query.iter().map(|(k, v)| json!([k, v])).collect::<Vec<_>>(),
            "order": ["q", "labelIds", "maxResults", "includeSpamTrash"],
            "cap": LIST_CAP,
            "absentMessages": "empty",
            "moreAvailable": "non-empty-next-page-token-or-cut",
            "nextPageToken": "never-followed",
            "notFound": "fails",
        },
        "get": {
            "segments": get,
            "query": METADATA_QUERY.iter().map(|(k, v)| json!([k, v])).collect::<Vec<_>>(),
            "cap": METADATA_CAP,
            "maxAttempts": MAX_METADATA_READS,
            "attempts": "every-request-counts",
            "notFound": "omitted",
            "otherFailure": "fails",
            "idMismatch": "fails",
        },
        "providerIds": {
            "format": ID_FORMAT,
            "maxBytes": MAX_ID_BYTES,
            "invalid": "omitted",
            "duplicate": "omitted",
        },
        "q": {
            "render": QUERY_RENDER,
            "order": ["from", "text"],
            "separator": " ",
            "whenEmpty": "omitted",
            "words": {
                "format": WORDS_FORMAT,
                "maxWords": MAX_WORDS,
                "maxWordChars": MAX_WORD_CHARS,
                "refused": OPERATOR_WORDS,
                "leadingHyphen": "refused",
            },
            "from": {"format": FROM_FORMAT},
        },
        "label": {"format": LABEL_FORMAT, "param": "labelIds", "refused": REFUSED_LABELS},
        "required": {"anyOf": REQUIRED_ANY_OF},
        "caps": {
            "subject": MAX_SUBJECT,
            "address": MAX_ADDRESS,
            "name": MAX_NAME,
            "preview": MAX_PREVIEW,
            "labels": MAX_LABELS,
            "headerBytes": MAX_HEADER_BYTES,
        },
        "mailHeaders": {
            "parser": "mail-parser/rfc5322-rfc2047",
            "duplicates": "first",
            "from": "first-mailbox-and-count",
            "unparseable": "omitted",
        },
        "receivedAt": "internal-date-ms/1",
        "preview": "snippet-entities-decoded",
        "webUrl": {"base": WEB_URL_BASE, "authuser": "verified-email", "fragment": "all/{id}"},
        "projection": op.projection.id(),
        "limitInput": op.limit_input,
        "inputs": op.inputs_json(),
    })
}

// ── the read ─────────────────────────────────────────────────────────────────

/// `origin` + `segments`, each pushed (and percent-encoded) one by one, never
/// a templated string. The result must have exactly those segments.
fn url_of(origin: &str, segments: &[String]) -> Result<String, CallFailure> {
    let bad = || CallFailure::reason("E_CALL_INPUT_INVALID", "path");
    let mut url = url::Url::parse(origin).map_err(|_| bad())?;
    url.path_segments_mut()
        .map_err(|_| bad())?
        .clear()
        .extend(segments.iter());
    let built = url.path_segments().map(Iterator::count).unwrap_or(0);
    if built != segments.len() || url.query().is_some() || url.fragment().is_some() {
        return Err(bad());
    }
    Ok(url.to_string())
}

/// A failed request, named by its phase. A 3xx is a refused redirect.
fn failed(phase: &'static str, failure: HttpFailure, cap: usize) -> CallFailure {
    let tagged = match failure {
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
    };
    tagged.with("phase", json!(phase))
}

fn shape(phase: &'static str, reason: &'static str) -> CallFailure {
    CallFailure::reason("E_CALL_FAILED", reason).with("phase", json!(phase))
}

/// The answer was larger than the caller's budget.
fn too_large(size: usize, cap: usize) -> CallFailure {
    CallFailure::reason("E_CALL_OUTPUT_TOO_LARGE", "output-too-large")
        .with("outputBytes", json!(size))
        .with("limitBytes", json!(cap))
}

/// The ids one list answer names, in Gmail's order.
#[derive(Debug, PartialEq)]
pub(super) struct Page {
    pub(super) ids: Vec<String>,
    pub(super) omitted: usize,
    pub(super) more: bool,
}

/// `messages.list`'s answer: at most `limit` exact, distinct ids. `None` when
/// the answer is not an object, or `messages` is there and not an array. An
/// absent `messages` is an empty result.
pub(super) fn list_page(body: &Json, limit: usize) -> Option<Page> {
    let body = body.as_object()?;
    let entries: &[Json] = match body.get("messages") {
        None | Some(Json::Null) => &[],
        Some(Json::Array(entries)) => entries,
        Some(_) => return None,
    };
    let limit = limit.min(MAX_METADATA_READS);
    let mut ids: Vec<String> = Vec::new();
    let mut omitted = 0usize;
    let mut cut = false;
    for item in entries {
        if ids.len() >= limit {
            cut = true;
            break;
        }
        match item
            .get("id")
            .and_then(Json::as_str)
            .filter(|id| is_message_id(id))
        {
            Some(id) if !ids.iter().any(|seen| seen == id) => ids.push(id.to_string()),
            _ => omitted += 1,
        }
    }
    let next = body
        .get("nextPageToken")
        .and_then(Json::as_str)
        .is_some_and(|token| !token.is_empty());
    Some(Page {
        ids,
        omitted,
        more: next || cut,
    })
}

/// List, then metadata per id, then the projection. Blocking.
///
/// `still_held` is the caller's slot check (`E_CREDENTIAL_CHANGED` once the
/// slot no longer holds the grant this token came from); it runs immediately
/// before every request, as late as it can be.
#[allow(clippy::too_many_arguments)] // the `read_data` inputs, passed through
pub(super) fn read(
    endpoints: &Endpoints,
    op: &Operation,
    token: &SlotToken,
    wire: &Wire,
    timeout: Duration,
    cap: usize,
    account: Option<VerifiedAccount<'_>>,
    still_held: &dyn Fn() -> Result<(), CallFailure>,
) -> Result<Json, CallFailure> {
    // Every entry point verifies the account first (`binds_account`); without
    // it nothing is read.
    let account = account.ok_or_else(|| CallFailure::new("E_BINDING_CHANGED"))?;
    let started = Instant::now();
    let budget = || remaining(timeout, started);
    let origin = endpoints.data_origin(op);

    let phase = "list";
    let list_url = url_of(origin, &wire.segments)?;
    still_held()?;
    let bytes = get_bounded(
        &list_url,
        &wire.query,
        &[],
        &token.access_token,
        budget()?,
        LIST_CAP,
        &[],
    )
    .map_err(|f| failed(phase, f, LIST_CAP))?;
    let body: Json = serde_json::from_slice(&bytes).map_err(|_| shape(phase, "response-shape"))?;
    let page = list_page(&body, wire.limit).ok_or_else(|| shape(phase, "response-shape"))?;

    let phase = "metadata";
    let query: Vec<(String, String)> = METADATA_QUERY
        .iter()
        .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
        .collect();
    let mut omitted = page.omitted;
    let mut messages = Vec::with_capacity(page.ids.len());
    // Bounded as it grows: past the budget nothing more is asked for.
    let mut size = 0usize;
    for (attempt, id) in page.ids.iter().enumerate() {
        if attempt >= MAX_METADATA_READS {
            return Err(shape(phase, "request-bound"));
        }
        let mut segments = wire.segments.clone();
        segments.push(id.clone());
        let url = url_of(origin, &segments)?;
        still_held()?;
        match get_bounded(
            &url,
            &query,
            &[],
            &token.access_token,
            budget()?,
            METADATA_CAP,
            &[],
        ) {
            Ok(bytes) => {
                let item: Json =
                    serde_json::from_slice(&bytes).map_err(|_| shape(phase, "response-shape"))?;
                if item.get("id").and_then(Json::as_str) != Some(id.as_str()) {
                    return Err(shape(phase, "id-mismatch"));
                }
                let entry = message_entry(&item, id, account.label);
                size += serde_json::to_vec(&entry).map_or(usize::MAX, |b| b.len()) + 1;
                if size > cap {
                    return Err(too_large(size, cap).with("phase", json!(phase)));
                }
                messages.push(entry);
            }
            // Deleted (or otherwise gone) between the two calls.
            Err(HttpFailure::Status(404, _)) => omitted += 1,
            Err(failure) => return Err(failed(phase, failure, METADATA_CAP)),
        }
    }
    let out = json!({
        "messages": messages,
        "more-available": page.more,
        "omitted": omitted,
    });
    let size = serde_json::to_vec(&out).map_or(usize::MAX, |bytes| bytes.len());
    if size > cap {
        return Err(too_large(size, cap));
    }
    Ok(out)
}

// ── projection ───────────────────────────────────────────────────────────────

/// One metadata answer as the call returns it. `id` is the requested (and
/// matched) id; `label` the verified email the web link opens in.
pub(super) fn message_entry(item: &Json, id: &str, label: Option<&str>) -> Json {
    let mut entry = Entry::new(id.to_string());
    match item.get("threadId") {
        None | Some(Json::Null) => {}
        Some(value) => match value.as_str().filter(|t| is_message_id(t)) {
            Some(thread) => entry.put("thread-id", json!(thread)),
            None => entry.truncated.push("thread-id"),
        },
    }
    let headers: &[Json] = item
        .get("payload")
        .and_then(|p| p.get("headers"))
        .and_then(Json::as_array)
        .map_or(&[], Vec::as_slice);
    // Of duplicate headers, the first. An empty value is an absent header, not
    // an unreadable one: there is nothing to name as cut.
    let header = |name: &str| {
        headers
            .iter()
            .find(|h| {
                h.get("name")
                    .and_then(Json::as_str)
                    .is_some_and(|n| n.eq_ignore_ascii_case(name))
            })
            .map(|h| h.get("value").and_then(Json::as_str))
            .filter(|value| !value.is_some_and(|v| v.trim().is_empty()))
    };
    match header("Subject") {
        None => {}
        Some(raw) => match raw.and_then(subject_text) {
            Some(subject) => entry.text("subject", Some(&Json::String(subject)), MAX_SUBJECT),
            None => entry.truncated.push("subject"),
        },
    }
    match header("From") {
        None => {}
        Some(raw) => match raw.and_then(senders) {
            Some((first, count)) => {
                let mut from = Map::new();
                let mut cut = false;
                for (key, value, max) in [
                    ("address", first.address, MAX_ADDRESS),
                    ("name", first.name, MAX_NAME),
                ] {
                    if let Some((text, was_cut)) = value.as_deref().and_then(|s| capped(s, max)) {
                        from.insert(key.into(), Json::String(text));
                        cut |= was_cut;
                    }
                }
                if from.is_empty() {
                    entry.truncated.push("from");
                } else {
                    entry.put("from", Json::Object(from));
                    if cut {
                        entry.truncated.push("from");
                    }
                }
                if count > 1 {
                    entry.put("from-count", json!(count));
                }
            }
            None => entry.truncated.push("from"),
        },
    }
    match item.get("internalDate") {
        None | Some(Json::Null) => {}
        Some(value) => match value.as_str().and_then(received_at) {
            Some(at) => entry.put("received-at", json!(at)),
            None => entry.truncated.push("received-at"),
        },
    }
    if let Some(snippet) = item.get("snippet").and_then(Json::as_str) {
        let text = html_text(snippet);
        entry.text("preview", Some(&Json::String(text)), MAX_PREVIEW);
    }
    match item.get("labelIds") {
        None | Some(Json::Null) => {}
        Some(Json::Array(labels)) => {
            let kept: Vec<&str> = labels
                .iter()
                .filter_map(Json::as_str)
                .filter(|l| is_label_id(l))
                .take(MAX_LABELS)
                .collect();
            if kept.len() != labels.len() {
                entry.truncated.push("label-ids");
            }
            entry.put("label-ids", json!(kept));
        }
        Some(_) => entry.truncated.push("label-ids"),
    }
    if let Some(url) = label.and_then(|email| web_url(email, id)) {
        entry.put("web-url", json!(url));
    }
    entry.finish()
}

/// A header value as one RFC 5322 header line: folds (a line break then
/// whitespace) unfolded; any other line break or control character refuses
/// the value, so it can never become a second header.
fn header_line(name: &str, raw: &str) -> Option<Vec<u8>> {
    if raw.len() > MAX_HEADER_BYTES {
        return None;
    }
    let mut value = String::with_capacity(raw.len());
    let mut chars = raw.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\r' | '\n' => {
                if c == '\r' && chars.peek() == Some(&'\n') {
                    chars.next();
                }
                if !matches!(chars.peek(), Some(' ' | '\t')) {
                    return None;
                }
            }
            c if c.is_control() && c != '\t' => return None,
            c => value.push(c),
        }
    }
    Some(format!("{name}: {value}\r\n\r\n").into_bytes())
}

/// The decoded subject (RFC 2047 encoded words included), or `None` when it
/// cannot be parsed.
fn subject_text(raw: &str) -> Option<String> {
    let line = header_line("Subject", raw)?;
    let message = mail_parser::MessageParser::new()
        .header_text(mail_parser::HeaderName::Subject)
        .parse_headers(&line)?;
    message.subject().map(str::to_string)
}

/// One mailbox of a `From` header, decoded.
#[derive(Debug, PartialEq)]
struct Sender {
    address: Option<String>,
    name: Option<String>,
}

/// The first mailbox of a `From` header and how many it names, or `None` when
/// it names none.
fn senders(raw: &str) -> Option<(Sender, usize)> {
    let line = header_line("From", raw)?;
    let message = mail_parser::MessageParser::new()
        .header_address(mail_parser::HeaderName::From)
        .parse_headers(&line)?;
    let from = message.from()?;
    let mut mailboxes = from
        .iter()
        .filter(|a| a.address.is_some() || a.name.is_some());
    let first = mailboxes.next()?;
    let sender = Sender {
        address: first.address.as_deref().map(str::to_string),
        name: first.name.as_deref().map(str::to_string),
    };
    Some((sender, 1 + mailboxes.count()))
}

/// `internalDate` (milliseconds since the epoch, a decimal int64 string) as an
/// RFC 3339 UTC instant with millisecond precision.
fn received_at(value: &str) -> Option<String> {
    if value.is_empty() || value.len() > 19 || !value.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let ms: i64 = value.parse().ok()?;
    let at = chrono::DateTime::<chrono::Utc>::from_timestamp_millis(ms)?;
    Some(at.to_rfc3339_opts(chrono::SecondsFormat::Millis, true))
}

/// `https://mail.google.com/mail/?authuser=<email>#all/<id>`, the email
/// query-encoded. A convenience link, never an identity.
fn web_url(email: &str, id: &str) -> Option<String> {
    let mut url = url::Url::parse(WEB_URL_BASE).ok()?;
    url.query_pairs_mut().append_pair("authuser", email);
    url.set_fragment(Some(&format!("all/{id}")));
    Some(url.to_string())
}
