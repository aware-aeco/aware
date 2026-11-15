//! The Gmail Knowledge-source reads (#838): `gmail.labels.list`,
//! `gmail.list-message-ids`, `gmail.get-message`, the rate-limit reason on
//! every Gmail row and the Google principal namespace (plan
//! `docs/superpowers/specs/2026-11-15-838-gmail-knowledge-reads-PLAN.md`, §4
//! governs). A child of the `gmail.search` tests, so it shares their homes,
//! grants and request helpers. Every request goes to a local fixture; the
//! answers are Gmail's raw JSON shapes, body parts base64url-encoded.

use super::*;
use base64::Engine as _;

const ID: &str = "18c1a2b3c4d5e6f7";
const LABELS_PATH: &str = "/gmail/v1/users/me/labels";

fn lop() -> &'static Operation {
    operation("google-workspace", "gmail.labels.list").unwrap()
}

fn iop() -> &'static Operation {
    operation("google-workspace", "gmail.list-message-ids").unwrap()
}

fn mop() -> &'static Operation {
    operation("google-workspace", "gmail.get-message").unwrap()
}

fn b64(text: &[u8]) -> String {
    base64::engine::general_purpose::URL_SAFE.encode(text)
}

/// A Gmail fixture: userinfo answers `SUB`, every other request `route(head)`.
fn fixture(route: impl Fn(&str) -> Reply + Send + Sync + 'static) -> Fixture {
    Fixture::start(Arc::new(move |head: &str| {
        if head.contains(" /v1/userinfo") {
            Reply::json(200, &userinfo(SUB))
        } else {
            route(head)
        }
    }))
}

fn request_for(home: &Path, op: &Operation, inputs: Json, max_output: u64) -> Map<String, Json> {
    let mut map = request(home);
    map.insert("command".into(), json!(op.command));
    map.insert("expectedOperationSha256".into(), json!(op.sha256()));
    map.insert("maxOutputBytes".into(), json!(max_output));
    set_inputs(&mut map, inputs);
    map
}

async fn call_op(
    op: &Operation,
    inputs: Json,
    max_output: u64,
    f: &Fixture,
) -> Result<Json, CallFailure> {
    let h = home();
    store(h.path(), None, &mail_and_readonly());
    call_with(
        h.path(),
        &parsed(&request_for(h.path(), op, inputs, max_output)),
        &endpoints(f),
    )
    .await
}

fn body_of(record: &Json) -> &Json {
    &record["result"]["body"]
}

fn data_heads(f: &Fixture) -> Vec<String> {
    f.requests()
        .into_iter()
        .filter(|h| !h.contains(" /v1/userinfo"))
        .collect()
}

fn ids_wire(inputs: Json) -> Result<Wire, CallFailure> {
    let Json::Object(map) = inputs else {
        unreachable!()
    };
    wire(iop(), &map)
}

fn q_of(w: &Wire) -> Option<String> {
    w.query
        .iter()
        .find(|(k, _)| k == "q")
        .map(|(_, v)| v.clone())
}

fn rate_limit_body(reason: &str) -> Json {
    json!({"error": {"code": 403, "message": "must-not-leave-message",
        "errors": [{"domain": "usageLimits", "reason": reason, "message": "must-not-leave-message"}],
        "status": "PERMISSION_DENIED"}})
}

fn with_header(mut reply: Reply, header: &str) -> Reply {
    reply.headers = format!("{header}\r\n");
    reply
}

// ── registration, scope, principal ───────────────────────────────────────────

#[test]
fn the_three_reads_are_reviewed_operations_on_every_route() {
    for op in [lop(), iop(), mop()] {
        assert_eq!(op.provider, Provider::Google);
        assert_eq!(op.any_of_scopes, [GMAIL_READONLY]);
        assert_eq!(
            op.request_scopes,
            [GOOGLE_OPENID, GOOGLE_EMAIL, GMAIL_READONLY]
        );
        assert!(op.binds_account && op.canonical_v2);
        assert!(handles_workflow("google-workspace", op.command));
        assert!(Endpoints::production(op).check_origins(op).is_ok());
        assert_eq!(op.route.origin, "https://gmail.googleapis.com");
    }
    // Writes and history stay out.
    for command in ["gmail.modify", "gmail.labels.create", "gmail.history.list"] {
        assert!(operation("google-workspace", command).is_none());
    }
}

#[tokio::test]
async fn a_slot_without_gmail_readonly_reads_nothing_on_any_of_the_three() {
    for (op, inputs) in [
        (lop(), json!({})),
        (
            iop(),
            json!({"label": "Label_12", "received-after": "2026-08-17T00:00:00Z"}),
        ),
        (mop(), json!({"id": ID})),
    ] {
        let h = home();
        store(h.path(), None, &mail_and_drive());
        let f = fixture(|_| Reply::json(200, &json!({})));
        let failure = call_with(
            h.path(),
            &parsed(&request_for(h.path(), op, inputs, 262_144)),
            &endpoints(&f),
        )
        .await
        .unwrap_err();
        assert_eq!(
            failure,
            CallFailure::new("E_CALL_SCOPE_MISSING"),
            "{}",
            op.command
        );
        assert!(f.requests().is_empty(), "{}", op.command);
    }
}

#[tokio::test]
async fn a_google_principal_names_its_namespace_and_keeps_its_binding() {
    let h = home();
    store(h.path(), None, &mail_and_readonly());
    let f = fixture(|_| Reply::json(200, &json!({})));
    for op in [op(), gop(), lop(), iop(), mop()] {
        let cap = capabilities_with(h.path(), op, None, Duration::from_secs(10), &endpoints(&f))
            .await
            .unwrap();
        let credential = &cap["credential"];
        assert_eq!(
            credential["principal"],
            json!({"integration": "google-workspace", "stable_id": SUB, "tenant_id": null,
                   "namespace": "google-account"}),
            "{}",
            op.command
        );
        // The binding is over (integration, slot, sub) alone, as before.
        assert_eq!(
            credential["binding_id"],
            json!(binding_id(
                "google-workspace",
                "google-workspace",
                SUB,
                None
            ))
        );
    }
    // Microsoft principals still carry none.
    let ms = Principal {
        sub: "u".into(),
        label: None,
        tenant: Some("t".into()),
    };
    assert!(principal_json(ms_op(), &ms).get("namespace").is_none());
}

// ── rate limit (every Gmail row) ─────────────────────────────────────────────

#[test]
fn retry_after_is_seconds_or_a_date_and_bounded() {
    assert_eq!(gmail::retry_after_secs("30"), Some(30));
    assert_eq!(gmail::retry_after_secs(" 0 "), Some(0));
    assert_eq!(gmail::retry_after_secs("999999"), Some(86_400));
    assert_eq!(
        gmail::retry_after_secs("99999999999999999999999"),
        Some(86_400)
    );
    assert_eq!(
        gmail::retry_after_secs("Sun, 06 Nov 1994 08:49:37 GMT"),
        Some(0)
    );
    let soon = (chrono::Utc::now() + chrono::Duration::seconds(120)).to_rfc2822();
    let secs = gmail::retry_after_secs(&soon).unwrap();
    assert!((100..=120).contains(&secs), "{secs}");
    for bad in ["", "-5", "1.5", "soon", "30s"] {
        assert_eq!(gmail::retry_after_secs(bad), None, "{bad}");
    }
}

