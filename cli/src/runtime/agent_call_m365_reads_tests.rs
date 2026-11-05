//! Microsoft 365 reads beyond `list-folder` (#709 slice B1): `get-drive-item`,
//! `outlook.mail.search`, Planner and SharePoint lists. A child of the
//! `list-folder` tests, so it shares their Graph fixture and helpers.

use super::*;

const PLAN: &str = "xqQg5FS2LkCp935s-FIFm2QAFkHM";
const SITE: &str = "contoso.sharepoint.com,2C712604-1370-44E7-A1F5-426573FDA80A,2D2244C3-251A-49EA-93A8-39E1C3A060FE";
const LIST: &str = "243bca4b-4e5e-45af-b37d-25f6135a740d";
/// Every read scope B1 needs, as a person who connected for all of them holds.
const ALL_SCOPES: &str =
    "openid profile offline_access User.Read Files.Read.All Mail.Read Tasks.Read Sites.Read.All";

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

/// The decoded query pairs of a request head, in order.
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
        .filter(|h| {
            !is_identity_path(&path_of(h))
        })
        .collect()
}

async fn call_read(
    command: &str,
    inputs: Json,
    data: Json,
) -> (Result<Json, CallFailure>, Fixture) {
    let h = ms_home();
    ms_store(h.path(), None, ALL_SCOPES);
    let f = graph(me(OID, UPN), Reply::json(200, &data));
    let out = call_with(
        h.path(),
        &parsed(&request_for(h.path(), command, inputs)),
        &ms_endpoints(&f),
    )
    .await;
    (out, f)
}

// ── grammars (no network) ────────────────────────────────────────────────────

#[test]
fn site_and_list_ids_accept_only_their_documented_shapes() {
    let site = IdKind::SharePointSiteId;
    assert!(site.accepts("root"));
    assert!(site.accepts(SITE));
    assert!(site.accepts("a-b.sharepoint.com,2c712604-1370-44e7-a1f5-426573fda80a,2D2244C3-251A-49EA-93A8-39E1C3A060FE"));
    for bad in [
        "",
        "..",
        ".",
        "ROOT",
        "contoso.sharepoint.com",
        "contoso.sharepoint.com,2C712604-1370-44E7-A1F5-426573FDA80A",
        "contoso..sharepoint.com,2C712604-1370-44E7-A1F5-426573FDA80A,2D2244C3-251A-49EA-93A8-39E1C3A060FE",
        ".contoso.com,2C712604-1370-44E7-A1F5-426573FDA80A,2D2244C3-251A-49EA-93A8-39E1C3A060FE",
        "-a.com,2C712604-1370-44E7-A1F5-426573FDA80A,2D2244C3-251A-49EA-93A8-39E1C3A060FE",
        "a/b,2C712604-1370-44E7-A1F5-426573FDA80A,2D2244C3-251A-49EA-93A8-39E1C3A060FE",
        "a.com,2C712604-1370-44E7-A1F5-426573FDA80G,2D2244C3-251A-49EA-93A8-39E1C3A060FE",
        "a.com,2C712604-1370-44E7-A1F5-426573FDA80A,2D2244C3-251A-49EA-93A8-39E1C3A060FE,x",
        "a%2Fb.com,2C712604-1370-44E7-A1F5-426573FDA80A,2D2244C3-251A-49EA-93A8-39E1C3A060FE",
    ] {
        assert!(!site.accepts(bad), "{bad}");
    }
    assert!(IdKind::Guid.accepts(LIST));
    for bad in [
        "",
        "Door schedule",
        "243bca4b4e5e45afb37d25f6135a740d",
        "243bca4b-4e5e-45af-b37d-25f6135a740",
        "{243bca4b-4e5e-45af-b37d-25f6135a740d}",
    ] {
        assert!(!IdKind::Guid.accepts(bad), "{bad}");
    }
}

