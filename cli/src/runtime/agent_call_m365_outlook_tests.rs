//! Outlook mail-folder reads (#763): folders, a folder's changes, one
//! message. A child of the `list-folder` tests, so it shares their Graph
//! fixture and helpers.

use super::*;

/// Real-shaped Outlook ids: base64url with `-`, `_` and `=` padding.
const FOLDER: &str = "AAMkAGI2THVSAAA-ZTk3LWI4ODAtNDg5NC05ZjNkLTFlNTZmMmI2NzQyNgAuAAAAAAAiQ8W967B7TKBjgx9rVEURAQAiIsqMbYjsT5e-T7KzowPTAAAAAAEMAAA=";
const PARENT: &str = "AAMkAGI2THVSAAA-ZTk3LWI4ODAtNDg5NC05ZjNkLTFlNTZmMmI2NzQyNgAuAAAAAAAiQ8W967B7TKBjgx9rVEURAQAiIsqMbYjsT5e-T7KzowPTAAAAAAEIAAA=";
/// What a person who connected with FloLess's read set holds for mail.
const MAIL_SCOPES: &str = "openid profile offline_access User.Read Mail.Read";

fn read(command: &str) -> &'static Operation {
    operation("microsoft-365", command).unwrap()
}

fn request_for(home: &Path, command: &str, inputs: Json) -> Map<String, Json> {
    let mut map = ms_request(home, inputs);
    map.insert("command".into(), json!(command));
    map.insert(
        "expectedOperationSha256".into(),
        json!(read(command).sha256()),
    );
    map
}

fn query_of(head: &str) -> Vec<(String, String)> {
    let path = path_of(head);
    let url = url::Url::parse(&format!("http://x{path}")).unwrap();
    url.query_pairs()
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect()
}

fn path_only(head: &str) -> String {
    path_of(head).split('?').next().unwrap_or("").to_string()
}

fn data_heads(f: &Fixture) -> Vec<String> {
    f.requests()
        .into_iter()
        .filter(|h| !is_identity_path(&path_of(h)))
        .collect()
}

async fn call_read(
    command: &str,
    scopes: &str,
    inputs: Json,
    data: Json,
) -> (Result<Json, CallFailure>, Fixture) {
    let h = ms_home();
    ms_store(h.path(), None, scopes);
    let f = graph(me(OID, UPN), Reply::json(200, &data));
    let out = call_with(
        h.path(),
        &parsed(&request_for(h.path(), command, inputs)),
        &ms_endpoints(&f),
    )
    .await;
    (out, f)
}

fn body_of(result: Result<Json, CallFailure>) -> Json {
    result.unwrap()["result"]["body"].clone()
}

fn folder(id: &str, name: &str) -> Json {
    json!({
        "id": id,
        "displayName": name,
        "parentFolderId": PARENT,
        "childFolderCount": 2,
        "totalItemCount": 318,
        "unreadItemCount": 4,
    })
}

// ── grammars (no network) ────────────────────────────────────────────────────

#[test]
fn outlook_ids_are_base64url_with_at_most_two_padding_characters() {
    let max = format!("{}==", "A".repeat(510));
    for good in [FOLDER, PARENT, "AQMkAD", "a_b-C9", "AAMk=", "AAMk==", &max] {
        assert!(is_outlook_id(good), "{good}");
        assert!(IdKind::OutlookId.accepts(good), "{good}");
        assert!(IdKind::OutlookFolderRef.accepts(good), "{good}");
    }
    let too_long = format!("{}==", "A".repeat(511));
    for bad in [
        "",
        "=",
        "==",
        "AAMk===",
        "AA=Mk",
        ".",
        "..",
        "AA.Mk",
        "a/b",
        "AA%2FMk",
        "AA+Mk",
        "AA:Mk",
        "AA Mk",
        "AA\u{0}Mk",
        "AAMk\n",
        "AAMkü",
        &too_long,
    ] {
        assert!(!is_outlook_id(bad), "{bad:?}");
        assert!(!IdKind::OutlookId.accepts(bad), "{bad:?}");
        assert!(!IdKind::OutlookFolderRef.accepts(bad), "{bad:?}");
    }
}

#[test]
fn a_well_known_folder_name_is_a_folder_reference_but_never_an_id() {
    for name in [
        "msgfolderroot",
        "Inbox",
        "SENTITEMS",
        "deleteditems",
        "archive",
    ] {
        assert!(IdKind::OutlookFolderRef.accepts(name), "{name}");
        assert!(!IdKind::OutlookId.accepts(name), "{name}");
    }
    // An id that merely starts like a name is an id.
    assert!(IdKind::OutlookId.accepts("inboxAAMk"));
}

// ── folder.list ──────────────────────────────────────────────────────────────

#[test]
fn the_folder_list_sends_a_fixed_select_page_size_and_skip() {
    let op = read("outlook.mail.folder.list");
    let wire = wire(op, &map(json!({"parent-id": "MsgFolderRoot"}))).unwrap();
    // A well-known name is sent in its one lower-case spelling.
    assert_eq!(
        wire.segments,
        ["v1.0", "me", "mailFolders", "msgfolderroot", "childFolders"]
    );
    assert_eq!(
        wire.query,
        [
            ("$top".to_string(), "50".to_string()),
            ("$skip".to_string(), "0".to_string()),
            ("$select".to_string(), MAIL_FOLDERS_SELECT.to_string()),
        ]
    );
    let wire = wire_for(op, json!({"parent-id": FOLDER, "page-size": 2, "skip": 40}));
    assert_eq!(wire.segments[3], FOLDER);
    assert_eq!(wire.skip, 40);
    assert_eq!(wire.query[0], ("$top".to_string(), "2".to_string()));
    assert_eq!(wire.query[1], ("$skip".to_string(), "40".to_string()));
    for bad in [
        json!({}),
        json!({"parent-id": "a/b"}),
        json!({"parent-id": FOLDER, "page-size": 101}),
        json!({"parent-id": FOLDER, "page-size": 0}),
        json!({"parent-id": FOLDER, "skip": -1}),
        json!({"parent-id": FOLDER, "skip": 100_001}),
        json!({"parent-id": FOLDER, "$filter": "x"}),
    ] {
        assert!(wire_err(op, bad.clone()), "{bad}");
    }
}

fn wire_for(op: &Operation, inputs: Json) -> Wire {
    wire(op, &map(inputs)).unwrap()
}

fn wire_err(op: &Operation, inputs: Json) -> bool {
    wire(op, &map(inputs)).is_err()
}

fn folders_ctx(skip: u64) -> ProjectionContext {
    ProjectionContext {
        limit: 2,
        skip,
        since: None,
        range: None,
    }
}

#[test]
fn next_skip_counts_every_entry_graph_returned_so_nothing_repeats() {
    let page = json!({
        "value": [folder(FOLDER, "Project 2411"), folder("bad/id", "Bad"), folder(PARENT, "RFIs")],
        "@odata.nextLink": "https://graph.microsoft.com/v1.0/me/mailFolders/x/childFolders?$skip=43",
    });
    let out = Projection::OutlookFolders
        .apply(&page, folders_ctx(40))
        .unwrap();
    // No local cut: three entries arrived with page-size 2, all are kept
    // (but the malformed one, counted), and the next page starts after all
    // three Graph sent.
    assert_eq!(out["folders"].as_array().unwrap().len(), 2);
    assert_eq!(out["omitted"], json!(1));
    assert_eq!(out["more-available"], json!(true));
    assert_eq!(out["next-skip"], json!(43));
    assert!(out.get("limit-reached").is_none());
    let first = &out["folders"][0];
    assert_eq!(first["id"], json!(FOLDER));
    assert_eq!(first["name"], json!("Project 2411"));
    assert_eq!(first["parent-id"], json!(PARENT));
    assert_eq!(first["child-count"], json!(2));
    assert_eq!(first["total-count"], json!(318));
    assert_eq!(first["unread-count"], json!(4));
}

#[test]
fn the_last_page_offers_no_next_skip_and_a_far_one_says_the_limit_is_reached() {
    let last = json!({"value": [folder(FOLDER, "A")]});
    let out = Projection::OutlookFolders
        .apply(&last, folders_ctx(0))
        .unwrap();
    assert_eq!(out["more-available"], json!(false));
    assert!(out.get("next-skip").is_none());
    let far = json!({
        "value": [folder(FOLDER, "A"), folder(PARENT, "B")],
        "@odata.nextLink": "https://graph.microsoft.com/v1.0/x",
    });
    let out = Projection::OutlookFolders
        .apply(&far, folders_ctx(99_999))
        .unwrap();
    assert!(out.get("next-skip").is_none());
    assert_eq!(out["limit-reached"], json!(true));
    let edge = Projection::OutlookFolders
        .apply(&far, folders_ctx(99_998))
        .unwrap();
    assert_eq!(edge["next-skip"], json!(100_000));
}

