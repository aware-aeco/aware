//! `google-workspace gmail.search` (plan
//! `docs/superpowers/specs/2026-11-10-gmail-search-PLAN.md`; §7 "Revision 2"
//! governs). A child of the `list-files` tests, so it shares their Google
//! fixture, homes and request helpers. Every request goes to a local fixture;
//! the production allowlist is used only where a test proves it refuses.

use super::*;

const MESSAGE_A: &str = "18c1a2b3c4d5e6f7";
const MESSAGE_B: &str = "18c1a2b3c4d5e6f8";
const MESSAGE_C: &str = "18c1a2b3c4d5e6f9";
/// A word that must never appear in a failure or a log.
const QUERY_SENTINEL: &str = "zebracrossing";
const SUBJECT_SENTINEL: &str = "must-not-leave-subject";

fn gop() -> &'static Operation {
    operation("google-workspace", "gmail.search").unwrap()
}

/// The default mail grant plus `gmail.search`'s scope.
fn mail_and_readonly() -> String {
    format!(
        "openid https://www.googleapis.com/auth/userinfo.email https://www.googleapis.com/auth/gmail.send {GMAIL_READONLY}"
    )
}

fn path_of(head: &str) -> String {
    head.split_whitespace().nth(1).unwrap_or("").to_string()
}

fn path_only(head: &str) -> String {
    path_of(head).split('?').next().unwrap_or("").to_string()
}

/// The decoded query pairs of a request head, in order.
fn query_of(head: &str) -> Vec<(String, String)> {
    let url = url::Url::parse(&format!("http://x{}", path_of(head))).unwrap();
    url.query_pairs()
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect()
}

const LIST_PATH: &str = "/gmail/v1/users/me/messages";

fn is_list(head: &str) -> bool {
    path_only(head) == LIST_PATH
}

/// The id a metadata read asked for.
fn get_id(head: &str) -> Option<String> {
    path_only(head)
        .strip_prefix("/gmail/v1/users/me/messages/")
        .map(str::to_string)
}

fn list_heads(f: &Fixture) -> Vec<String> {
    f.requests().into_iter().filter(|h| is_list(h)).collect()
}

fn get_heads(f: &Fixture) -> Vec<String> {
    f.requests()
        .into_iter()
        .filter(|h| get_id(h).is_some())
        .collect()
}

fn list_of(ids: &[&str]) -> Json {
    json!({
        "messages": ids.iter().map(|id| json!({"id": id, "threadId": format!("t{id}")})).collect::<Vec<_>>(),
        "resultSizeEstimate": ids.len(),
    })
}

/// A `format=metadata` answer as Gmail sends it.
fn metadata(id: &str) -> Json {
    json!({
        "id": id,
        "threadId": format!("t{id}"),
        "labelIds": ["INBOX", "UNREAD", "Label_12"],
        "snippet": "Please review RFI 042 &amp; reply",
        "sizeEstimate": 4321,
        "historyId": "991",
        "internalDate": "1762761600123",
        "payload": {
            "mimeType": "multipart/alternative",
            "headers": [
                {"name": "From", "value": "Ada Lovelace <ada@acme.com>"},
                {"name": "Subject", "value": "RFI 042"},
                {"name": "Date", "value": "Mon, 10 Nov 2025 08:00:00 +0000"}
            ]
        }
    })
}

type Meta = Arc<dyn Fn(&str) -> Reply + Send + Sync>;

fn ok_meta() -> Meta {
    Arc::new(|id: &str| Reply::json(200, &metadata(id)))
}

/// The Gmail fixture: userinfo answers `SUB` (verified email `EMAIL`), the
/// list answers `list`, each metadata read answers `meta(id)`.
fn gmail(list: Reply, meta: Meta) -> Fixture {
    Fixture::start(Arc::new(move |head: &str| {
        if head.contains(" /v1/userinfo") {
            Reply::json(200, &userinfo(SUB))
        } else if let Some(id) = get_id(head) {
            meta(&id)
        } else {
            list.clone()
        }
    }))
}

fn search_request(home: &Path, inputs: Json) -> Map<String, Json> {
    let mut map = request(home);
    map.insert("command".into(), json!("gmail.search"));
    map.insert("expectedOperationSha256".into(), json!(gop().sha256()));
    set_inputs(&mut map, inputs);
    map
}

async fn search(inputs: Json, list: Reply, meta: Meta) -> (Result<Json, CallFailure>, Fixture) {
    let h = home();
    store(h.path(), None, &mail_and_readonly());
    let f = gmail(list, meta);
    let out = call_with(
        h.path(),
        &parsed(&search_request(h.path(), inputs)),
        &endpoints(&f),
    )
    .await;
    (out, f)
}

fn wire_for(inputs: Json) -> Result<Wire, CallFailure> {
    let Json::Object(map) = inputs else {
        unreachable!()
    };
    wire(gop(), &map)
}

fn pairs(list: &[(&str, &str)]) -> Vec<(String, String)> {
    list.iter()
        .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
        .collect()
}

/// Nothing a failure carries holds the token, a query word or message text.
fn assert_no_sentinel(failure: &CallFailure) {
    let text = format!(
        "{} {} {}",
        failure.code,
        failure.message(),
        Json::Object(failure.details.clone())
    );
    assert_clean(&text);
    assert!(!text.contains(QUERY_SENTINEL), "{text}");
    assert!(!text.contains(SUBJECT_SENTINEL), "{text}");
}

// ── grammar (no network) ─────────────────────────────────────────────────────

#[test]
fn gmail_words_are_plain_text_and_never_a_gmail_operator() {
    let words = gmail::Format::Words;
    assert_eq!(
        words.terms("RFI  042 beam-B12 a.b@c żuraw"),
        Some(vec![
            "RFI".into(),
            "042".into(),
            "beam-B12".into(),
            "a.b@c".into(),
            "żuraw".into()
        ])
    );
    let seventeen = vec!["w"; 17].join(" ");
    let long_word = "w".repeat(65);
    for bad in [
        "",
        "   ",
        "OR",
        "budget or draft",
        "budget AND draft",
        "a AROUND b",
        "around",
        "NOT",
        "not draft",
        "-draft",
        "budget -draft",
        "subject:RFI",
        "a:b",
        "in:spam",
        "RFI*",
        "(RFI)",
        "{RFI}",
        "[RFI]",
        "\"RFI\"",
        "+RFI",
        "a\\b",
        "a#b",
        "a\tb",
        seventeen.as_str(),
        long_word.as_str(),
    ] {
        assert_eq!(words.terms(bad), None, "{bad}");
    }
    assert_eq!(words.as_str(), "gmail-search-words/1");
    let from = gmail::Format::From;
    assert_eq!(
        from.terms("client+rfi@acme.co.uk"),
        Some(vec!["from:client+rfi@acme.co.uk".into()])
    );
    for bad in [
        "",
        "acme.com",
        "@acme.com",
        "a@",
        "a b@acme.com",
        "a@acme..com",
        "a@-acme.com",
        "a\"b@acme.com",
        "a@acme.com OR b@x.com",
        "a@acme.com)",
    ] {
        assert_eq!(from.terms(bad), None, "{bad}");
    }
    assert_eq!(from.as_str(), "gmail-from/1");
}

/// R2-5: the Graph grammar and rendering are not the Gmail ones, either way.
#[test]
fn the_graph_search_grammar_is_byte_for_byte_unchanged_and_separate() {
    // Graph refuses its KQL operators and accepts Gmail's AROUND: two grammars.
    assert!(SearchFormat::Words.terms("AROUND").is_some());
    assert!(SearchFormat::Words.terms("NEAR").is_none());
    assert!(gmail::Format::Words.terms("AROUND").is_none());
    assert!(gmail::Format::Words.terms("NEAR").is_some());
    assert_eq!(SearchFormat::Words.as_str(), "graph-search-words/1");
    assert_eq!(SearchFormat::FromAddress.as_str(), "graph-search-from/1");
    // Outlook's wire is still the quoted `$search`, terms required.
    let outlook = operation("microsoft-365", "outlook.mail.search").unwrap();
    let Json::Object(inputs) = json!({"text": "RFI 042", "from": "a@b.com"}) else {
        unreachable!()
    };
    let w = wire(outlook, &inputs).unwrap();
    assert_eq!(
        w.query,
        pairs(&[
            ("$top", "25"),
            ("$search", "\"RFI 042 from:a@b.com\""),
            ("$select", MAIL_SEARCH_SELECT),
        ])
    );
    let Json::Object(none) = json!({}) else {
        unreachable!()
    };
    assert_eq!(
        wire(outlook, &none).unwrap_err(),
        CallFailure::input("text")
    );
    // And a Gmail row never renders `$search`.
    let g = wire_for(json!({"text": "RFI"})).unwrap();
    assert!(g.query.iter().all(|(k, _)| k != "$search"));
}