#[test]
fn mail_search_takes_plain_words_and_one_address_never_kql() {
    let words = SearchFormat::Words;
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
    for bad in [
        "",
        "   ",
        "OR",
        "budget or draft",
        "budget NOT draft",
        "near",
        "-draft",
        "budget -draft",
        "subject:RFI",
        "RFI*",
        "(RFI)",
        "\"RFI\"",
        "a\\b",
        "a&b",
        "a#b",
        "a=b",
        "a%b",
        "a\tb",
        seventeen.as_str(),
    ] {
        assert_eq!(words.terms(bad), None, "{bad}");
    }
    let from = SearchFormat::FromAddress;
    assert_eq!(
        from.terms("a%b+c.d@acme.co.uk"),
        Some(vec!["from:a%b+c.d@acme.co.uk".into()])
    );
    for bad in [
        "",
        "acme.com",
        "@acme.com",
        "a@",
        "a@b..c",
        "a@-b.c",
        "a b@c.d",
        "a@b.c d",
        "a\"@b.c",
        "a@b@c.d",
        "a:b@c.d",
    ] {
        assert_eq!(from.terms(bad), None, "{bad}");
    }
}

// ── the wire each command sends ──────────────────────────────────────────────

/// A command, its inputs, the URL it must build and the query it must send.
type WireCase<'a> = (&'a str, Json, &'a str, Vec<(&'a str, &'a str)>);

#[test]
fn each_read_builds_exactly_its_documented_request() {
    let endpoints = Endpoints {
        token: TokenPin::MicrosoftLogin,
        identity_url: MICROSOFT_ME_URL.into(),
        data_origin: None,
        allowed_origins: vec![MICROSOFT_GRAPH_ORIGIN.into()],
    };
    let items_url = format!("https://graph.microsoft.com/v1.0/sites/{SITE}/lists/{LIST}/items");
    let cases: Vec<WireCase> = vec![
        (
            "get-drive-item",
            json!({"drive-id": DRIVE, "item-id": "01ABC!123"}),
            "https://graph.microsoft.com/v1.0/drives/b!AbC-_9xYz/items/01ABC!123",
            vec![("$select", GET_ITEM_SELECT)],
        ),
        (
            "get-drive-item",
            json!({"drive-id": "me", "item-id": "root"}),
            "https://graph.microsoft.com/v1.0/me/drive/items/root",
            vec![("$select", GET_ITEM_SELECT)],
        ),
        (
            "outlook.mail.search",
            json!({"text": "RFI 042", "from": "a%b+c@acme.com", "page-size": 5}),
            "https://graph.microsoft.com/v1.0/me/messages",
            vec![
                ("$top", "5"),
                ("$search", "\"RFI 042 from:a%b+c@acme.com\""),
                ("$select", MAIL_SEARCH_SELECT),
            ],
        ),
        (
            "outlook.mail.search",
            json!({"from": "pm@acme.com"}),
            "https://graph.microsoft.com/v1.0/me/messages",
            vec![
                ("$top", "25"),
                ("$search", "\"from:pm@acme.com\""),
                ("$select", MAIL_SEARCH_SELECT),
            ],
        ),
        (
            "planner.plan.list",
            json!({"page-size": 7}),
            "https://graph.microsoft.com/v1.0/me/planner/plans",
            vec![],
        ),
        (
            "planner.task.list",
            json!({"plan-id": PLAN}),
            "https://graph.microsoft.com/v1.0/planner/plans/xqQg5FS2LkCp935s-FIFm2QAFkHM/tasks",
            vec![],
        ),
        (
            "sharepoint.list.list",
            json!({"site-id": "root"}),
            "https://graph.microsoft.com/v1.0/sites/root/lists",
            vec![("$select", SP_LISTS_SELECT)],
        ),
        (
            "sharepoint.list.read",
            json!({"site-id": SITE, "list-id": LIST, "page-size": 3}),
            &items_url,
            vec![("$expand", "fields")],
        ),
    ];
    for (command, inputs, url, query) in cases {
        let op = read(command);
        let wire = wire(op, &map(inputs.clone())).unwrap();
        assert_eq!(
            endpoints.data_url(op, &wire).unwrap(),
            url,
            "{command} {inputs}"
        );
        let sent: Vec<(&str, &str)> = wire
            .query
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        assert_eq!(sent, query, "{command} {inputs}");
    }
    // Local page bounds are applied to the projection, never sent.
    let wire = wire(read("planner.plan.list"), &map(json!({"page-size": 7}))).unwrap();
    assert_eq!(wire.limit, 7);
    let wire = wire_default_limit(
        "sharepoint.list.read",
        json!({"site-id": "root", "list-id": LIST}),
    );
    assert_eq!(wire, 100);
    assert_eq!(
        wire_default_limit("outlook.mail.search", json!({"text": "x"})),
        25
    );
}

