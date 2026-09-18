//! Gate: every inline script in the HTML this crate emits parses (#518).
//!
//! The crate hands browsers roughly 150 KB of JavaScript — the viewer-3d
//! renderer's module, its classic bootstrap script and its import map, the
//! `aware report` search script, and the two OAuth callback pages. Every other
//! Rust assertion on that JavaScript is `String::contains`, and a syntax error
//! passes a `contains`: with `const input = =` planted in `commands/report.rs`
//! and `var var post=` in `render/viewer_3d.rs`, all 51 `render::viewer_3d` and
//! all 17 `commands::report` tests still passed.
//!
//! ## What is checked: the rendered output, not the source text
//!
//! Each surface below is RENDERED through the same function production calls,
//! and the HTML that comes out is what gets checked. An earlier version of this
//! gate read Rust source and inferred the HTML with a hand-rolled lexer; ten
//! review rounds found eighteen ways that inference was wrong — `concat!`,
//! `format!`, fragmented delimiters, `<script>` mentioned in prose — because
//! Rust has more ways to build a string than a regex has to recognise one.
//! Checking the bytes the program emits makes that whole class impossible: the
//! compiler has already resolved every macro and every interpolation.
//!
//! The pipeline has two halves, each given only the job its tool is good at:
//!
//! 1. **Rust tokenizes the HTML with `lol_html`**, a spec-compliant tokenizer,
//!    so HTML comments, quoted `>` in attributes, tag-name case and the
//!    script-data states are answered by construction rather than by pattern.
//!    [`script_goal`] then applies the HTML spec's rule for which grammar, if
//!    any, a browser parses the element's text under.
//! 2. **Node parses each executable body** under that goal
//!    (`cli/scripts/parse-check-scripts.mjs`) — V8 is the parser the viewer
//!    actually meets in Chromium, and CI's `ubuntu-latest` ships Node.
//!
//! ## Negative controls, run on every invocation
//!
//! A checker that has quietly stopped finding scripts, or stopped telling the
//! goals apart, reports every page clean. So [`CONTROLS`] feed deliberately
//! broken — and deliberately valid — HTML through the identical pipeline in the
//! same test, and each asserts the specific outcome that proves one property
//! (e.g. `import` must FAIL as a classic script and PASS as a module; a script
//! inside an HTML comment must not be found at all). If a control stops
//! producing its outcome, the gate fails before it vouches for anything.
//!
//! ## What this does NOT prove
//!
//! - **Completeness of the surface list.** [`surfaces`] is a table a human
//!   keeps. A new module that emits HTML must add itself; nothing discovers it.
//!   Discovery was the unsound half of the old design, and every surface that
//!   emits HTML today is listed here — including the four that carry no script,
//!   whose inventory is pinned at zero so a script added to one later is
//!   checked instead of silently shipped.
//! - **Behaviour.** A parse is a syntax check. Whether the viewer draws the
//!   right thing is `tests/browser/run.mjs`'s job, which needs Playwright and
//!   runs outside CI.
//! - **Every input.** Each surface renders one fixture, chosen to push hostile
//!   text (`</script>`, `<!--`, U+2028) through every place author-controlled
//!   data reaches the page, since that is how a correct template emits a broken
//!   script. It is a sample, not a proof over all inputs.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::rc::Rc;

use lol_html::{RewriteStrSettings, element, rewrite_str, text};
use serde_json::{Value, json};

/// How a browser treats a `<script>` element's text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Goal {
    /// Parsed with the Script grammar.
    Classic,
    /// Parsed with the Module grammar.
    Module,
    /// Parsed as JSON.
    ImportMap,
    /// Has a `src`: the browser fetches that and ignores the element's text.
    External,
    /// Any other type — a data block the browser never executes.
    Data,
}

/// One `<script>` element as a browser would see it.
#[derive(Debug)]
struct Script {
    goal: Goal,
    body: String,
}