#[test]
fn only_googles_rate_limit_reasons_make_a_403_a_rate_limit() {
    for reason in ["rateLimitExceeded", "userRateLimitExceeded"] {
        assert!(gmail::names_rate_limit(
            rate_limit_body(reason).to_string().as_bytes()
        ));
    }
    for body in [
        rate_limit_body("insufficientPermissions").to_string(),
        rate_limit_body("dailyLimitExceeded").to_string(),
        json!({"error": {"code": 403, "message": "rateLimitExceeded"}}).to_string(),
        "not json rateLimitExceeded".into(),
        String::new(),
    ] {
        assert!(!gmail::names_rate_limit(body.as_bytes()), "{body}");
    }
}

#[tokio::test]
async fn a_rate_limit_is_named_on_every_gmail_row_and_a_plain_403_is_not() {
    let cases: Vec<(Reply, Json)> = vec![
        (
            with_header(Reply::json(429, &json!({})), "Retry-After: 17"),
            json!({"reason": "rate-limited", "status": 429, "retry-after-secs": 17}),
        ),
        // A 429 is a rate limit by its status alone, body or not.
        (
            Reply {
                status: 429,
                headers: String::new(),
                body: b"<html>slow down</html>".to_vec(),
                delay: Duration::ZERO,
            },
            json!({"reason": "rate-limited", "status": 429}),
        ),
        (
            with_header(
                Reply::json(403, &rate_limit_body("userRateLimitExceeded")),
                "Retry-After: 5",
            ),
            json!({"reason": "rate-limited", "status": 403, "retry-after-secs": 5}),
        ),
        (
            Reply::json(403, &rate_limit_body("rateLimitExceeded")),
            json!({"reason": "rate-limited", "status": 403}),
        ),
        // A permission 403 stays the status alone — Retry-After or not.
        (
            with_header(
                Reply::json(403, &rate_limit_body("insufficientPermissions")),
                "Retry-After: 5",
            ),
            json!({"status": 403}),
        ),
        // An oversized 403 body is never read as a reason.
        (
            Reply {
                status: 403,
                headers: String::new(),
                body: format!(
                    "{{\"error\":{{\"errors\":[{{\"reason\":\"rateLimitExceeded\"}}],\"pad\":\"{}\"}}}}",
                    "x".repeat(20_000)
                )
                .into_bytes(),
                delay: Duration::ZERO,
            },
            json!({"status": 403}),
        ),
    ];
    let rows: Vec<(&Operation, Json, &str)> = vec![
        (gop(), json!({"text": "RFI"}), "list"),
        (lop(), json!({}), "labels"),
        (
            iop(),
            json!({"text": "RFI", "received-after": "2026-08-17T00:00:00Z"}),
            "list",
        ),
        (mop(), json!({"id": ID}), "message"),
    ];
    for (reply, expected) in &cases {
        for (op, inputs, phase) in &rows {
            let reply = reply.clone();
            let f = fixture(move |_| reply.clone());
            let failure = call_op(op, inputs.clone(), 262_144, &f).await.unwrap_err();
            assert_eq!(failure.code, "E_CALL_FAILED", "{} {expected}", op.command);
            let mut want = expected.as_object().unwrap().clone();
            want.insert("phase".into(), json!(phase));
            assert_eq!(failure.details, want, "{}", op.command);
            assert!(!format!("{:?}", failure).contains("must-not-leave"));
        }
    }
}

#[tokio::test]
async fn a_rate_limited_metadata_read_of_gmail_search_is_named_too() {
    let f = fixture(|head| {
        if path_only(head) == LIST_PATH {
            Reply::json(200, &list_of(&[ID]))
        } else {
            with_header(Reply::json(429, &json!({})), "Retry-After: 3")
        }
    });
    let failure = call_op(gop(), json!({"text": "RFI"}), 262_144, &f)
        .await
        .unwrap_err();
    assert_eq!(
        Json::Object(failure.details),
        json!({"reason": "rate-limited", "status": 429, "retry-after-secs": 3, "phase": "metadata"})
    );
}

// ── gmail.labels.list ────────────────────────────────────────────────────────

fn labels_body() -> Json {
    json!({"labels": [
        {"id": "INBOX", "name": "INBOX", "type": "system",
         "messageListVisibility": "hide", "labelListVisibility": "labelShow"},
        {"id": "Label_12", "name": "Project 2417/RFIs", "type": "user"},
        {"id": "Label_13", "name": "x".repeat(300), "type": "user"},
        {"id": "bad id!", "name": "Bad", "type": "user"},
        {"id": "Label_12", "name": "Repeat", "type": "user"},
        {"id": "Label_14", "type": "weird"}
    ]})
}

#[tokio::test]
async fn labels_are_listed_with_their_type_and_long_names_cut_and_named() {
    let f = fixture(|_| Reply::json(200, &labels_body()));
    let record = call_op(lop(), json!({}), 262_144, &f).await.unwrap();
    let heads = data_heads(&f);
    assert_eq!(heads.len(), 1);
    assert_eq!(path_of(&heads[0]), LABELS_PATH);
    assert_eq!(
        *body_of(&record),
        json!({
            "labels": [
                {"id": "INBOX", "name": "INBOX", "type": "system"},
                {"id": "Label_12", "name": "Project 2417/RFIs", "type": "user"},
                {"id": "Label_13", "name": "x".repeat(256), "type": "user", "truncated": ["name"]},
                {"id": "Label_14", "truncated": ["name", "type"]}
            ],
            "omitted": 2,
            "complete": false,
        })
    );
}

#[tokio::test]
async fn an_absent_label_list_is_empty_and_complete() {
    let f = fixture(|_| Reply::json(200, &json!({})));
    let record = call_op(lop(), json!({}), 262_144, &f).await.unwrap();
    assert_eq!(
        *body_of(&record),
        json!({"labels": [], "omitted": 0, "complete": true})
    );
    let f = fixture(|_| Reply::json(200, &json!({"labels": "nope"})));
    let failure = call_op(lop(), json!({}), 262_144, &f).await.unwrap_err();
    assert_eq!(failure.reason_str(), Some("response-shape"));
}

#[test]
fn labels_past_the_budget_are_counted_never_silently_cut() {
    let list: Vec<Json> = (0..400)
        .map(|i| json!({"id": format!("Label_{i}"), "name": format!("Label number {i}"), "type": "user"}))
        .collect();
    let body = json!({"labels": list});
    let cap = 4096;
    let out = gmail::reads::project_labels(&body, cap).unwrap().unwrap();
    let kept = out["labels"].as_array().unwrap().len();
    assert!(kept > 10 && kept < 400, "{kept}");
    assert_eq!(out["omitted"], json!(400 - kept));
    assert_eq!(out["complete"], json!(false));
    // The whole envelope fits, counters included.
    assert!(serde_json::to_vec(&out).unwrap().len() <= cap);
    // The order is Gmail's: the first ones are kept.
    assert_eq!(out["labels"][0]["id"], json!("Label_0"));
    // A budget too small for even the empty envelope is a failure, not an
    // empty list that would read as "no labels".
    let failure = gmail::reads::project_labels(&body, 10).unwrap_err();
    assert_eq!(failure.code, "E_CALL_OUTPUT_TOO_LARGE");
    assert_eq!(failure.reason_str(), Some("output-too-large"));
    // Everything fits: complete.
    let out = gmail::reads::project_labels(&body, 1024 * 1024)
        .unwrap()
        .unwrap();
    assert_eq!(
        (out["omitted"].clone(), out["complete"].clone()),
        (json!(0), json!(true))
    );
}