fn wire_default_limit(command: &str, inputs: Json) -> usize {
    wire(read(command), &map(inputs)).unwrap().limit
}

#[test]
fn inputs_outside_their_grammar_or_bounds_are_refused_before_any_url_exists() {
    let refused: Vec<(&str, Json, &str)> = vec![
        ("outlook.mail.search", json!({}), "text"),
        ("outlook.mail.search", json!({"page-size": 5}), "text"),
        (
            "outlook.mail.search",
            json!({"text": "budget -draft"}),
            "text",
        ),
        ("outlook.mail.search", json!({"text": "a OR b"}), "text"),
        (
            "outlook.mail.search",
            json!({"text": "x", "from": "nobody"}),
            "from",
        ),
        (
            "outlook.mail.search",
            json!({"text": "x", "page-size": 51}),
            "page-size",
        ),
        (
            "outlook.mail.search",
            json!({"text": "x", "page-size": 0}),
            "page-size",
        ),
        (
            "outlook.mail.search",
            json!({"text": "x", "query": "subject:x"}),
            "",
        ),
        ("planner.task.list", json!({}), "plan-id"),
        ("planner.task.list", json!({"plan-id": "a/b"}), "plan-id"),
        ("planner.task.list", json!({"plan-id": ".."}), "plan-id"),
        (
            "planner.task.list",
            json!({"plan-id": PLAN, "page-size": 201}),
            "page-size",
        ),
        (
            "planner.task.list",
            json!({"plan-id": PLAN, "completed": "open-only"}),
            "",
        ),
        ("sharepoint.list.list", json!({"site-id": ".."}), "site-id"),
        (
            "sharepoint.list.read",
            json!({"site-id": "root", "list-id": "Door schedule"}),
            "list-id",
        ),
        (
            "sharepoint.list.read",
            json!({"site-id": "root", "list-id": LIST, "filter": "x eq 1"}),
            "",
        ),
        ("get-drive-item", json!({"drive-id": "me"}), "item-id"),
        (
            "get-drive-item",
            json!({"drive-id": "me", "item-id": "root", "sharing-url": "https://x"}),
            "",
        ),
    ];
    for (command, inputs, input) in refused {
        let failure = wire(read(command), &map(inputs.clone())).unwrap_err();
        assert_eq!(failure.code, "E_CALL_INPUT_INVALID", "{command} {inputs}");
        if input.is_empty() {
            assert_eq!(
                failure.details["reason"],
                json!("unknown-input"),
                "{command} {inputs}"
            );
        } else {
            assert_eq!(failure.details["input"], json!(input), "{command} {inputs}");
        }
    }
}