#[test]
fn a_malformed_parent_id_is_withheld_and_bad_counts_are_left_out() {
    let mut odd = folder(FOLDER, "A");
    odd["parentFolderId"] = json!("../x");
    odd["childFolderCount"] = json!(-1);
    odd["totalItemCount"] = json!("318");
    odd["unreadItemCount"] = Json::Null;
    let mut rootish = folder(PARENT, "B");
    rootish["parentFolderId"] = Json::Null;
    let out = Projection::OutlookFolders
        .apply(&json!({"value": [odd, rootish]}), folders_ctx(0))
        .unwrap();
    let a = &out["folders"][0];
    assert!(a.get("parent-id").is_none());
    assert_eq!(a["parent-id-withheld"], json!(true));
    for key in ["child-count", "total-count", "unread-count"] {
        assert!(a.get(key).is_none(), "{key}");
    }
    let b = &out["folders"][1];
    assert!(b.get("parent-id").is_none());
    assert!(b.get("parent-id-withheld").is_none());
    assert_eq!(out["omitted"], json!(0));
}

#[tokio::test]
async fn a_folder_list_call_reads_the_child_folders_with_mail_read_basic_alone() {
    let data = json!({"value": [folder(FOLDER, "Project 2411")]});
    let basic = "openid profile offline_access User.Read Mail.ReadBasic";
    let (out, f) = call_read(
        "outlook.mail.folder.list",
        basic,
        json!({"parent-id": "msgfolderroot"}),
        data,
    )
    .await;
    let body = body_of(out);
    assert_eq!(body["folders"][0]["id"], json!(FOLDER));
    let heads = data_heads(&f);
    assert_eq!(heads.len(), 1);
    assert_eq!(
        path_only(&heads[0]),
        "/v1.0/me/mailFolders/msgfolderroot/childFolders"
    );
    let query = query_of(&heads[0]);
    assert!(query.contains(&("$select".to_string(), MAIL_FOLDERS_SELECT.to_string())));
    assert!(!heads[0].to_ascii_lowercase().contains("prefer:"));
}

#[tokio::test]
async fn without_a_mail_scope_the_folder_list_sends_nothing_for_data() {
    let (out, f) = call_read(
        "outlook.mail.folder.list",
        "openid profile offline_access User.Read Files.Read.All",
        json!({"parent-id": "msgfolderroot"}),
        json!({"value": []}),
    )
    .await;
    assert_eq!(out.unwrap_err().code, "E_CALL_SCOPE_MISSING");
    assert!(data_heads(&f).is_empty());
    let _ = MAIL_SCOPES;
}

// ── digests of every row shipped before #763 ─────────────────────────────────

/// Every reviewed row on `main` before #763 (v0.174.0), with its digest,
/// captured from that build — not from this table. Adding the Outlook rows
/// must move none of them.
const PRE_763_DIGESTS: &[(&str, &str, &str)] = &[
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
];

/// The rows #763 adds.
pub(super) const OUTLOOK_763: &[&str] = &[
    "outlook.mail.folder.list",
    "outlook.mail.folder.list-messages",
    "outlook.mail.folder.list-message-ids",
    "outlook.mail.get-message",
];

#[test]
fn every_row_shipped_before_763_keeps_its_digest() {
    let actual: Vec<(&str, &str, String)> = OPERATIONS
        .iter()
        .filter(|op| !OUTLOOK_763.contains(&op.command))
        // The Trimble Connect rows (#788) pin their own digests.
        .filter(|op| op.agent != "trimble-connect")
        // `gmail.search` pins its own digest (agent_call_gmail_tests.rs).
        .filter(|op| op.command != "gmail.search")
        .map(|op| (op.agent, op.command, op.sha256()))
        .collect();
    let pinned: Vec<(&str, &str, String)> = PRE_763_DIGESTS
        .iter()
        .map(|(a, c, d)| (*a, *c, (*d).to_string()))
        .collect();
    assert_eq!(actual, pinned);
    // And every #763 row is in the table.
    for command in OUTLOOK_763 {
        assert!(operation("microsoft-365", command).is_some(), "{command}");
    }
}

// ── get-message ──────────────────────────────────────────────────────────────

const MESSAGE: &str = "AAkALgAAAAAAHYQDEapmEc2byACqAC-EWg0AWGZ2nVTOvEucmpChI3lNLAAAZQVE4AAA=";

fn message() -> Json {
    json!({
        "@odata.type": "#microsoft.graph.message",
        "id": MESSAGE,
        "internetMessageId": "<CAF7xYz@mail.acme.com>",
        "conversationId": "AAQkAGI2THVSAAA-ZTk3LWI4ODAtNDg5NC05ZjNkLTFlNTZmMmI2NzQyNgAQAH2Vqa=",
        "subject": "RE: RFI 042 - beam B12",
        "from": {"emailAddress": {"address": "engineer@acme.com", "name": "A. Engineer"}},
        "toRecipients": [{"emailAddress": {"address": "ada@contoso.com", "name": "Ada"}}],
        "ccRecipients": [],
        "receivedDateTime": "2026-10-30T09:12:00Z",
        "sentDateTime": "2026-10-30T09:11:58Z",
        "lastModifiedDateTime": "2026-10-30T09:12:01Z",
        "hasAttachments": true,
        "bodyPreview": "The reaction at grid C is 120 kN",
        "webLink": "https://outlook.office365.com/owa/?ItemID=AAkAL&exvsurl=1",
        "isDraft": false,
        "parentFolderId": FOLDER,
        "changeKey": "CQAAABYAAABYZnadVM68S5yakKEjeU0sAABlCHts",
        "body": {"contentType": "html", "content": "<html><head><style>p{}</style></head><body><p>Hi Ada,</p><p>The reaction at grid C is <b>120 kN</b>.</p><script>x()</script></body></html>"},
        "uniqueBody": {"contentType": "html", "content": "<p>The reaction at grid C is <b>120 kN</b>.</p>"},
        "attachments": [
            {"@odata.type": "#microsoft.graph.fileAttachment", "id": "AAMkAGI2AAABEgAQAK=", "name": "calc.pdf", "contentType": "application/pdf", "size": 183_220, "isInline": false},
            {"@odata.type": "#microsoft.graph.fileAttachment", "id": "AAMkAGI2AAABEgAQAL=", "name": "logo.png", "contentType": "image/png", "size": 2_048, "isInline": true},
        ],
    })
}

fn project_message(body: &Json) -> Json {
    Projection::OutlookMessage
        .apply(body, folders_ctx(0))
        .unwrap()
}

#[test]
fn get_message_asks_for_both_bodies_attachment_names_and_immutable_ids() {
    let op = read("outlook.mail.get-message");
    let wire = wire_for(op, json!({"id": MESSAGE}));
    assert_eq!(wire.segments, ["v1.0", "me", "messages", MESSAGE]);
    assert_eq!(
        wire.query,
        [
            ("$select".to_string(), GET_MESSAGE_SELECT.to_string()),
            ("$expand".to_string(), GET_MESSAGE_EXPAND.to_string()),
        ]
    );
    assert!(GET_MESSAGE_SELECT.contains("toRecipients,ccRecipients"));
    for field in ["body", "uniqueBody", "parentFolderId", "changeKey"] {
        assert!(GET_MESSAGE_SELECT.split(',').any(|f| f == field), "{field}");
    }
    assert!(!GET_MESSAGE_EXPAND.contains("contentBytes"));
    assert_eq!(op.fixed_headers, [("Prefer", "IdType=\"ImmutableId\"")]);
    assert_eq!(op.read_cap, Some(8 * 1024 * 1024));
    // A folder name is never a message id.
    for bad in [json!({"id": "inbox"}), json!({"id": "a/b"}), json!({})] {
        assert!(wire_err(op, bad.clone()), "{bad}");
    }
}

#[test]
fn a_message_is_projected_as_text_with_its_folder_version_and_attachment_names() {
    let out = project_message(&message());
    assert_eq!(out["id"], json!(MESSAGE));
    assert_eq!(out["kind"], json!("message"));
    assert_eq!(out["internet-message-id"], json!("<CAF7xYz@mail.acme.com>"));
    assert_eq!(out["subject"], json!("RE: RFI 042 - beam B12"));
    assert_eq!(
        out["from"],
        json!({"address": "engineer@acme.com", "name": "A. Engineer"})
    );
    assert_eq!(
        out["to"],
        json!([{"address": "ada@contoso.com", "name": "Ada"}])
    );
    assert_eq!(out["cc"], json!([]));
    assert_eq!(out["recipients-omitted"], json!(0));
    assert_eq!(out["parent-folder-id"], json!(FOLDER));
    assert_eq!(
        out["change-key"],
        json!("CQAAABYAAABYZnadVM68S5yakKEjeU0sAABlCHts")
    );
    assert_eq!(out["draft"], json!(false));
    assert_eq!(out["has-attachments"], json!(true));
    // HTML is reduced in AWARE: style and script text dropped, markup gone.
    assert_eq!(
        out["text"],
        json!("Hi Ada,\nThe reaction at grid C is 120 kN.")
    );
    assert_eq!(
        out["unique-text"],
        json!("The reaction at grid C is 120 kN.")
    );
    assert_eq!(out["attachments-state"], json!("complete"));
    assert_eq!(out["attachments-omitted"], json!(0));
    let files = out["attachments"].as_array().unwrap();
    assert_eq!(files.len(), 2);
    assert_eq!(files[0]["kind"], json!("file"));
    assert_eq!(files[0]["name"], json!("calc.pdf"));
    assert_eq!(files[0]["size"], json!(183_220));
    assert_eq!(files[0]["inline"], json!(false));
    assert_eq!(files[1]["inline"], json!(true));
    assert!(out.get("truncated").is_none());
}

