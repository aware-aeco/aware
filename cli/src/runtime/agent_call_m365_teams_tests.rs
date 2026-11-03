//! Microsoft Teams reads (#709 slice C): teams, channels, channel messages,
//! chats and chat messages. A child of the `list-folder` tests, so it shares
//! their Graph fixture and helpers.

use super::*;

const TEAM: &str = "ebf3b108-5234-4e22-b93d-656d7dae5874";
const CHANNEL: &str = "19:a1b2c3d4e5f6@thread.tacv2";
const CHAT: &str = "19:2da4c29f6d7041eca70b638b43d45437@thread.v2";
const ONE_ON_ONE: &str =
    "19:8b081ef6-4792-4def-b2c9-c363a1bf41d5_5031bb31-22c0-4f6f-9f73-91d34ab2b32d@unq.gbl.spaces";
const SINCE: &str = "2026-11-02T10:00:00Z";
/// Every scope slice C reads with, as a person who connected for all of them holds.
const TEAMS_SCOPES: &str = "openid profile offline_access User.Read Team.ReadBasic.All \
     Channel.ReadBasic.All ChannelMessage.Read.All Chat.ReadBasic Chat.Read";

type WireCase<'a> = (&'a str, Json, &'a str, Vec<(&'a str, &'a str)>);

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

fn is_identity(head: &str) -> bool {
    let p = path_of(head);
    p == "/v1.0/me" || p.starts_with("/v1.0/me?")
}

fn data_heads(f: &Fixture) -> Vec<String> {
    f.requests()
        .into_iter()
        .filter(|h| !is_identity(h))
        .collect()
}

fn has_prefer_header(head: &str) -> bool {
    head.lines()
        .any(|l| l.eq_ignore_ascii_case("prefer: include-unknown-enum-members"))
}

async fn call_read(
    command: &str,
    inputs: Json,
    data: Json,
) -> (Result<Json, CallFailure>, Fixture) {
    let h = ms_home();
    ms_store(h.path(), None, TEAMS_SCOPES);
    let f = graph(me(OID, UPN), Reply::json(200, &data));
    let out = call_with(
        h.path(),
        &parsed(&request_for(h.path(), command, inputs)),
        &ms_endpoints(&f),
    )
    .await;
    (out, f)
}

fn ctx(limit: usize, since: Option<&str>) -> ProjectionContext {
    ProjectionContext {
        range: None,
        limit,
        since: since.map(|s| utc_timestamp(s).unwrap()),
    }
}

fn msg(id: &str, created: &str) -> Json {
    json!({
        "id": id,
        "createdDateTime": created,
        "lastModifiedDateTime": created,
        "messageType": "message",
        "from": {"user": {"id": "u-1", "displayName": "Anna"}},
        "body": {"contentType": "text", "content": format!("text of {id}")},
    })
}

fn thread(id: &str, created: &str, replies: Vec<Json>) -> Json {
    let mut root = msg(id, created);
    root["replies"] = Json::Array(replies);
    root
}