#[tokio::test]
async fn a_refused_input_sends_no_request_at_all() {
    for (command, inputs) in [
        ("outlook.mail.search", json!({"text": "budget -draft"})),
        ("outlook.mail.search", json!({"text": "a&b"})),
        (
            "sharepoint.list.read",
            json!({"site-id": "..", "list-id": LIST}),
        ),
        ("planner.task.list", json!({"plan-id": "%2e%2e"})),
    ] {
        let (out, f) = call_read(command, inputs.clone(), json!({"value": []})).await;
        assert_eq!(
            out.unwrap_err().code,
            "E_CALL_INPUT_INVALID",
            "{command} {inputs}"
        );
        assert!(
            f.requests().is_empty(),
            "{command} {inputs}: {:?}",
            f.requests()
        );
    }
}

// ── digests ──────────────────────────────────────────────────────────────────

#[test]
fn slice_b_digests_bind_their_page_bound_formats_and_locations() {
    for command in [
        "get-drive-item",
        "outlook.mail.search",
        "planner.plan.list",
        "planner.task.list",
        "sharepoint.list.list",
        "sharepoint.list.read",
    ] {
        let canonical = canonical_string(&read(command).canonical());
        assert!(canonical.contains("\"canonical\":\"v2\""), "{command}");
        assert!(canonical.contains("\"limitInput\":"), "{command}");
    }
    let mail = canonical_string(&read("outlook.mail.search").canonical());
    for needle in [
        "\"format\":\"graph-search-words/1\"",
        "\"format\":\"graph-search-from/1\"",
        "\"location\":\"search\"",
        "\"limitInput\":\"page-size\"",
        "\"projection\":\"graph-messages/1\"",
        "\"Mail.Read\"",
    ] {
        assert!(mail.contains(needle), "{needle} not in {mail}");
    }
    let tasks = canonical_string(&read("planner.task.list").canonical());
    assert!(tasks.contains("\"location\":\"local\""));
    let read_items = canonical_string(&read("sharepoint.list.read").canonical());
    assert!(read_items.contains("\"pattern\":\"sharepoint-site-id\""));
    assert!(read_items.contains("\"pattern\":\"guid\""));
    assert!(read_items.contains("[\"$expand\",\"fields\"]"));

    // Each new semantic moves the digest.
    let base = read("outlook.mail.search");
    let mut other_limit = base.clone();
    other_limit.limit_input = None;
    assert_ne!(other_limit.sha256(), base.sha256());
    let mut other_format = base.clone();
    let mut inputs = base.inputs.to_vec();
    inputs[0].search = Some(SearchFormat::FromAddress);
    other_format.inputs = Box::leak(inputs.into_boxed_slice());
    assert_ne!(other_format.sha256(), base.sha256());
    let local = read("planner.plan.list");
    let mut sent = local.clone();
    let mut inputs = local.inputs.to_vec();
    inputs[0].location = Location::Query;
    sent.inputs = Box::leak(inputs.into_boxed_slice());
    assert_ne!(sent.sha256(), local.sha256());
    let mut v1 = local.clone();
    v1.canonical_v2 = false;
    assert_ne!(v1.sha256(), local.sha256());
    // Every operation's digest is distinct.
    let mut digests: Vec<String> = OPERATIONS.iter().map(Operation::sha256).collect();
    digests.sort();
    digests.dedup();
    assert_eq!(digests.len(), OPERATIONS.len());
}

// ── projections ──────────────────────────────────────────────────────────────