/// The MIME type strings the HTML spec treats as JavaScript
/// ("JavaScript MIME type essence match"). An exact, case-insensitive match
/// after trimming; a parameter such as `; charset=utf-8` makes it a data block.
const JS_MIME_TYPES: [&str; 16] = [
    "application/ecmascript",
    "application/javascript",
    "application/x-ecmascript",
    "application/x-javascript",
    "text/ecmascript",
    "text/javascript",
    "text/javascript1.0",
    "text/javascript1.1",
    "text/javascript1.2",
    "text/javascript1.3",
    "text/javascript1.4",
    "text/javascript1.5",
    "text/jscript",
    "text/livescript",
    "text/x-ecmascript",
    "text/x-javascript",
];

/// The grammar a browser parses a script element's text under — the HTML
/// spec's "prepare the script element" steps 8–10, plus the `src` rule.
fn script_goal(type_attr: Option<&str>, language_attr: Option<&str>, has_src: bool) -> Goal {
    let source_type = match (type_attr, language_attr) {
        // A present, non-empty `type` wins.
        (Some(t), _) if !t.is_empty() => t.to_string(),
        // No `type` at all, but a non-empty `language`: "text/" + language.
        (None, Some(l)) if !l.is_empty() => format!("text/{l}"),
        // An empty `type`, or neither attribute: classic.
        _ => String::new(),
    };
    let essence = source_type
        .trim_matches(|c: char| c.is_ascii_whitespace())
        .to_ascii_lowercase();
    let goal = if essence.is_empty() || JS_MIME_TYPES.contains(&essence.as_str()) {
        Goal::Classic
    } else if essence == "module" {
        Goal::Module
    } else if essence == "importmap" {
        Goal::ImportMap
    } else {
        Goal::Data
    };
    // A `src` means the element's text is never parsed: a classic or module
    // script fetches the file instead, and an import map with `src` is an error.
    if has_src && goal != Goal::Data {
        Goal::External
    } else {
        goal
    }
}

/// Every `<script>` element in `html`, in document order.
fn scripts_in(html: &str) -> Vec<Script> {
    let found: Rc<RefCell<Vec<Script>>> = Rc::new(RefCell::new(Vec::new()));
    let on_start = Rc::clone(&found);
    let on_text = Rc::clone(&found);
    rewrite_str(
        html,
        RewriteStrSettings::new()
            .append_element_content_handler(element!("script", move |el| {
                let goal = script_goal(
                    el.get_attribute("type").as_deref(),
                    el.get_attribute("language").as_deref(),
                    el.has_attribute("src"),
                );
                on_start.borrow_mut().push(Script {
                    goal,
                    body: String::new(),
                });
                Ok(())
            }))
            .append_element_content_handler(text!("script", move |chunk| {
                if let Some(last) = on_text.borrow_mut().last_mut() {
                    last.body.push_str(chunk.as_str());
                }
                Ok(())
            })),
    )
    .unwrap_or_else(|e| panic!("lol_html could not tokenize the page: {e}"));
    found.take()
}

/// One page to check, and the script inventory it must carry.
struct Page {
    name: String,
    html: String,
    goals: Vec<Goal>,
}

/// What the Node half reported for one executable script.
#[derive(Debug)]
struct Checked {
    page: String,
    index: usize,
    goal: Goal,
    error: Option<String>,
}