#[test]
fn label_ids_are_opaque_and_bounded() {
    let label = IdKind::GmailLabel;
    for good in [
        "INBOX",
        "SENT",
        "STARRED",
        "Label_12",
        "CATEGORY_UPDATES",
        "a-b",
    ] {
        assert!(label.accepts(good), "{good}");
    }
    let long = "L".repeat(65);
    for bad in [
        "",
        "Label 12",
        "Label/12",
        "in:inbox",
        "a.b",
        "..",
        "%41",
        "Ł",
        long.as_str(),
        // Spam and Trash are never searched.
        "SPAM",
        "TRASH",
        "spam",
        "Trash",
    ] {
        assert!(!label.accepts(bad), "{bad}");
    }
    // Only the input refuses them; they are still label ids.
    assert!(gmail::is_label_id("TRASH"));
    assert_eq!(label.as_str(), "gmail-label/1");
}

// ── the rendered request (no network) ────────────────────────────────────────

#[test]
fn the_rendered_query_is_exact_for_each_input_combination() {
    let fixed = ("includeSpamTrash", "false");
    let cases = [
        (
            json!({"text": "RFI 042"}),
            vec![("q", "RFI 042"), ("maxResults", "25"), fixed],
        ),
        (
            json!({"from": "client@acme.com"}),
            vec![("q", "from:client@acme.com"), ("maxResults", "25"), fixed],
        ),
        // Only a label: `q` is omitted, never sent empty.
        (
            json!({"label": "Label_12", "page-size": 5}),
            vec![("labelIds", "Label_12"), ("maxResults", "5"), fixed],
        ),
        (
            json!({"text": "RFI  042", "from": "client@acme.com", "label": "INBOX", "page-size": 50}),
            vec![
                ("q", "from:client@acme.com RFI 042"),
                ("labelIds", "INBOX"),
                ("maxResults", "50"),
                fixed,
            ],
        ),
    ];
    for (inputs, expected) in cases {
        let w = wire_for(inputs.clone()).unwrap();
        assert_eq!(w.query, pairs(&expected), "{inputs}");
        assert_eq!(
            w.segments,
            ["gmail", "v1", "users", "me", "messages"],
            "{inputs}"
        );
        assert!(w.headers.is_empty());
    }
}

// ── refusals send nothing ────────────────────────────────────────────────────

#[tokio::test]
async fn every_refused_input_sends_no_request_at_all() {
    let seventeen = vec!["w"; 17].join(" ");
    let cases = [
        (json!({}), "text"),
        (json!({"page-size": 5}), "text"),
        (json!({"text": "OR"}), "text"),
        (json!({"text": "-x"}), "text"),
        (json!({"text": "a:b"}), "text"),
        (json!({"text": "\"x\""}), "text"),
        (json!({"text": "(x)"}), "text"),
        (json!({"text": "{x}"}), "text"),
        (json!({"text": "x*"}), "text"),
        (json!({"text": "a AROUND b"}), "text"),
        (json!({"text": seventeen}), "text"),
        (json!({"text": ""}), "text"),
        (json!({"from": "not-an-address"}), "from"),
        (json!({"label": "in:spam"}), "label"),
        (json!({"label": ""}), "label"),
        (json!({"label": "SPAM"}), "label"),
        (json!({"text": "RFI", "label": "TRASH"}), "label"),
        (json!({"text": "RFI", "page-size": 0}), "page-size"),
        (json!({"text": "RFI", "page-size": 51}), "page-size"),
        (json!({"text": "RFI", "page-size": "5"}), "page-size"),
        (json!({"text": 5}), "text"),
    ];
    for (inputs, input) in cases {
        let (out, f) = search(inputs.clone(), Reply::json(200, &list_of(&[])), ok_meta()).await;
        assert_eq!(out.unwrap_err(), CallFailure::input(input), "{inputs}");
        assert!(f.requests().is_empty(), "{inputs}");
    }
    let (out, f) = search(
        json!({"query": "from:x@y.com"}),
        Reply::json(200, &list_of(&[])),
        ok_meta(),
    )
    .await;
    assert_eq!(
        out.unwrap_err(),
        CallFailure::reason("E_CALL_INPUT_INVALID", "unknown-input")
    );
    assert!(f.requests().is_empty());
}

#[tokio::test]
async fn a_slot_without_gmail_readonly_is_refused_before_any_request() {
    for grant in [
        // The default mail grant, and one connected for Drive only.
        "openid https://www.googleapis.com/auth/userinfo.email https://www.googleapis.com/auth/gmail.send".to_string(),
        mail_and_drive(),
        format!("{} https://www.googleapis.com/auth/gmail.metadata", mail_and_drive()),
    ] {
        let h = home();
        store(h.path(), None, &grant);
        let f = gmail(Reply::json(200, &list_of(&[MESSAGE_A])), ok_meta());
        let failure = call_with(
            h.path(),
            &parsed(&search_request(h.path(), json!({"text": "RFI"}))),
            &endpoints(&f),
        )
        .await
        .unwrap_err();
        assert_eq!(failure, CallFailure::new("E_CALL_SCOPE_MISSING"), "{grant}");
        assert!(f.requests().is_empty(), "{grant}");
        // The capability names exactly what to reconnect with.
        let cap = capabilities_with(h.path(), gop(), None, Duration::from_secs(10), &endpoints(&f))
            .await
            .unwrap();
        assert_eq!(cap["unavailableCode"], json!("scope-missing"));
        let reason = cap["unavailableReason"].as_str().unwrap();
        assert!(reason.contains("Gmail read access"), "{reason}");
        assert!(
            reason.contains("--oauth --scopes openid,https://www.googleapis.com/auth/userinfo.email,https://www.googleapis.com/auth/gmail.readonly"),
            "{reason}"
        );
        // The capability read is identity only: no Gmail request.
        assert!(list_heads(&f).is_empty() && get_heads(&f).is_empty());
    }
}

/// Drive and Gmail read scopes stand in for each other nowhere.
#[tokio::test]
async fn gmail_readonly_never_lists_drive_and_drive_never_searches_gmail() {
    let h = home();
    store(
        h.path(),
        None,
        "openid https://www.googleapis.com/auth/userinfo.email https://www.googleapis.com/auth/gmail.send https://www.googleapis.com/auth/gmail.readonly",
    );
    let f = google(SUB, Reply::json(200, &files_body()));
    let failure = call_with(h.path(), &parsed(&request(h.path())), &endpoints(&f))
        .await
        .unwrap_err();
    assert_eq!(failure, CallFailure::new("E_CALL_SCOPE_MISSING"));
    assert!(f.requests().is_empty());
    assert_eq!(
        op().any_of_scopes,
        [DRIVE_READONLY, DRIVE_METADATA_READONLY]
    );
    assert_eq!(gop().any_of_scopes, [GMAIL_READONLY]);
}

/// The regression matrix: `list-files` on a slot holding send, readonly and
/// both Drive scopes still lists.
#[tokio::test]
async fn list_files_still_lists_on_the_combined_grant() {
    let h = home();
    store(
        h.path(),
        None,
        &format!("{} {DRIVE_READONLY} {GMAIL_READONLY}", mail_and_drive()),
    );
    let f = google(SUB, Reply::json(200, &files_body()));
    let record = call_with(h.path(), &parsed(&request(h.path())), &endpoints(&f))
        .await
        .unwrap();
    assert_eq!(
        record["result"]["body"]["files"].as_array().unwrap().len(),
        2
    );
}

// ── the two phases ───────────────────────────────────────────────────────────