#[test]
fn a_drive_item_never_carries_a_download_url() {
    let item = project_drive_item(&json!({
        "id": "01ABC!123",
        "name": "DoorSchedule.xlsx",
        "size": 48213,
        "file": {"mimeType": "application/pdf"},
        "lastModifiedDateTime": "2026-10-01T10:00:00Z",
        "webUrl": "https://acme.sharepoint.com/x.xlsx",
        "parentReference": {"driveId": DRIVE},
        "@microsoft.graph.downloadUrl": "https://must-not-leave.example/signed",
        "createdBy": {"user": {"email": "must-not-leave"}}
    }))
    .unwrap();
    assert_eq!(
        item,
        json!({"id": "01ABC!123", "drive-id": DRIVE, "name": "DoorSchedule.xlsx", "is-folder": false,
               "size": 48213, "mime-type": "application/pdf",
               "modified-time": "2026-10-01T10:00:00Z", "web-url": "https://acme.sharepoint.com/x.xlsx"})
    );
    assert_ms_clean(&item.to_string());
    assert!(project_drive_item(&json!({"name": "no id"})).is_none());
    assert!(project_drive_item(&json!({"id": "a\u{0001}b"})).is_none());
    assert!(project_drive_item(&json!({"id": "x", "parentReference": {"driveId": ""}})).is_none());
    let long = project_drive_item(&json!({"id": "x", "name": "n".repeat(1100)})).unwrap();
    assert_eq!(long["name"].as_str().unwrap().chars().count(), 1024);
    assert_eq!(long["truncated"], json!(["name"]));
}

#[test]
fn a_mail_page_keeps_headers_and_the_preview_never_the_body() {
    let body = json!({
        "value": [
            {"id": "AAMk1", "subject": "RE: RFI 042", "receivedDateTime": "2026-10-30T09:12:00Z",
             "from": {"emailAddress": {"address": "engineer@acme.com", "name": "A. Engineer"}},
             "bodyPreview": "p".repeat(600), "hasAttachments": true, "webLink": "https://outlook/x",
             "body": {"content": "must-not-leave"}},
            {"id": "", "subject": "dropped"},
            {"id": "AAMk3", "subject": "no sender"}
        ],
        "@odata.nextLink": "https://graph.microsoft.com/v1.0/me/messages?$skip=3"
    });
    let out = project_collection(&body, 10, "messages", message_entry).unwrap();
    let first = &out["messages"][0];
    assert_eq!(first["subject"], json!("RE: RFI 042"));
    assert_eq!(
        first["from"],
        json!({"address": "engineer@acme.com", "name": "A. Engineer"})
    );
    assert_eq!(first["preview"].as_str().unwrap().len(), 512);
    assert_eq!(first["truncated"], json!(["preview"]));
    assert_eq!(first["has-attachments"], json!(true));
    assert_eq!(
        out["messages"][1],
        json!({"id": "AAMk3", "subject": "no sender", "has-attachments": false})
    );
    assert_eq!(out["omitted"], json!(1));
    assert_eq!(out["more-available"], json!(true));
    assert_ms_clean(&out.to_string());
}

#[test]
fn plans_show_their_typed_container_never_the_owner() {
    let body = json!({"value": [
        {"id": "p1", "title": "Group plan", "owner": "must-not-leave",
         "container": {"containerId": "ebf3b108-5234", "type": "group", "url": "must-not-leave"},
         "createdDateTime": "2026-09-01T08:00:00Z"},
        {"id": "p2", "title": "Roster plan", "container": {"containerId": "roster-1", "type": "roster"}},
        {"id": "p3", "title": "No container"},
        {"id": "p4", "container": {"containerId": "bad\u{0002}id", "type": "group"}}
    ]});
    let out = project_collection(&body, 10, "plans", plan_entry).unwrap();
    assert_eq!(
        out,
        json!({
            "plans": [
                {"id": "p1", "title": "Group plan", "container": {"id": "ebf3b108-5234", "type": "group"},
                 "created-at": "2026-09-01T08:00:00Z"},
                {"id": "p2", "title": "Roster plan", "container": {"id": "roster-1", "type": "roster"}},
                {"id": "p3", "title": "No container"}
            ],
            "more-available": false,
            "omitted": 1
        })
    );
    // A local cut is reported even with no continuation from Graph.
    let cut = project_collection(&body, 2, "plans", plan_entry).unwrap();
    assert_eq!(cut["plans"].as_array().unwrap().len(), 2);
    assert_eq!(cut["more-available"], json!(true));
}