// ── gmail.list-message-ids: inputs and the window ────────────────────────────

#[test]
fn the_window_is_widened_a_second_each_way_in_epoch_seconds() {
    let w = ids_wire(json!({
        "label": "Label_12",
        "received-after": "2026-08-17T00:00:00Z",
        "received-before": "2026-11-15T12:00:00.250Z",
    }))
    .unwrap();
    // 2026-08-17T00:00:00Z is 1786924800; 12:00:00.250 on 2026-11-15 is
    // 1794744000.25, ceiled to 1794744001.
    assert_eq!(
        w.query,
        pairs(&[
            ("q", "after:1786924799 before:1794744002"),
            ("labelIds", "Label_12"),
            ("maxResults", "500"),
            ("includeSpamTrash", "false"),
        ])
    );
    // A fractional `after` floors; a whole `before` is not rounded up first.
    let w = ids_wire(json!({
        "from": "eng@acme.com",
        "text": "RFI 042",
        "received-after": "2026-08-17T00:00:00.999Z",
        "received-before": "2026-08-18T00:00:00Z",
        "page-size": 50,
        "page-token": "09876543210987654321",
    }))
    .unwrap();
    assert_eq!(
        w.query,
        pairs(&[
            (
                "q",
                "from:eng@acme.com RFI 042 after:1786924799 before:1787011201"
            ),
            ("maxResults", "50"),
            ("pageToken", "09876543210987654321"),
            ("includeSpamTrash", "false"),
        ])
    );
    // Equal bounds are one instant, not an empty window.
    assert!(
        ids_wire(
            json!({"label": "INBOX", "received-after": "2026-08-17T00:00:00Z",
                        "received-before": "2026-08-17T00:00:00Z"})
        )
        .is_ok()
    );
}

#[test]
fn every_bad_window_or_selection_is_refused_before_any_request() {
    let after = "2026-08-17T00:00:00Z";
    for (inputs, input) in [
        // A date window alone selects nothing.
        (json!({"received-after": after}), "text"),
        (json!({"label": "INBOX"}), "received-after"),
        (
            json!({"label": "INBOX", "received-after": "2026-08-17"}),
            "received-after",
        ),
        (
            json!({"label": "INBOX", "received-after": "2026-08-17T00:00:00+01:00"}),
            "received-after",
        ),
        (
            json!({"label": "INBOX", "received-after": "1970-01-01T00:00:00Z"}),
            "received-after",
        ),
        (
            json!({"label": "INBOX", "received-after": after, "received-before": "2026-08-16T23:59:59Z"}),
            "received-before",
        ),
        (json!({"label": "SPAM", "received-after": after}), "label"),
        (json!({"label": "trash", "received-after": after}), "label"),
        (json!({"text": "in:spam", "received-after": after}), "text"),
        (json!({"text": "a OR b", "received-after": after}), "text"),
        (
            json!({"label": "INBOX", "received-after": after, "page-size": 0}),
            "page-size",
        ),
        (
            json!({"label": "INBOX", "received-after": after, "page-size": 501}),
            "page-size",
        ),
        (
            json!({"label": "INBOX", "received-after": after, "page-token": "has space"}),
            "page-token",
        ),
        (
            json!({"label": "INBOX", "received-after": after, "page-token": ""}),
            "page-token",
        ),
        (
            json!({"label": "INBOX", "received-after": after, "page-token": "t".repeat(1025)}),
            "page-token",
        ),
        (
            json!({"label": "INBOX", "received-after": after, "page-token": "tok\u{e9}"}),
            "page-token",
        ),
    ] {
        assert_eq!(
            ids_wire(inputs.clone()).unwrap_err(),
            CallFailure::input(input),
            "{inputs}"
        );
    }
    // The token's rule is opaque printable ASCII: Gmail's digits and anything
    // base64-like pass, query-encoded on the wire.
    for token in ["09876543210987654321", "a+b/c=d~e_f-g.h", &"t".repeat(1024)] {
        let w = ids_wire(json!({"label": "INBOX", "received-after": after, "page-token": token}))
            .unwrap();
        assert!(
            w.query
                .contains(&("pageToken".to_string(), token.to_string()))
        );
    }
    assert_eq!(
        q_of(&ids_wire(json!({"label": "INBOX", "received-after": after})).unwrap()),
        Some("after:1786924799".into())
    );
}

#[test]
fn gmail_search_still_renders_exactly_as_shipped() {
    let Json::Object(inputs) = json!({"label": "INBOX"}) else {
        unreachable!()
    };
    let w = wire(gop(), &inputs).unwrap();
    assert_eq!(
        w.query,
        pairs(&[
            ("labelIds", "INBOX"),
            ("maxResults", "25"),
            ("includeSpamTrash", "false")
        ])
    );
    let Json::Object(none) = json!({}) else {
        unreachable!()
    };
    assert_eq!(wire(gop(), &none).unwrap_err(), CallFailure::input("text"));
    // A date bound is no input of gmail.search.
    let Json::Object(dated) = json!({"text": "RFI", "received-after": "2026-08-17T00:00:00Z"})
    else {
        unreachable!()
    };
    assert_eq!(
        wire(gop(), &dated).unwrap_err().reason_str(),
        Some("unknown-input")
    );
}

// ── gmail.list-message-ids: pages ────────────────────────────────────────────