#[test]
fn table_cells_stay_apart_so_adjacent_values_never_run_together() {
    // A Graph HTML body with a table: `<td>12</td><td>34</td>` must not read
    // back as "1234", which would silently change the mail's data.
    let mut body = message();
    let table = "<table><tr><th>Grid</th><th>kN</th></tr><tr><td>12</td><td> 34 </td></tr></table>";
    body["body"] = json!({"contentType": "html", "content": table});
    body["uniqueBody"] = json!({"contentType": "html", "content": table});
    let out = project_message(&body);
    assert_eq!(out["text"], json!("Grid\tkN\n12\t34"));
    assert_eq!(out["unique-text"], json!("Grid\tkN\n12\t34"));
}

#[test]
fn what_graph_did_not_send_stays_absent_and_null_stays_null() {
    let sparse = json!({"id": MESSAGE, "subject": null, "isDraft": null});
    let out = project_message(&sparse);
    assert_eq!(out["subject"], Json::Null);
    assert_eq!(out["draft"], Json::Null);
    for absent in [
        "kind",
        "from",
        "to",
        "cc",
        "recipients-omitted",
        "has-attachments",
        "received-at",
        "text",
        "unique-text",
        "parent-folder-id",
        "change-key",
    ] {
        assert!(out.get(absent).is_none(), "{absent}");
    }
    assert_eq!(out["attachments-state"], json!("unavailable"));
}

#[test]
fn a_cleared_subject_or_preview_stays_an_empty_string_never_absent() {
    // Graph sends "" after a subject is cleared; dropping it would make it
    // read as unknown, and a reader would keep the old subject.
    let cleared = json!({"id": MESSAGE, "subject": "", "bodyPreview": "\u{0007}"});
    let out = project_message(&cleared);
    assert_eq!(out["subject"], json!(""));
    let entries = vec![json!({
        "id": MESSAGE,
        "subject": "",
        "bodyPreview": "",
        "lastModifiedDateTime": "",
    })];
    let delta = project_delta(&delta_page(entries)).unwrap();
    let fields = &delta["changes"][0]["fields"];
    assert_eq!(fields["subject"], json!(""));
    assert_eq!(fields["preview"], json!(""));
    // A date is never empty: an empty one stays dropped as malformed.
    assert!(fields.get("last-modified-at").is_none());
}

#[test]
fn malformed_join_ids_are_withheld_never_cleaned() {
    let mut m = message();
    m["internetMessageId"] = json!("no angle brackets");
    m["conversationId"] = json!("AAQk/../x");
    m["parentFolderId"] = json!("AA MK");
    m["changeKey"] = json!("has space");
    let out = project_message(&m);
    for (key, flag) in [
        ("internet-message-id", "internet-message-id-withheld"),
        ("conversation-id", "conversation-id-withheld"),
        ("parent-folder-id", "parent-folder-id-withheld"),
        ("change-key", "change-key-withheld"),
    ] {
        assert!(out.get(key).is_none(), "{key}");
        assert_eq!(out[flag], json!(true), "{flag}");
    }
    // An unusable message id is no message at all.
    let mut bad = message();
    bad["id"] = json!("a/b");
    assert!(
        Projection::OutlookMessage
            .apply(&bad, folders_ctx(0))
            .is_none()
    );
}

#[test]
fn every_id_get_message_emits_is_accepted_where_it_is_used() {
    let out = project_message(&message());
    assert!(IdKind::OutlookId.accepts(out["id"].as_str().unwrap()));
    assert!(IdKind::OutlookId.accepts(out["parent-folder-id"].as_str().unwrap()));
    assert!(IdKind::OutlookFolderRef.accepts(out["parent-folder-id"].as_str().unwrap()));
    // An id of 512 bytes goes out and comes back in; 513 does neither.
    let long = format!("{}=", "A".repeat(511));
    let mut m = message();
    m["id"] = json!(long);
    assert_eq!(project_message(&m)["id"], json!(long));
    assert!(IdKind::OutlookId.accepts(&long));
    m["id"] = json!(format!("{}=", "A".repeat(512)));
    assert!(
        Projection::OutlookMessage
            .apply(&m, folders_ctx(0))
            .is_none()
    );
}

#[test]
fn a_long_body_is_cut_and_named_and_recipients_are_capped_and_counted() {
    let mut m = message();
    m["body"] = json!({"contentType": "text", "content": "x".repeat(70_000)});
    m["toRecipients"] = Json::Array(
        (0..60)
            .map(|i| json!({"emailAddress": {"address": format!("p{i}@acme.com")}}))
            .collect(),
    );
    m["ccRecipients"] = json!([{"emailAddress": {"address": "q@acme.com"}}, {"no": "address"}]);
    let out = project_message(&m);
    assert_eq!(out["text"].as_str().unwrap().chars().count(), 65_536);
    let cut = out["truncated"].as_array().unwrap();
    assert!(cut.contains(&json!("text")));
    assert_eq!(out["to"].as_array().unwrap().len(), 50);
    assert_eq!(out["cc"].as_array().unwrap().len(), 1);
    assert_eq!(out["recipients-omitted"], json!(11));
}

#[test]
fn attachment_payloads_are_never_projected_and_a_partial_list_says_so() {
    let mut m = message();
    m["attachments"][0]["contentBytes"] = json!("U0VDUkVULUJZVEVT");
    m["attachments"][1] = json!({"@odata.type": "#microsoft.graph.itemAttachment", "id": "AAMkAGI2AAABEgAQAM=", "name": "Fwd", "item": {"subject": "SECRET-ITEM"}});
    m["attachments@odata.nextLink"] = json!("https://graph.microsoft.com/v1.0/x");
    let out = project_message(&m);
    let text = out.to_string();
    assert!(
        !text.contains("U0VDUkVULUJZVEVT") && !text.contains("SECRET-ITEM"),
        "{text}"
    );
    assert_eq!(out["attachments"][0]["content-withheld"], json!(true));
    assert_eq!(out["attachments"][1]["kind"], json!("item"));
    assert_eq!(out["attachments"][1]["content-withheld"], json!(true));
    assert_eq!(out["attachments-state"], json!("partial"));
    // Cut at 20, the rest counted; a malformed id is counted too.
    let mut many = message();
    let mut list: Vec<Json> = (0..21)
        .map(|i| json!({"id": format!("AAMkAtt{i}"), "name": format!("f{i}")}))
        .collect();
    list.insert(0, json!({"id": "bad/id", "name": "x"}));
    many["attachments"] = Json::Array(list);
    let out = project_message(&many);
    assert_eq!(out["attachments"].as_array().unwrap().len(), 20);
    assert_eq!(out["attachments-omitted"], json!(2));
    assert_eq!(out["attachments-state"], json!("partial"));
    assert_eq!(out["attachments"][0]["kind"], json!("other"));
}

#[tokio::test]
async fn a_get_message_call_sends_the_immutable_id_preference_on_the_data_request_only() {
    let (out, f) = call_read(
        "outlook.mail.get-message",
        MAIL_SCOPES,
        json!({"id": MESSAGE}),
        message(),
    )
    .await;
    let body = body_of(out);
    assert_eq!(
        body["unique-text"],
        json!("The reaction at grid C is 120 kN.")
    );
    let heads = data_heads(&f);
    assert_eq!(heads.len(), 1);
    assert_eq!(path_only(&heads[0]), format!("/v1.0/me/messages/{MESSAGE}"));
    assert!(
        heads[0]
            .lines()
            .any(|l| l.eq_ignore_ascii_case("prefer: IdType=\"ImmutableId\""))
    );
    for head in f.requests() {
        if is_identity_path(&path_of(&head)) {
            assert!(!head.to_ascii_lowercase().contains("prefer:"), "{head}");
        }
    }
}