#[test]
fn task_assignees_beyond_the_cap_or_malformed_are_counted() {
    let mut assignments = Map::new();
    for i in 0..51 {
        assignments.insert(format!("user-{i:02}"), json!({"orderHint": "x"}));
    }
    assignments.insert("bad\u{0003}key".into(), json!({}));
    let body = json!({"value": [
        {"id": "t1", "title": "Issue IFC set", "bucketId": "b1", "percentComplete": 50,
         "dueDateTime": "2026-11-10T17:00:00Z", "assignments": assignments},
        {"id": "t2", "bucketId": "bad\u{0004}bucket"},
        {"id": "t3", "percentComplete": 150, "completedDateTime": "2026-10-01T00:00:00Z"}
    ]});
    let out = project_collection(&body, 10, "tasks", task_entry).unwrap();
    let first = &out["tasks"][0];
    assert_eq!(first["assigned-to"].as_array().unwrap().len(), 50);
    assert_eq!(first["assigned-to-omitted"], json!(2));
    assert_eq!(first["percent-complete"], json!(50));
    assert_eq!(first["due-date"], json!("2026-11-10T17:00:00Z"));
    assert_eq!(
        out["tasks"][1],
        json!({"id": "t3", "completed-at": "2026-10-01T00:00:00Z", "assigned-to": [], "assigned-to-omitted": 0})
    );
    assert_eq!(out["omitted"], json!(1));
}

#[test]
fn list_rows_pass_scalars_and_count_everything_else() {
    let mut many = Map::new();
    for i in 0..205 {
        many.insert(format!("C{i:03}"), json!(i));
    }
    let body = json!({"value": [
        {"id": "2", "fields": {
            "@odata.etag": "\"x\"",
            "Title": "D-101",
            "Quantity": 4,
            "Approved": true,
            "Notes": null,
            "AuthorLookupId": "12",
            "Long": "x".repeat(3000),
            "Tags": ["a", "b"],
            "TagsLong": ["y".repeat(2100)],
            "Person": {"LookupId": 12, "LookupValue": "must-not-leave"},
            "TooMany": vec![1; 51],
            "Mixed": ["a", {"x": 1}],
            "": "empty key"
        }},
        {"id": "3"},
        {"id": "4", "fields": many},
        {"fields": {"Title": "no id"}}
    ]});
    let out = project_collection(&body, 10, "items", list_item_entry).unwrap();
    let row = &out["items"][0];
    let fields = row["fields"].as_object().unwrap();
    assert_eq!(fields["Title"], json!("D-101"));
    assert_eq!(fields["Quantity"], json!(4));
    assert_eq!(fields["Approved"], json!(true));
    assert_eq!(fields["Notes"], json!(null));
    assert_eq!(fields["AuthorLookupId"], json!("12"));
    assert_eq!(fields["Long"].as_str().unwrap().len(), 2048);
    assert_eq!(fields["Tags"], json!(["a", "b"]));
    assert_eq!(fields["TagsLong"][0].as_str().unwrap().len(), 2048);
    assert!(!fields.contains_key("@odata.etag"));
    assert!(!fields.contains_key("Person"));
    // Person, TooMany, Mixed and the empty key; the annotation is not a column.
    assert_eq!(row["skipped-fields"], json!(4));
    assert_eq!(row["truncated-fields"], json!(["Long", "TagsLong"]));
    assert!(row.get("fields-missing").is_none());
    assert_eq!(
        out["items"][1],
        json!({"id": "3", "fields": {}, "skipped-fields": 0, "fields-missing": true})
    );
    assert_eq!(out["items"][2]["fields"].as_object().unwrap().len(), 200);
    assert_eq!(out["items"][2]["skipped-fields"], json!(5));
    assert_eq!(out["omitted"], json!(1));
    assert_ms_clean(&out.to_string());
}