#[tokio::test]
async fn a_page_returns_every_id_and_the_token_that_continues_it() {
    let f = fixture(|head| {
        let token = query_of(head)
            .into_iter()
            .find(|(k, _)| k == "pageToken")
            .map(|(_, v)| v);
        match token.as_deref() {
            None => Reply::json(
                200,
                &json!({
                    "messages": [
                        {"id": "18c1a2b3c4d5e6f7", "threadId": "18c1a2b3c4d5e6f0"},
                        {"id": "18c1a2b3c4d5e6f8", "threadId": "not a thread!"},
                        {"id": "18c1a2b3c4d5e6f7", "threadId": "18c1a2b3c4d5e6f0"},
                        {"id": "../../labels", "threadId": "x"},
                        {"id": "18c1a2b3c4d5e6f9"}
                    ],
                    "nextPageToken": "13093938138367064813",
                    "resultSizeEstimate": 201
                }),
            ),
            Some("13093938138367064813") => Reply::json(
                200,
                &json!({
                    "messages": [{"id": "18c1a2b3c4d5e6fa", "threadId": "18c1a2b3c4d5e6fa"}],
                    "resultSizeEstimate": 1
                }),
            ),
            Some(_) => Reply::json(500, &json!({})),
        }
    });
    let inputs =
        json!({"label": "Label_12", "received-after": "2026-08-17T00:00:00Z", "page-size": 2});
    let record = call_op(iop(), inputs, 262_144, &f).await.unwrap();
    // Never cut at page-size: Gmail sent five entries, all kept or counted.
    assert_eq!(
        *body_of(&record),
        json!({
            "messages": [
                {"id": "18c1a2b3c4d5e6f7", "thread-id": "18c1a2b3c4d5e6f0"},
                {"id": "18c1a2b3c4d5e6f8", "truncated": ["thread-id"]},
                {"id": "18c1a2b3c4d5e6f9", "truncated": ["thread-id"]}
            ],
            "next-page-token": "13093938138367064813",
            "more-available": true,
            "omitted": 2,
        })
    );
    let heads = data_heads(&f);
    assert_eq!(heads.len(), 1, "the token is returned, never followed");
    assert_eq!(path_only(&heads[0]), LIST_PATH);
    // The next page is the same call with the token sent back.
    let inputs = json!({"label": "Label_12", "received-after": "2026-08-17T00:00:00Z",
                        "page-size": 2, "page-token": "13093938138367064813"});
    let record = call_op(iop(), inputs, 262_144, &f).await.unwrap();
    assert_eq!(
        *body_of(&record),
        json!({
            "messages": [{"id": "18c1a2b3c4d5e6fa", "thread-id": "18c1a2b3c4d5e6fa"}],
            "more-available": false,
            "omitted": 0,
        })
    );
}

#[test]
fn a_continuation_that_cannot_be_sent_back_is_withheld_never_dropped() {
    for token in [json!("has space"), json!("t".repeat(1025)), json!(42)] {
        let out =
            gmail::reads::project_message_ids(&json!({"messages": [], "nextPageToken": token}))
                .unwrap();
        assert_eq!(
            out,
            json!({"messages": [], "next-page-token-withheld": true, "more-available": true, "omitted": 0}),
            "{token}"
        );
    }
    for end in [
        json!({}),
        json!({"nextPageToken": ""}),
        json!({"nextPageToken": null}),
    ] {
        let out = gmail::reads::project_message_ids(&end).unwrap();
        assert_eq!(out["more-available"], json!(false));
        assert!(out.get("next-page-token").is_none());
    }
    assert!(gmail::reads::project_message_ids(&json!({"messages": {}})).is_none());
    assert!(gmail::reads::project_message_ids(&json!([])).is_none());
}

#[tokio::test]
async fn a_full_page_of_maximal_ids_fits_the_raw_cap() {
    let ids: Vec<Json> = (0..500)
        .map(|i| {
            let id = format!("{i:0>64}");
            json!({"id": id, "threadId": id})
        })
        .collect();
    let body =
        json!({"messages": ids, "nextPageToken": "t".repeat(1024), "resultSizeEstimate": 5000});
    let f = fixture(move |_| Reply::json(200, &body));
    let inputs = json!({"label": "INBOX", "received-after": "2026-08-17T00:00:00Z"});
    let record = call_op(iop(), inputs, 1024 * 1024, &f).await.unwrap();
    assert_eq!(body_of(&record)["messages"].as_array().unwrap().len(), 500);
}

#[tokio::test]
async fn an_answer_over_the_callers_budget_is_too_large_with_its_size() {
    let ids: Vec<Json> = (0..100)
        .map(|i| json!({"id": format!("18c1a2b3c4d5{i:04}"), "threadId": format!("18c1a2b3c4d5{i:04}")}))
        .collect();
    let f = fixture(move |_| Reply::json(200, &json!({"messages": ids})));
    let inputs = json!({"label": "INBOX", "received-after": "2026-08-17T00:00:00Z"});
    let failure = call_op(iop(), inputs, 1024, &f).await.unwrap_err();
    assert_eq!(failure.code, "E_CALL_OUTPUT_TOO_LARGE");
    assert_eq!(failure.reason_str(), Some("output-too-large"));
    assert!(failure.details["outputBytes"].as_u64().unwrap() > 1024);
    // A raw answer over its own cap names that cap.
    let big = Reply {
        status: 200,
        headers: String::new(),
        body: vec![b' '; 300 * 1024],
        delay: Duration::ZERO,
    };
    let f = fixture(move |_| big.clone());
    let inputs = json!({"label": "INBOX", "received-after": "2026-08-17T00:00:00Z"});
    let failure = call_op(iop(), inputs, 1024 * 1024, &f).await.unwrap_err();
    assert_eq!(
        Json::Object(failure.details),
        json!({"reason": "raw-too-large", "limitBytes": 256 * 1024, "phase": "list"})
    );
}

// ── gmail.list-message-ids: failure reasons ──────────────────────────────────

/// The listing answers `list` (status); the label probe answers `probe`.
fn listing(list: u16, probe: u16) -> Fixture {
    fixture(move |head| {
        if path_only(head).starts_with(LABELS_PATH) {
            Reply::json(
                probe,
                &json!({"id": "Label_12", "name": "x", "type": "user"}),
            )
        } else {
            Reply::json(
                list,
                &json!({"error": {"code": list, "message": "must-not-leave-message"}}),
            )
        }
    })
}

#[tokio::test]
async fn a_rejected_page_token_never_reads_as_a_deleted_label() {
    let after = "2026-08-17T00:00:00Z";
    for (list, probe, inputs, reason, probed) in [
        (
            404,
            404,
            json!({"label": "Label_12", "received-after": after}),
            "label-not-found",
            true,
        ),
        (
            400,
            404,
            json!({"label": "Label_12", "received-after": after, "page-token": "123"}),
            "label-not-found",
            true,
        ),
        (
            400,
            200,
            json!({"label": "Label_12", "received-after": after, "page-token": "123"}),
            "page-token-invalid",
            true,
        ),
        (
            400,
            200,
            json!({"label": "Label_12", "received-after": after}),
            "request-invalid",
            true,
        ),
        (
            400,
            500,
            json!({"label": "Label_12", "received-after": after}),
            "request-invalid",
            true,
        ),
        (
            400,
            500,
            json!({"label": "Label_12", "received-after": after, "page-token": "123"}),
            "request-invalid",
            true,
        ),
        (
            400,
            200,
            json!({"text": "RFI", "received-after": after, "page-token": "123"}),
            "page-token-invalid",
            false,
        ),
        (
            400,
            200,
            json!({"text": "RFI", "received-after": after}),
            "request-invalid",
            false,
        ),
        (
            404,
            200,
            json!({"text": "RFI", "received-after": after}),
            "request-invalid",
            false,
        ),
    ] {
        let f = listing(list, probe);
        let failure = call_op(iop(), inputs.clone(), 262_144, &f)
            .await
            .unwrap_err();
        assert_eq!(
            Json::Object(failure.details.clone()),
            json!({"reason": reason, "status": list, "phase": "list"}),
            "{inputs}"
        );
        let probes: Vec<String> = data_heads(&f)
            .into_iter()
            .filter(|h| path_only(h).starts_with(LABELS_PATH))
            .collect();
        assert_eq!(!probes.is_empty(), probed, "{inputs}");
        if probed {
            assert_eq!(probes.len(), 1);
            assert_eq!(path_only(&probes[0]), format!("{LABELS_PATH}/Label_12"));
        }
        assert!(!format!("{failure:?}").contains("must-not-leave"));
    }
    // A rate-limited probe reports the rate limit, not a guess.
    let f = fixture(|head| {
        if path_only(head).starts_with(LABELS_PATH) {
            with_header(Reply::json(429, &json!({})), "Retry-After: 9")
        } else {
            Reply::json(400, &json!({}))
        }
    });
    let failure = call_op(
        iop(),
        json!({"label": "Label_12", "received-after": after}),
        262_144,
        &f,
    )
    .await
    .unwrap_err();
    assert_eq!(
        Json::Object(failure.details),
        json!({"reason": "rate-limited", "status": 429, "retry-after-secs": 9, "phase": "label-probe"})
    );
    // Any other listing status is the status alone, and no probe is made.
    let f = listing(500, 404);
    let failure = call_op(
        iop(),
        json!({"label": "Label_12", "received-after": after}),
        262_144,
        &f,
    )
    .await
    .unwrap_err();
    assert_eq!(
        Json::Object(failure.details),
        json!({"status": 500, "phase": "list"})
    );
    assert_eq!(data_heads(&f).len(), 1);
}