#[tokio::test]
async fn a_search_lists_once_reads_each_message_once_and_returns_only_the_projection() {
    let mut list = list_of(&[MESSAGE_A, MESSAGE_B, MESSAGE_C]);
    list["nextPageToken"] = json!("next-page-token-never-followed");
    let (out, f) = search(
        json!({"text": "RFI 042", "from": "ada@acme.com", "page-size": 3}),
        Reply::json(200, &list),
        ok_meta(),
    )
    .await;
    let record = out.unwrap();
    let body = &record["result"]["body"];
    // One identity read, one list, exactly three metadata reads, in order.
    assert_eq!(f.count("/v1/userinfo"), 1);
    let lists = list_heads(&f);
    assert_eq!(lists.len(), 1);
    assert_eq!(
        query_of(&lists[0]),
        pairs(&[
            ("q", "from:ada@acme.com RFI 042"),
            ("maxResults", "3"),
            ("includeSpamTrash", "false"),
        ])
    );
    let gets = get_heads(&f);
    assert_eq!(
        gets.iter().filter_map(|h| get_id(h)).collect::<Vec<_>>(),
        [MESSAGE_A, MESSAGE_B, MESSAGE_C]
    );
    for head in &gets {
        assert_eq!(
            query_of(head),
            pairs(&[
                ("format", "metadata"),
                ("metadataHeaders", "From"),
                ("metadataHeaders", "Subject"),
                ("metadataHeaders", "Date"),
            ])
        );
    }
    // Every request carried the one captured token.
    for head in f.requests() {
        assert!(head.contains(&format!("Bearer {TOKEN}")), "{head}");
    }
    // The token is never followed.
    assert!(
        f.requests()
            .iter()
            .all(|h| !h.contains("pageToken") && !h.contains("next-page-token"))
    );
    assert_eq!(body["more-available"], json!(true));
    assert_eq!(body["omitted"], json!(0));
    let messages = body["messages"].as_array().unwrap();
    assert_eq!(messages.len(), 3);
    assert_eq!(
        messages[0],
        json!({
            "id": MESSAGE_A,
            "thread-id": format!("t{MESSAGE_A}"),
            "subject": "RFI 042",
            "from": {"address": "ada@acme.com", "name": "Ada Lovelace"},
            "received-at": "2025-11-10T08:00:00.123Z",
            "preview": "Please review RFI 042 & reply",
            "label-ids": ["INBOX", "UNREAD", "Label_12"],
            "web-url": format!("https://mail.google.com/mail/?authuser=ada%40example.com#all/{MESSAGE_A}"),
        })
    );
    let text = body.to_string();
    assert!(!text.contains("sizeEstimate") && !text.contains("historyId"));
    assert!(!text.contains("payload") && !text.contains("Mon, 10 Nov"));
    assert!(!text.contains("u/0"));
    assert_clean(&record.to_string());
}

#[tokio::test]
async fn an_absent_messages_list_is_an_empty_result_and_reads_nothing_more() {
    let (out, f) = search(
        json!({"label": "Label_12"}),
        Reply::json(200, &json!({"resultSizeEstimate": 0})),
        ok_meta(),
    )
    .await;
    let body = out.unwrap()["result"]["body"].clone();
    assert_eq!(
        body,
        json!({"messages": [], "more-available": false, "omitted": 0})
    );
    assert!(get_heads(&f).is_empty());
    // An empty `nextPageToken` is no continuation (R2-11).
    let (out, _) = search(
        json!({"label": "Label_12"}),
        Reply::json(200, &json!({"messages": [], "nextPageToken": ""})),
        ok_meta(),
    )
    .await;
    assert_eq!(
        out.unwrap()["result"]["body"]["more-available"],
        json!(false)
    );
    // `messages` that is not a list is not an empty result.
    let (out, _) = search(
        json!({"label": "Label_12"}),
        Reply::json(200, &json!({"messages": {"id": MESSAGE_A}})),
        ok_meta(),
    )
    .await;
    let failure = out.unwrap_err();
    assert_eq!(failure.code, "E_CALL_FAILED");
    assert_eq!(failure.details["reason"], json!("response-shape"));
    assert_eq!(failure.details["phase"], json!("list"));
}

/// R2-2: provider ids are inputs too.
#[tokio::test]
async fn bad_and_repeated_ids_are_dropped_before_any_metadata_request() {
    let long = "a".repeat(65);
    let list = json!({"messages": [
        {"id": MESSAGE_A},
        {"id": MESSAGE_A},
        {"id": "../../drive/v3/files"},
        {"id": "a%2Fb"},
        {"id": "a b"},
        {"id": ""},
        {"id": long},
        {"id": 7},
        {"threadId": "x"},
        {"id": MESSAGE_B},
    ]});
    let (out, f) = search(json!({"text": "RFI"}), Reply::json(200, &list), ok_meta()).await;
    let body = out.unwrap()["result"]["body"].clone();
    assert_eq!(
        get_heads(&f)
            .iter()
            .filter_map(|h| get_id(h))
            .collect::<Vec<_>>(),
        [MESSAGE_A, MESSAGE_B]
    );
    assert_eq!(body["omitted"], json!(8));
    assert_eq!(body["messages"].as_array().unwrap().len(), 2);
    // Only Gmail's message route was ever requested.
    for head in f.requests() {
        let path = path_only(&head);
        assert!(
            path == "/v1/userinfo" || path == LIST_PATH || get_id(&head).is_some(),
            "{path}"
        );
    }
}

#[test]
fn the_list_page_bounds_ids_and_names_a_local_cut() {
    let ids: Vec<String> = (0..60).map(|i| format!("m{i}")).collect();
    let body = json!({"messages": ids.iter().map(|id| json!({"id": id})).collect::<Vec<_>>()});
    let page = gmail::list_page(&body, 3).unwrap();
    assert_eq!(page.ids, ["m0", "m1", "m2"]);
    assert!(page.more, "cut locally");
    // Never more than the request-count bound, whatever the limit says.
    let page = gmail::list_page(&body, 1000).unwrap();
    assert_eq!(page.ids.len(), gmail::MAX_METADATA_READS);
    assert_eq!(gmail::MAX_METADATA_READS, 50);
    assert_eq!(gmail::MAX_PAGE, 50);
    let page = gmail::list_page(&json!({"messages": [{"id": "m1"}]}), 3).unwrap();
    assert!(!page.more);
    assert!(gmail::list_page(&json!([]), 3).is_none());
}

#[tokio::test]
async fn a_message_gone_between_the_calls_is_omitted_and_the_attempt_counts() {
    let meta: Meta = Arc::new(|id: &str| {
        if id == MESSAGE_B {
            Reply::json(404, &json!({"error": {"message": SUBJECT_SENTINEL}}))
        } else {
            Reply::json(200, &metadata(id))
        }
    });
    let (out, f) = search(
        json!({"text": "RFI", "page-size": 3}),
        Reply::json(200, &list_of(&[MESSAGE_A, MESSAGE_B, MESSAGE_C])),
        meta,
    )
    .await;
    let body = out.unwrap()["result"]["body"].clone();
    assert_eq!(body["omitted"], json!(1));
    assert_eq!(
        body["messages"]
            .as_array()
            .unwrap()
            .iter()
            .map(|m| m["id"].as_str().unwrap())
            .collect::<Vec<_>>(),
        [MESSAGE_A, MESSAGE_C]
    );
    assert_eq!(get_heads(&f).len(), 3, "the 404 was one attempt");
    assert_eq!(body["more-available"], json!(false));
}

#[tokio::test]
async fn any_other_metadata_failure_fails_the_call_with_its_status_only() {
    for status in [500u16, 403, 429, 401] {
        let meta: Meta = Arc::new(move |id: &str| {
            if id == MESSAGE_B {
                Reply::json(status, &json!({"error": {"message": SUBJECT_SENTINEL}}))
            } else {
                Reply::json(200, &metadata(id))
            }
        });
        let (out, f) = search(
            json!({"text": QUERY_SENTINEL}),
            Reply::json(200, &list_of(&[MESSAGE_A, MESSAGE_B, MESSAGE_C])),
            meta,
        )
        .await;
        let failure = out.unwrap_err();
        assert_eq!(failure.code, "E_CALL_FAILED", "{status}");
        assert_eq!(failure.details["status"], json!(status));
        assert_eq!(failure.details["phase"], json!("metadata"));
        assert_no_sentinel(&failure);
        // Nothing after the failure.
        assert_eq!(get_heads(&f).len(), 2, "{status}");
    }
}