#[test]
fn site_lists_keep_name_link_and_template() {
    let body = json!({"value": [
        {"id": LIST, "displayName": "Door schedule", "webUrl": "https://acme/Lists/Doors",
         "list": {"template": "genericList", "hidden": false}, "lastModifiedDateTime": "2026-10-28T14:02:00Z",
         "createdBy": {"user": {"email": "must-not-leave"}}}
    ]});
    let out = project_collection(&body, 10, "lists", list_entry).unwrap();
    assert_eq!(
        out,
        json!({"lists": [{"id": LIST, "name": "Door schedule", "web-url": "https://acme/Lists/Doors",
                          "template": "genericList", "modified-time": "2026-10-28T14:02:00Z"}],
               "more-available": false, "omitted": 0})
    );
}

// ── through the call path ────────────────────────────────────────────────────

#[tokio::test]
async fn each_read_runs_through_the_bound_call_and_returns_only_its_projection() {
    let next = "https://evil.example/v1.0/next";
    let items_path = format!("/v1.0/sites/{SITE}/lists/{LIST}/items");
    let cases: Vec<(&str, Json, Json, &str)> = vec![
        (
            "get-drive-item",
            json!({"drive-id": "me", "item-id": "root"}),
            json!({"id": "root-id", "name": "root", "folder": {}, "parentReference": {"driveId": DRIVE},
                   "@microsoft.graph.downloadUrl": "https://must-not-leave.example"}),
            "/v1.0/me/drive/items/root",
        ),
        (
            "outlook.mail.search",
            json!({"text": "RFI żuraw", "from": "a+b@acme.com"}),
            json!({"value": [{"id": "m1", "subject": "RFI"}], "@odata.nextLink": next}),
            "/v1.0/me/messages",
        ),
        (
            "planner.plan.list",
            json!({}),
            json!({"value": [{"id": PLAN, "title": "Plan"}], "@odata.nextLink": next}),
            "/v1.0/me/planner/plans",
        ),
        (
            "planner.task.list",
            json!({"plan-id": PLAN, "page-size": 1}),
            json!({"value": [{"id": "t1"}, {"id": "t2"}]}),
            "/v1.0/planner/plans/xqQg5FS2LkCp935s-FIFm2QAFkHM/tasks",
        ),
        (
            "sharepoint.list.list",
            json!({"site-id": "root"}),
            json!({"value": [{"id": LIST, "displayName": "Doors"}]}),
            "/v1.0/sites/root/lists",
        ),
        (
            "sharepoint.list.read",
            json!({"site-id": SITE, "list-id": LIST}),
            json!({"value": [{"id": "2", "fields": {"Title": "D-101"}}]}),
            &items_path,
        ),
    ];
    for (command, inputs, data, path) in cases {
        let (out, f) = call_read(command, inputs.clone(), data).await;
        let out = out.unwrap_or_else(|e| panic!("{command}: {e:?}"));
        let body = &out["result"]["body"];
        assert_ms_clean(&out.to_string());
        let heads = data_heads(&f);
        assert_eq!(
            heads.len(),
            1,
            "{command}: one data request, nextLink never followed"
        );
        assert_eq!(path_only(&heads[0]), path, "{command}");
        assert!(heads[0].contains(&format!("Bearer {MS_TOKEN}")));
        match command {
            "get-drive-item" => {
                assert_eq!(body["id"], json!("root-id"));
                assert_eq!(body["is-folder"], json!(true));
            }
            "outlook.mail.search" => {
                let query = query_of(&heads[0]);
                let search: Vec<&(String, String)> =
                    query.iter().filter(|(k, _)| k == "$search").collect();
                assert_eq!(search.len(), 1);
                assert_eq!(search[0].1, "\"RFI żuraw from:a+b@acme.com\"");
                assert_eq!(query.len(), 3, "{query:?}");
                assert_eq!(body["more-available"], json!(true));
            }
            "planner.plan.list" => {
                assert!(query_of(&heads[0]).is_empty());
                assert_eq!(body["plans"][0]["id"], json!(PLAN));
                assert_eq!(body["more-available"], json!(true));
            }
            "planner.task.list" => {
                assert!(query_of(&heads[0]).is_empty());
                assert_eq!(body["tasks"].as_array().unwrap().len(), 1);
                assert_eq!(body["more-available"], json!(true));
            }
            "sharepoint.list.list" => {
                assert!(query_of(&heads[0]).iter().all(|(k, _)| k != "$top"));
                assert_eq!(body["lists"][0]["name"], json!("Doors"));
            }
            "sharepoint.list.read" => {
                assert_eq!(
                    query_of(&heads[0]),
                    vec![("$expand".to_string(), "fields".to_string())]
                );
                assert_eq!(body["items"][0]["fields"]["Title"], json!("D-101"));
            }
            _ => unreachable!(),
        }
    }
}