// ── gmail.get-message ────────────────────────────────────────────────────────

fn header(name: &str, value: &str) -> Json {
    json!({"name": name, "value": value})
}

fn text_part(mime: &str, text: &str) -> Json {
    json!({
        "partId": "0",
        "mimeType": mime,
        "filename": "",
        "headers": [header("Content-Type", &format!("{mime}; charset=\"UTF-8\"")),
                    header("Content-Transfer-Encoding", "quoted-printable")],
        "body": {"size": text.len(), "data": b64(text.as_bytes())}
    })
}

fn file_part(name: &str, mime: &str, size: u64) -> Json {
    json!({
        "partId": "2",
        "mimeType": mime,
        "filename": name,
        "headers": [header("Content-Type", &format!("{mime}; name=\"{name}\"")),
                    header("Content-Disposition", &format!("attachment; filename=\"{name}\""))],
        "body": {"attachmentId": "ANGjdJ8w-attachment_id-must-not-be-fetched", "size": size}
    })
}

fn multipart(mime: &str, parts: Vec<Json>) -> Json {
    json!({"mimeType": mime, "filename": "", "headers": [header("Content-Type", &format!("{mime}; boundary=\"b\""))],
           "body": {"size": 0}, "parts": parts})
}

/// A `format=full` answer as Gmail sends it, around `payload`.
fn full(payload: Json) -> Json {
    let mut payload = payload;
    let mut headers = vec![
        header("Delivered-To", "ada@example.com"),
        header(
            "Received",
            "by 2002:a05:6000:1:b0:1 with SMTP id x; Thu, 30 Oct 2026 02:12:00 -0700 (PDT)",
        ),
        header("From", "\"A. Engineer\" <engineer@acme.com>"),
        header("To", "Ada <ada@example.com>, bob@example.com"),
        header("Cc", "=?UTF-8?B?xbt1cmF3?= <crane@example.com>"),
        header("Subject", "RE: RFI 042 - beam B12 connection"),
        header("Date", "Thu, 30 Oct 2026 09:12:00 +0000"),
        header("Message-ID", "<abc@acme.com>"),
    ];
    if let Some(own) = payload.get("headers").and_then(Json::as_array) {
        headers.extend(own.iter().cloned());
    }
    payload["headers"] = Json::Array(headers);
    json!({
        "id": ID,
        "threadId": "18c1a2b3c4d5e6f0",
        "labelIds": ["INBOX", "Label_12", "CATEGORY_PERSONAL"],
        "snippet": "Please see the attached sketch",
        "sizeEstimate": 60213,
        "historyId": "991",
        "internalDate": "1793351520000",
        "payload": payload
    })
}

/// The projection with body fetches answered by `bodies` (by attachment id),
/// counting them.
fn project(item: &Json, bodies: &[(&str, Option<&str>)]) -> (Result<Json, CallFailure>, usize) {
    let mut calls = 0usize;
    let out = {
        let mut fetch = |id: &str| -> Result<Option<String>, CallFailure> {
            calls += 1;
            Ok(bodies
                .iter()
                .find(|(k, _)| *k == id)
                .and_then(|(_, v)| v.map(|t| b64(t.as_bytes()))))
        };
        gmail::reads::project_message(item, ID, &mut fetch)
    };
    (out, calls)
}

fn typical() -> Json {
    full(multipart(
        "multipart/mixed",
        vec![
            multipart(
                "multipart/alternative",
                vec![
                    text_part(
                        "text/plain",
                        "Please see the attached sketch.\r\nThe connection is fine.",
                    ),
                    text_part(
                        "text/html",
                        "<div>Please see the <b>attached</b> sketch.</div>",
                    ),
                ],
            ),
            file_part("B12-sketch.pdf", "application/pdf", 48213),
        ],
    ))
}

#[tokio::test]
async fn a_message_is_read_once_in_full_and_projected() {
    let f = fixture(|_| Reply::json(200, &typical()));
    let record = call_op(mop(), json!({"id": ID}), 262_144, &f)
        .await
        .unwrap();
    let heads = data_heads(&f);
    assert_eq!(heads.len(), 1, "an attachment is never fetched");
    assert_eq!(path_only(&heads[0]), format!("{LIST_PATH}/{ID}"));
    assert_eq!(query_of(&heads[0]), pairs(&[("format", "full")]));
    assert!(heads[0].contains(&format!("Bearer {TOKEN}")));
    assert_eq!(
        *body_of(&record),
        json!({
            "id": ID,
            "thread-id": "18c1a2b3c4d5e6f0",
            "label-ids": ["INBOX", "Label_12", "CATEGORY_PERSONAL"],
            "label-ids-complete": true,
            "received-at": "2026-10-30T09:12:00.000Z",
            "subject": "RE: RFI 042 - beam B12 connection",
            "from": {"address": "engineer@acme.com", "name": "A. Engineer"},
            "to": [{"address": "ada@example.com", "name": "Ada"}, {"address": "bob@example.com"}],
            "cc": [{"address": "crane@example.com", "name": "Żuraw"}],
            "recipients-omitted": 0,
            "text": "Please see the attached sketch.\nThe connection is fine.",
            "text-source": "text/plain",
            "text-state": "complete",
            "attachments": [{"name": "B12-sketch.pdf", "type": "application/pdf", "size": 48213}],
            "attachments-omitted": 0,
            "attachments-state": "complete",
        })
    );
}

#[test]
fn an_html_only_message_gives_its_html_as_text() {
    let item = full(text_part(
        "text/html",
        "<html><head><style>p{color:red}</style><script>alert(1)</script></head><body><p>Beam&nbsp;B12 &amp; C4</p><img src=\"https://tracker.example/x.png\"><table><tr><td>12</td><td>34</td></tr></table></body></html>",
    ));
    let (out, calls) = project(&item, &[]);
    let out = out.unwrap();
    assert_eq!(calls, 0);
    assert_eq!(out["text"], json!("Beam B12 & C4\n12\t34"));
    assert_eq!(out["text-source"], json!("text/html"));
    assert_eq!(out["text-state"], json!("complete"));
    // A tracking image is neither fetched nor named: it is markup, not a part.
    assert_eq!(out["attachments"], json!([]));
}