#[tokio::test]
async fn a_list_failure_fails_the_call_and_reads_no_message() {
    for status in [404u16, 400, 500] {
        let (out, f) = search(
            json!({"text": QUERY_SENTINEL}),
            Reply::json(status, &json!({"error": {"message": QUERY_SENTINEL}})),
            ok_meta(),
        )
        .await;
        let failure = out.unwrap_err();
        assert_eq!(failure.code, "E_CALL_FAILED");
        assert_eq!(failure.details["status"], json!(status));
        assert_eq!(failure.details["phase"], json!("list"));
        assert_no_sentinel(&failure);
        assert!(get_heads(&f).is_empty());
    }
}

#[tokio::test]
async fn a_metadata_answer_for_another_message_fails_the_call() {
    for answer in [
        json!({"id": MESSAGE_C, "snippet": SUBJECT_SENTINEL}),
        json!({"snippet": SUBJECT_SENTINEL}),
        json!({"id": format!("{MESSAGE_B} ")}),
    ] {
        let meta: Meta = {
            let answer = answer.clone();
            Arc::new(move |id: &str| {
                if id == MESSAGE_B {
                    Reply::json(200, &answer)
                } else {
                    Reply::json(200, &metadata(id))
                }
            })
        };
        let (out, f) = search(
            json!({"text": "RFI"}),
            Reply::json(200, &list_of(&[MESSAGE_A, MESSAGE_B, MESSAGE_C])),
            meta,
        )
        .await;
        let failure = out.unwrap_err();
        assert_eq!(
            failure,
            CallFailure::reason("E_CALL_FAILED", "id-mismatch").with("phase", json!("metadata")),
            "{answer}"
        );
        assert_no_sentinel(&failure);
        assert_eq!(get_heads(&f).len(), 2);
    }
}

// ── one budget (R2-3) ────────────────────────────────────────────────────────

#[tokio::test]
async fn one_deadline_covers_every_request_and_returns_nothing_partial() {
    let slow: Meta = Arc::new(|id: &str| Reply {
        delay: Duration::from_millis(400),
        ..Reply::json(200, &metadata(id))
    });
    let h = home();
    store(h.path(), None, &mail_and_readonly());
    let ids = ["m1", "m2", "m3", "m4", "m5", "m6"];
    let f = gmail(Reply::json(200, &list_of(&ids)), slow);
    let mut req = search_request(h.path(), json!({"text": "RFI"}));
    req.insert("timeoutMs".into(), json!(1000));
    let started = Instant::now();
    let failure = call_with(h.path(), &parsed(&req), &endpoints(&f))
        .await
        .unwrap_err();
    assert_eq!(failure.code, "E_CALL_TIMEOUT");
    assert!(started.elapsed() < Duration::from_millis(1800));
    // Each answer alone was within the deadline; their sum was not. The read
    // itself stops at the deadline: once the call has failed, no further
    // metadata request is sent (each would have fit a fresh per-request
    // timeout, so only the one shared budget stops them).
    std::thread::sleep(Duration::from_millis(2500));
    let sent = get_heads(&f).len();
    assert!(
        sent < ids.len(),
        "{sent} metadata requests after the deadline"
    );
    assert!(sent <= 3, "{sent}");
}

#[tokio::test]
async fn the_output_budget_bounds_what_accumulates_and_stops_the_reads() {
    let wordy: Meta = Arc::new(|id: &str| {
        let mut m = metadata(id);
        m["payload"]["headers"][1]["value"] = json!("s".repeat(400));
        Reply::json(200, &m)
    });
    let ids: Vec<String> = (0..10).map(|i| format!("m{i}")).collect();
    let id_refs: Vec<&str> = ids.iter().map(String::as_str).collect();
    let h = home();
    store(h.path(), None, &mail_and_readonly());
    let f = gmail(Reply::json(200, &list_of(&id_refs)), wordy);
    let mut req = search_request(h.path(), json!({"text": "RFI"}));
    req.insert("maxOutputBytes".into(), json!(1024));
    let failure = call_with(h.path(), &parsed(&req), &endpoints(&f))
        .await
        .unwrap_err();
    assert_eq!(failure.code, "E_CALL_OUTPUT_TOO_LARGE");
    assert_eq!(failure.details["reason"], json!("output-too-large"));
    assert_eq!(failure.details["limitBytes"], json!(1024));
    // Accumulation stopped at the budget: not every message was asked for.
    assert!(get_heads(&f).len() <= 3, "{}", get_heads(&f).len());
}

#[tokio::test]
async fn a_metadata_answer_over_its_cap_fails_the_call() {
    let huge: Meta = Arc::new(|id: &str| {
        let mut m = metadata(id);
        m["snippet"] = json!("x".repeat(70 * 1024));
        Reply::json(200, &m)
    });
    let (out, _) = search(
        json!({"text": "RFI"}),
        Reply::json(200, &list_of(&[MESSAGE_A])),
        huge,
    )
    .await;
    let failure = out.unwrap_err();
    assert_eq!(failure.code, "E_CALL_OUTPUT_TOO_LARGE");
    assert_eq!(failure.details["reason"], json!("raw-too-large"));
    assert_eq!(failure.details["phase"], json!("metadata"));
}

// ── one account, both phases (R2-9) ──────────────────────────────────────────

fn replace_slot(home: &Path) {
    crate::auth::keychain::store_token(
        &token(
            &mail_and_readonly(),
            Some("99999999-2222-4333-8444-555555555555"),
            "new",
        ),
        None,
        home,
    )
    .unwrap();
}

/// A fixture that reconnects (`Replace`) or disconnects (`Remove`) the slot
/// while answering the `trigger`ing request.
#[derive(Clone, Copy, PartialEq)]
enum Change {
    Replace,
    Remove,
}

#[derive(Clone, Copy, PartialEq)]
enum Trigger {
    Identity,
    List,
    FirstMetadata,
}

fn changing(home: &Path, change: Change, trigger: Trigger) -> Fixture {
    let home = home.to_path_buf();
    let fired = Arc::new(Mutex::new(false));
    Fixture::start(Arc::new(move |head: &str| {
        let now = if head.contains(" /v1/userinfo") {
            Trigger::Identity
        } else if get_id(head).is_some() {
            Trigger::FirstMetadata
        } else {
            Trigger::List
        };
        let mut done = fired.lock().unwrap();
        if now == trigger && !*done {
            *done = true;
            match change {
                Change::Replace => replace_slot(&home),
                Change::Remove => {
                    let _ = crate::auth::keychain::delete_token("google-workspace", None, &home);
                }
            }
        }
        drop(done);
        match now {
            Trigger::Identity => Reply::json(200, &userinfo(SUB)),
            Trigger::FirstMetadata => {
                Reply::json(200, &metadata(&get_id(head).unwrap_or_default()))
            }
            Trigger::List => Reply::json(200, &list_of(&[MESSAGE_A, MESSAGE_B, MESSAGE_C])),
        }
    }))
}

fn workflow_manifest() -> Agent {
    serde_yaml::from_str(&shipped_manifest()).unwrap()
}

#[derive(Clone, Copy, Debug)]
enum Route {
    Call,
    Workflow,
    Invoke,
}

async fn run_on(route: Route, home: &Path, e: &Endpoints) -> Result<Json, CallFailure> {
    let inputs = json!({"text": "RFI"});
    let t = Duration::from_secs(10);
    match route {
        Route::Call => call_with(home, &parsed(&search_request(home, inputs)), e)
            .await
            .map(|record| record["result"]["body"].clone()),
        Route::Workflow => {
            run_for_workflow_with(
                home,
                &workflow_manifest(),
                "gmail.search",
                &inputs,
                Some(e),
                t,
                1 << 20,
            )
            .await
        }
        Route::Invoke => {
            run_reviewed(
                home,
                &workflow_manifest(),
                "gmail.search",
                &inputs,
                None,
                Some(e),
                t,
                1 << 20,
            )
            .await
        }
    }
}

#[tokio::test]
async fn every_route_verifies_the_account_and_builds_the_link_from_it() {
    for route in [Route::Call, Route::Workflow, Route::Invoke] {
        let h = home();
        store(h.path(), None, &mail_and_readonly());
        let f = gmail(Reply::json(200, &list_of(&[MESSAGE_A])), ok_meta());
        let body = run_on(route, h.path(), &endpoints(&f)).await.unwrap();
        assert_eq!(f.count("/v1/userinfo"), 1, "{route:?}");
        assert_eq!(
            body["messages"][0]["web-url"],
            json!(format!(
                "https://mail.google.com/mail/?authuser=ada%40example.com#all/{MESSAGE_A}"
            )),
            "{route:?}"
        );
    }
}