#[tokio::test]
async fn a_body_over_the_callers_budget_says_so_and_how_much_was_needed() {
    let h = ms_home();
    ms_store(h.path(), None, MAIL_SCOPES);
    let mut m = message();
    m["body"] = json!({"contentType": "text", "content": "y".repeat(4_000)});
    let f = graph(me(OID, UPN), Reply::json(200, &m));
    let mut request = request_for(h.path(), "outlook.mail.get-message", json!({"id": MESSAGE}));
    request.insert("maxOutputBytes".into(), json!(2_048));
    let failure = call_with(h.path(), &parsed(&request), &ms_endpoints(&f))
        .await
        .unwrap_err();
    assert_eq!(failure.code, "E_CALL_OUTPUT_TOO_LARGE");
    assert_eq!(failure.details["reason"], json!("output-too-large"));
    assert!(failure.details["outputBytes"].as_u64().unwrap() > 4_000);
    // The same request with the largest budget succeeds: the raw answer was
    // read under the operation's own cap, not the caller's.
    request.insert("maxOutputBytes".into(), json!(1_048_576));
    let ok = call_with(h.path(), &parsed(&request), &ms_endpoints(&f)).await;
    assert_eq!(body_of(ok)["text"].as_str().unwrap().len(), 4_000);
}

#[tokio::test]
async fn a_message_over_the_raw_cap_is_raw_too_large() {
    let mut m = message();
    m["body"] = json!({"contentType": "text", "content": "z".repeat(OUTLOOK_READ_CAP)});
    let (out, _f) = call_read(
        "outlook.mail.get-message",
        MAIL_SCOPES,
        json!({"id": MESSAGE}),
        m,
    )
    .await;
    let failure = out.unwrap_err();
    assert_eq!(failure.code, "E_CALL_OUTPUT_TOO_LARGE");
    assert_eq!(failure.details["reason"], json!("raw-too-large"));
}

#[test]
fn rows_without_a_read_cap_keep_an_unlabelled_size_failure() {
    // Old rows are unchanged: no `readCap` in their digest, no reason on a
    // too-large answer (pinned by `every_row_shipped_before_763_keeps_its_digest`
    // and by the failure mapping, which keys on `read_cap`).
    for op in OPERATIONS
        .iter()
        .filter(|op| !OUTLOOK_763.contains(&op.command))
        // The Trimble Connect rows (#788) carry a read cap and their own failures.
        .filter(|op| op.agent != "trimble-connect")
    {
        assert!(op.read_cap.is_none(), "{}", op.command);
        assert!(!canonical_string(&op.canonical()).contains("readCap"));
    }
    let op = read("outlook.mail.get-message");
    assert!(canonical_string(&op.canonical()).contains("\"readCap\":8388608"));
}

// ── folder.list-messages (delta) ─────────────────────────────────────────────

const NEXT: &str =
    "https://graph.microsoft.com/v1.0/me/mailFolders('AAMk')/messages/delta?$skiptoken=";
const DELTA: &str =
    "https://graph.microsoft.com/v1.0/me/mailFolders('AAMk')/messages/delta?$deltatoken=";
/// A state token holding the characters that break a naive round trip.
const TOKEN_DECODED: &str = "Lm9_pQ%2B/x+y=z==";
const TOKEN_ENCODED: &str = "Lm9_pQ%252B%2Fx%2By%3Dz%3D%3D";

fn delta_op() -> &'static Operation {
    read("outlook.mail.folder.list-messages")
}

fn binding() -> String {
    binding_id("microsoft-365", "microsoft-365", OID, Some(TID))
}

fn cursor(kind: CursorKind, token: &str) -> String {
    DeltaCursor {
        binding: binding(),
        folder: FOLDER.to_string(),
        kind,
        op: delta_op().sha256(),
        token: token.to_string(),
    }
    .encode()
}

fn upsert(id: &str, subject: &str) -> Json {
    json!({
        "@odata.type": "#microsoft.graph.message",
        "id": id,
        "subject": subject,
        "receivedDateTime": "2026-11-06T08:00:00Z",
        "from": {"emailAddress": {"address": "a@acme.com"}},
    })
}

fn removal(id: &str) -> Json {
    json!({"id": id, "@removed": {"reason": "deleted"}})
}

fn page(entries: Vec<Json>, link: (&str, &str)) -> Json {
    let mut body = json!({"value": entries});
    body[link.0] = json!(link.1);
    body
}

fn delta_page(entries: Vec<Json>) -> Json {
    page(entries, ("@odata.deltaLink", &format!("{DELTA}tok-final")))
}

fn project_delta(body: &Json) -> Result<Json, CallFailure> {
    project_outlook_delta(body, delta_op(), FOLDER, &binding())
}

#[test]
fn a_first_round_sends_the_headers_select_and_a_page_size_preference() {
    let wire = wire_for(delta_op(), json!({"folder-id": FOLDER}));
    assert_eq!(
        wire.segments,
        ["v1.0", "me", "mailFolders", FOLDER, "messages", "delta"]
    );
    assert_eq!(
        wire.query,
        [("$select".to_string(), MAIL_HEADERS_SELECT.to_string())]
    );
    assert!(
        !MAIL_HEADERS_SELECT
            .split(',')
            .any(|f| f == "body" || f == "uniqueBody")
    );
    assert_eq!(
        wire.headers,
        [(
            "Prefer".to_string(),
            "odata.maxpagesize=25, IdType=\"ImmutableId\"".to_string()
        )]
    );
    assert_eq!(wire.folder.as_deref(), Some(FOLDER));
    let small = wire_for(delta_op(), json!({"folder-id": FOLDER, "page-size": 1}));
    assert_eq!(
        small.headers[0].1,
        "odata.maxpagesize=1, IdType=\"ImmutableId\""
    );
    // A real folder id only: no well-known name, no time filter, no OData.
    for bad in [
        json!({"folder-id": "inbox"}),
        json!({"folder-id": FOLDER, "page-size": 51}),
        json!({"folder-id": FOLDER, "page-size": 0}),
        json!({"folder-id": FOLDER, "received-after": "2026-11-01T00:00:00Z"}),
        json!({"folder-id": FOLDER, "$filter": "x"}),
    ] {
        assert!(wire_err(delta_op(), bad.clone()), "{bad}");
    }
}

#[test]
fn a_cursor_request_sends_only_its_token_and_keeps_the_page_size_in_the_header() {
    for (kind, param) in [
        (CursorKind::Skip, "$skiptoken"),
        (CursorKind::Delta, "$deltatoken"),
    ] {
        let c = cursor(kind, TOKEN_DECODED);
        let wire = wire_for(
            delta_op(),
            json!({"folder-id": FOLDER, "cursor": c, "page-size": 5}),
        );
        assert_eq!(
            wire.query,
            [(param.to_string(), TOKEN_DECODED.to_string())],
            "{param}"
        );
        assert_eq!(
            wire.headers[0].1,
            "odata.maxpagesize=5, IdType=\"ImmutableId\""
        );
    }
}

#[test]
fn a_cursor_of_another_folder_or_operation_is_refused_and_a_malformed_one_too() {
    let other_folder = DeltaCursor {
        binding: binding(),
        folder: PARENT.to_string(),
        kind: CursorKind::Delta,
        op: delta_op().sha256(),
        token: "t".into(),
    }
    .encode();
    let other_op = DeltaCursor {
        binding: binding(),
        folder: FOLDER.to_string(),
        kind: CursorKind::Delta,
        op: read("outlook.mail.get-message").sha256(),
        token: "t".into(),
    }
    .encode();
    for c in [other_folder, other_op] {
        let failure =
            wire(delta_op(), &map(json!({"folder-id": FOLDER, "cursor": c}))).unwrap_err();
        assert_eq!(failure.details["reason"], json!("cursor-context"));
    }
    use base64::Engine as _;
    let b64 = |v: Json| {
        format!(
            "odc1.{}",
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(v.to_string())
        )
    };
    let good = json!({"v": 1, "binding": binding(), "folder": FOLDER, "kind": "delta",
                      "op": delta_op().sha256(), "token": "t"});
    assert!(DeltaCursor::decode(&b64(good.clone())).is_ok());
    let mut extra = good.clone();
    extra["more"] = json!(1);
    let mut v2 = good.clone();
    v2["v"] = json!(2);
    let mut kind = good.clone();
    kind["kind"] = json!("full");
    let mut empty = good.clone();
    empty["token"] = json!("");
    let mut control = good.clone();
    control["token"] = json!("a\nb");
    let mut huge = good.clone();
    huge["token"] = json!("t".repeat(MAX_STATE_TOKEN_BYTES + 1));
    let mut folder = good.clone();
    folder["folder"] = json!("a/b");
    for bad in [
        String::new(),
        "odc1.".into(),
        "odc2.e30".into(),
        "odc1.e30=".into(),
        "odc1.!!".into(),
        b64(json!([])),
        b64(extra),
        b64(v2),
        b64(kind),
        b64(empty),
        b64(control),
        b64(huge),
        b64(folder),
    ] {
        let failure = DeltaCursor::decode(&bad).unwrap_err();
        assert_eq!(failure.details["input"], json!("cursor"), "{bad}");
    }
}

