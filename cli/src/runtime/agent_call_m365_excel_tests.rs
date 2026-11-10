//! Microsoft 365 Excel reads and opening a file by its sharing link (#709
//! slice B2). A child of the `list-folder` tests, so it shares their Graph
//! fixture and helpers.

use super::*;

/// Every row that shipped before slice B2, with the operation digest it
/// shipped with. A change here changes what an approved `.lock` pinned, so B2
/// must leave each one byte for byte.
const SHIPPED_DIGESTS: &[(&str, &str, &str)] = &[
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
];

#[test]
fn rows_shipped_before_b2_keep_their_digests() {
    let actual: Vec<(String, String, String)> = OPERATIONS
        .iter()
        .filter(|op| !op.command.starts_with("excel.") && op.command != "get-shared-item")
        // Rows added after B2 have their own pins (#763).
        .filter(|op| !super::outlook::OUTLOOK_763.contains(&op.command))
        // The Trimble Connect rows (#788) pin their own digests.
        .filter(|op| op.agent != "trimble-connect")
        // `gmail.search` pins its own digest (agent_call_gmail_tests.rs).
        .filter(|op| op.command != "gmail.search")
        .map(|op| (op.agent.to_string(), op.command.to_string(), op.sha256()))
        .collect();
    let pinned: Vec<(String, String, String)> = SHIPPED_DIGESTS
        .iter()
        .map(|(a, c, d)| (a.to_string(), c.to_string(), d.to_string()))
        .collect();
    assert_eq!(actual, pinned);
}

const ITEM: &str = "01ABCDEF";
const SHEET_ID: &str = "{FC034FA8-F8CC-4D24-9C0A-02A81B7792A0}";
const SHARE: &str = "https://contoso-my.sharepoint.com/:x:/g/personal/ada_contoso_com/EaB~1?e=x";
/// `u!` + unpadded base64url of `SHARE`, computed outside AWARE.
const SHARE_TOKEN: &str = "u!aHR0cHM6Ly9jb250b3NvLW15LnNoYXJlcG9pbnQuY29tLzp4Oi9nL3BlcnNvbmFsL2FkYV9jb250b3NvX2NvbS9FYUJ-MT9lPXg";
const FILES_SCOPES: &str = "openid profile offline_access User.Read Files.ReadWrite";
const B2: [&str; 3] = [
    "excel.worksheet.list",
    "excel.range.read",
    "get-shared-item",
];

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

fn is_identity(head: &str) -> bool {
    is_identity_path(&path_of(head))
}

fn data_heads(f: &Fixture) -> Vec<String> {
    f.requests()
        .into_iter()
        .filter(|h| !is_identity(h))
        .collect()
}

fn ask(range: &str) -> RangeAsk {
    a1_range(range).unwrap()
}

fn project_range(range: &str, body: &Json) -> Option<Json> {
    Projection::ExcelRange.apply(
        body,
        ProjectionContext {
            skip: 0,
            limit: 100,
            since: None,
            range: Some(ask(range)),
        },
    )
}

/// A well-formed Graph range answer for `address` (`Sheet!A1:B2` form), every
/// cell a string.
fn range_answer(address: &str, rows: usize, columns: usize) -> Json {
    let cell = |r: usize, c: usize| json!(format!("r{r}c{c}"));
    let grid = |f: &dyn Fn(usize, usize) -> Json| {
        Json::Array(
            (0..rows)
                .map(|r| Json::Array((0..columns).map(|c| f(r, c)).collect()))
                .collect(),
        )
    };
    json!({
        "address": address,
        "rowCount": rows,
        "columnCount": columns,
        "values": grid(&cell),
        "text": grid(&cell),
        "valueTypes": grid(&|_, _| json!("String")),
    })
}