#[test]
fn an_alternative_prefers_plain_and_falls_back_to_html() {
    // HTML first in the alternative: plain still wins.
    let item = full(multipart(
        "multipart/alternative",
        vec![
            text_part("text/html", "<p>html</p>"),
            text_part("text/plain", "plain"),
        ],
    ));
    assert_eq!(project(&item, &[]).0.unwrap()["text"], json!("plain"));
    // A related HTML tree with an inline image, no plain version.
    let mut image = file_part("", "image/png", 99);
    image["headers"] = json!([
        header("Content-ID", "<img1>"),
        header("Content-Disposition", "inline")
    ]);
    let item = full(multipart(
        "multipart/alternative",
        vec![multipart(
            "multipart/related",
            vec![text_part("text/html", "<p>Only <i>HTML</i></p>"), image],
        )],
    ));
    let out = project(&item, &[]).0.unwrap();
    assert_eq!(out["text"], json!("Only HTML"));
    assert_eq!(out["text-source"], json!("text/html"));
    // The inline image is listed (unnamed), never read.
    assert_eq!(
        out["attachments"],
        json!([{"type": "image/png", "size": 99}])
    );
}

#[test]
fn text_is_never_read_from_an_attachment_or_an_enclosed_message() {
    let mut named = text_part("text/plain", "ATTACHED-NOTES-MUST-NOT-BE-TEXT");
    named["filename"] = json!("notes.txt");
    let mut disposed = text_part("text/plain", "DISPOSED-MUST-NOT-BE-TEXT");
    disposed["headers"] = json!([header("Content-Disposition", "attachment")]);
    let mut enclosed = multipart(
        "multipart/alternative",
        vec![text_part("text/plain", "ENCLOSED-MUST-NOT-BE-TEXT")],
    );
    enclosed["mimeType"] = json!("message/rfc822");
    enclosed["filename"] = json!("");
    let mut named_tree = multipart(
        "multipart/mixed",
        vec![text_part("text/plain", "NAMED-TREE-MUST-NOT-BE-TEXT")],
    );
    named_tree["filename"] = json!("bundle.eml");
    let item = full(multipart(
        "multipart/mixed",
        vec![
            text_part("text/plain", "The body."),
            named,
            disposed,
            enclosed,
            named_tree,
        ],
    ));
    let (out, calls) = project(&item, &[]);
    let out = out.unwrap();
    assert_eq!(calls, 0);
    assert_eq!(out["text"], json!("The body."));
    assert!(!out.to_string().contains("MUST-NOT-BE-TEXT"));
    let names: Vec<Json> = out["attachments"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a.get("name").cloned().unwrap_or(json!(null)))
        .collect();
    assert_eq!(
        names,
        [
            json!("notes.txt"),
            json!(null),
            json!(null),
            json!("bundle.eml")
        ]
    );
    assert_eq!(out["attachments"][2]["type"], json!("message/rfc822"));
}

#[test]
fn mixed_bodies_are_joined_in_order() {
    let item = full(multipart(
        "multipart/mixed",
        vec![
            text_part("text/plain", "First section."),
            file_part("a.pdf", "application/pdf", 1),
            text_part("text/html", "<p>Second <b>section</b>.</p>"),
        ],
    ));
    let out = project(&item, &[]).0.unwrap();
    assert_eq!(out["text"], json!("First section.\nSecond section."));
    assert_eq!(out["text-source"], json!("mixed"));
}

#[test]
fn any_file_name_but_the_empty_string_makes_a_part_an_attachment() {
    // Spaces only, or a name Gmail sent as something other than a string:
    // still a file, so it is listed and neither read nor fetched.
    let mut spaces = text_part("text/plain", "SPACES-NAME-MUST-NOT-BE-TEXT");
    spaces["filename"] = json!("   ");
    let mut stored_apart = text_part("text/plain", "");
    stored_apart["filename"] = json!(" ");
    stored_apart["body"] = json!({"attachmentId": "ANGjdJ8w-named-body", "size": 9});
    let mut odd = text_part("text/plain", "ODD-NAME-MUST-NOT-BE-TEXT");
    odd["filename"] = json!(7);
    let item = full(multipart(
        "multipart/mixed",
        vec![
            text_part("text/plain", "The body."),
            spaces,
            stored_apart,
            odd,
        ],
    ));
    let (out, calls) = project(&item, &[("ANGjdJ8w-named-body", Some("NAMED"))]);
    let out = out.unwrap();
    assert_eq!(calls, 0, "a named part is never fetched");
    assert_eq!(out["text"], json!("The body."));
    assert_eq!(out["text-state"], json!("complete"));
    assert!(!out.to_string().contains("MUST-NOT-BE-TEXT"));
    assert_eq!(out["attachments"].as_array().unwrap().len(), 3);
}

#[test]
fn a_cut_text_is_named_even_when_another_state_comes_first() {
    let mut unreadable = text_part("text/plain", "");
    unreadable["body"]["data"] = json!("!!!not base64!!!");
    let mut apart = text_part("text/plain", "");
    apart["body"] = json!({"attachmentId": "ANGjdJ8w-gone-body", "size": 9});
    for (other, state) in [(unreadable, "undecodable"), (apart, "external-not-fetched")] {
        let item = full(multipart(
            "multipart/mixed",
            vec![text_part("text/plain", &"word ".repeat(30_000)), other],
        ));
        let out = project(&item, &[]).0.unwrap();
        assert_eq!(out["text-state"], json!(state));
        assert_eq!(out["text"].as_str().unwrap().chars().count(), 65_536);
        assert!(
            out["truncated"]
                .as_array()
                .unwrap()
                .contains(&json!("text")),
            "{state}: the cut is still named"
        );
    }
    // And no cut, no claim.
    let out = project(&typical(), &[]).0.unwrap();
    assert!(out.get("truncated").is_none());
}

#[test]
fn a_charset_is_decoded_and_an_unknown_one_is_named() {
    // "Żuraw" in ISO-8859-2: Ż is 0xAF.
    let latin2: &[u8] = &[0xAF, b'u', b'r', b'a', b'w'];
    let mut part = text_part("text/plain", "");
    part["headers"] = json!([header("Content-Type", "text/plain; charset=ISO-8859-2")]);
    part["body"]["data"] = json!(b64(latin2));
    let out = project(&full(part.clone()), &[]).0.unwrap();
    assert_eq!(out["text"], json!("Żuraw"));
    // Padding or none, both read.
    part["body"]["data"] = json!(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(latin2));
    assert_eq!(
        project(&full(part.clone()), &[]).0.unwrap()["text"],
        json!("Żuraw")
    );
    part["headers"] = json!([header("Content-Type", "text/plain; charset=x-klingon")]);
    let out = project(&full(part.clone()), &[]).0.unwrap();
    assert_eq!(out["text-state"], json!("undecodable"));
    assert!(out.get("text").is_none());
    part["headers"] = json!([header("Content-Type", "text/plain")]);
    part["body"]["data"] = json!("!!!not base64!!!");
    assert_eq!(
        project(&full(part), &[]).0.unwrap()["text-state"],
        json!("undecodable")
    );
}