#[test]
fn a_cursor_round_trips_every_token_byte() {
    let c = cursor(CursorKind::Skip, TOKEN_DECODED);
    let back = DeltaCursor::decode(&c).unwrap();
    assert_eq!(back.token, TOKEN_DECODED);
    assert_eq!(back.kind, CursorKind::Skip);
    assert!(c.starts_with("odc1.") && c.len() <= MAX_CURSOR_BYTES);
    // The token Graph wrote in its link, decoded exactly once.
    assert_eq!(
        link_token(&format!("{NEXT}{TOKEN_ENCODED}"), CursorKind::Skip).unwrap(),
        TOKEN_DECODED
    );
}

#[test]
fn changes_come_back_in_graphs_order_with_unidentifiable_ones_kept_in_place() {
    let body = page(
        vec![
            upsert(MESSAGE, "first"),
            removal(PARENT),
            upsert("bad/id", "never shown"),
            json!({"id": "x y", "@removed": {"reason": "deleted"}}),
            json!("not an object"),
            json!({"id": MESSAGE}),
            removal(MESSAGE),
        ],
        ("@odata.nextLink", &format!("{NEXT}tok-2")),
    );
    let out = project_delta(&body).unwrap();
    let changes = out["changes"].as_array().unwrap();
    let kinds: Vec<&str> = changes
        .iter()
        .map(|c| c["change"].as_str().unwrap())
        .collect();
    assert_eq!(
        kinds,
        [
            "upsert",
            "removed",
            "unidentifiable",
            "unidentifiable",
            "unidentifiable",
            "upsert",
            "removed"
        ]
    );
    assert_eq!(changes[0]["id"], json!(MESSAGE));
    assert_eq!(changes[0]["fields"]["subject"], json!("first"));
    assert_eq!(changes[0]["fields"]["kind"], json!("message"));
    assert!(changes[0]["fields"].get("id").is_none());
    assert_eq!(
        changes[1],
        json!({"change": "removed", "id": PARENT, "reason": "deleted"})
    );
    assert_eq!(
        changes[2],
        json!({"change": "unidentifiable", "position": 2})
    );
    assert_eq!(
        changes[4],
        json!({"change": "unidentifiable", "position": 4})
    );
    // An id-only update says nothing more than that: no invented fields.
    assert_eq!(
        changes[5],
        json!({"change": "upsert", "id": MESSAGE, "fields": {}})
    );
    // The same id as upsert, then removal: both kept, never merged.
    assert_eq!(changes[6]["id"], json!(MESSAGE));
    assert_eq!(out["unidentifiable"], json!(3));
    assert!(!out.to_string().contains("never shown"));
    // The cursor is still returned: the stream moves on, the count says why
    // it is not complete.
    assert_eq!(out["more-available"], json!(true));
    let c = DeltaCursor::decode(out["cursor"].as_str().unwrap()).unwrap();
    assert_eq!(c.kind, CursorKind::Skip);
    assert_eq!(c.token, "tok-2");
    assert_eq!(c.folder, FOLDER);
    assert_eq!(c.binding, binding());
    assert_eq!(c.op, delta_op().sha256());
}

#[test]
fn the_final_page_carries_a_delta_cursor_and_says_the_round_is_done() {
    let out = project_delta(&delta_page(vec![])).unwrap();
    assert_eq!(out["changes"], json!([]));
    assert_eq!(out["more-available"], json!(false));
    let c = DeltaCursor::decode(out["cursor"].as_str().unwrap()).unwrap();
    assert_eq!((c.kind, c.token.as_str()), (CursorKind::Delta, "tok-final"));
}

#[test]
fn a_page_needs_exactly_one_supported_continuation_link() {
    let both = json!({"value": [], "@odata.nextLink": format!("{NEXT}a"), "@odata.deltaLink": format!("{DELTA}b")});
    let neither = json!({"value": []});
    for body in [
        both,
        neither,
        json!({"@odata.deltaLink": format!("{DELTA}b")}),
    ] {
        let failure = project_delta(&body).unwrap_err();
        assert_eq!(failure.details["reason"], json!("response-shape"), "{body}");
    }
    let long = "t".repeat(MAX_STATE_TOKEN_BYTES + 1);
    for (key, link) in [
        (
            "@odata.nextLink",
            "https://evil.example/v1.0/me/messages/delta?$skiptoken=a".to_string(),
        ),
        (
            "@odata.nextLink",
            "http://graph.microsoft.com/v1.0/x?$skiptoken=a".to_string(),
        ),
        (
            "@odata.nextLink",
            "https://graph.microsoft.com:444/v1.0/x?$skiptoken=a".to_string(),
        ),
        ("@odata.nextLink", format!("{NEXT}a&$select=body")),
        ("@odata.nextLink", format!("{DELTA}a")),
        ("@odata.deltaLink", format!("{NEXT}a")),
        ("@odata.nextLink", NEXT.to_string()),
        ("@odata.nextLink", format!("{NEXT}{long}")),
        ("@odata.nextLink", format!("{NEXT}a#frag")),
        ("@odata.nextLink", "not a url".to_string()),
    ] {
        let failure = project_delta(&page(vec![], (key, &link))).unwrap_err();
        assert_eq!(
            failure.details["reason"],
            json!("cursor-link-unsupported"),
            "{link}"
        );
        // Neither the link nor its token reaches the failure.
        assert!(!format!("{failure:?}").contains("evil"), "{failure:?}");
    }
}

async fn delta_call(
    inputs: Json,
    reply: Reply,
    max_output: Option<u64>,
) -> (Result<Json, CallFailure>, Fixture) {
    let h = ms_home();
    ms_store(h.path(), None, MAIL_SCOPES);
    let f = graph(me(OID, UPN), reply);
    let mut request = request_for(h.path(), "outlook.mail.folder.list-messages", inputs);
    if let Some(max) = max_output {
        request.insert("maxOutputBytes".into(), json!(max));
    }
    let out = call_with(h.path(), &parsed(&request), &ms_endpoints(&f)).await;
    (out, f)
}

#[tokio::test]
async fn a_whole_page_comes_back_however_many_entries_graph_sent() {
    // Page size is a preference: 120 entries with page-size 25 are all
    // returned, because a cut entry would be lost once the cursor moves on.
    let entries: Vec<Json> = (0..120).map(|i| upsert(&format!("AAkA{i}"), "s")).collect();
    let (out, f) = delta_call(
        json!({"folder-id": FOLDER, "page-size": 25}),
        Reply::json(200, &delta_page(entries)),
        Some(1 << 20),
    )
    .await;
    let body = body_of(out);
    assert_eq!(body["changes"].as_array().unwrap().len(), 120);
    let heads = data_heads(&f);
    assert_eq!(heads.len(), 1);
    assert_eq!(
        path_only(&heads[0]),
        format!("/v1.0/me/mailFolders/{FOLDER}/messages/delta")
    );
    assert!(heads[0].lines().any(|l| l.eq_ignore_ascii_case(
        "prefer: odata.maxpagesize=25, IdType=\"ImmutableId\""
    )));
}

#[tokio::test]
async fn a_cursor_reaches_graph_as_its_one_token_encoded_once() {
    let c = cursor(CursorKind::Skip, TOKEN_DECODED);
    let (out, f) = delta_call(
        json!({"folder-id": FOLDER, "cursor": c}),
        Reply::json(200, &delta_page(vec![removal(MESSAGE)])),
        None,
    )
    .await;
    assert_eq!(body_of(out)["changes"][0]["change"], json!("removed"));
    let head = &data_heads(&f)[0];
    assert_eq!(
        query_of(head),
        [("$skiptoken".to_string(), TOKEN_DECODED.to_string())]
    );
}

#[tokio::test]
async fn a_cursor_bound_to_another_account_sends_nothing_at_all() {
    let stranger = DeltaCursor {
        binding: binding_id(
            "microsoft-365",
            "microsoft-365",
            OID,
            Some("0a1b2c3d-0000-4000-8000-00000000abcd"),
        ),
        folder: FOLDER.to_string(),
        kind: CursorKind::Delta,
        op: delta_op().sha256(),
        token: "t".into(),
    }
    .encode();
    let (out, f) = delta_call(
        json!({"folder-id": FOLDER, "cursor": stranger}),
        Reply::json(200, &delta_page(vec![])),
        None,
    )
    .await;
    assert_eq!(out.unwrap_err().details["reason"], json!("cursor-context"));
    assert!(f.requests().is_empty(), "{:?}", f.requests());
}