#[tokio::test]
async fn without_the_read_scope_nothing_is_sent_and_the_hint_names_it() {
    for (command, inputs, scope, hint) in [
        (
            "outlook.mail.search",
            json!({"text": "RFI"}),
            "offline_access User.Read Files.Read.All Mail.ReadBasic",
            "--scopes User.Read,Mail.Read",
        ),
        (
            "planner.task.list",
            json!({"plan-id": PLAN}),
            "offline_access User.Read Mail.Read",
            "--scopes User.Read,Tasks.Read",
        ),
        (
            "sharepoint.list.read",
            json!({"site-id": "root", "list-id": LIST}),
            "offline_access Tasks.Read Sites.Read.All",
            "--scopes User.Read,Sites.Read.All",
        ),
    ] {
        let h = ms_home();
        ms_store(h.path(), None, scope);
        let f = graph(me(OID, UPN), Reply::json(200, &json!({"value": []})));
        let cap = capabilities_with(
            h.path(),
            read(command),
            None,
            Duration::from_secs(10),
            &ms_endpoints(&f),
        )
        .await
        .unwrap();
        assert_eq!(cap["unavailableCode"], json!("scope-missing"), "{command}");
        assert!(
            cap["unavailableReason"].as_str().unwrap().contains(hint),
            "{command}: {}",
            cap["unavailableReason"]
        );
        assert_eq!(
            cap["permissions"]["requestScopes"],
            json!(read(command).request_scopes)
        );
        assert_eq!(cap["permissions"]["adminConsent"], json!(false));
        let failure = call_with(
            h.path(),
            &parsed(&request_for(h.path(), command, inputs)),
            &ms_endpoints(&f),
        )
        .await
        .unwrap_err();
        assert_eq!(failure.code, "E_CALL_SCOPE_MISSING", "{command}");
        assert!(data_heads(&f).is_empty(), "{command}");
    }
}

#[tokio::test]
async fn a_workflow_node_reads_planner_tasks_bounded_locally() {
    let h = ms_home();
    ms_store(h.path(), None, "offline_access Tasks.Read");
    let f = graph(
        me(OID, UPN),
        Reply::json(
            200,
            &json!({"value": [{"id": "t1"}, {"id": "t2"}, {"id": "t3"}]}),
        ),
    );
    let manifest: Agent = serde_yaml::from_str(&ms_runnable_manifest()).unwrap();
    let out = run_for_workflow_with(
        h.path(),
        &manifest,
        "planner.task.list",
        &json!({"plan-id": PLAN, "page-size": 2}),
        Some(&ms_endpoints(&f)),
        Duration::from_secs(10),
        1 << 20,
    )
    .await
    .unwrap();
    assert_eq!(out["tasks"].as_array().unwrap().len(), 2);
    assert_eq!(out["more-available"], json!(true));
    let heads = data_heads(&f);
    assert_eq!(heads.len(), 1);
    assert!(query_of(&heads[0]).is_empty());
    assert!(
        f.requests()
            .iter()
            .all(|h| !path_of(h).starts_with("/v1.0/me?"))
    );
}