#[test]
fn a_body_stored_apart_is_fetched_within_its_bound_or_named() {
    let mut external = text_part("text/plain", "");
    external["body"] = json!({"attachmentId": "ANGjdJ_body-part-1", "size": 120000});
    let item = full(multipart("multipart/mixed", vec![external.clone()]));
    let (out, calls) = project(&item, &[("ANGjdJ_body-part-1", Some("The long body."))]);
    let out = out.unwrap();
    assert_eq!(calls, 1);
    assert_eq!(out["text"], json!("The long body."));
    assert_eq!(out["text-state"], json!("complete"));
    // Not fetched (gone, too large): said, never "no body".
    let (out, _) = project(&item, &[]);
    let out = out.unwrap();
    assert_eq!(out["text-state"], json!("external-not-fetched"));
    assert!(out.get("text").is_none());
    // At most four body reads per message.
    let parts: Vec<Json> = (0..6)
        .map(|i| {
            let mut p = external.clone();
            p["body"]["attachmentId"] = json!(format!("body{i}"));
            p
        })
        .collect();
    let bodies: Vec<(String, Option<&str>)> =
        (0..6).map(|i| (format!("body{i}"), Some("x"))).collect();
    let bodies: Vec<(&str, Option<&str>)> = bodies.iter().map(|(k, v)| (k.as_str(), *v)).collect();
    let (out, calls) = project(&full(multipart("multipart/mixed", parts)), &bodies);
    assert_eq!(calls, 4);
    assert_eq!(out.unwrap()["text-state"], json!("external-not-fetched"));
}

#[tokio::test]
async fn a_body_read_goes_to_the_message_route_and_its_failures_are_its_own() {
    let mut external = text_part("text/plain", "");
    external["body"] = json!({"attachmentId": "ANGjdJ_body-part-1", "size": 120000});
    let item = full(multipart(
        "multipart/mixed",
        vec![external, file_part("a.pdf", "application/pdf", 5)],
    ));
    for (status, expected) in [
        (200, Ok("The long body.")),
        (404, Err(json!(null))),
        (
            429,
            Err(json!({"reason": "rate-limited", "status": 429, "phase": "body"})),
        ),
        (500, Err(json!({"status": 500, "phase": "body"}))),
    ] {
        let item = item.clone();
        let f = fixture(move |head| {
            if path_only(head).contains("/attachments/") {
                Reply::json(status, &json!({"size": 14, "data": b64(b"The long body.")}))
            } else {
                Reply::json(200, &item)
            }
        });
        let out = call_op(mop(), json!({"id": ID}), 262_144, &f).await;
        let fetches: Vec<String> = data_heads(&f)
            .into_iter()
            .filter(|h| path_only(h).contains("/attachments/"))
            .collect();
        assert_eq!(
            fetches.len(),
            1,
            "{status}: only the body part, never a.pdf"
        );
        assert_eq!(
            path_only(&fetches[0]),
            format!("{LIST_PATH}/{ID}/attachments/ANGjdJ_body-part-1")
        );
        match expected {
            Ok(text) => assert_eq!(body_of(&out.unwrap())["text"], json!(text)),
            Err(Json::Null) => {
                assert_eq!(
                    body_of(&out.unwrap())["text-state"],
                    json!("external-not-fetched")
                );
            }
            Err(details) => assert_eq!(Json::Object(out.unwrap_err().details), details),
        }
    }
}

#[tokio::test]
async fn an_unusable_body_id_is_never_a_url_segment() {
    for bad in ["../../../labels", "a b", "a/b", "a%2Fb", ""] {
        let mut external = text_part("text/plain", "");
        external["body"] = json!({"attachmentId": bad, "size": 10});
        let item = full(multipart("multipart/mixed", vec![external]));
        let f = fixture(move |_| Reply::json(200, &item));
        let record = call_op(mop(), json!({"id": ID}), 262_144, &f)
            .await
            .unwrap();
        assert_eq!(
            body_of(&record)["text-state"],
            json!("external-not-fetched"),
            "{bad}"
        );
        assert_eq!(data_heads(&f).len(), 1, "{bad}: only the message read");
    }
}

#[tokio::test]
async fn a_gone_message_is_gone_and_another_id_fails() {
    let f = fixture(|_| Reply::json(404, &json!({"error": {"code": 404}})));
    let failure = call_op(mop(), json!({"id": ID}), 262_144, &f)
        .await
        .unwrap_err();
    assert_eq!(
        Json::Object(failure.details),
        json!({"reason": "gone", "status": 404, "phase": "message"})
    );
    let mut other = typical();
    other["id"] = json!("18c1a2b3c4d5e6ff");
    let f = fixture(move |_| Reply::json(200, &other));
    let failure = call_op(mop(), json!({"id": ID}), 262_144, &f)
        .await
        .unwrap_err();
    assert_eq!(failure.reason_str(), Some("id-mismatch"));
    // A bad id is refused before any request.
    for bad in ["", "../labels", "a b", &"a".repeat(65)] {
        let f = fixture(|_| Reply::json(200, &typical()));
        let failure = call_op(mop(), json!({"id": bad}), 262_144, &f)
            .await
            .unwrap_err();
        assert_eq!(failure, CallFailure::input("id"), "{bad}");
        assert!(f.requests().is_empty());
    }
}

#[tokio::test]
async fn a_message_over_the_budget_steps_up_by_its_size() {
    let long = "word ".repeat(30_000);
    let item = full(text_part("text/plain", &long));
    let f = fixture(move |_| Reply::json(200, &item));
    let failure = call_op(mop(), json!({"id": ID}), 16 * 1024, &f)
        .await
        .unwrap_err();
    assert_eq!(failure.code, "E_CALL_OUTPUT_TOO_LARGE");
    assert_eq!(failure.reason_str(), Some("output-too-large"));
    let size = failure.details["outputBytes"].as_u64().unwrap();
    // Retrying with the size it named succeeds; the text is cut and said so.
    let record = call_op(mop(), json!({"id": ID}), size, &f).await.unwrap();
    let body = body_of(&record);
    assert_eq!(body["text-state"], json!("truncated"));
    assert_eq!(body["text"].as_str().unwrap().chars().count(), 65_536);
    assert!(
        body["truncated"]
            .as_array()
            .unwrap()
            .contains(&json!("text"))
    );
}

#[test]
fn label_ids_are_mandatory_and_say_when_they_are_incomplete() {
    let mut item = typical();
    for (labels, kept, complete) in [
        (json!(null), json!([]), true),
        (
            json!(["INBOX", "bad label!", "Label_1"]),
            json!(["INBOX", "Label_1"]),
            false,
        ),
        (json!("INBOX"), json!([]), false),
        (json!(["INBOX", 7]), json!(["INBOX"]), false),
    ] {
        item["labelIds"] = labels.clone();
        let out = project(&item, &[]).0.unwrap();
        assert_eq!(out["label-ids"], kept, "{labels}");
        assert_eq!(out["label-ids-complete"], json!(complete), "{labels}");
    }
    let many: Vec<String> = (0..501).map(|i| format!("Label_{i}")).collect();
    item["labelIds"] = json!(many);
    let out = project(&item, &[]).0.unwrap();
    assert_eq!(out["label-ids"].as_array().unwrap().len(), 500);
    assert_eq!(out["label-ids-complete"], json!(false));
    item.as_object_mut().unwrap().remove("labelIds");
    let out = project(&item, &[]).0.unwrap();
    assert_eq!(
        (out["label-ids"].clone(), out["label-ids-complete"].clone()),
        (json!([]), json!(true))
    );
}