#[tokio::test]
async fn a_dead_cursor_is_told_apart_from_a_missing_folder() {
    let secret = "SECRET-IN-ERROR-BODY";
    let cases: Vec<(u16, Json, Option<&str>)> = vec![
        (
            410,
            json!({"error": {"code": "whatever", "message": secret}}),
            Some("cursor-expired"),
        ),
        (
            404,
            json!({"error": {"code": "SyncStateNotFound", "message": secret}}),
            Some("cursor-expired"),
        ),
        (
            400,
            json!({"error": {"code": "syncStateInvalid"}}),
            Some("cursor-expired"),
        ),
        (
            404,
            json!({"error": {"code": "ErrorItemNotFound", "message": secret}}),
            None,
        ),
        (
            404,
            json!({"error": {"code": format!("syncStateNotFound{}", "x".repeat(ERROR_BODY_CAP))}}),
            None,
        ),
        (403, json!({"error": {"code": "syncStateNotFound"}}), None),
    ];
    for (status, error, reason) in cases {
        let (out, _f) = delta_call(
            json!({"folder-id": FOLDER}),
            Reply::json(status, &error),
            None,
        )
        .await;
        let failure = out.unwrap_err();
        assert_eq!(failure.code, "E_CALL_FAILED");
        assert_eq!(failure.details["status"], json!(status), "{error}");
        assert_eq!(
            failure.details.get("reason").and_then(Json::as_str),
            reason,
            "{error}"
        );
        assert!(!format!("{failure:?}").contains(secret));
        assert!(!format!("{failure:?}").contains("ErrorItemNotFound"));
    }
    // Other rows never read an error body: the same dead-cursor code on a
    // message read is its status alone.
    let h = ms_home();
    ms_store(h.path(), None, MAIL_SCOPES);
    let f = graph(
        me(OID, UPN),
        Reply::json(404, &json!({"error": {"code": "syncStateNotFound"}})),
    );
    let request = request_for(h.path(), "outlook.mail.get-message", json!({"id": MESSAGE}));
    let failure = call_with(h.path(), &parsed(&request), &ms_endpoints(&f))
        .await
        .unwrap_err();
    assert_eq!(failure.details["status"], json!(404));
    assert!(failure.details.get("reason").is_none(), "{failure:?}");
}

#[tokio::test]
async fn a_delta_page_over_budget_names_which_budget_it_broke() {
    let entries: Vec<Json> = (0..40)
        .map(|i| upsert(&format!("AAkA{i}"), &"s".repeat(900)))
        .collect();
    let (out, _f) = delta_call(
        json!({"folder-id": FOLDER}),
        Reply::json(200, &delta_page(entries)),
        Some(4_096),
    )
    .await;
    let failure = out.unwrap_err();
    assert_eq!(failure.code, "E_CALL_OUTPUT_TOO_LARGE");
    assert_eq!(failure.details["reason"], json!("output-too-large"));
    assert!(failure.details["outputBytes"].as_u64().unwrap() > 4_096);
    // Never a bad record: the same page fits the largest budget.
    let entries: Vec<Json> = (0..40)
        .map(|i| upsert(&format!("AAkA{i}"), &"s".repeat(900)))
        .collect();
    let (out, _f) = delta_call(
        json!({"folder-id": FOLDER}),
        Reply::json(200, &delta_page(entries)),
        Some(1_048_576),
    )
    .await;
    assert_eq!(body_of(out)["changes"].as_array().unwrap().len(), 40);
    // An empty page whose cursor alone exceeds a 1 KiB budget is an output
    // overflow too, never a bad record.
    let long = "t".repeat(2_000);
    let (out, _f) = delta_call(
        json!({"folder-id": FOLDER}),
        Reply::json(
            200,
            &page(vec![], ("@odata.deltaLink", &format!("{DELTA}{long}"))),
        ),
        Some(1_024),
    )
    .await;
    assert_eq!(
        out.unwrap_err().details["reason"],
        json!("output-too-large")
    );
}

async fn workflow_delta(
    home: &Path,
    manifest: &Agent,
    f: &Fixture,
    inputs: Json,
) -> Result<Json, CallFailure> {
    run_for_workflow_with(
        home,
        manifest,
        "outlook.mail.folder.list-messages",
        &inputs,
        Some(&ms_endpoints(f)),
        Duration::from_secs(10),
        1 << 20,
    )
    .await
}

#[tokio::test]
async fn a_workflow_or_invoke_run_verifies_the_account_its_cursor_belongs_to() {
    let h = ms_home();
    ms_store(h.path(), None, MAIL_SCOPES);
    let manifest: Agent = serde_yaml::from_str(&ms_runnable_manifest()).unwrap();
    // The same account: the page comes back with a cursor minted for it.
    let f = graph(
        me(OID, UPN),
        Reply::json(200, &delta_page(vec![upsert(MESSAGE, "s")])),
    );
    let out = workflow_delta(
        h.path(),
        &manifest,
        &f,
        json!({"folder-id": FOLDER, "cursor": cursor(CursorKind::Delta, "t")}),
    )
    .await
    .unwrap();
    let c = DeltaCursor::decode(out["cursor"].as_str().unwrap()).unwrap();
    assert_eq!(c.binding, binding());
    // The slot now belongs to someone else: the old cursor is refused after
    // the identity read and before any data request.
    let f = graph(
        me("9f1c2b3a-2222-4222-8333-944455556666", UPN),
        Reply::json(200, &delta_page(vec![])),
    );
    let failure = workflow_delta(
        h.path(),
        &manifest,
        &f,
        json!({"folder-id": FOLDER, "cursor": cursor(CursorKind::Delta, "t")}),
    )
    .await
    .unwrap_err();
    assert_eq!(failure.details["reason"], json!("cursor-context"));
    assert!(data_heads(&f).is_empty(), "{:?}", f.requests());
    // Without User.Read the account cannot be verified, so nothing is read.
    ms_store(h.path(), None, "offline_access Mail.Read");
    let f = graph(me(OID, UPN), Reply::json(200, &delta_page(vec![])));
    let failure = workflow_delta(h.path(), &manifest, &f, json!({"folder-id": FOLDER}))
        .await
        .unwrap_err();
    assert_eq!(failure.code, "E_CALL_SCOPE_MISSING");
    assert!(f.requests().is_empty());
}

#[test]
fn the_delta_digest_binds_what_makes_its_requests_and_its_cursor() {
    let base = delta_op().clone();
    let digest = base.sha256();
    let canonical = canonical_string(&base.canonical());
    for needle in [
        "\"bindsAccount\":true",
        "\"readCap\":8388608",
        "\"delta-state\"",
        "\"outlook-delta-cursor/1\"",
        "\"prefer-maxpagesize\"",
        "\"outlook-delta/1\"",
        "IdType=\\\"ImmutableId\\\"",
    ] {
        assert!(canonical.contains(needle), "{needle}: {canonical}");
    }
    let mut changed = Vec::new();
    let mut m = base.clone();
    m.read_cap = Some(1);
    changed.push(m.sha256());
    let mut m = base.clone();
    m.binds_account = false;
    changed.push(m.sha256());
    let mut m = base.clone();
    m.fixed_headers = &[];
    changed.push(m.sha256());
    let mut m = base.clone();
    m.fixed_query = &[];
    changed.push(m.sha256());
    let mut m = base.clone();
    m.projection = Projection::OutlookMessage;
    changed.push(m.sha256());
    let mut m = base.clone();
    const NO_CURSOR: &[InputMap] = &[
        InputMap::path_long("folder-id", IdKind::OutlookId, 512),
        InputMap::prefer_page_size("page-size", 25, 50),
    ];
    m.inputs = NO_CURSOR;
    changed.push(m.sha256());
    for d in changed {
        assert_ne!(d, digest);
    }
}

#[tokio::test]
async fn a_delta_page_over_the_raw_cap_is_raw_too_large_and_returns_no_cursor() {
    // One record Graph will not hand over in 8 MiB, even at page-size 1.
    let big = upsert(MESSAGE, &"z".repeat(OUTLOOK_READ_CAP));
    let (out, _f) = delta_call(
        json!({"folder-id": FOLDER, "page-size": 1}),
        Reply::json(200, &delta_page(vec![big])),
        Some(1_048_576),
    )
    .await;
    let failure = out.unwrap_err();
    assert_eq!(failure.code, "E_CALL_OUTPUT_TOO_LARGE");
    assert_eq!(failure.details["reason"], json!("raw-too-large"));
    assert!(!format!("{failure:?}").contains("odc1."));
}