#[tokio::test]
async fn a_changed_slot_stops_every_route_before_the_next_request() {
    for route in [Route::Call, Route::Workflow, Route::Invoke] {
        for change in [Change::Replace, Change::Remove] {
            // Before the list: the identity read is the last request.
            let h = home();
            store(h.path(), None, &mail_and_readonly());
            let f = changing(h.path(), change, Trigger::Identity);
            let failure = run_on(route, h.path(), &endpoints(&f)).await.unwrap_err();
            assert_eq!(
                failure,
                CallFailure::reason("E_CREDENTIAL_CHANGED", "replaced"),
                "{route:?}"
            );
            assert!(list_heads(&f).is_empty(), "{route:?}");
            assert!(get_heads(&f).is_empty(), "{route:?}");

            // During the list: no metadata read.
            let h = home();
            store(h.path(), None, &mail_and_readonly());
            let f = changing(h.path(), change, Trigger::List);
            let failure = run_on(route, h.path(), &endpoints(&f)).await.unwrap_err();
            assert_eq!(failure.code, "E_CREDENTIAL_CHANGED", "{route:?}");
            assert!(get_heads(&f).is_empty(), "{route:?}");

            // Between metadata reads: the one in flight could not be recalled,
            // nothing after it is sent and nothing partial returned.
            let h = home();
            store(h.path(), None, &mail_and_readonly());
            let f = changing(h.path(), change, Trigger::FirstMetadata);
            let failure = run_on(route, h.path(), &endpoints(&f)).await.unwrap_err();
            assert_eq!(failure.code, "E_CREDENTIAL_CHANGED", "{route:?}");
            assert_eq!(get_heads(&f).len(), 1, "{route:?}");
        }
    }
}

#[tokio::test]
async fn another_google_account_in_the_slot_is_refused_before_gmail() {
    let h = home();
    store(h.path(), None, &mail_and_readonly());
    let f = Fixture::start(Arc::new(|head: &str| {
        if head.contains(" /v1/userinfo") {
            Reply::json(200, &userinfo("someone-else"))
        } else {
            Reply::json(200, &list_of(&[MESSAGE_A]))
        }
    }));
    let failure = call_with(
        h.path(),
        &parsed(&search_request(h.path(), json!({"text": "RFI"}))),
        &endpoints(&f),
    )
    .await
    .unwrap_err();
    assert_eq!(failure, CallFailure::new("E_BINDING_CHANGED"));
    assert!(list_heads(&f).is_empty());
}

// ── where the token may go ───────────────────────────────────────────────────

#[test]
fn the_token_goes_only_to_the_exact_google_allowlist() {
    let e = Endpoints::production(gop());
    let mut allowed = e.allowed_origins.clone();
    allowed.sort();
    assert_eq!(
        allowed,
        [
            "https://gmail.googleapis.com",
            "https://openidconnect.googleapis.com",
            "https://www.googleapis.com"
        ]
    );
    assert_eq!(gop().route.origin, "https://gmail.googleapis.com");
    assert!(e.check_origins(gop()).is_ok());
    assert!(matches!(&e.token, TokenPin::Exact(url) if url == GOOGLE_TOKEN_URL));
}

#[tokio::test]
async fn an_origin_outside_the_allowlist_receives_nothing() {
    let h = home();
    store(h.path(), None, &mail_and_readonly());
    let sink = gmail(Reply::json(200, &list_of(&[MESSAGE_A])), ok_meta());
    // The data origin, then the identity read, pointed at the sink while the
    // allowlist stays the production one.
    let production = Endpoints::production(gop());
    let data = Endpoints {
        data_origin: Some(sink.origin()),
        ..production.clone()
    };
    let identity = Endpoints {
        identity_url: format!("{}/v1/userinfo", sink.origin()),
        ..production
    };
    for e in [data, identity] {
        let failure = call_with(
            h.path(),
            &parsed(&search_request(h.path(), json!({"text": "RFI"}))),
            &e,
        )
        .await
        .unwrap_err();
        assert_eq!(failure.code, "E_CALL_ORIGIN_NOT_ALLOWED");
        let failure = run_for_workflow_with(
            h.path(),
            &workflow_manifest(),
            "gmail.search",
            &json!({"text": "RFI"}),
            Some(&e),
            Duration::from_secs(5),
            1 << 20,
        )
        .await
        .unwrap_err();
        assert_eq!(failure.code, "E_CALL_ORIGIN_NOT_ALLOWED");
    }
    assert!(sink.requests().is_empty());
}

#[tokio::test]
async fn a_redirect_on_either_phase_is_never_followed_with_the_token() {
    let sink = Fixture::start(Arc::new(|_: &str| Reply::json(200, &list_of(&[MESSAGE_A]))));
    let redirect = Reply {
        headers: format!("Location: {}/stolen\r\n", sink.origin()),
        ..Reply::json(302, &json!({}))
    };
    // On the list.
    let (out, f) = search(json!({"text": "RFI"}), redirect.clone(), ok_meta()).await;
    let failure = out.unwrap_err();
    assert_eq!(
        failure,
        CallFailure::reason("E_CALL_FAILED", "redirect-refused")
            .with("status", json!(302))
            .with("phase", json!("list"))
    );
    assert!(get_heads(&f).is_empty());
    // On a metadata read.
    let meta: Meta = {
        let redirect = redirect.clone();
        Arc::new(move |_: &str| redirect.clone())
    };
    let (out, f) = search(
        json!({"text": "RFI"}),
        Reply::json(200, &list_of(&[MESSAGE_A, MESSAGE_B])),
        meta,
    )
    .await;
    assert_eq!(
        out.unwrap_err(),
        CallFailure::reason("E_CALL_FAILED", "redirect-refused")
            .with("status", json!(302))
            .with("phase", json!("metadata"))
    );
    assert_eq!(get_heads(&f).len(), 1);
    assert!(sink.requests().is_empty(), "a redirect was followed");
}

#[tokio::test]
async fn a_token_endpoint_override_never_receives_the_refresh_token() {
    let h = home();
    let mut t = token(&mail_and_readonly(), Some(GENERATION), TOKEN);
    t.expires_at = 1;
    t.refresh_token = Some("refresh-secret".into());
    crate::auth::keychain::store_token(&t, None, h.path()).unwrap();
    let sink = Fixture::start(Arc::new(|_: &str| {
        Reply::json(200, &json!({"access_token": "x"}))
    }));
    std::fs::create_dir_all(h.path().join("oauth")).unwrap();
    std::fs::write(
        h.path().join("oauth/google-workspace.yaml"),
        format!("token_url: {}/token\n", sink.origin()),
    )
    .unwrap();
    let f = gmail(Reply::json(200, &list_of(&[MESSAGE_A])), ok_meta());
    for route in [Route::Call, Route::Workflow, Route::Invoke] {
        let failure = run_on(route, h.path(), &endpoints(&f)).await.unwrap_err();
        assert_eq!(
            failure,
            CallFailure::reason("E_CREDENTIAL_EXPIRED", "token-endpoint"),
            "{route:?}"
        );
    }
    assert!(sink.requests().is_empty());
    assert!(f.requests().is_empty());
}

#[tokio::test]
async fn no_token_query_or_message_text_reaches_the_log() {
    let (out, _) = search(
        json!({"text": QUERY_SENTINEL}),
        Reply::json(500, &json!({"error": {"message": QUERY_SENTINEL}})),
        ok_meta(),
    )
    .await;
    let failure = out.unwrap_err();
    assert_no_sentinel(&failure);
    let logs = tempfile::tempdir().unwrap();
    append_log_line(
        logs.path(),
        "call",
        "google-workspace",
        "gmail.search",
        failure.code,
    );
    let line = std::fs::read_to_string(logs.path().join("agent-call.log")).unwrap();
    assert!(line.contains("agent-call call google-workspace gmail.search E_CALL_FAILED"));
    assert!(!line.contains(QUERY_SENTINEL) && !line.contains(TOKEN));
}

// ── projection (R2-6, R2-7, R2-8) ────────────────────────────────────────────