#[test]
fn every_list_is_bounded_and_says_so() {
    // Recipients past 100 are counted.
    let to: Vec<String> = (0..120).map(|i| format!("p{i}@example.com")).collect();
    let mut item = typical();
    item["payload"]["headers"] = json!([header("To", &to.join(", "))]);
    let out = project(&item, &[]).0.unwrap();
    assert_eq!(out["to"].as_array().unwrap().len(), 100);
    assert_eq!(out["recipients-omitted"], json!(20));
    assert_eq!(out["cc"], json!([]));
    // Attachments past 50 are counted and the list is partial.
    let files: Vec<Json> = (0..55)
        .map(|i| file_part(&format!("f{i}.pdf"), "application/pdf", 1))
        .collect();
    let mut long = file_part(&"n".repeat(1100), "application/pdf", 1);
    long["mimeType"] = json!(format!("application/{}", "x".repeat(300)));
    let mut parts = vec![long];
    parts.extend(files);
    let out = project(&full(multipart("multipart/mixed", parts)), &[])
        .0
        .unwrap();
    assert_eq!(out["attachments"].as_array().unwrap().len(), 50);
    assert_eq!(out["attachments-omitted"], json!(6));
    assert_eq!(out["attachments-state"], json!("partial"));
    assert_eq!(out["attachments"][0]["truncated"], json!(["name", "type"]));
    assert_eq!(out["text-state"], json!("none"));
    // A tree past the depth or part cap: never complete, never "none".
    let mut deep = text_part("text/plain", "deep text");
    for _ in 0..20 {
        deep = multipart("multipart/mixed", vec![deep]);
    }
    let out = project(
        &full(multipart(
            "multipart/mixed",
            vec![text_part("text/plain", "top"), deep],
        )),
        &[],
    )
    .0
    .unwrap();
    assert_eq!(out["text-state"], json!("mime-cut"));
    assert_eq!(out["text"], json!("top"));
    assert_eq!(out["attachments-state"], json!("partial"));
    assert!(
        out["truncated"]
            .as_array()
            .unwrap()
            .contains(&json!("parts"))
    );
    let wide: Vec<Json> = (0..300)
        .map(|i| text_part("text/plain", &format!("s{i}")))
        .collect();
    let out = project(&full(multipart("multipart/mixed", wide)), &[])
        .0
        .unwrap();
    assert_eq!(out["text-state"], json!("mime-cut"));
}

#[test]
fn missing_fields_are_named_never_invented() {
    let item = json!({"id": ID, "payload": {"mimeType": "multipart/mixed", "parts": []}});
    let out = project(&item, &[]).0.unwrap();
    assert_eq!(
        out,
        json!({
            "id": ID,
            "label-ids": [],
            "label-ids-complete": true,
            "to": [],
            "cc": [],
            "recipients-omitted": 0,
            "text-state": "none",
            "attachments": [],
            "attachments-omitted": 0,
            "attachments-state": "complete",
            "truncated": ["thread-id", "received-at"],
        })
    );
    // An empty plain part is an empty body, read in full.
    let out = project(&full(text_part("text/plain", "")), &[]).0.unwrap();
    assert_eq!(
        (out["text"].clone(), out["text-state"].clone()),
        (json!(""), json!("complete"))
    );
}

// ── digests ──────────────────────────────────────────────────────────────────

#[test]
fn the_new_rows_have_their_own_canonical_forms_and_gmail_search_keeps_its_own() {
    for op in [lop(), iop(), mop()] {
        let canonical = op.canonical();
        assert_eq!(canonical["canonical"], json!("v2"));
        assert_eq!(canonical["command"], json!(op.command));
        assert_eq!(canonical["rateLimit"]["reason"], json!("rate-limited"));
        assert_eq!(canonical["bindsAccount"], json!(true));
    }
    assert!(gop().canonical().get("rateLimit").is_none());
    let digests: std::collections::HashSet<String> =
        OPERATIONS.iter().map(Operation::sha256).collect();
    assert_eq!(
        digests.len(),
        OPERATIONS.len(),
        "every row's digest is its own"
    );
}

/// The digests FloLess reviews: a change to any of these rows must be a
/// deliberate re-review, never a side effect.
#[test]
fn the_new_rows_digests_are_pinned() {
    assert_eq!(
        lop().sha256(),
        "4402e06da2eb13b68157f4a139d208c9f71fca24ea6d0a7c24a0cc855460d7a6"
    );
    assert_eq!(
        iop().sha256(),
        "148f85bda55541545634fdf0496fc2ab35b203f2600e54b51dabc327cb752bcc"
    );
    assert_eq!(
        mop().sha256(),
        "139ff48f72daf472c0db49b3da7e9248f551760154f394d580d333c7e85fd885"
    );
}

#[test]
fn every_security_relevant_change_moves_a_new_rows_digest() {
    let base = mop().sha256();
    let mut changed = mop().clone();
    changed.fixed_query = &[("format", "raw")];
    assert_ne!(changed.sha256(), base);
    let mut changed = mop().clone();
    changed.any_of_scopes = &[DRIVE_READONLY];
    assert_ne!(changed.sha256(), base);
    let mut changed = iop().clone();
    changed.fixed_query = &[];
    assert_ne!(changed.sha256(), iop().sha256());
    let mut changed = lop().clone();
    changed.binds_account = false;
    assert_ne!(changed.sha256(), lop().sha256());
}

// ── the shipped manifest ─────────────────────────────────────────────────────

#[test]
fn the_2_4_0_manifest_offers_the_three_reads_with_exactly_the_reviewed_inputs() {
    let raw: serde_yaml::Value = serde_yaml::from_str(&shipped_manifest()).unwrap();
    assert_eq!(raw["version"].as_str(), Some("2.4.0"));
    assert_eq!(raw["minimum-cli-version"].as_str(), Some("0.179.0"));
    for op in [lop(), iop(), mop()] {
        let command = &raw["commands"][op.command];
        assert!(command["status"].is_null(), "{} is runnable", op.command);
        assert_eq!(command["mode"].as_str(), Some("read"));
        let declared: Vec<String> = command["inputs"]
            .as_mapping()
            .map(|m| {
                m.keys()
                    .filter_map(|k| k.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        let reviewed: Vec<String> = op.inputs.iter().map(|i| i.source.to_string()).collect();
        assert_eq!(declared, reviewed, "{}", op.command);
        let scopes: Vec<&str> = command["permissions"]["scopes"]
            .as_sequence()
            .unwrap()
            .iter()
            .filter_map(|s| s.as_str())
            .collect();
        assert_eq!(scopes, op.request_scopes, "{}", op.command);
    }
}