#[tokio::test]
async fn an_invoke_run_refuses_a_cursor_once_its_slot_belongs_to_another_account() {
    let h = ms_home();
    // `--as work`: the default slot holds no mail scope, so only `work` can serve.
    ms_store(h.path(), None, "offline_access User.Read");
    ms_store(h.path(), Some("work"), MAIL_SCOPES);
    let manifest: Agent = serde_yaml::from_str(&ms_runnable_manifest()).unwrap();
    // A cursor belongs to a slot as well as an account.
    let work = DeltaCursor {
        binding: binding_id(
            "microsoft-365",
            &account_name("microsoft-365", Some("work")),
            OID,
            Some(TID),
        ),
        folder: FOLDER.to_string(),
        kind: CursorKind::Delta,
        op: delta_op().sha256(),
        token: "t".into(),
    }
    .encode();
    let invoke = |f: &Fixture, c: &str| {
        let endpoints = ms_endpoints(f);
        let args = json!({"folder-id": FOLDER, "cursor": c});
        let home = h.path().to_path_buf();
        let manifest = &manifest;
        async move {
            run_reviewed(
                &home,
                manifest,
                "outlook.mail.folder.list-messages",
                &args,
                Some("work"),
                Some(&endpoints),
                Duration::from_secs(10),
                1 << 20,
            )
            .await
        }
    };
    let f = graph(me(OID, UPN), Reply::json(200, &delta_page(vec![])));
    assert!(invoke(&f, &work).await.is_ok());
    // The default slot's cursor, though the same account, is not `work`'s.
    let f = graph(me(OID, UPN), Reply::json(200, &delta_page(vec![])));
    let failure = invoke(&f, &cursor(CursorKind::Delta, "t"))
        .await
        .unwrap_err();
    assert_eq!(failure.details["reason"], json!("cursor-context"));
    assert!(data_heads(&f).is_empty(), "{:?}", f.requests());
    let f = graph(
        me("9f1c2b3a-2222-4222-8333-944455556666", UPN),
        Reply::json(200, &delta_page(vec![])),
    );
    let failure = invoke(&f, &work).await.unwrap_err();
    assert_eq!(failure.details["reason"], json!("cursor-context"));
    assert!(data_heads(&f).is_empty(), "{:?}", f.requests());
}

#[tokio::test]
async fn a_delta_page_over_the_largest_budget_fails_without_a_cursor() {
    // Raw well under 8 MiB, projected over 1 MiB: the caller's next step is a
    // smaller page on the same cursor, so the failure must advance nothing.
    let entries: Vec<Json> = (0..900)
        .map(|i| {
            let mut e = upsert(&format!("AAkA{i}"), &"s".repeat(1_000));
            e["bodyPreview"] = json!("p".repeat(500));
            e
        })
        .collect();
    let body = delta_page(entries);
    assert!(body.to_string().len() < OUTLOOK_READ_CAP);
    let (out, _f) = delta_call(
        json!({"folder-id": FOLDER, "cursor": cursor(CursorKind::Skip, "t")}),
        Reply::json(200, &body),
        Some(1_048_576),
    )
    .await;
    let failure = out.unwrap_err();
    assert_eq!(failure.code, "E_CALL_OUTPUT_TOO_LARGE");
    assert_eq!(failure.details["reason"], json!("output-too-large"));
    assert!(failure.details["outputBytes"].as_u64().unwrap() > 1_048_576);
    assert!(!format!("{failure:?}").contains("odc1."));
}

// ── folder.list-message-ids (what a folder holds now) ────────────────────────

fn ids_op() -> &'static Operation {
    read("outlook.mail.folder.list-message-ids")
}

fn listed(id: &str, received: &str, modified: &str) -> Json {
    json!({
        "@odata.etag": "W/\"CQAAABYAAAA\"",
        "id": id,
        "receivedDateTime": received,
        "lastModifiedDateTime": modified,
        "isDraft": false,
    })
}

fn project_ids(body: &Json) -> Json {
    Projection::OutlookMessageIds
        .apply(body, folders_ctx(0))
        .unwrap()
}

#[test]
fn a_listing_sends_one_window_filter_ordered_by_the_received_date() {
    let wire = wire_for(
        ids_op(),
        json!({"folder-id": FOLDER, "received-after": "2026-08-08T00:00:00Z"}),
    );
    assert_eq!(
        wire.segments,
        ["v1.0", "me", "mailFolders", FOLDER, "messages"]
    );
    assert_eq!(
        wire.query,
        [
            (
                "$filter".to_string(),
                "receivedDateTime ge 2026-08-08T00:00:00Z".to_string()
            ),
            ("$top".to_string(), "500".to_string()),
            ("$skip".to_string(), "0".to_string()),
            ("$select".to_string(), MAIL_IDS_SELECT.to_string()),
            ("$orderby".to_string(), "receivedDateTime desc".to_string()),
        ]
    );
    // Messages sharing one received time are paged within a window of that
    // one instant by `skip`.
    let tie = wire_for(
        ids_op(),
        json!({
            "folder-id": FOLDER,
            "received-after": "2026-11-06T08:00:00Z",
            "received-before": "2026-11-06T08:00:00Z",
            "page-size": 1000,
            "skip": 2000,
        }),
    );
    assert_eq!(
        tie.query[0].1,
        "receivedDateTime ge 2026-11-06T08:00:00Z and receivedDateTime le 2026-11-06T08:00:00Z"
    );
    assert_eq!(tie.query[2], ("$skip".to_string(), "2000".to_string()));
    assert_eq!(tie.skip, 2000);
    // Both bounds are one filter, the ordered property first, in one
    // normalised spelling.
    let wire = wire_for(
        ids_op(),
        json!({
            "folder-id": FOLDER,
            "received-after": "2026-08-08T00:00:00Z",
            "received-before": "2026-11-06T08:00:00.5000000Z",
            "page-size": 1000,
        }),
    );
    assert_eq!(
        wire.query[0],
        (
            "$filter".to_string(),
            "receivedDateTime ge 2026-08-08T00:00:00Z and receivedDateTime le 2026-11-06T08:00:00.500Z"
                .to_string()
        )
    );
    assert_eq!(wire.query[1], ("$top".to_string(), "1000".to_string()));
    assert_eq!(wire.query.iter().filter(|(k, _)| k == "$filter").count(), 1);
    // The same instant on both sides is a window of one instant, not an error.
    let _ = wire_for(
        ids_op(),
        json!({"folder-id": FOLDER, "received-after": "2026-08-08T00:00:00Z", "received-before": "2026-08-08T00:00:00Z"}),
    );
    for bad in [
        json!({"folder-id": FOLDER}),
        json!({"received-after": "2026-08-08T00:00:00Z"}),
        json!({"folder-id": "inbox", "received-after": "2026-08-08T00:00:00Z"}),
        json!({"folder-id": FOLDER, "received-after": "2026-08-08"}),
        json!({"folder-id": FOLDER, "received-after": "2026-08-08T00:00:00+02:00"}),
        json!({"folder-id": FOLDER, "received-after": "2026-08-08T00:00:00Z or true"}),
        json!({"folder-id": FOLDER, "received-after": "2026-08-08T00:00:00Z", "received-before": "x"}),
        // An empty window would read as an empty folder.
        json!({"folder-id": FOLDER, "received-after": "2026-08-08T00:00:01Z", "received-before": "2026-08-08T00:00:00Z"}),
        json!({"folder-id": FOLDER, "received-after": "2026-08-08T00:00:00Z", "page-size": 0}),
        json!({"folder-id": FOLDER, "received-after": "2026-08-08T00:00:00Z", "page-size": 1001}),
        json!({"folder-id": FOLDER, "received-after": "2026-08-08T00:00:00Z", "skip": -1}),
        json!({"folder-id": FOLDER, "received-after": "2026-08-08T00:00:00Z", "skip": 100_001}),
        json!({"folder-id": FOLDER, "received-after": "2026-08-08T00:00:00Z", "$filter": "x"}),
    ] {
        assert!(wire_err(ids_op(), bad.clone()), "{bad}");
    }
}

#[test]
fn a_window_bound_is_named_in_the_digest_and_moves_no_earlier_row() {
    let canonical = canonical_string(&ids_op().canonical());
    assert!(canonical.contains("\"operator\":\"ge\""), "{canonical}");
    assert!(canonical.contains("\"operator\":\"le\""), "{canonical}");
    for op in OPERATIONS
        .iter()
        .filter(|op| op.command != ids_op().command)
    {
        assert!(
            !canonical_string(&op.canonical()).contains("\"operator\""),
            "{}",
            op.command
        );
    }
    // A `since` still renders `gt`, where it stood.
    let chat = read("teams.chat.list-messages");
    let wire = wire_for(
        chat,
        json!({"chat-id": "19:abc@thread.v2", "since": "2026-11-01T00:00:00Z"}),
    );
    assert_eq!(
        wire.query[0],
        (
            "$filter".to_string(),
            "lastModifiedDateTime gt 2026-11-01T00:00:00Z".to_string()
        )
    );
}