async fn call_read(
    command: &str,
    inputs: Json,
    data: Json,
) -> (Result<Json, CallFailure>, Fixture) {
    let h = ms_home();
    ms_store(h.path(), None, FILES_SCOPES);
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
fn worksheet_refs_are_a_braced_id_or_a_name_excel_accepts() {
    let kind = IdKind::WorksheetRef;
    for good in [
        SHEET_ID,
        "{fc034fa8-f8cc-4d24-9c0a-02a81b7792a0}",
        "Sheet1",
        "Load Table",
        "Loads!2026",
        "Ü-Träger",
        "a",
        &"x".repeat(31),
    ] {
        assert!(kind.accepts(good), "{good}");
    }
    for bad in [
        "",
        " Sheet1",
        "Sheet1 ",
        ".",
        "..",
        "Load/Table",
        "Load\\Table",
        "Load?",
        "Load*",
        "[Load]",
        "Load:Table",
        "O'Brien",
        "Load\tTable",
        "Load\nTable",
        "{FC034FA8-F8CC-4D24-9C0A-02A81B7792A}",
        "{FC034FA8F8CC4D249C0A02A81B7792A0}",
        "{not-a-guid}",
        &"x".repeat(32),
    ] {
        assert!(!kind.accepts(bad), "{bad:?}");
    }
}

#[test]
fn a1_ranges_have_one_spelling_and_a_cell_bound() {
    for (given, sent) in [
        ("A1", "A1"),
        ("a1", "A1"),
        ("a1:a1", "A1"),
        ("XFD1048576:XFD1048576", "XFD1048576"),
        ("A1:D5", "A1:D5"),
        ("d5:a1", "A1:D5"),
        ("D1:A5", "A1:D5"),
        ("aa10:ab12", "AA10:AB12"),
        ("A1:CV50", "A1:CV50"),
    ] {
        assert_eq!(ask(given).address(), sent, "{given}");
    }
    assert_eq!(ask("A1:CV50").cells(), 5_000);
    let d = ask("B2:D5");
    assert_eq!((d.rows(), d.columns()), (4, 3));
    for bad in [
        "",
        "A1:CV51",
        "A1:XFD1",
        "$A$1",
        "A$1",
        "Sheet1!A1",
        "A:A",
        "1:1",
        "A",
        "1",
        "A0",
        "A01",
        "XFE1",
        "A1048577",
        "AAAA1",
        "A1:B2,C3",
        "A1 B2",
        " A1",
        "A1 ",
        "A1:",
        ":A1",
        "A1:B2:C3",
        "A-1",
        "Ａ1",
    ] {
        assert!(a1_range(bad).is_none(), "{bad:?}");
        assert!(!IdKind::ExcelA1Range.accepts(bad), "{bad:?}");
    }
    // Graph's own answers carry `$`; only the answer parser takes it.
    assert_eq!(a1_rect("$A$1:$D$5", true), Some(ask("A1:D5")));
    assert_eq!(a1_rect("$A$1:$D$5", false), None);
}

#[test]
fn sharing_urls_are_only_work_or_school_https_links() {
    let kind = IdKind::SharingUrl;
    for good in [
        SHARE,
        "https://contoso.sharepoint.com/:x:/s/Steel/EbCdEf?e=AbC123",
        "https://Contoso.SharePoint.com/:f:/r/sites/Steel",
        "https://a-b.sharepoint.com/x",
    ] {
        assert!(kind.accepts(good), "{good}");
    }
    let long = format!("https://contoso.sharepoint.com/{}", "a".repeat(2048));
    for bad in [
        "",
        "http://contoso.sharepoint.com/x",
        "https://1drv.ms/x/s!AbC",
        "https://onedrive.live.com/redir?resid=1",
        "https://contoso.sharepoint.com",
        "https://contoso.sharepoint.com?e=1",
        "https://contoso.sharepoint.com.evil.example/x",
        "https://evil.example/contoso.sharepoint.com/x",
        "https://ada@contoso.sharepoint.com/x",
        "https://contoso.sharepoint.com:443/x",
        "https://sharepoint.com/x",
        "https://.sharepoint.com/x",
        "https://-contoso.sharepoint.com/x",
        "https://contoso-.sharepoint.com/x",
        "https://a.b.sharepoint.com/x",
        "https://contoso.sharepoint.com/x#frag",
        "https://contoso.sharepoint.com/a b",
        "https://contoso.sharepoint.com/é",
        "https://contoso.sharepoint.com/a\"b",
        "https://contoso.sharepoint.com/a{b}",
        "https://contoso.sharepoint.com/a\nb",
        &long,
    ] {
        assert!(!kind.accepts(bad), "{bad:?}");
    }
}

// ── the request ──────────────────────────────────────────────────────────────

#[test]
fn each_b2_read_builds_exactly_its_documented_request() {
    let endpoints = Endpoints {
        token: TokenPin::MicrosoftLogin,
        identity_url: MICROSOFT_ME_URL.into(),
        data_origin: None,
        allowed_origins: vec![MICROSOFT_GRAPH_ORIGIN.into()],
        topics_origins: Vec::new(),
    };
    let drive = format!("https://graph.microsoft.com/v1.0/drives/{DRIVE}/items/{ITEM}/workbook");
    let mine = format!("https://graph.microsoft.com/v1.0/me/drive/items/{ITEM}/workbook");
    let cases: Vec<(&str, Json, String, &str)> = vec![
        (
            "excel.worksheet.list",
            json!({"drive-id": DRIVE, "item-id": ITEM}),
            format!("{drive}/worksheets"),
            WORKSHEETS_SELECT,
        ),
        (
            "excel.worksheet.list",
            json!({"drive-id": "me", "item-id": ITEM, "page-size": 5}),
            format!("{mine}/worksheets"),
            WORKSHEETS_SELECT,
        ),
        (
            "excel.range.read",
            json!({"drive-id": DRIVE, "item-id": ITEM, "worksheet": SHEET_ID, "range": "d5:a1"}),
            format!(
                "{drive}/worksheets/%7BFC034FA8-F8CC-4D24-9C0A-02A81B7792A0%7D/range(address='A1:D5')"
            ),
            RANGE_SELECT,
        ),
        (
            "excel.range.read",
            json!({"drive-id": "me", "item-id": ITEM, "worksheet": "Load Table", "range": "b7:b7"}),
            format!("{mine}/worksheets/Load%20Table/range(address='B7')"),
            RANGE_SELECT,
        ),
        (
            "get-shared-item",
            json!({"sharing-url": SHARE}),
            format!("https://graph.microsoft.com/v1.0/shares/{SHARE_TOKEN}/driveItem"),
            GET_ITEM_SELECT,
        ),
    ];
    for (command, inputs, url, select) in cases {
        let op = read(command);
        let wire = wire(op, &map(inputs.clone())).unwrap();
        assert_eq!(
            endpoints.data_url(op, &wire).unwrap(),
            url,
            "{command} {inputs}"
        );
        assert_eq!(
            wire.query,
            vec![("$select".to_string(), select.to_string())],
            "{command} {inputs}: page bounds are local, never sent"
        );
        assert!(
            op.fixed_headers.is_empty(),
            "{command}: no Prefer, no session"
        );
    }
    let range = wire(
        read("excel.range.read"),
        &map(json!({"drive-id": DRIVE, "item-id": ITEM, "worksheet": "S", "range": "c3:a1"})),
    )
    .unwrap();
    assert_eq!(range.range, Some(ask("A1:C3")));
    let sheets = wire(
        read("excel.worksheet.list"),
        &map(json!({"drive-id": DRIVE, "item-id": ITEM, "page-size": 5})),
    )
    .unwrap();
    assert_eq!((sheets.limit, sheets.range), (5, None));
}

#[test]
fn b2_inputs_outside_their_grammar_or_bounds_are_refused() {
    let range = |worksheet: &str, range: &str| json!({"drive-id": DRIVE, "item-id": ITEM, "worksheet": worksheet, "range": range});
    let refused: Vec<(&str, Json, &str)> = vec![
        (
            "excel.worksheet.list",
            json!({"drive-id": DRIVE}),
            "item-id",
        ),
        (
            "excel.worksheet.list",
            json!({"drive-id": "../me", "item-id": ITEM}),
            "drive-id",
        ),
        (
            "excel.worksheet.list",
            json!({"drive-id": DRIVE, "item-id": ITEM, "page-size": 201}),
            "page-size",
        ),
        ("excel.range.read", range("Sheet1", "A1:CV51"), "range"),
        ("excel.range.read", range("Sheet1", "Sheet1!A1"), "range"),
        ("excel.range.read", range("Sheet1", "A:A"), "range"),
        ("excel.range.read", range("Sheet1", "$A$1"), "range"),
        ("excel.range.read", range("Sheet1", "A1')/delete"), "range"),
        (
            "excel.range.read",
            range("Sheet1", &"A".repeat(33)),
            "range",
        ),
        ("excel.range.read", range("../../root", "A1"), "worksheet"),
        ("excel.range.read", range("O'Brien", "A1"), "worksheet"),
        ("excel.range.read", range("{x}", "A1"), "worksheet"),
        (
            "excel.range.read",
            json!({"drive-id": DRIVE, "item-id": ITEM, "worksheet": "S"}),
            "range",
        ),
        (
            "excel.range.read",
            json!({"drive-id": DRIVE, "item-id": ITEM, "worksheet": "S", "range": "A1", "session": "x"}),
            "",
        ),
        ("get-shared-item", json!({}), "sharing-url"),
        (
            "get-shared-item",
            json!({"sharing-url": "https://evil.example/x"}),
            "sharing-url",
        ),
        (
            "get-shared-item",
            json!({"sharing-url": format!("https://contoso.sharepoint.com/{}", "a".repeat(2030))}),
            "sharing-url",
        ),
        (
            "get-shared-item",
            json!({"sharing-url": SHARE, "redeem": true}),
            "",
        ),
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
async fn a_refused_b2_input_sends_no_request_at_all() {
    for (command, inputs) in [
        (
            "excel.range.read",
            json!({"drive-id": DRIVE, "item-id": ITEM, "worksheet": "S", "range": "A1:Z1000"}),
        ),
        (
            "excel.range.read",
            json!({"drive-id": DRIVE, "item-id": ITEM, "worksheet": "a/b", "range": "A1"}),
        ),
        (
            "get-shared-item",
            json!({"sharing-url": "https://1drv.ms/x/s!AbC"}),
        ),
    ] {
        let (out, f) = call_read(command, inputs.clone(), json!({"value": []})).await;
        assert_eq!(out.unwrap_err().code, "E_CALL_INPUT_INVALID", "{command}");
        assert!(f.requests().is_empty(), "{command}: {:?}", f.requests());
    }
}

// ── digests ──────────────────────────────────────────────────────────────────

#[test]
fn b2_digests_bind_their_renderers_grammars_bounds_and_scopes() {
    let range = canonical_string(&read("excel.range.read").canonical());
    for needle in [
        "\"canonical\":\"v2\"",
        "{\"input\":\"range\",\"render\":\"excel-range/1\"}",
        "\"pattern\":\"excel-a1-range/1\"",
        "\"maxCells\":5000",
        "\"pattern\":\"worksheet-ref\"",
        "\"projection\":\"excel-range/1\"",
        "\"Files.ReadWrite\"",
    ] {
        assert!(range.contains(needle), "{needle} not in {range}");
    }
    let share = canonical_string(&read("get-shared-item").canonical());
    for needle in [
        "{\"input\":\"sharing-url\",\"render\":\"share-token/1\"}",
        "\"pattern\":\"sharing-url/1\"",
        "\"Sites.ReadWrite.All\"",
        "\"projection\":\"graph-item/1\"",
    ] {
        assert!(share.contains(needle), "{needle} not in {share}");
    }
    let sheets = canonical_string(&read("excel.worksheet.list").canonical());
    assert!(
        sheets.contains("\"projection\":\"excel-worksheets/1\""),
        "{sheets}"
    );
    for command in B2 {
        let canonical = canonical_string(&read(command).canonical());
        assert!(!canonical.contains("\"headers\""), "{command}");
        assert!(!canonical.contains("\"maxCells\"") || command == "excel.range.read");
    }

    let base = read("excel.range.read");
    // Rendering the range as a plain segment moves the digest.
    let mut plain = base.clone();
    let mut segments = base.route.segments.to_vec();
    *segments.last_mut().unwrap() = Seg::Input("range");
    plain.route.segments = Box::leak(segments.into_boxed_slice());
    assert_ne!(plain.sha256(), base.sha256());
    // So does another grammar for the range, or a wider scope set.
    let mut loose = base.clone();
    let mut inputs = base.inputs.to_vec();
    inputs[3].id_kind = Some(IdKind::GraphOpaqueId);
    loose.inputs = Box::leak(inputs.into_boxed_slice());
    assert_ne!(loose.sha256(), base.sha256());
    let mut longer = base.clone();
    let mut inputs = base.inputs.to_vec();
    inputs[3].max_bytes = Some(33);
    longer.inputs = Box::leak(inputs.into_boxed_slice());
    assert_ne!(longer.sha256(), base.sha256());
    let mut wider = base.clone();
    wider.request_scopes = &["User.Read", "Files.ReadWrite.All"];
    assert_ne!(wider.sha256(), base.sha256());
    let mut other = base.clone();
    other.projection = Projection::ExcelWorksheets;
    assert_ne!(other.sha256(), base.sha256());

    let base = read("get-shared-item");
    let mut raw = base.clone();
    let mut segments = base.route.segments.to_vec();
    segments[2] = Seg::Input("sharing-url");
    raw.route.segments = Box::leak(segments.into_boxed_slice());
    assert_ne!(raw.sha256(), base.sha256());
    let mut narrower = base.clone();
    narrower.any_of_scopes = MICROSOFT_WORKBOOK_SCOPES;
    assert_ne!(narrower.sha256(), base.sha256());

    let mut digests: Vec<String> = OPERATIONS.iter().map(Operation::sha256).collect();
    digests.sort();
    digests.dedup();
    assert_eq!(digests.len(), OPERATIONS.len());
}

// ── the range projection ─────────────────────────────────────────────────────

#[test]
fn a_range_answer_passes_on_values_text_and_types_exactly() {
    let body = json!({
        "address": "'Load Table'!A1:C2",
        "rowCount": 2,
        "columnCount": 3,
        "values": [["Mark", "Load", "Checked"], ["B1\r\nrev A", 125.5, true]],
        "text": [["Mark", "Load", "Checked"], ["B1\r\nrev A", "125.50", "TRUE"]],
        "valueTypes": [["String", "String", "String"], ["String", "Double", "Boolean"]],
        "@odata.context": "must-not-leave",
        "formulas": [["must-not-leave"]],
    });
    let out = project_range("a1:c2", &body).unwrap();
    assert_eq!(
        out,
        json!({
            "address": "'Load Table'!A1:C2",
            "row-count": 2,
            "column-count": 3,
            "values": [["Mark", "Load", "Checked"], ["B1\r\nrev A", 125.5, true]],
            "text": [["Mark", "Load", "Checked"], ["B1\r\nrev A", "125.50", "TRUE"]],
            "value-types": [["String", "String", "String"], ["String", "Double", "Boolean"]],
            "truncated-cells": [],
            "truncated-cells-omitted": 0,
        })
    );
    assert_ms_clean(&out.to_string());
    // A date is Excel's serial number; an empty cell and an error keep their
    // own types; whitespace and control characters are never rewritten.
    let body = json!({
        "address": "Sheet1!A1:D1",
        "rowCount": 1,
        "columnCount": 4,
        "values": [[46021, "", "#DIV/0!", "  \u{1}x\t"]],
        "text": [["2025-12-29", "", "#DIV/0!", "  \u{1}x\t"]],
        "valueTypes": [["Integer", "Empty", "Error", "String"]],
    });
    let out = project_range("A1:D1", &body).unwrap();
    assert_eq!(out["values"], body["values"]);
    assert_eq!(out["text"], body["text"]);
}

#[test]
fn the_answer_must_be_the_rectangle_asked_for() {
    // One cell has one spelling on both sides, `$` allowed in the answer only.
    for asked in ["A1", "a1:a1"] {
        assert!(
            project_range(asked, &range_answer("Sheet1!A1", 1, 1)).is_some(),
            "{asked}"
        );
        assert!(
            project_range(asked, &range_answer("Sheet1!$A$1", 1, 1)).is_some(),
            "{asked}"
        );
    }
    assert!(
        project_range(
            "XFD1048576:XFD1048576",
            &range_answer("Sheet1!XFD1048576", 1, 1)
        )
        .is_some()
    );
    assert!(project_range("B2:A1", &range_answer("'Load Table'!A1:B2", 2, 2)).is_some());
    assert!(project_range("A1:B2", &range_answer("'Loads!2026'!$A$1:$B$2", 2, 2)).is_some());
    for (asked, answer) in [
        ("A1:B2", range_answer("Sheet1!A1:B3", 2, 2)),
        ("A1:B2", range_answer("Sheet1!B2:C3", 2, 2)),
        ("A1:B2", range_answer("A1:B2", 2, 2)),
        ("A1:B2", range_answer("Sheet1!A1:B2", 3, 2)),
        ("A1:B2", range_answer("Sheet1!A1:B2", 2, 3)),
        ("A1:B3", range_answer("Sheet1!A1:B3", 2, 2)),
        ("A1", range_answer("Sheet1!A1:A2", 1, 1)),
    ] {
        assert!(project_range(asked, &answer).is_none(), "{asked} {answer}");
    }
}

#[test]
fn a_malformed_range_answer_is_never_a_partial_table() {
    let good = range_answer("Sheet1!A1:B2", 2, 2);
    assert!(project_range("A1:B2", &good).is_some());
    let broken = |edit: &dyn Fn(&mut Json)| {
        let mut body = good.clone();
        edit(&mut body);
        body
    };
    let cases: Vec<(&str, Json)> = vec![
        ("ragged row", broken(&|b| b["values"][1] = json!(["x"]))),
        (
            "extra row",
            broken(&|b| b["text"].as_array_mut().unwrap().push(json!(["a", "b"]))),
        ),
        ("null cell", broken(&|b| b["values"][0][0] = Json::Null)),
        ("array cell", broken(&|b| b["values"][0][0] = json!(["x"]))),
        (
            "object cell",
            broken(&|b| b["values"][0][0] = json!({"v": 1})),
        ),
        ("numeric text", broken(&|b| b["text"][0][1] = json!(1))),
        (
            "unknown type",
            broken(&|b| b["valueTypes"][1][1] = json!("Formula")),
        ),
        ("typed type", broken(&|b| b["valueTypes"][1][1] = json!(1))),
        (
            "no values",
            broken(&|b| _ = b.as_object_mut().unwrap().remove("values")),
        ),
        (
            "no text",
            broken(&|b| _ = b.as_object_mut().unwrap().remove("text")),
        ),
        (
            "no types",
            broken(&|b| _ = b.as_object_mut().unwrap().remove("valueTypes")),
        ),
        ("matrix not array", broken(&|b| b["values"] = json!("x"))),
        ("row not array", broken(&|b| b["values"][0] = json!("x"))),
        (
            "no address",
            broken(&|b| _ = b.as_object_mut().unwrap().remove("address")),
        ),
        ("numeric address", broken(&|b| b["address"] = json!(1))),
        ("text rowCount", broken(&|b| b["rowCount"] = json!("2"))),
        (
            "negative columnCount",
            broken(&|b| b["columnCount"] = json!(-2)),
        ),
        ("not an object", json!([["x"]])),
    ];
    for (name, body) in cases {
        assert!(project_range("A1:B2", &body).is_none(), "{name}");
    }
}

#[test]
fn only_overlong_cells_are_cut_and_each_is_named_once() {
    let long = "é".repeat(MAX_CELL_TEXT + 1);
    let kept = "é".repeat(MAX_CELL_TEXT);
    // None cut.
    let out = project_range("A1:B2", &range_answer("S!A1:B2", 2, 2)).unwrap();
    assert_eq!(out["truncated-cells"], json!([]));
    assert_eq!(out["truncated-cells-omitted"], json!(0));
    // One cell whose value and text are both cut is listed once, zero-based.
    let mut body = range_answer("S!A1:B2", 2, 2);
    body["values"][1][0] = json!(long);
    body["text"][1][0] = json!(long);
    body["text"][0][1] = json!(kept);
    let out = project_range("A1:B2", &body).unwrap();
    assert_eq!(out["truncated-cells"], json!([[1, 0]]));
    assert_eq!(out["truncated-cells-omitted"], json!(0));
    assert_eq!(out["values"][1][0], json!(kept));
    assert_eq!(out["text"][1][0], json!(kept));
    assert_eq!(out["text"][0][1], json!(kept), "exactly the limit is whole");
    // 101 cut cells: the first 100 in row-major order, the rest counted.
    let mut body = range_answer("S!A1:K11", 11, 11);
    for n in 0..101 {
        body["text"][n / 11][n % 11] = json!(long);
    }
    let out = project_range("A1:K11", &body).unwrap();
    let listed = out["truncated-cells"].as_array().unwrap();
    assert_eq!(listed.len(), 100);
    assert_eq!(listed[0], json!([0, 0]));
    assert_eq!(listed[11], json!([1, 0]));
    assert_eq!(listed[99], json!([9, 0]));
    assert_eq!(out["truncated-cells-omitted"], json!(1));
}

#[test]
fn worksheets_keep_only_braced_ids() {
    let body = json!({"value": [
        {"id": SHEET_ID, "name": "Load Table", "position": 0, "visibility": "Visible",
         "@odata.id": "must-not-leave"},
        {"id": "Sheet2", "name": "bad id"},
        {"id": "{00000000-0000-0000-0000-000000000001}", "name": "Hidden", "position": 1,
         "visibility": "Hidden"},
    ]});
    let out = Projection::ExcelWorksheets
        .apply(
            &body,
            ProjectionContext {
                skip: 0,
                limit: 100,
                since: None,
                range: None,
            },
        )
        .unwrap();
    assert_eq!(
        out,
        json!({
            "worksheets": [
                {"id": SHEET_ID, "name": "Load Table", "position": 0, "visibility": "Visible"},
                {"id": "{00000000-0000-0000-0000-000000000001}", "name": "Hidden", "position": 1,
                 "visibility": "Hidden"},
            ],
            "more-available": false,
            "omitted": 1,
        })
    );
}

// ── through the bound call ───────────────────────────────────────────────────

#[tokio::test]
async fn each_b2_read_runs_through_the_bound_call_as_a_bare_get() {
    let workbook = format!("/v1.0/drives/{DRIVE}/items/{ITEM}/workbook");
    let cases: Vec<(&str, Json, Json, String)> = vec![
        (
            "excel.worksheet.list",
            json!({"drive-id": DRIVE, "item-id": ITEM, "page-size": 1}),
            json!({"value": [{"id": SHEET_ID, "name": "A"},
                             {"id": "{00000000-0000-0000-0000-000000000001}", "name": "B"}]}),
            format!("{workbook}/worksheets"),
        ),
        (
            "excel.range.read",
            json!({"drive-id": DRIVE, "item-id": ITEM, "worksheet": SHEET_ID, "range": "A1:B2"}),
            range_answer("Sheet1!A1:B2", 2, 2),
            format!(
                "{workbook}/worksheets/%7BFC034FA8-F8CC-4D24-9C0A-02A81B7792A0%7D/range(address='A1:B2')"
            ),
        ),
        (
            "get-shared-item",
            json!({"sharing-url": SHARE}),
            json!({"id": ITEM, "name": "Loads.xlsx", "size": 10,
                   "file": {"mimeType": "application/vnd.ms-excel"},
                   "parentReference": {"driveId": DRIVE},
                   "@microsoft.graph.downloadUrl": "https://must-not-leave.example/x"}),
            format!("/v1.0/shares/{SHARE_TOKEN}/driveItem"),
        ),
    ];
    for (command, inputs, data, path) in cases {
        let (out, f) = call_read(command, inputs.clone(), data).await;
        let out = out.unwrap_or_else(|e| panic!("{command}: {e:?}"));
        assert_ms_clean(&out.to_string());
        let heads = data_heads(&f);
        assert_eq!(heads.len(), 1, "{command}: one data request");
        let head = &heads[0];
        assert!(head.starts_with("GET "), "{command}: {head}");
        assert_eq!(path_of(head).split('?').next().unwrap(), path, "{command}");
        let (fields, rest) = head.split_once("\r\n\r\n").unwrap();
        assert!(rest.is_empty(), "{command}: a body was sent");
        for line in fields.lines().skip(1) {
            let name = line.split(':').next().unwrap().to_ascii_lowercase();
            assert!(
                !matches!(
                    name.as_str(),
                    "prefer" | "workbook-session-id" | "content-length" | "transfer-encoding"
                ),
                "{command}: sent {line}"
            );
        }
        let body = &out["result"]["body"];
        match command {
            "excel.worksheet.list" => {
                assert_eq!(body["worksheets"].as_array().unwrap().len(), 1);
                assert_eq!(body["more-available"], json!(true));
            }
            "excel.range.read" => assert_eq!(body["values"][1][1], json!("r1c1")),
            "get-shared-item" => {
                assert_eq!(body["id"], json!(ITEM));
                assert_eq!(body["drive-id"], json!(DRIVE));
            }
            _ => unreachable!(),
        }
    }
}

#[tokio::test]
async fn a_range_answer_over_the_budget_fails_before_or_after_projection() {
    let inputs = json!({"drive-id": DRIVE, "item-id": ITEM, "worksheet": "S", "range": "A1:B2"});
    let run = |answer: Json, cap: usize| {
        let manifest: Agent = serde_yaml::from_str(&ms_runnable_manifest()).unwrap();
        let inputs = inputs.clone();
        async move {
            let h = ms_home();
            ms_store(h.path(), None, FILES_SCOPES);
            let f = graph(me(OID, UPN), Reply::json(200, &answer));
            run_for_workflow_with(
                h.path(),
                &manifest,
                "excel.range.read",
                &inputs,
                Some(&ms_endpoints(&f)),
                Duration::from_secs(10),
                cap,
            )
            .await
        }
    };
    let answer = range_answer("S!A1:B2", 2, 2);
    let raw = answer.to_string().len();
    let projected = project_range("A1:B2", &answer).unwrap().to_string().len();
    assert!(projected > raw, "the projection adds labels");
    assert!(run(answer.clone(), projected).await.is_ok());
    // Under the cap as sent, over it as projected.
    let failure = run(answer.clone(), raw).await.unwrap_err();
    assert_eq!(failure.code, "E_CALL_OUTPUT_TOO_LARGE");
    // Over the cap as sent: refused before projection.
    let failure = run(answer, raw - 1).await.unwrap_err();
    assert_eq!(failure.code, "E_CALL_OUTPUT_TOO_LARGE");
}

#[tokio::test]
async fn without_a_files_write_grant_nothing_is_sent() {
    for command in B2 {
        let h = ms_home();
        ms_store(
            h.path(),
            None,
            "offline_access User.Read Files.Read.All Sites.Read.All",
        );
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
            cap["unavailableReason"]
                .as_str()
                .unwrap()
                .contains("--scopes User.Read,Files.ReadWrite"),
            "{command}: {}",
            cap["unavailableReason"]
        );
        assert_eq!(
            cap["permissions"]["adminConsent"],
            json!(false),
            "{command}"
        );
        let inputs = match command {
            "excel.worksheet.list" => json!({"drive-id": DRIVE, "item-id": ITEM}),
            "excel.range.read" => {
                json!({"drive-id": DRIVE, "item-id": ITEM, "worksheet": "S", "range": "A1"})
            }
            _ => json!({"sharing-url": SHARE}),
        };
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
    // The broader grants Microsoft documents are accepted too.
    for scope in ["Files.ReadWrite.All", "Sites.ReadWrite.All"] {
        let h = ms_home();
        ms_store(h.path(), None, &format!("offline_access User.Read {scope}"));
        let f = graph(me(OID, UPN), Reply::json(200, &json!({"value": []})));
        let cap = capabilities_with(
            h.path(),
            read("get-shared-item"),
            None,
            Duration::from_secs(10),
            &ms_endpoints(&f),
        )
        .await
        .unwrap();
        assert!(cap["unavailableCode"].is_null(), "{scope}: {cap}");
    }
}

#[tokio::test]
async fn a_workflow_node_reads_a_range_by_sheet_name() {
    let h = ms_home();
    ms_store(h.path(), None, "offline_access Files.ReadWrite");
    let f = graph(
        me(OID, UPN),
        Reply::json(200, &range_answer("'Load Table'!B2:C3", 2, 2)),
    );
    let manifest: Agent = serde_yaml::from_str(&ms_runnable_manifest()).unwrap();
    let out = run_for_workflow_with(
        h.path(),
        &manifest,
        "excel.range.read",
        &json!({"drive-id": "me", "item-id": ITEM, "worksheet": "Load Table", "range": "c3:b2"}),
        Some(&ms_endpoints(&f)),
        Duration::from_secs(10),
        1 << 20,
    )
    .await
    .unwrap();
    assert_eq!(out["address"], json!("'Load Table'!B2:C3"));
    assert_eq!(out["row-count"], json!(2));
    let heads = data_heads(&f);
    assert_eq!(heads.len(), 1);
    assert_eq!(
        path_of(&heads[0]).split('?').next().unwrap(),
        format!(
            "/v1.0/me/drive/items/{ITEM}/workbook/worksheets/Load%20Table/range(address='B2:C3')"
        )
    );
}