/// Extract every page's scripts, assert each page's inventory, and parse every
/// executable body in ONE Node invocation.
fn check(pages: &[Page]) -> Vec<Checked> {
    let dir = tempfile::tempdir().expect("scratch dir");
    let mut pending: BTreeMap<String, (String, usize, Goal)> = BTreeMap::new();
    for (p, page) in pages.iter().enumerate() {
        let scripts = scripts_in(&page.html);
        let goals: Vec<Goal> = scripts.iter().map(|s| s.goal).collect();
        assert_eq!(
            goals, page.goals,
            "{}: the page's <script> inventory changed. If a script was added or \
             removed on purpose, update this surface's expected goals in \
             emitted_js_gate.rs so the new one is checked; if not, the renderer \
             stopped emitting what it did.",
            page.name
        );
        for (i, script) in scripts.iter().enumerate() {
            let ext = match script.goal {
                Goal::Classic => "js",
                Goal::Module => "mjs",
                Goal::ImportMap => "json",
                Goal::External | Goal::Data => continue,
            };
            assert!(
                !script.body.trim().is_empty(),
                "{}: script #{i} ({:?}) is empty — an empty body parses, so it \
                 would pass while checking nothing",
                page.name,
                script.goal
            );
            let file = format!("p{p:03}-s{i:02}.{ext}");
            std::fs::write(dir.path().join(&file), &script.body).expect("write script");
            pending.insert(file, (page.name.clone(), i, script.goal));
        }
    }

    let report = run_node(dir.path());
    let mut checked = Vec::new();
    for entry in report {
        let file = entry["file"].as_str().expect("report entry names its file");
        let (page, index, goal) = pending
            .remove(file)
            .unwrap_or_else(|| panic!("node reported {file}, which this gate never wrote"));
        checked.push(Checked {
            page,
            index,
            goal,
            error: entry["error"].as_str().map(str::to_string),
        });
    }
    assert!(
        pending.is_empty(),
        "node never reported on {:?} — a script that is not checked must not be \
         counted as passing",
        pending.keys().collect::<Vec<_>>()
    );
    checked
}

fn checker_script() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts/parse-check-scripts.mjs")
}

fn run_node(dir: &Path) -> Vec<Value> {
    let output = Command::new("node")
        .arg(checker_script())
        .arg(dir)
        .output()
        .unwrap_or_else(|e| {
            panic!(
                "could not run `node` ({e}). This gate parse-checks the JavaScript \
                 the CLI emits and needs Node on PATH — CI's ubuntu-latest ships \
                 one. It fails rather than skips: a gate that skips when its tool \
                 is missing reports green over nothing."
            )
        });
    assert!(
        output.status.success(),
        "parse-check-scripts.mjs failed to run ({}):\n{}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    let parsed: Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|e| {
        panic!(
            "parse-check-scripts.mjs printed something that is not JSON ({e}):\n{}",
            String::from_utf8_lossy(&output.stdout)
        )
    });
    parsed
        .as_array()
        .cloned()
        .expect("parse-check-scripts.mjs prints a JSON array")
}

/// Expected result of one executable script in a control page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Parses,
    Fails,
}

/// Negative (and positive) controls. Each proves one property of the pipeline;
/// the name says which, and the expected goals + outcomes are what prove it.
///
/// `(name, html, expected goals, expected outcome per EXECUTABLE script)`.
const CONTROLS: &[(&str, &str, &[Goal], &[Outcome])] = &[
    (
        "a syntax error in a classic script is caught",
        "<p>x</p><script>const x = ;</script>",
        &[Goal::Classic],
        &[Outcome::Fails],
    ),
    (
        "an error inside a function body is caught, not skipped by lazy parsing",
        "<script>function f() { return 1 +; }</script>",
        &[Goal::Classic],
        &[Outcome::Fails],
    ),
    (
        "module syntax fails under the classic goal",
        "<script>import x from \"y\";</script>",
        &[Goal::Classic],
        &[Outcome::Fails],
    ),
    (
        "the same module syntax passes under the module goal",
        "<script type=\"module\">import x from \"y\"; export const z = x;</script>",
        &[Goal::Module],
        &[Outcome::Parses],
    ),
    (
        "a syntax error in a module is caught, including inside a function",
        "<script type=\"module\">function f() { const x = ; }</script>",
        &[Goal::Module],
        &[Outcome::Fails],
    ),
    (
        "type and tag-name case and whitespace follow the spec",
        "<SCRIPT TYPE=\" Module \">await 1;</SCRIPT >",
        &[Goal::Module],
        &[Outcome::Parses],
    ),
    (
        "top-level await fails as a classic script",
        "<script>await 1;</script>",
        &[Goal::Classic],
        &[Outcome::Fails],
    ),
    (
        "a malformed import map is caught",
        "<script type=\"importmap\">{\"imports\": {,}}</script>",
        &[Goal::ImportMap],
        &[Outcome::Fails],
    ),
    (
        "an explicit JavaScript MIME type is a classic script",
        "<script type=\"text/javascript\">const x = ;</script>",
        &[Goal::Classic],
        &[Outcome::Fails],
    ),
    (
        "a language attribute alone selects the type",
        "<script language=\"javascript\">const x = ;</script><script language=\"vbscript\">x</script>",
        &[Goal::Classic, Goal::Data],
        &[Outcome::Fails],
    ),
    (
        "a data block is not parsed",
        "<script type=\"text/plain\">const x = ;</script>",
        &[Goal::Data],
        &[],
    ),
    (
        "a src script's text is ignored, as a browser ignores it",
        "<script src=\"x.js\">const x = ;</script>",
        &[Goal::External],
        &[],
    ),
    (
        "a script inside an HTML comment is not a script",
        "<!-- <script>const x = ;</script> --><p>ok</p>",
        &[],
        &[],
    ),
    (
        "a quoted > in an attribute does not end the tag early",
        "<script data-note=\"a > b\">const y = ;</script>",
        &[Goal::Classic],
        &[Outcome::Fails],
    ),
    (
        "markup-like text inside a script is script text, not tags",
        "<script>const s = \"<b>\"; const y = ;</script>",
        &[Goal::Classic],
        &[Outcome::Fails],
    ),
];