fn ids(body: &Json) -> Vec<String> {
    body["messages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| m["id"].as_str().unwrap().to_string())
        .collect()
}

// ── grammars (no network) ────────────────────────────────────────────────────

#[test]
fn teams_thread_ids_accept_only_their_documented_shapes() {
    let kind = IdKind::TeamsThreadId;
    for good in [
        CHANNEL,
        CHAT,
        ONE_ON_ONE,
        "19:meeting_MjdhNjM4YzUtYzExZi00OTFkLTkzZTAtNTVlNmZmMDhkNGU2@thread.v2",
        "19:abc@thread.skype",
    ] {
        assert!(kind.accepts(good), "{good}");
    }
    for bad in [
        "",
        "19:",
        "19:@thread.tacv2",
        "a1b2c3d4e5f6@thread.tacv2",
        "20:a1b2c3d4e5f6@thread.tacv2",
        "19:a1b2c3d4e5f6@thread.tacv3",
        "19:a1b2c3d4e5f6@thread.tacv2/..",
        "19:a1b2/c3@thread.tacv2",
        "19:a1b2%2Fc3@thread.tacv2",
        "19:a1b2.c3@thread.tacv2",
        "19:a1 b2@thread.tacv2",
        "19:a1b2@thread.tacv2?x=1",
        "19:a1b2@thread.tacv2#x",
        "19:..@thread.tacv2",
        "..",
    ] {
        assert!(!kind.accepts(bad), "{bad}");
    }
    let long = format!("19:{}@thread.v2", "a".repeat(201));
    assert!(!kind.accepts(&long));
    let longest = format!("19:{}@thread.v2", "a".repeat(200));
    assert!(kind.accepts(&longest));
}

#[test]
fn since_is_exactly_a_utc_instant() {
    assert!(utc_timestamp("2026-11-02T10:00:00Z").is_some());
    assert!(utc_timestamp("2026-11-02T10:00:00.5Z").is_some());
    assert!(utc_timestamp("2026-11-02T10:00:00.1234567Z").is_some());
    assert!(utc_timestamp("2024-02-29T00:00:00Z").is_some());
    for bad in [
        "",
        "2026-11-02",
        "2026-11-02T10:00:00",
        "2026-11-02T10:00:00+00:00",
        "2026-11-02T10:00:00+01:00",
        "2026-11-02 10:00:00Z",
        "2026-11-02T10:00Z",
        "2026-11-02T24:00:00Z",
        "2026-11-02T10:60:00Z",
        "2026-11-02T10:00:60Z",
        "2026-02-30T10:00:00Z",
        "2025-02-29T10:00:00Z",
        "2026-13-01T10:00:00Z",
        "2026-11-02T10:00:00.Z",
        "2026-11-02T10:00:00.12345678Z",
        "2026-11-02T10:00:00z",
        "+2026-11-02T10:00:00Z",
        "2026-11-02T10:00:00Z ",
        "lastModifiedDateTime gt 2026",
    ] {
        assert!(utc_timestamp(bad).is_none(), "{bad}");
    }
}

#[test]
fn each_teams_read_builds_exactly_its_documented_request() {
    let endpoints = Endpoints {
        token: TokenPin::MicrosoftLogin,
        identity_url: MICROSOFT_ME_URL.into(),
        data_origin: None,
        allowed_origins: vec![MICROSOFT_GRAPH_ORIGIN.into()],
    };
    let channel_messages = format!(
        "https://graph.microsoft.com/v1.0/teams/{TEAM}/channels/19:a1b2c3d4e5f6@thread.tacv2/messages"
    );
    let chat_messages = "https://graph.microsoft.com/v1.0/chats/19:2da4c29f6d7041eca70b638b43d45437@thread.v2/messages".to_string();
    let channels = format!("https://graph.microsoft.com/v1.0/teams/{TEAM}/channels");
    let cases: Vec<WireCase> = vec![
        (
            "teams.team.list",
            json!({"page-size": 3}),
            "https://graph.microsoft.com/v1.0/me/joinedTeams",
            vec![],
        ),
        (
            "teams.channel.list",
            json!({"team-id": TEAM}),
            &channels,
            vec![("$select", CHANNELS_SELECT)],
        ),
        (
            "teams.channel.list-messages",
            json!({"team-id": TEAM, "channel-id": CHANNEL, "since": SINCE, "page-size": 10}),
            &channel_messages,
            vec![("$top", "10"), ("$expand", "replies")],
        ),
        (
            "teams.chat.list",
            json!({}),
            "https://graph.microsoft.com/v1.0/me/chats",
            vec![
                ("$top", "25"),
                ("$orderby", "lastMessagePreview/createdDateTime desc"),
            ],
        ),
        (
            "teams.chat.list-messages",
            json!({"chat-id": CHAT, "since": "2026-11-02T10:00:00.5Z"}),
            &chat_messages,
            vec![
                (
                    "$filter",
                    "lastModifiedDateTime gt 2026-11-02T10:00:00.500Z",
                ),
                ("$top", "20"),
                ("$orderby", "lastModifiedDateTime desc"),
            ],
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
    // The channel `since` is never sent; it reaches the projection.
    let wire = wire(
        read("teams.channel.list-messages"),
        &map(json!({"team-id": TEAM, "channel-id": CHANNEL, "since": SINCE})),
    )
    .unwrap();
    assert!(
        wire.query
            .iter()
            .all(|(k, v)| k != "$filter" && !v.contains("2026"))
    );
    assert_eq!(wire.since, utc_timestamp(SINCE));
    assert_eq!(wire.limit, 20);
    assert_eq!(
        wire_limit("teams.team.list", json!({})),
        100,
        "a local bound"
    );
}

fn wire_limit(command: &str, inputs: Json) -> usize {
    wire(read(command), &map(inputs)).unwrap().limit
}

#[test]
fn teams_inputs_outside_their_grammar_or_bounds_are_refused() {
    let refused: Vec<(&str, Json, &str)> = vec![
        ("teams.channel.list", json!({}), "team-id"),
        (
            "teams.channel.list",
            json!({"team-id": "Acme Tower"}),
            "team-id",
        ),
        ("teams.channel.list", json!({"team-id": ".."}), "team-id"),
        (
            "teams.channel.list-messages",
            json!({"team-id": TEAM, "channel-id": CHANNEL}),
            "since",
        ),
        (
            "teams.channel.list-messages",
            json!({"team-id": TEAM, "channel-id": "General", "since": SINCE}),
            "channel-id",
        ),
        (
            "teams.channel.list-messages",
            json!({"team-id": TEAM, "channel-id": CHANNEL, "since": "yesterday"}),
            "since",
        ),
        (
            "teams.channel.list-messages",
            json!({"team-id": TEAM, "channel-id": CHANNEL, "since": SINCE, "page-size": 51}),
            "page-size",
        ),
        (
            "teams.chat.list-messages",
            json!({"chat-id": CHAT, "since": "2026-11-02T10:00:00Z or true"}),
            "since",
        ),
        (
            "teams.chat.list-messages",
            json!({"chat-id": CHAT, "since": "2026-11-02T10:00:00+01:00"}),
            "since",
        ),
        (
            "teams.chat.list-messages",
            json!({"chat-id": "19:x@thread.v2/../../me", "since": SINCE}),
            "chat-id",
        ),
        (
            "teams.chat.list-messages",
            json!({"chat-id": CHAT, "since": SINCE, "filter": "x"}),
            "",
        ),
        ("teams.chat.list", json!({"page-size": 0}), "page-size"),
        ("teams.team.list", json!({"page-size": 201}), "page-size"),
    ];
    for (command, inputs, input) in refused {
        let failure = wire(read(command), &map(inputs.clone())).unwrap_err();
        assert_eq!(failure.code, "E_CALL_INPUT_INVALID", "{command} {inputs}");
        if input.is_empty() {
            assert_eq!(failure.details["reason"], json!("unknown-input"));
        } else {
            assert_eq!(failure.details["input"], json!(input), "{command} {inputs}");
        }
    }
}

#[tokio::test]
async fn a_refused_teams_input_sends_no_request_at_all() {
    for (command, inputs) in [
        (
            "teams.chat.list-messages",
            json!({"chat-id": CHAT, "since": "2026-11-02"}),
        ),
        (
            "teams.channel.list-messages",
            json!({"team-id": TEAM, "channel-id": "19:a%2F@thread.tacv2", "since": SINCE}),
        ),
        ("teams.channel.list", json!({"team-id": "%2e%2e"})),
    ] {
        let (out, f) = call_read(command, inputs.clone(), json!({"value": []})).await;
        assert_eq!(out.unwrap_err().code, "E_CALL_INPUT_INVALID", "{command}");
        assert!(f.requests().is_empty(), "{command}: {:?}", f.requests());
    }
}

// ── digests ──────────────────────────────────────────────────────────────────

#[test]
fn teams_digests_bind_their_formats_locations_and_headers() {
    let teams = [
        "teams.team.list",
        "teams.channel.list",
        "teams.channel.list-messages",
        "teams.chat.list",
        "teams.chat.list-messages",
    ];
    for command in teams {
        let canonical = canonical_string(&read(command).canonical());
        assert!(canonical.contains("\"canonical\":\"v2\""), "{command}");
        let sends_headers = command.ends_with("list-messages");
        assert_eq!(
            canonical.contains("[\"Prefer\",\"include-unknown-enum-members\"]"),
            sends_headers,
            "{command}"
        );
    }
    // Every earlier row's canonical form names no headers, so its digest is
    // the one it shipped with (list-folder and Google list-files are pinned
    // byte for byte elsewhere).
    for op in OPERATIONS {
        if !op.command.starts_with("teams.") {
            assert!(op.fixed_headers.is_empty(), "{}", op.command);
            assert!(
                !canonical_string(&op.canonical()).contains("\"headers\""),
                "{}",
                op.command
            );
        }
    }
    let channel = canonical_string(&read("teams.channel.list-messages").canonical());
    for needle in [
        "\"format\":\"graph-utc-timestamp/1\"",
        "\"location\":\"local\"",
        "\"pattern\":\"teams-thread-id\"",
        "\"pattern\":\"guid\"",
        "\"projection\":\"teams-channel-messages/1\"",
        "\"adminConsent\":true",
        "[\"$expand\",\"replies\"]",
    ] {
        assert!(channel.contains(needle), "{needle} not in {channel}");
    }
    let chat = canonical_string(&read("teams.chat.list-messages").canonical());
    for needle in [
        "\"location\":\"filter\"",
        "\"name\":\"lastModifiedDateTime\"",
        "\"projection\":\"teams-chat-messages/1\"",
    ] {
        assert!(chat.contains(needle), "{needle} not in {chat}");
    }
    // Moving `since` from Graph's filter to the projection moves the digest,
    // as does dropping the header.
    let base = read("teams.chat.list-messages");
    let mut local = base.clone();
    let mut inputs = base.inputs.to_vec();
    inputs[1].location = Location::Local;
    local.inputs = Box::leak(inputs.into_boxed_slice());
    assert_ne!(local.sha256(), base.sha256());
    let mut quiet = base.clone();
    quiet.fixed_headers = &[];
    assert_ne!(quiet.sha256(), base.sha256());
    let mut untyped = base.clone();
    let mut inputs = base.inputs.to_vec();
    inputs[1].timestamp = false;
    untyped.inputs = Box::leak(inputs.into_boxed_slice());
    assert_ne!(untyped.sha256(), base.sha256());
    let mut digests: Vec<String> = OPERATIONS.iter().map(Operation::sha256).collect();
    digests.sort();
    digests.dedup();
    assert_eq!(digests.len(), OPERATIONS.len());
}

// ── message text ─────────────────────────────────────────────────────────────

#[test]
fn html_bodies_become_plain_text_without_markup_or_scripts() {
    let text = |html: &str| message_text(&html_text(html), MAX_MESSAGE_TEXT).unwrap().0;
    assert_eq!(
        text("<p>Grid <b>C</b> reaction</p><p>is   120&nbsp;kN</p>"),
        "Grid C reaction\nis 120 kN"
    );
    assert_eq!(
        text("line one<br>line two<br/><br>line three"),
        "line one\nline two\nline three"
    );
    assert_eq!(
        text(
            "<div>before<script>alert('x')</script><style>p{}</style><template>t</template>after</div>"
        ),
        "beforeafter"
    );
    // Encoded markup is decoded once and stays literal text.
    assert_eq!(
        text("<p>use &lt;b&gt; for bold &amp;amp;</p>"),
        "use <b> for bold &amp;"
    );
    // A mention keeps its text.
    assert_eq!(
        text("<p><at id=\"0\">Anna Nowak</at> please check</p>"),
        "Anna Nowak please check"
    );
    assert_eq!(text("<ul><li>one</li><li>two</li></ul>"), "one\ntwo");
    // Attributes never reach the text.
    assert_eq!(
        text("<a href=\"https://evil.example/x\">link</a><img src=\"https://evil.example/i\">"),
        "link"
    );
    // Control characters other than newlines and tabs are removed.
    assert_eq!(message_text("a\u{7}b\u{0}c\r\n d", 10).unwrap().0, "abc\nd");
    assert!(message_text(&html_text("<p> </p><br>"), 10).is_none());
    let (cut, was_cut) = message_text(&"é".repeat(5000), MAX_MESSAGE_TEXT).unwrap();
    assert!(was_cut);
    assert_eq!(cut.chars().count(), MAX_MESSAGE_TEXT);
}

#[test]
fn attachment_urls_pass_only_as_plain_sharepoint_references() {
    assert_eq!(
        attachment_url("https://contoso.sharepoint.com/sites/P/Shared%20Documents/calc.pdf")
            .as_deref(),
        Some("https://contoso.sharepoint.com/sites/P/Shared%20Documents/calc.pdf")
    );
    assert!(
        attachment_url("https://CONTOSO-my.SharePoint.com/personal/a/Documents/x.pdf").is_some()
    );
    for bad in [
        "https://contoso.sharepoint.com/sites/x/_layouts/15/download.aspx?UniqueId=x&tempauth=SENTINEL",
        "https://contoso.sharepoint.com/sites/x/a.pdf#SENTINEL",
        "https://contoso.sharepoint.com/sites/x/a.pdf?",
        "https://graph.microsoft.com/v1.0/chats/x/messages/1/hostedContents/a/$value",
        "https://contoso.sharepoint.com/sites/x/hostedContents/a",
        "https://contoso.sharepoint.com/sites/x/HOSTEDCONTENTS/a",
        "https://contoso.sharepoint.com/sites/x/hosted%43ontents/a",
        "https://contoso.sharepoint.com/sites/x/%2568ostedContents/a",
        "https://user:SENTINEL@contoso.sharepoint.com/a.pdf",
        "https://contoso.sharepoint.com:8443/a.pdf",
        "http://contoso.sharepoint.com/a.pdf",
        "https://evil.com/x.sharepoint.com/a.pdf",
        "https://contoso.sharepoint.com.evil.com/a.pdf",
        "https://a.b.sharepoint.com/a.pdf",
        "https://sharepoint.com/a.pdf",
        "https://contoso.sharepoint.com/a%zz.pdf",
        "javascript:alert(1)",
        "",
    ] {
        assert!(attachment_url(bad).is_none(), "{bad}");
    }
}

// ── message projection ───────────────────────────────────────────────────────

#[test]
fn a_message_keeps_text_sender_and_references_never_payloads() {
    let body = json!({"value": [{
        "id": "1",
        "createdDateTime": "2026-11-02T11:00:00Z",
        "lastModifiedDateTime": "2026-11-02T11:00:00Z",
        "messageType": "message",
        "etag": "must-not-leave",
        "from": {"user": {"id": "u-1", "displayName": "Anna", "userIdentityType": "aadUser"}},
        "body": {"contentType": "html", "content": "<p>Beam <b>B12</b></p>"},
        "attachments": [
            {"id": "a1", "name": "calc.pdf", "contentType": "reference",
             "contentUrl": "https://contoso.sharepoint.com/sites/P/calc.pdf"},
            {"id": "a2", "contentType": "reference",
             "contentUrl": "https://contoso.sharepoint.com/x/download.aspx?tempauth=must-not-leave"},
            {"id": "a3", "contentType": "application/vnd.microsoft.card.adaptive",
             "content": "{\"body\":\"must-not-leave\"}"},
            {"id": "a4", "contentType": "forwardedMessageReference",
             "content": "{\"originalMessage\":\"must-not-leave\"}"},
            {"id": "a5", "contentType": "something/new", "content": {"x": "must-not-leave"}},
            {"id": "", "contentType": "reference"}
        ],
        "hostedContents": [{"id": "must-not-leave"}]
    }]});
    let out = project_teams_messages(&body, ctx(20, None), false).unwrap();
    assert_ms_clean(&out.to_string());
    let m = &out["messages"][0];
    assert_eq!(m["text"], json!("Beam B12"));
    assert_eq!(
        m["sender"],
        json!({"kind": "user", "id": "u-1", "display-name": "Anna"})
    );
    assert!(m.get("sender-withheld").is_none());
    let atts = m["attachments"].as_array().unwrap();
    assert_eq!(atts.len(), 5);
    assert_eq!(
        atts[0]["url"],
        json!("https://contoso.sharepoint.com/sites/P/calc.pdf")
    );
    assert_eq!(atts[1]["url-withheld"], json!(true));
    assert!(atts[1].get("url").is_none());
    for (i, att) in atts.iter().enumerate().skip(2) {
        assert_eq!(att["content-withheld"], json!(true), "{i}");
    }
    assert!(atts[0].get("content-withheld").is_none());
    assert_eq!(m["attachments-omitted"], json!(1));
    assert!(out.get("filtered-before-since").is_none());
}

#[test]
fn senders_deleted_and_system_messages_are_labelled_never_dropped() {
    let body = json!({"value": [
        {"id": "1", "createdDateTime": "2026-11-02T11:00:00Z", "messageType": "systemEventMessage",
         "from": null, "body": {"contentType": "html", "content": "<systemEventMessage/>"},
         "eventDetail": {"@odata.type": "#microsoft.graph.chatRenamedEventMessageDetail"}},
        {"id": "2", "createdDateTime": "2026-11-02T11:00:00Z", "messageType": "message",
         "deletedDateTime": "2026-11-02T12:00:00Z",
         "from": {"application": {"id": "app-1", "displayName": "Bot"}},
         "body": {"contentType": "text", "content": "must-not-leave"}},
        {"id": "3", "createdDateTime": "2026-11-02T11:00:00Z", "messageType": "message",
         "from": {"user": {"id": "", "displayName": "must-not-leave"}},
         "body": {"contentType": "text", "content": "hi"}},
        {"id": "", "createdDateTime": "2026-11-02T11:00:00Z"}
    ]});
    let out = project_teams_messages(&body, ctx(20, None), false).unwrap();
    assert_ms_clean(&out.to_string());
    let m = out["messages"].as_array().unwrap();
    assert_eq!(m.len(), 3);
    assert_eq!(out["omitted"], json!(1));
    assert_eq!(m[0]["message-type"], json!("systemEventMessage"));
    assert_eq!(m[0]["sender"], json!({"kind": "unknown"}));
    assert!(m[0].get("sender-withheld").is_none());
    assert_eq!(m[1]["deleted"], json!(true));
    assert_eq!(m[1]["text"], json!(""));
    assert_eq!(
        m[1]["sender"],
        json!({"kind": "application", "id": "app-1", "display-name": "Bot"})
    );
    assert_eq!(m[2]["sender"], json!({"kind": "unknown"}));
    assert_eq!(m[2]["sender-withheld"], json!(true));
}

#[test]
fn a_channel_thread_counts_by_its_whole_activity_not_its_root() {
    let old = "2026-11-01T09:00:00Z";
    let new = "2026-11-02T11:00:00Z";
    let mut many: Vec<Json> = (0..60).map(|i| msg(&format!("r{i}"), old)).collect();
    many.push(msg("r-new", new)); // beyond the 50-reply cap
    let mut unseen = thread("unseen", old, vec![msg("u1", old)]);
    unseen["replies@odata.nextLink"] = json!("https://graph.microsoft.com/next");
    let mut bad_time = thread("bad-time", old, vec![]);
    bad_time["lastModifiedDateTime"] = json!("not a time");
    let body = json!({"value": [
        thread("old-root-new-reply", old, vec![msg("reply", new)]),
        thread("old-root-capped-reply", old, many),
        unseen,
        bad_time,
        thread("new", new, vec![]),
        thread("old", old, vec![msg("old-reply", old)]),
    ]});
    let out = project_teams_messages(&body, ctx(20, Some(SINCE)), true).unwrap();
    assert_eq!(
        ids(&out),
        vec![
            "old-root-new-reply",
            "old-root-capped-reply",
            "unseen",
            "bad-time",
            "new"
        ]
    );
    assert_eq!(out["filtered-before-since"], json!(1));
    assert_eq!(out["more-available"], json!(false));
    let m = out["messages"].as_array().unwrap();
    assert_eq!(m[0]["replies"][0]["id"], json!("reply"));
    assert!(m[0].get("activity-unknown").is_none());
    // Activity uses every reply received, not only those kept.
    assert_eq!(m[1]["replies"].as_array().unwrap().len(), MAX_REPLIES);
    assert_eq!(m[1]["replies-omitted"], json!(11));
    assert_eq!(m[1]["replies-more-available"], json!(true));
    assert!(m[1].get("activity-unknown").is_none());
    assert_eq!(m[2]["activity-unknown"], json!(true));
    assert_eq!(m[2]["replies-more-available"], json!(true));
    assert_eq!(m[2]["replies-omitted"], json!(0));
    assert_eq!(m[3]["activity-unknown"], json!(true));
    // A later `since` filters more: the same page, a different answer.
    let later = project_teams_messages(&body, ctx(20, Some("2026-11-03T00:00:00Z")), true).unwrap();
    assert_eq!(ids(&later), vec!["unseen", "bad-time"]);
    assert_eq!(later["filtered-before-since"], json!(4));
}

#[test]
fn nested_reply_continuation_is_reported_separately_from_the_page() {
    let new = "2026-11-02T11:00:00Z";
    let mut root = thread("t", new, vec![msg("a", new), msg("b", new)]);
    root["replies@odata.nextLink"] = json!("https://graph.microsoft.com/v1.0/must-not-leave");
    let out =
        project_teams_messages(&json!({"value": [root]}), ctx(20, Some(SINCE)), true).unwrap();
    assert_ms_clean(&out.to_string());
    let m = &out["messages"][0];
    assert_eq!(m["replies-more-available"], json!(true));
    assert_eq!(m["replies-omitted"], json!(0));
    assert_eq!(out["more-available"], json!(false));
}

#[test]
fn more_available_means_a_continuation_or_a_real_cut_never_a_full_page() {
    let new = "2026-11-02T11:00:00Z";
    let old = "2026-11-01T09:00:00Z";
    let page = |n: usize, at: &str| -> Json {
        json!({"value": (0..n).map(|i| thread(&format!("m{i}"), at, vec![])).collect::<Vec<_>>()})
    };
    let exact = project_teams_messages(&page(3, new), ctx(3, Some(SINCE)), true).unwrap();
    assert_eq!(ids(&exact).len(), 3);
    assert_eq!(exact["more-available"], json!(false));
    let over = project_teams_messages(&page(4, new), ctx(3, Some(SINCE)), true).unwrap();
    assert_eq!(ids(&over).len(), 3);
    assert_eq!(over["more-available"], json!(true));
    let all_old = project_teams_messages(&page(3, old), ctx(3, Some(SINCE)), true).unwrap();
    assert!(ids(&all_old).is_empty());
    assert_eq!(all_old["more-available"], json!(false));
    assert_eq!(all_old["filtered-before-since"], json!(3));
    let mut linked = page(1, new);
    linked["@odata.nextLink"] = json!("https://graph.microsoft.com/v1.0/must-not-leave");
    let linked = project_teams_messages(&linked, ctx(3, None), false).unwrap();
    assert_eq!(linked["more-available"], json!(true));
    assert_ms_clean(&linked.to_string());
}

#[test]
fn a_reply_with_a_rejected_sender_is_kept_and_labelled() {
    let new = "2026-11-02T11:00:00Z";
    let mut reply = msg("r", new);
    reply["from"] = json!({"user": {"id": 42}});
    let out = project_teams_messages(
        &json!({"value": [thread("t", new, vec![reply])]}),
        ctx(20, Some(SINCE)),
        true,
    )
    .unwrap();
    let r = &out["messages"][0]["replies"][0];
    assert_eq!(r["sender"], json!({"kind": "unknown"}));
    assert_eq!(r["sender-withheld"], json!(true));
    assert!(r.get("replies").is_none());
}

#[test]
fn teams_channels_and_chats_keep_only_valid_ids() {
    let teams = project_collection(
        &json!({"value": [
            {"id": TEAM, "displayName": "Acme", "description": "d", "internalId": "must-not-leave"},
            {"id": "not-a-guid", "displayName": "x"}
        ]}),
        10,
        "teams",
        team_entry,
    )
    .unwrap();
    assert_ms_clean(&teams.to_string());
    assert_eq!(
        teams["teams"],
        json!([{"id": TEAM, "name": "Acme", "description": "d"}])
    );
    assert_eq!(teams["omitted"], json!(1));
    let channels = project_collection(
        &json!({"value": [
            {"id": CHANNEL, "displayName": "Steel", "membershipType": "standard",
             "createdDateTime": "2026-09-01T08:00:00Z", "email": "must-not-leave"},
            {"id": "General", "displayName": "x"}
        ]}),
        10,
        "channels",
        channel_entry,
    )
    .unwrap();
    assert_ms_clean(&channels.to_string());
    assert_eq!(
        channels["channels"][0]["membership-type"],
        json!("standard")
    );
    assert_eq!(channels["omitted"], json!(1));
    let chats = project_collection(
        &json!({"value": [
            {"id": ONE_ON_ONE, "topic": null, "chatType": "oneOnOne",
             "createdDateTime": "2026-10-01T08:00:00Z", "lastUpdatedDateTime": "2026-11-02T10:00:00Z",
             "tenantId": "must-not-leave"},
            {"id": "x", "chatType": "group"}
        ]}),
        10,
        "chats",
        chat_entry,
    )
    .unwrap();
    assert_ms_clean(&chats.to_string());
    assert_eq!(chats["chats"][0]["chat-type"], json!("oneOnOne"));
    assert!(chats["chats"][0].get("topic").is_none());
    assert_eq!(chats["omitted"], json!(1));
}

// ── the call path ────────────────────────────────────────────────────────────

#[tokio::test]
async fn each_teams_read_runs_through_the_bound_call() {
    let next = "https://evil.example/v1.0/next";
    let channel_path = format!("/v1.0/teams/{TEAM}/channels/19:a1b2c3d4e5f6@thread.tacv2/messages");
    let chat_path =
        "/v1.0/chats/19:2da4c29f6d7041eca70b638b43d45437@thread.v2/messages".to_string();
    let channels_path = format!("/v1.0/teams/{TEAM}/channels");
    let cases: Vec<(&str, Json, Json, String)> = vec![
        (
            "teams.team.list",
            json!({"page-size": 1}),
            json!({"value": [{"id": TEAM}, {"id": "ebf3b108-5234-4e22-b93d-656d7dae5875"}]}),
            "/v1.0/me/joinedTeams".into(),
        ),
        (
            "teams.channel.list",
            json!({"team-id": TEAM}),
            json!({"value": [{"id": CHANNEL, "displayName": "Steel"}]}),
            channels_path,
        ),
        (
            "teams.channel.list-messages",
            json!({"team-id": TEAM, "channel-id": CHANNEL, "since": SINCE}),
            json!({"value": [thread("t1", "2026-11-02T11:00:00Z", vec![])], "@odata.nextLink": next}),
            channel_path,
        ),
        (
            "teams.chat.list",
            json!({}),
            json!({"value": [{"id": CHAT, "chatType": "group", "topic": "Steel"}]}),
            "/v1.0/me/chats".into(),
        ),
        (
            "teams.chat.list-messages",
            json!({"chat-id": CHAT, "since": SINCE}),
            json!({"value": [msg("c1", "2026-11-02T11:00:00Z")]}),
            chat_path,
        ),
    ];
    for (command, inputs, data, path) in cases {
        let (out, f) = call_read(command, inputs.clone(), data).await;
        let out = out.unwrap_or_else(|e| panic!("{command}: {e:?}"));
        assert_ms_clean(&out.to_string());
        let body = &out["result"]["body"];
        let heads = data_heads(&f);
        assert_eq!(
            heads.len(),
            1,
            "{command}: one data request, nothing followed"
        );
        assert_eq!(path_only(&heads[0]), path, "{command}");
        // The Prefer header goes on the message reads' data request, never on
        // the identity read.
        assert_eq!(
            has_prefer_header(&heads[0]),
            command.ends_with("list-messages"),
            "{command}"
        );
        for identity in f.requests().iter().filter(|h| is_identity(h)) {
            assert!(!has_prefer_header(identity), "{command}");
        }
        match command {
            "teams.team.list" => {
                assert!(query_of(&heads[0]).is_empty());
                assert_eq!(body["teams"].as_array().unwrap().len(), 1);
                assert_eq!(body["more-available"], json!(true));
            }
            "teams.channel.list" => assert_eq!(body["channels"][0]["name"], json!("Steel")),
            "teams.channel.list-messages" => {
                assert_eq!(ids(body), vec!["t1"]);
                assert_eq!(body["more-available"], json!(true));
                assert!(query_of(&heads[0]).iter().all(|(k, _)| k != "$filter"));
            }
            "teams.chat.list" => assert_eq!(body["chats"][0]["topic"], json!("Steel")),
            "teams.chat.list-messages" => {
                assert_eq!(ids(body), vec!["c1"]);
                assert!(query_of(&heads[0]).contains(&(
                    "$filter".to_string(),
                    "lastModifiedDateTime gt 2026-11-02T10:00:00Z".to_string()
                )));
            }
            _ => unreachable!(),
        }
    }
}

#[tokio::test]
async fn only_since_changes_what_the_channel_read_returns() {
    let data = json!({"value": [
        thread("a", "2026-11-02T09:00:00Z", vec![msg("a-reply", "2026-11-02T12:00:00Z")]),
        thread("c", "2026-11-02T09:30:00Z", vec![]),
        thread("b", "2026-11-02T12:30:00Z", vec![]),
    ]});
    let read_since = |since: &'static str| {
        let data = data.clone();
        async move {
            let (out, _f) = call_read(
                "teams.channel.list-messages",
                json!({"team-id": TEAM, "channel-id": CHANNEL, "since": since}),
                data,
            )
            .await;
            out.unwrap()["result"]["body"].clone()
        }
    };
    let early = read_since("2026-11-02T08:00:00Z").await;
    assert_eq!(ids(&early), vec!["a", "c", "b"]);
    assert_eq!(early["filtered-before-since"], json!(0));
    let cut = read_since("2026-11-02T10:00:00Z").await;
    assert_eq!(ids(&cut), vec!["a", "b"]);
    assert_eq!(cut["filtered-before-since"], json!(1));
}

#[tokio::test]
async fn without_a_teams_scope_nothing_is_sent_and_admin_consent_is_said() {
    for (command, inputs, scope, hint, admin) in [
        (
            "teams.channel.list-messages",
            json!({"team-id": TEAM, "channel-id": CHANNEL, "since": SINCE}),
            "offline_access User.Read Channel.ReadBasic.All Chat.Read",
            "--scopes User.Read,ChannelMessage.Read.All",
            true,
        ),
        (
            "teams.chat.list-messages",
            json!({"chat-id": CHAT, "since": SINCE}),
            "offline_access User.Read Chat.ReadBasic",
            "--scopes User.Read,Chat.Read",
            false,
        ),
        (
            "teams.team.list",
            json!({}),
            "offline_access User.Read Chat.Read",
            "--scopes User.Read,Team.ReadBasic.All",
            false,
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
            cap["permissions"]["adminConsent"],
            json!(admin),
            "{command}"
        );
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
async fn a_workflow_node_reads_chat_messages_with_the_rendered_filter() {
    let h = ms_home();
    ms_store(h.path(), None, "offline_access Chat.Read");
    let f = graph(
        me(OID, UPN),
        Reply::json(
            200,
            &json!({"value": [msg("m1", "2026-11-02T11:00:00Z"), msg("m2", "2026-11-02T10:30:00Z")]}),
        ),
    );
    let manifest: Agent = serde_yaml::from_str(&ms_runnable_manifest()).unwrap();
    let out = run_for_workflow_with(
        h.path(),
        &manifest,
        "teams.chat.list-messages",
        &json!({"chat-id": CHAT, "since": SINCE, "page-size": 1}),
        Some(&ms_endpoints(&f)),
        Duration::from_secs(10),
        1 << 20,
    )
    .await
    .unwrap();
    assert_eq!(ids(&out), vec!["m1"]);
    assert_eq!(out["more-available"], json!(true));
    let heads = data_heads(&f);
    assert_eq!(heads.len(), 1);
    assert!(has_prefer_header(&heads[0]));
    assert!(query_of(&heads[0]).contains(&("$top".to_string(), "1".to_string())));
    assert!(
        f.requests()
            .iter()
            .all(|h| !path_of(h).starts_with("/v1.0/me?"))
    );
}