fn with_headers(headers: Json) -> Json {
    let mut m = metadata(MESSAGE_A);
    m["payload"]["headers"] = headers;
    m
}

fn entry(item: &Json) -> Json {
    gmail::message_entry(item, MESSAGE_A, Some(EMAIL))
}

#[test]
fn headers_are_decoded_before_caps_and_the_first_of_duplicates_wins() {
    // RFC 2047 encoded words, in both encodings.
    let m = entry(&with_headers(json!([
        {"name": "Subject", "value": "=?UTF-8?B?UkZJIDA0MiDFvHVyYXc=?="},
        {"name": "From", "value": "=?ISO-8859-1?Q?Andr=E9_Pirard?= <pirard@example.be>"},
    ])));
    assert_eq!(m["subject"], json!("RFI 042 żuraw"));
    assert_eq!(
        m["from"],
        json!({"address": "pirard@example.be", "name": "André Pirard"})
    );
    // A folded header is unfolded.
    let m = entry(&with_headers(json!([
        {"name": "subject", "value": "RFI 042\r\n beam B12"},
        {"name": "FROM", "value": "Ada\r\n\tLovelace <ada@acme.com>"},
    ])));
    assert_eq!(m["subject"], json!("RFI 042 beam B12"));
    assert_eq!(m["from"]["address"], json!("ada@acme.com"));
    // Duplicates: the first.
    let m = entry(&with_headers(json!([
        {"name": "Subject", "value": "first"},
        {"name": "Subject", "value": "second"},
        {"name": "From", "value": "a@acme.com"},
        {"name": "From", "value": "b@acme.com"},
    ])));
    assert_eq!(m["subject"], json!("first"));
    assert_eq!(m["from"], json!({"address": "a@acme.com"}));
    assert!(m.get("from-count").is_none());
    // Several senders: the first, and how many.
    let m = entry(&with_headers(json!([
        {"name": "From", "value": "Ada <ada@acme.com>, Bob <bob@acme.com>, carol@acme.com"},
    ])));
    assert_eq!(m["from"], json!({"address": "ada@acme.com", "name": "Ada"}));
    assert_eq!(m["from-count"], json!(3));
    // Caps apply to the decoded text and are named.
    let long = "s".repeat(1500);
    let m = entry(&with_headers(json!([{"name": "Subject", "value": long}])));
    assert_eq!(m["subject"].as_str().unwrap().chars().count(), 1024);
    assert_eq!(m["truncated"], json!(["subject"]));
}

#[test]
fn an_unusable_header_omits_its_field_and_keeps_the_message() {
    // Empty subject: no subject, nothing claimed cut.
    let m = entry(&with_headers(json!([{"name": "Subject", "value": ""}])));
    assert!(m.get("subject").is_none());
    assert!(m.get("truncated").is_none(), "{m}");
    assert_eq!(m["id"], json!(MESSAGE_A));
    // A line break that is not a fold could start a second header: refused.
    for injected in [
        "RFI\r\nBcc: x@evil.com",
        "RFI\nFrom: x@evil.com",
        "RFI\r\n",
        "RFI\u{0000}x",
    ] {
        let m = entry(&with_headers(json!([
            {"name": "Subject", "value": injected},
            {"name": "From", "value": format!("ada@acme.com{}", &injected[3..])},
        ])));
        assert!(m.get("subject").is_none(), "{injected:?}");
        assert!(m.get("from").is_none(), "{injected:?}");
        let truncated = m["truncated"].as_array().unwrap();
        assert!(truncated.contains(&json!("subject")), "{injected:?}");
        assert!(truncated.contains(&json!("from")), "{injected:?}");
        assert!(!m.to_string().contains("evil"));
    }
    // An empty From is absent; a From that names no mailbox, and a value
    // that is not a string, are unreadable and named.
    let m = entry(&with_headers(json!([
        {"name": "From", "value": "  "},
        {"name": "Subject", "value": 7},
    ])));
    assert!(m.get("from").is_none() && m.get("subject").is_none());
    assert_eq!(m["truncated"], json!(["subject"]));
    let m = entry(&with_headers(json!([
        {"name": "From", "value": "undisclosed-recipients:;"},
    ])));
    assert!(m.get("from").is_none());
    assert_eq!(m["truncated"], json!(["from"]));
    // No headers at all: the message stays, with what it has.
    let mut bare = metadata(MESSAGE_A);
    bare.as_object_mut().unwrap().remove("payload");
    let m = entry(&bare);
    assert!(m.get("subject").is_none() && m.get("from").is_none());
    assert_eq!(m["received-at"], json!("2025-11-10T08:00:00.123Z"));
}

#[test]
fn received_at_and_preview_may_be_missing_and_are_never_invented() {
    let mut m = metadata(MESSAGE_A);
    let obj = m.as_object_mut().unwrap();
    obj.remove("internalDate");
    obj.remove("snippet");
    let e = entry(&m);
    assert!(e.get("received-at").is_none() && e.get("preview").is_none());
    assert!(e.get("truncated").is_none());
    for bad in [
        json!("-1"),
        json!("+1762761600123"),
        json!("1762761600123.5"),
        json!(""),
        json!("99999999999999999999"),
        json!("9223372036854775807"),
        json!(1762761600123u64),
    ] {
        let mut m = metadata(MESSAGE_A);
        m["internalDate"] = bad.clone();
        let e = entry(&m);
        assert!(e.get("received-at").is_none(), "{bad}");
        assert_eq!(e["truncated"], json!(["received-at"]), "{bad}");
    }
    let mut m = metadata(MESSAGE_A);
    m["internalDate"] = json!("0");
    assert_eq!(entry(&m)["received-at"], json!("1970-01-01T00:00:00.000Z"));
    // The preview is Gmail's snippet as text, capped.
    let mut m = metadata(MESSAGE_A);
    m["snippet"] = json!(format!("it&#39;s {}", "p".repeat(600)));
    let e = entry(&m);
    let preview = e["preview"].as_str().unwrap();
    assert!(preview.starts_with("it's "));
    assert_eq!(preview.chars().count(), 512);
    assert_eq!(e["truncated"], json!(["preview"]));
}

#[test]
fn thread_and_label_ids_are_exact_or_named() {
    let mut m = metadata(MESSAGE_A);
    m["threadId"] = json!("../x");
    m["labelIds"] = json!(["INBOX", "in:spam", 7, "Label_9"]);
    let e = entry(&m);
    assert!(e.get("thread-id").is_none());
    assert_eq!(e["label-ids"], json!(["INBOX", "Label_9"]));
    assert_eq!(e["truncated"], json!(["thread-id", "label-ids"]));
    let mut m = metadata(MESSAGE_A);
    m["labelIds"] = json!((0..150).map(|i| format!("Label_{i}")).collect::<Vec<_>>());
    let e = entry(&m);
    assert_eq!(e["label-ids"].as_array().unwrap().len(), 100);
    assert_eq!(e["truncated"], json!(["label-ids"]));
}

#[test]
fn the_web_link_opens_in_the_verified_account_or_is_omitted() {
    let m = metadata(MESSAGE_A);
    let e = gmail::message_entry(&m, MESSAGE_A, Some("ada+site@example.com"));
    assert_eq!(
        e["web-url"],
        json!(format!(
            "https://mail.google.com/mail/?authuser=ada%2Bsite%40example.com#all/{MESSAGE_A}"
        ))
    );
    let e = gmail::message_entry(&m, MESSAGE_A, None);
    assert!(e.get("web-url").is_none());
}

#[tokio::test]
async fn an_unverified_email_gives_no_web_link() {
    let h = home();
    store(h.path(), None, &mail_and_readonly());
    let f = Fixture::start(Arc::new(|head: &str| {
        if head.contains(" /v1/userinfo") {
            Reply::json(
                200,
                &json!({"sub": SUB, "email": "ada@example.com", "email_verified": false}),
            )
        } else if let Some(id) = get_id(head) {
            Reply::json(200, &metadata(&id))
        } else {
            Reply::json(200, &list_of(&[MESSAGE_A]))
        }
    }));
    let record = call_with(
        h.path(),
        &parsed(&search_request(h.path(), json!({"text": "RFI"}))),
        &endpoints(&f),
    )
    .await
    .unwrap();
    let message = &record["result"]["body"]["messages"][0];
    assert_eq!(message["id"], json!(MESSAGE_A));
    assert!(message.get("web-url").is_none());
}