fn control_pages() -> Vec<Page> {
    CONTROLS
        .iter()
        .map(|(name, html, goals, _)| Page {
            name: format!("control: {name}"),
            html: (*html).to_string(),
            goals: goals.to_vec(),
        })
        .collect()
}

/// Text that breaks a template which forgets to neutralise it: closes the
/// script element, opens the tokenizer's escaped state, and ends a JS line.
const HOSTILE: &str = "</script><script>x<!--<script \u{2028}\u{2029}'\"`${";

/// Every HTML surface the crate emits, rendered through the function
/// production calls, with the script inventory each must carry.
fn surfaces() -> Vec<Page> {
    use Goal::{Classic, ImportMap, Module};

    let viewer_scene = json!({
        "meta": { "name": HOSTILE, "units": "mm", "up": "z" },
        "groups": [{ "key": "g", "label": HOSTILE, "color": "#60a5fa" }],
        "elements": [{
            "id": HOSTILE, "kind": "member", "group": "g",
            "from": [0, 0, 0], "to": [0, 0, 1000],
            "section": { "w": 100, "d": 200 },
            "meta": { "profile": HOSTILE }
        }],
        "panels": [{ "title": HOSTILE, "columns": [HOSTILE], "rows": [[HOSTILE]] }]
    });
    let viewer =
        crate::render::viewer_3d::viewer_3d_render(&json!({ "scene": viewer_scene }), true)
            .expect("viewer-3d renders the gate fixture");

    // Built as JSON and deserialized through the manifest types, so the fixture
    // travels the same serde path a real manifest does. (YAML would need its own
    // escaping for U+2028, which is part of what HOSTILE is testing.)
    let agent = crate::manifest::loader::DiscoveredAgent {
        manifest: serde_json::from_value(json!({
            "agent": "gate", "version": "1.0.0", "description": HOSTILE,
            "stateful": false, "license": "MIT",
            "transport": { "cli": { "binary": "aware-gate" } },
            "commands": { "run": { "lifecycle": "single", "description": HOSTILE } }
        }))
        .expect("gate agent manifest parses"),
        root: PathBuf::from("."),
    };

    let lock: crate::app_lock::LockFile = serde_json::from_value(json!({
        "source-hash": "abc", "compiled-at": "2026-01-01T00:00:00Z",
        "compiler-version": "0.0.0", "app": HOSTILE, "version": "0.1.0",
        "agent-pins": {},
        "nodes": [{
            "id": HOSTILE, "kind": "agent", "agent": HOSTILE,
            "command": HOSTILE, "mode": "write"
        }]
    }))
    .expect("gate lockfile parses");

    let ui = crate::render::ui::ui_render(&crate::render::ui::tests::render_args(), true)
        .expect("ui render renders its fixture");

    let page = |name: &str, html: &str, goals: &[Goal]| Page {
        name: name.to_string(),
        html: html.to_string(),
        goals: goals.to_vec(),
    };
    vec![
        page(
            "viewer-3d render",
            viewer["html"].as_str().expect("viewer-3d returns html"),
            &[Classic, ImportMap, Module],
        ),
        page(
            "aware report substrate",
            &crate::commands::report::render_substrate_html(&[agent]),
            &[Classic],
        ),
        page(
            "OAuth PKCE callback page",
            crate::auth::pkce::CALLBACK_PAGE,
            &[Classic],
        ),
        page(
            "token paste: stored page",
            &crate::auth::paste::render_success_page(),
            &[Classic],
        ),
        // A real integration id, not HOSTILE: this page interpolates the id
        // unescaped, and that is sound only because `aware connect` validates it
        // against the closed list in `auth::config::for_integration` before the
        // paste flow can run. Feeding HOSTILE here reports a script injection
        // that production cannot reach — the first run of this gate did exactly
        // that, and the upstream allowlist is why it was not a bug.
        page(
            "token paste: form page",
            &crate::auth::paste::render_paste_form("trimble-connect"),
            &[],
        ),
        page(
            "ui render",
            ui["html"].as_str().expect("ui render returns html"),
            &[],
        ),
        page(
            "html report",
            &crate::render::html_report::render_report_with(
                HOSTILE,
                &json!([{ "name": HOSTILE, "note": HOSTILE }]),
                &crate::render::html_report::TableOptions::default(),
            ),
            &[],
        ),
        page(
            "app glass box",
            &crate::commands::app::render_glass_box_html(&lock),
            &[],
        ),
    ]
}