#[test]
fn every_listed_message_is_kept_with_exact_dates_or_withheld_ones() {
    let mut bad_date = listed(PARENT, "2026-11-05 10:00", "2026-11-05T10:00:00Z");
    bad_date["isDraft"] = json!("no");
    let mut nulls = listed(MESSAGE, "2026-11-04T10:00:00Z", "x");
    nulls["receivedDateTime"] = Json::Null;
    let bare = json!({"id": FOLDER, "@odata.etag": "e"});
    let page = json!({
        "value": [
            listed(FOLDER, "2026-11-06T08:00:00Z", "2026-11-06T09:30:00.1234567Z"),
            listed("bad/id", "2026-11-06T07:00:00Z", "2026-11-06T07:00:00Z"),
            bad_date,
            nulls,
            bare,
        ],
        "@odata.nextLink": "https://graph.microsoft.com/v1.0/me/mailFolders/x/messages?$skip=5",
    });
    let out = project_ids(&page);
    let messages = out["messages"].as_array().unwrap();
    // No local cut (the page bound is 2 here): every entry with a good id is
    // kept, the malformed id is counted, never cleaned.
    assert_eq!(messages.len(), 4);
    assert_eq!(out["omitted"], json!(1));
    assert_eq!(out["more-available"], json!(true));
    assert_eq!(
        messages[0],
        json!({
            "id": FOLDER,
            "received-at": "2026-11-06T08:00:00Z",
            "last-modified-at": "2026-11-06T09:30:00.1234567Z",
            "draft": false,
        })
    );
    assert_eq!(
        messages[1],
        json!({
            "id": PARENT,
            "received-at-withheld": true,
            "last-modified-at": "2026-11-05T10:00:00Z",
            "draft": null,
        })
    );
    assert_eq!(messages[2]["received-at"], Json::Null);
    assert_eq!(messages[2]["last-modified-at-withheld"], json!(true));
    // What Graph did not send stays absent; nothing else is projected.
    assert_eq!(messages[3], json!({"id": FOLDER}));
    let last = project_ids(&json!({"value": []}));
    assert_eq!(last["more-available"], json!(false));
    assert_eq!(last["messages"], json!([]));
}

#[test]
fn a_listing_page_offers_the_next_skip_from_every_entry_graph_sent() {
    // Graph may send fewer entries than asked and still say more follow, and
    // a malformed id is counted, not kept: the next page starts after every
    // entry Graph sent, whatever the page size.
    let short = json!({
        "value": [
            listed(FOLDER, "2026-11-06T08:00:00Z", "2026-11-06T08:00:00Z"),
            listed("bad/id", "2026-11-06T08:00:00Z", "2026-11-06T08:00:00Z"),
            listed(PARENT, "2026-11-06T08:00:00Z", "2026-11-06T08:00:00Z"),
        ],
        "@odata.nextLink": "https://graph.microsoft.com/v1.0/me/mailFolders/x/messages?$skip=53",
    });
    let out = Projection::OutlookMessageIds
        .apply(&short, folders_ctx(50))
        .unwrap();
    assert_eq!(out["messages"].as_array().unwrap().len(), 2);
    assert_eq!(out["omitted"], json!(1));
    assert_eq!(out["more-available"], json!(true));
    assert_eq!(out["next-skip"], json!(53));
    assert!(out.get("limit-reached").is_none());
    // The last page offers none.
    let last = json!({"value": [listed(FOLDER, "2026-11-06T08:00:00Z", "2026-11-06T08:00:00Z")]});
    let out = Projection::OutlookMessageIds
        .apply(&last, folders_ctx(53))
        .unwrap();
    assert_eq!(out["more-available"], json!(false));
    assert!(out.get("next-skip").is_none());
    assert!(out.get("limit-reached").is_none());
    // Past the largest skip AWARE sends, the limit is named instead.
    let two = json!({
        "value": [
            listed(FOLDER, "2026-11-06T08:00:00Z", "2026-11-06T08:00:00Z"),
            listed(PARENT, "2026-11-06T08:00:00Z", "2026-11-06T08:00:00Z"),
        ],
        "@odata.nextLink": "https://graph.microsoft.com/v1.0/x",
    });
    let far = Projection::OutlookMessageIds
        .apply(&two, folders_ctx(99_999))
        .unwrap();
    assert!(far.get("next-skip").is_none());
    assert_eq!(far["limit-reached"], json!(true));
    let edge = Projection::OutlookMessageIds
        .apply(&two, folders_ctx(99_998))
        .unwrap();
    assert_eq!(edge["next-skip"], json!(100_000));
}

#[test]
fn a_listing_page_continues_from_graphs_own_offset_when_it_names_one() {
    let at = "2026-11-06T08:00:00Z";
    let page = |ids: &[&str], link: &str| {
        json!({
            "value": ids.iter().map(|id| listed(id, at, at)).collect::<Vec<_>>(),
            "@odata.nextLink": link,
        })
    };
    let next = |body: &Json, skip: u64| {
        Projection::OutlookMessageIds
            .apply(body, folders_ctx(skip))
            .unwrap()
    };
    let graph = "https://graph.microsoft.com/v1.0/me/mailFolders/x/messages?%24top=3&%24skip=60";
    // Graph passed over items it did not return: its offset is the next page,
    // not the 53 a count of returned entries gives.
    assert_eq!(
        next(&page(&[FOLDER, PARENT, MESSAGE], graph), 50)["next-skip"],
        json!(60)
    );
    // An empty page still moves on when Graph names a later offset.
    assert_eq!(next(&page(&[], graph), 50)["next-skip"], json!(60));
    // An empty page with no usable offset cannot be continued by skip: more is
    // available, and neither a next-skip nor the limit is claimed.
    let bare = next(
        &page(&[], "https://graph.microsoft.com/v1.0/x?%24skiptoken=abc"),
        50,
    );
    assert_eq!(bare["more-available"], json!(true));
    assert!(bare.get("next-skip").is_none());
    assert!(bare.get("limit-reached").is_none());
    // An offset that does not move forward, or a link off Graph, is not used:
    // the count of what came back is, which can repeat but never skips.
    let three = [FOLDER, PARENT, MESSAGE];
    let behind = "https://graph.microsoft.com/v1.0/x?%24skip=40";
    assert_eq!(next(&page(&three, behind), 50)["next-skip"], json!(53));
    let elsewhere = "https://example.com/v1.0/x?%24skip=60";
    assert_eq!(next(&page(&three, elsewhere), 50)["next-skip"], json!(53));
    let twice = "https://graph.microsoft.com/v1.0/x?%24skip=60&%24skip=70";
    assert_eq!(next(&page(&three, twice), 50)["next-skip"], json!(53));
    // Graph's offset past the largest skip AWARE sends is the limit.
    let far = "https://graph.microsoft.com/v1.0/x?%24skip=100001";
    let out = next(&page(&three, far), 50);
    assert!(out.get("next-skip").is_none());
    assert_eq!(out["limit-reached"], json!(true));
}

#[tokio::test]
async fn a_listing_call_reads_the_folder_with_mail_read_basic_and_immutable_ids() {
    let data = json!({"value": [listed(MESSAGE, "2026-11-06T08:00:00Z", "2026-11-06T08:00:00Z")]});
    let basic = "openid profile offline_access User.Read Mail.ReadBasic";
    let (out, f) = call_read(
        "outlook.mail.folder.list-message-ids",
        basic,
        json!({"folder-id": FOLDER, "received-after": "2026-08-08T00:00:00Z"}),
        data,
    )
    .await;
    let body = body_of(out);
    assert_eq!(body["messages"][0]["id"], json!(MESSAGE));
    let heads = data_heads(&f);
    assert_eq!(heads.len(), 1);
    assert_eq!(
        path_only(&heads[0]),
        format!("/v1.0/me/mailFolders/{FOLDER}/messages")
    );
    let query = query_of(&heads[0]);
    assert!(query.contains(&(
        "$filter".to_string(),
        "receivedDateTime ge 2026-08-08T00:00:00Z".to_string()
    )));
    assert!(query.contains(&("$orderby".to_string(), "receivedDateTime desc".to_string())));
    assert!(
        heads[0]
            .lines()
            .any(|l| l.eq_ignore_ascii_case("prefer: IdType=\"ImmutableId\""))
    );
}

#[tokio::test]
async fn a_listing_over_the_raw_cap_is_raw_too_large_and_without_a_scope_sends_nothing() {
    let huge: Vec<Json> = (0..30_000)
        .map(|i| {
            listed(
                &format!("AAkA{i:0>300}"),
                "2026-11-06T08:00:00Z",
                "2026-11-06T08:00:00Z",
            )
        })
        .collect();
    let body = json!({"value": huge});
    assert!(body.to_string().len() > OUTLOOK_READ_CAP);
    let (out, _f) = call_read(
        "outlook.mail.folder.list-message-ids",
        MAIL_SCOPES,
        json!({"folder-id": FOLDER, "received-after": "2026-08-08T00:00:00Z"}),
        body,
    )
    .await;
    let failure = out.unwrap_err();
    assert_eq!(failure.code, "E_CALL_OUTPUT_TOO_LARGE");
    assert_eq!(failure.details["reason"], json!("raw-too-large"));
    let (out, f) = call_read(
        "outlook.mail.folder.list-message-ids",
        "openid profile offline_access User.Read Files.Read.All",
        json!({"folder-id": FOLDER, "received-after": "2026-08-08T00:00:00Z"}),
        json!({"value": []}),
    )
    .await;
    assert_eq!(out.unwrap_err().code, "E_CALL_SCOPE_MISSING");
    assert!(data_heads(&f).is_empty());
}