// ── the digest (R2-1) ────────────────────────────────────────────────────────

/// Every row on `main` before `gmail.search` (cb998fd41, v0.176.0), with its
/// digest, captured from that build — not from this table. Adding the Gmail
/// row must move none of them.
const PRE_GMAIL_DIGESTS: &[(&str, &str, &str)] = &[
    (
        "google-workspace",
        "list-files",
        "aa42474ff6700da478edd1d1412946a6edbcf471dbda0481328026be1eb6bea9",
    ),
    (
        "microsoft-365",
        "list-folder",
        "8550a5bd95a2927de7de681c6ea9e99d63eff6f6eb09a14ed01745c311a090fe",
    ),
    (
        "microsoft-365",
        "get-drive-item",
        "faee3449ff069728925d57061cf9d43d0366a4eb0af7fbee4deb5dda47f9fb29",
    ),
    (
        "microsoft-365",
        "outlook.mail.search",
        "7452a9471a86530d878977358f4ee255a7c2fb76f5cdfd51ce851d1294c2cd87",
    ),
    (
        "microsoft-365",
        "planner.plan.list",
        "634031001eee428cc51ed2b7d114aae7bcdb924e62f0de6b73bc74a7153cdb70",
    ),
    (
        "microsoft-365",
        "planner.task.list",
        "f2c3172dd79e7768e95f5411920be78e808b63a95372e3d9d1a12c66115c17f5",
    ),
    (
        "microsoft-365",
        "sharepoint.list.list",
        "544740910592f4a945821c8234e04a9f34f88194595388ae0ac90eda364cbf10",
    ),
    (
        "microsoft-365",
        "sharepoint.list.read",
        "bfec42fa84b9d08566d1ce3fefc1649377fc459798a3a0ea8139ea70b0446f81",
    ),
    (
        "microsoft-365",
        "teams.team.list",
        "c3c7cf77cd83d4754899885792fe6011288ed31f3f6eaf6e8311d620967d388d",
    ),
    (
        "microsoft-365",
        "teams.channel.list",
        "0fba96baa5a62f6e8e0711439e58c11ddef155ba5273c394218b5bad0a67ca17",
    ),
    (
        "microsoft-365",
        "teams.channel.list-messages",
        "1d0617947557a15488821f475b9ba3748130a0882227f45bf14a1f7a24a2d737",
    ),
    (
        "microsoft-365",
        "teams.chat.list",
        "d0e54676a1ea6367e361c738bc439bf6e2e3233f6646c30d5ced498fc8994073",
    ),
    (
        "microsoft-365",
        "teams.chat.list-messages",
        "979de466f55c5a66525e38c11c83388be69aa28e9efc7749b7b2eeb67d92cf21",
    ),
    (
        "microsoft-365",
        "outlook.mail.folder.list",
        "07e9c21ac659581f47361fef6fb82345f964d48bd0dbbd728660a01036de346f",
    ),
    (
        "microsoft-365",
        "outlook.mail.folder.list-messages",
        "78bf086f36557ce257d6faafcbe97fa05edec5c9ba8a62e4aed712a37da8acbd",
    ),
    (
        "microsoft-365",
        "outlook.mail.folder.list-message-ids",
        "f3f2fd14ea22e5767d6d501b54b9bd73775b877f842d3908849c93f3ccc13676",
    ),
    (
        "microsoft-365",
        "outlook.mail.get-message",
        "85fdf0e7275721e1c4d9e0f759e0791cba21d4af8409ac9bf0669ca990273136",
    ),
    (
        "microsoft-365",
        "excel.worksheet.list",
        "8e60b9f920f350d6673f8219410c099085975e347c427e414c60d78c9e087620",
    ),
    (
        "microsoft-365",
        "excel.range.read",
        "69c4a5661b5b429c2f89b47d024af9618d35e31c7177083b8885a87b456a3168",
    ),
    (
        "microsoft-365",
        "get-shared-item",
        "850a0a486fb40882ae2ef36d5b1a5370a786c53eb9c87ff5c6bae074199dfa9e",
    ),
    (
        "trimble-connect",
        "list-projects",
        "55f8790f4c48c6c7dd9a377dc9a603759fb7471f6c6db8fae0eb2cc9793418e0",
    ),
    (
        "trimble-connect",
        "list-topics",
        "fa5f47aea774e9d16f8221f0175e908f3af0eadaf347939bc860d19d7e65cb1a",
    ),
    (
        "trimble-connect",
        "get-topic",
        "f271613696ed33ac2aea92e4c15ce1e7ecb376101760581900bd4f31ce89027b",
    ),
    (
        "trimble-connect",
        "list-comments",
        "e550a55d8f9d94f261abdbff05293a1e37994ef1b152959dbe9b82df5132c9d5",
    ),
    (
        "trimble-connect",
        "list-todos",
        "259c556c778b4094bd677ddd66fb27c924a7294b7bee17cf762d04d0e76946b4",
    ),
];

#[test]
fn every_row_shipped_before_gmail_search_keeps_its_digest() {
    let actual: Vec<(&str, &str, String)> = OPERATIONS
        .iter()
        .filter(|op| op.command != "gmail.search")
        .map(|op| (op.agent, op.command, op.sha256()))
        .collect();
    let pinned: Vec<(&str, &str, String)> = PRE_GMAIL_DIGESTS
        .iter()
        .map(|(a, c, d)| (*a, *c, (*d).to_string()))
        .collect();
    assert_eq!(actual, pinned);
}

const GMAIL_CANONICAL: &str = r#"{"adminConsent":false,"agent":"google-workspace","bindsAccount":true,"budget":{"deadline":"one-for-all-requests","partial":"never"},"canonical":"v2","caps":{"address":320,"headerBytes":16384,"labels":100,"name":256,"preview":512,"subject":1024},"command":"gmail.search","effect":"read","get":{"attempts":"every-request-counts","cap":65536,"idMismatch":"fails","maxAttempts":50,"notFound":"omitted","otherFailure":"fails","query":[["format","metadata"],["metadataHeaders","From"],["metadataHeaders","Subject"],["metadataHeaders","Date"]],"segments":["gmail","v1","users","me","messages",{"format":"gmail-id/1","render":"provider-id"}]},"identity":{"label":"email-when-verified","query":[],"subject":"sub","url":"https://openidconnect.googleapis.com/v1/userinfo"},"identityScopes":[],"inputs":[{"default":null,"format":"gmail-search-words/1","location":"gmail-q","maxBytes":1024,"maximum":null,"minimum":null,"name":"q","required":false,"scalar":"string","source":"text"},{"default":null,"format":"gmail-from/1","location":"gmail-q","maxBytes":320,"maximum":null,"minimum":null,"name":"q","required":false,"scalar":"string","source":"from"},{"default":null,"location":"query","maxBytes":64,"maximum":null,"minimum":null,"name":"labelIds","pattern":"gmail-label/1","required":false,"scalar":"string","source":"label"},{"default":25,"location":"query","maxBytes":null,"maximum":50,"minimum":1,"name":"maxResults","required":false,"scalar":"integer","source":"page-size"}],"integration":"google-workspace","label":{"format":"gmail-label/1","param":"labelIds","refused":["SPAM","TRASH"]},"limitInput":"page-size","list":{"absentMessages":"empty","cap":65536,"moreAvailable":"non-empty-next-page-token-or-cut","nextPageToken":"never-followed","notFound":"fails","order":["q","labelIds","maxResults","includeSpamTrash"],"query":[["includeSpamTrash","false"]],"segments":["gmail","v1","users","me","messages"]},"mailHeaders":{"duplicates":"first","from":"first-mailbox-and-count","parser":"mail-parser/rfc5322-rfc2047","unparseable":"omitted"},"method":"GET","origin":"https://gmail.googleapis.com","preview":"snippet-entities-decoded","projection":"gmail-messages/1","provider":"Google","providerIds":{"duplicate":"omitted","format":"gmail-id/1","invalid":"omitted","maxBytes":64},"q":{"from":{"format":"gmail-from/1"},"order":["from","text"],"render":"gmail-q/1","separator":" ","whenEmpty":"omitted","words":{"format":"gmail-search-words/1","leadingHyphen":"refused","maxWordChars":64,"maxWords":16,"refused":["OR","AND","AROUND","NOT"]}},"receivedAt":"internal-date-ms/1","redirects":"refused","requestScopes":["openid","https://www.googleapis.com/auth/userinfo.email","https://www.googleapis.com/auth/gmail.readonly"],"required":{"anyOf":["text","from","label"]},"schema":"aware.agent-call-operation/v1","scopes":["https://www.googleapis.com/auth/gmail.readonly"],"slotCheck":"before-every-request","transport":"rest","webUrl":{"authuser":"verified-email","base":"https://mail.google.com/mail/","fragment":"all/{id}"}}"#;
const GMAIL_DIGEST: &str = "8740f942bb48e1a453aecb816cc44f7b95055c117db96a5f74b33e505523fc6d";