#[test]
fn every_inline_script_the_crate_emits_parses() {
    let controls = control_pages();
    let real = surfaces();
    let mut pages = controls;
    let control_count = pages.len();
    pages.extend(real);

    let checked = check(&pages);

    // Controls first: until each produces its expected outcome, nothing below
    // is evidence of anything.
    for (c, (name, _, _, outcomes)) in CONTROLS.iter().enumerate() {
        let page = &pages[c].name;
        let got: Vec<Outcome> = checked
            .iter()
            .filter(|r| &r.page == page)
            .map(|r| {
                if r.error.is_some() {
                    Outcome::Fails
                } else {
                    Outcome::Parses
                }
            })
            .collect();
        assert_eq!(
            got,
            *outcomes,
            "negative control {name:?} no longer produces its outcome, so the \
             checker cannot be trusted to find what that control proves. \
             Results: {:?}",
            checked
                .iter()
                .filter(|r| &r.page == page)
                .collect::<Vec<_>>()
        );
    }

    let failures: Vec<&Checked> = checked
        .iter()
        .filter(|r| !pages[..control_count].iter().any(|p| p.name == r.page))
        .filter(|r| r.error.is_some())
        .collect();
    assert!(
        failures.is_empty(),
        "shipped JavaScript does not parse — a browser would refuse these scripts:\n{}",
        failures
            .iter()
            .map(|r| format!(
                "  {} script #{} ({:?}): {}",
                r.page,
                r.index,
                r.goal,
                r.error.as_deref().unwrap_or_default()
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn script_goal_follows_the_html_spec() {
    use Goal::*;
    for (type_attr, language, src, want) in [
        (None, None, false, Classic),
        (Some(""), None, false, Classic),
        (Some(""), Some("vbscript"), false, Classic), // an empty type ignores language
        (Some("  TEXT/JavaScript "), None, false, Classic),
        (Some("text/javascript; charset=utf-8"), None, false, Data),
        (Some("module"), None, false, Module),
        (Some("importmap"), None, false, ImportMap),
        (Some("application/json"), None, false, Data),
        (None, Some("JavaScript"), false, Classic),
        (None, Some("vbscript"), false, Data),
        (None, None, true, External),
        (Some("module"), None, true, External),
        (Some("text/plain"), None, true, Data),
        (Some("importmap"), None, true, External),
    ] {
        assert_eq!(
            script_goal(type_attr, language, src),
            want,
            "type={type_attr:?} language={language:?} src={src}"
        );
    }
}