#[test]
fn the_gmail_digest_is_pinned_byte_for_byte() {
    assert_eq!(canonical_string(&gop().canonical()), GMAIL_CANONICAL);
    assert_eq!(gop().sha256(), GMAIL_DIGEST);
}

#[test]
fn every_security_relevant_change_moves_the_gmail_digest() {
    let base = gop().clone();
    let digest = base.sha256();
    let mut changed: Vec<(&str, String)> = Vec::new();
    let mut m = base.clone();
    m.fixed_query = &[("includeSpamTrash", "true")];
    changed.push(("spam-and-trash", m.sha256()));
    let mut m = base.clone();
    m.fixed_query = &[];
    changed.push(("no-fixed-query", m.sha256()));
    let mut m = base.clone();
    m.any_of_scopes = &[GMAIL_READONLY, "https://mail.google.com/"];
    changed.push(("scopes", m.sha256()));
    let mut m = base.clone();
    m.request_scopes = &[GMAIL_READONLY];
    changed.push(("request-scopes", m.sha256()));
    let mut m = base.clone();
    m.binds_account = false;
    changed.push(("binds-account", m.sha256()));
    let mut m = base.clone();
    m.route.origin = "https://www.googleapis.com";
    changed.push(("origin", m.sha256()));
    let mut m = base.clone();
    m.route.segments = &[
        Seg::Lit("gmail"),
        Seg::Lit("v1"),
        Seg::Lit("users"),
        Seg::Lit("me"),
        Seg::Lit("threads"),
    ];
    changed.push(("route", m.sha256()));
    let mut m = base.clone();
    m.projection = Projection::GraphMessages;
    changed.push(("projection", m.sha256()));
    let mut m = base.clone();
    m.limit_input = None;
    changed.push(("limit-input", m.sha256()));
    // Each input's grammar, bound and wire name.
    let mut inputs = base.inputs.to_vec();
    inputs[0].gmail = Some(gmail::Format::From);
    let mut m = base.clone();
    m.inputs = Box::leak(inputs.into_boxed_slice());
    changed.push(("text-grammar", m.sha256()));
    let mut inputs = base.inputs.to_vec();
    inputs[0].max_bytes = Some(4096);
    let mut m = base.clone();
    m.inputs = Box::leak(inputs.into_boxed_slice());
    changed.push(("text-bytes", m.sha256()));
    let mut inputs = base.inputs.to_vec();
    inputs[2].id_kind = Some(IdKind::GraphOpaqueId);
    let mut m = base.clone();
    m.inputs = Box::leak(inputs.into_boxed_slice());
    changed.push(("label-grammar", m.sha256()));
    let mut inputs = base.inputs.to_vec();
    inputs[2].location = Location::GmailQuery;
    let mut m = base.clone();
    m.inputs = Box::leak(inputs.into_boxed_slice());
    changed.push(("label-in-q", m.sha256()));
    let mut inputs = base.inputs.to_vec();
    inputs[3].maximum = Some(500);
    let mut m = base.clone();
    m.inputs = Box::leak(inputs.into_boxed_slice());
    changed.push(("page-bound", m.sha256()));
    for (what, other) in &changed {
        assert_ne!(*other, digest, "{what} did not move the digest");
    }
    // The Gmail-specific bounds and renderings are spelled in the digest.
    let canonical = canonical_string(&gop().canonical());
    for needle in [
        "\"canonical\":\"v2\"",
        "[\"format\",\"metadata\"]",
        "[\"metadataHeaders\",\"From\"]",
        "[\"metadataHeaders\",\"Subject\"]",
        "[\"metadataHeaders\",\"Date\"]",
        "[\"includeSpamTrash\",\"false\"]",
        "\"gmail-search-words/1\"",
        "\"gmail-from/1\"",
        "\"gmail-label/1\"",
        "\"gmail-id/1\"",
        "\"gmail-q/1\"",
        "\"gmail-messages/1\"",
        "\"maxAttempts\":50",
        "\"anyOf\":[\"text\",\"from\",\"label\"]",
        "\"refused\":[\"OR\",\"AND\",\"AROUND\",\"NOT\"]",
        "\"refused\":[\"SPAM\",\"TRASH\"]",
        "\"cap\":65536",
        "\"subject\":1024",
        "\"address\":320",
        "\"name\":256",
        "\"preview\":512",
        "\"redirects\":\"refused\"",
        "https://gmail.googleapis.com",
        GMAIL_READONLY,
    ] {
        assert!(canonical.contains(needle), "{needle} not in {canonical}");
    }
    // list-files keeps its v1 form and digest.
    assert_eq!(
        op().sha256(),
        "aa42474ff6700da478edd1d1412946a6edbcf471dbda0481328026be1eb6bea9"
    );
}

#[test]
fn the_capability_publishes_the_typed_inputs_and_the_permissions() {
    let inputs = gop().inputs_json();
    let by_source = |source: &str| -> Json {
        inputs
            .as_array()
            .unwrap()
            .iter()
            .find(|i| i["source"] == json!(source))
            .cloned()
            .unwrap()
    };
    assert_eq!(by_source("text")["format"], json!("gmail-search-words/1"));
    assert_eq!(by_source("text")["location"], json!("gmail-q"));
    assert_eq!(by_source("from")["format"], json!("gmail-from/1"));
    assert_eq!(by_source("label")["pattern"], json!("gmail-label/1"));
    assert_eq!(by_source("label")["name"], json!("labelIds"));
    assert_eq!(by_source("page-size")["name"], json!("maxResults"));
    assert_eq!(by_source("page-size")["maximum"], json!(50));
    assert_eq!(by_source("page-size")["default"], json!(25));
    assert_eq!(
        gop().permissions_json(),
        json!({
            "identityScopes": [],
            "anyOfScopes": [GMAIL_READONLY],
            "requestScopes": ["openid", "https://www.googleapis.com/auth/userinfo.email", GMAIL_READONLY],
            "adminConsent": false,
        })
    );
}

// ── the manifest and the version floor (R2-12) ───────────────────────────────

#[test]
fn the_shipped_manifest_offers_gmail_search_with_exactly_the_reviewed_inputs() {
    let agent = workflow_manifest();
    // Read from the YAML, not the parsed `Agent`: the contract migration guard
    // forbids the run path's sources from reading manifest fields it ignores.
    let raw: serde_yaml::Value = serde_yaml::from_str(&shipped_manifest()).unwrap();
    assert_eq!(raw["version"].as_str(), Some("2.3.0"));
    assert!(command_runnable(&agent, gop()));
    assert!(manifest_slot(&agent, gop(), false).is_ok());
    let command = &agent.commands["gmail.search"];
    let mut declared: Vec<&str> = command
        .inputs
        .as_mapping()
        .unwrap()
        .keys()
        .filter_map(|k| k.as_str())
        .collect();
    declared.sort_unstable();
    let mut reviewed: Vec<&str> = gop().inputs.iter().map(|i| i.source).collect();
    reviewed.sort_unstable();
    assert_eq!(declared, reviewed);
    let permissions = command.permissions.as_ref().unwrap();
    assert_eq!(permissions.scopes, gop().request_scopes);
    assert!(!permissions.admin_consent);
}

#[test]
fn the_2_3_0_manifest_needs_cli_0_178_0() {
    let agent = workflow_manifest();
    assert!(crate::validate::runtime_requirement_error(&agent, "0.177.0").is_some());
    assert!(
        crate::validate::runtime_requirement_error(&agent, crate::validate::CURRENT_CLI_VERSION)
            .is_none()
    );
    assert_eq!(crate::validate::CURRENT_CLI_VERSION, "0.178.0");
}
