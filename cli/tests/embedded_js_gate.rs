//! Gate: the JavaScript this crate emits is parse-checked by CI.
//!
//! The crate bakes roughly 148 KB of JavaScript into HTML it hands to a browser
//! — the viewer-3d renderer's module and its classic bootstrap script, the
//! `aware report` search script, and the two OAuth callback pages. Every Rust
//! assertion on any of it is `String::contains`, and a syntax error passes a
//! `contains`.
//!
//! Measured, not assumed. With `const input = =` planted in
//! `src/commands/report.rs`'s `SCRIPT` and `var var post=` planted in
//! `src/render/viewer_3d.rs`'s bootstrap script, `cargo test --bin aware
//! render::viewer_3d` reported `51 passed; 0 failed` and `commands::report`
//! reported `17 passed; 0 failed`. Both broken scripts would have shipped.
//!
//! `tests/browser/run.mjs` held the only parse check and did not close this, for
//! two independent reasons. It ran nowhere — it is a manual pre-PR gate needing
//! Playwright and a CDN, and its header justified staying out of CI partly with
//! "this repo has no Rust CI job at all", which `ci.yml` (#298) ended. And it
//! reached one of the six blocks: it sliced `const TEMPLATE` out of
//! `viewer_3d.rs` and checked the `type="module"` script inside it, so the
//! classic bootstrap script — the handshake that decides whether an embedding
//! client is told the render failed — and `report.rs`'s script were covered by
//! nothing at all, in CI or out of it.
//!
//! CLAUDE.md §Engineering rules — "Verify before answering", "No corner-cutting"
//! — is what that defeats: a green check is the claim that the tests passed, and
//! for the shipped JavaScript the claim was never evaluated. Parsing needs no
//! browser, no Playwright and no network, so `scripts/parse-check-embedded-js.mjs`
//! runs it in `ci.yml` over every block, discovered rather than named.
//!
//! What this file adds is what the script cannot assert about itself: that CI
//! still runs it, still runs its negative control, and still has JavaScript to
//! find. Without the last one a refactor that moved the templates elsewhere
//! would leave a gate reporting a clean crate over nothing — the failure mode
//! `tests/dotnet_suites_gate.rs` records from #433.
//!
//! ## What this file deliberately does NOT assert, and why
//!
//! It used to police the internal shape of `tests/browser/run.mjs` — that step 0
//! calls the shared entry point and reimplements none of its parts. Those
//! assertions are gone. They needed to know which text in a JavaScript file is
//! code, and doing that from Rust meant hand-rolling a lexer, which was wrong
//! four separate ways in review (#518): it read `//` only, so a block comment
//! fooled it; then it read comments but not strings, so the function's name in a
//! diagnostic message satisfied "must call"; and its regex-literal escape hatch
//! matched four spellings out of infinitely many, while `const re=/[/*]/` both
//! slips through and desynchronises the stripper for everything after it.
//!
//! The honest reading of that record: a Rust integration test is the wrong place
//! to decide what a JavaScript expression is, and each fix bought one spelling.
//! The cost was real and the benefit was not — those assertions guarded a file CI
//! never executes (`run.mjs` needs Playwright and a CDN; no workflow runs it),
//! and in eight review rounds they caught no defect in shipped code while
//! generating four findings of their own.
//!
//! What actually protects users is untouched: `scripts/parse-check-embedded-js.mjs`
//! runs in `ci.yml` on every PR, with its own 27-case negative control and a
//! per-file inventory floor, and since #518's restructuring that floor lives
//! *inside* `checkEmbeddedScripts` — so the drift the deleted assertions were
//! written to catch is now structurally impossible rather than merely policed.
//! The residual risk is that someone edits `run.mjs` to stop calling it, which
//! shows up as a missing step-0 section to whoever runs the manual gate.
//!
//! `strip_js_comments` therefore survives scoped to where it began and is sound:
//! the four-line `EXPECTED` table, which contains no strings but paths and no
//! regular expressions at all.

use std::path::PathBuf;

/// Repository root — `cli/`'s parent, where the workflow lives.
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap_or_else(|| panic!("{} has no parent", env!("CARGO_MANIFEST_DIR")))
        .to_path_buf()
}

fn read(rel: &str) -> String {
    let path = repo_root().join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

struct Step {
    name: String,
    body: String,
}

/// Split a workflow's steps in order.
///
/// Scoped to the step rather than the file, for the reason
/// `tests/lockfile_gate.rs` records: a `contains` over the whole of `ci.yml`
/// passes for the wrong reason. Here specifically, the step's own explanatory
/// comment names `parse-check-embedded-js.mjs` twice, so deleting the commands
/// would leave the prose describing them — which is why comment lines are
/// dropped before anything is asserted.
fn steps(workflow: &str) -> Vec<Step> {
    workflow
        .split("\n      - ")
        .skip(1) // the job header, before the first step
        .map(|chunk| Step {
            name: chunk
                .lines()
                .next()
                .and_then(|first| first.strip_prefix("name: "))
                .unwrap_or_default() // a bare `uses:` step has no name
                .trim()
                .to_string(),
            body: chunk
                .lines()
                .filter(|line| !line.trim_start().starts_with('#'))
                .collect::<Vec<_>>()
                .join("\n"),
        })
        .collect()
}

fn step_index(steps: &[Step], name: &str) -> usize {
    steps
        .iter()
        .position(|step| step.name == name)
        .unwrap_or_else(|| {
            panic!(
                "ci.yml has no step named {name:?} — it was renamed or removed, and \
                 this test can no longer tell whether its gate still runs. Steps \
                 present: {:?}",
                steps.iter().map(|s| &s.name).collect::<Vec<_>>()
            )
        })
}

const STEP: &str = "shipped inline JavaScript parses";
const SCRIPT: &str = "scripts/parse-check-embedded-js.mjs";

#[test]
fn ci_still_parse_checks_the_javascript_this_crate_emits() {
    let source = read(".github/workflows/ci.yml");
    let steps = steps(&source);

    // A floor, not an is-empty check: a splitter that recovered one step would
    // pass an emptiness test having silently dropped the rest, and the lookup
    // below would then fail for the wrong reason.
    assert!(
        steps.len() > 8,
        "parsed only {} steps out of ci.yml — the splitter is broken, so nothing \
         below means anything",
        steps.len()
    );

    let step = &steps[step_index(&steps, STEP)];

    // Half one: the check itself.
    assert!(
        step.body.contains(&format!("node {SCRIPT}\n"))
            || step.body.trim_end().ends_with(&format!("node {SCRIPT}")),
        "the {STEP:?} step no longer runs `node {SCRIPT}` without flags, so the \
         crate's shipped JavaScript is parsed by nothing. Step body:\n{}",
        step.body
    );

    // Half two: the negative control, and it must come FIRST. A scanner that has
    // stopped matching anything reports every crate clean, so a run without a
    // preceding self-test is a step that looks like a gate and is not — the same
    // pairing `no-hardcoded-string-offsets.py` and `run-agent-python-tests.py`
    // already carry.
    let self_test = step.body.find(&format!("node {SCRIPT} --self-test"));
    let plain = step.body.find(&format!("node {SCRIPT}\n"));
    assert!(
        self_test.is_some(),
        "the {STEP:?} step no longer runs `--self-test`. Without it, a scanner \
         that matches nothing at all reports the crate clean forever. Step \
         body:\n{}",
        step.body
    );
    if let (Some(control), Some(run)) = (self_test, plain) {
        assert!(
            control < run,
            "the {STEP:?} step runs the check before its negative control, so a \
             broken scanner reports green before anything establishes it still \
             classifies. Step body:\n{}",
            step.body
        );
    }

    assert!(
        repo_root().join("cli").join(SCRIPT).is_file(),
        "ci.yml runs cli/{SCRIPT}, which does not exist"
    );
}

/// The gate discovers scripts rather than naming them, so it reports a clean
/// crate when it finds none. This is what notices that case in the suite.
///
/// Deliberately not a re-implementation of the scan: duplicating the tokenizer
/// in Rust would give two classifiers to keep in agreement. It anchors the
/// templates the scan is known to reach, which is what a refactor would move.
///
/// All FOUR files, not the two obvious ones. An earlier draft anchored only the
/// viewer and the report, so losing an OAuth page's script was invisible here
/// (Codex review, #518) — and that is the half of the inventory easiest to drop
/// by accident, being one line each rather than a named `const`.
#[test]
fn the_crate_still_carries_inline_scripts_for_the_gate_to_reach() {
    for (file, marker) in [
        ("cli/src/render/viewer_3d.rs", "<script type=\"module\">"),
        ("cli/src/render/viewer_3d.rs", "<script type=\"importmap\">"),
        ("cli/src/render/viewer_3d.rs", "<script>"),
        (
            "cli/src/commands/report.rs",
            "const SCRIPT: &str = r#\"<script>",
        ),
        ("cli/src/auth/pkce.rs", "<script>"),
        ("cli/src/auth/paste.rs", "<script>"),
    ] {
        assert!(
            read(file).contains(marker),
            "{file} no longer contains {marker:?}. If the template moved, point \
             cli/{SCRIPT} at its new home — a scan that reaches no JavaScript \
             reports a clean crate over nothing, which is the failure this gate \
             exists to prevent."
        );
    }
}

/// The scan's own floor must name every file that carries a script.
///
/// The floor started as a bare count — "at least 5 blocks in at least 3 files"
/// against a real inventory of 6 in 4 — so losing one script still cleared it and
/// went unchecked with every gate green (Codex review, #518). A count below what
/// is actually there is not a floor, and only the script can say what it expects,
/// so this checks that its table still names each file rather than restating the
/// numbers and giving two tables to keep in agreement.
#[test]
fn the_scan_floor_still_names_every_file_that_carries_a_script() {
    let script = read(&format!("cli/{SCRIPT}"));

    // The ACTIVE keys of the object, not a substring of its text. Two ways a text
    // scan passed for the wrong reason, each found by review (#518): the same
    // quoted paths appear again in `FLOOR_FIXTURES` further down (so scope to the
    // braces), and an entry COMMENTED OUT rather than deleted still leaves its
    // path in the braces (so drop `//` lines). `Object.entries(EXPECTED)` ignores
    // a commented entry, and the JS self-test adapts to the smaller object, so
    // without this the file's floor could vanish with every guard green.
    let keys = expected_keys(&script);

    for file in [
        "src/render/viewer_3d.rs",
        "src/commands/report.rs",
        "src/auth/pkce.rs",
        "src/auth/paste.rs",
    ] {
        assert!(
            keys.iter().any(|k| k == file),
            "cli/{SCRIPT}'s EXPECTED table no longer has an active entry for \
             {file:?} (deleted, or commented out), so a refactor that stops the \
             scan reaching that file's script would leave its syntax checked by \
             nothing while the gate reports green. Active keys: {keys:?}"
        );
    }
}

/// The active keys of `const EXPECTED = { … }` in the scan script.
///
/// A commented-out entry is not a key, in EITHER JavaScript comment form. The
/// first version dropped only `//` lines, so `/* 'src/auth/paste.rs': 1, */` left
/// the path in the text while `Object.entries(EXPECTED)` no longer returned it —
/// the per-file floor gone with this test still green, which is the same hole one
/// comment syntax further along (Codex review, #518). [`strip_js_comments`]
/// removes both before any key is read.
///
/// Fails loudly rather than returning an empty list — a table this cannot find is
/// a table nothing is checking, and a silent empty result would turn the
/// assertions above into a test that passes for want of anything to disagree
/// with.
/// The raw text between `const EXPECTED = {` and its closing `};` in the scan
/// script — the only part of that file [`strip_js_comments`] is ever shown.
fn expected_body(script: &str) -> String {
    let start = script
        .find("const EXPECTED = {")
        .unwrap_or_else(|| panic!("cli/{SCRIPT} no longer declares `const EXPECTED = {{`"));
    let rest = &script[start + "const EXPECTED = {".len()..];
    let end = rest
        .find("};")
        .unwrap_or_else(|| panic!("cli/{SCRIPT}'s EXPECTED table is not closed by `}};`"));
    rest[..end].to_string()
}

fn expected_keys(script: &str) -> Vec<String> {
    let body = strip_js_comments(&expected_body(script));

    let mut keys = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // `'path': n,` — the key is the first single-quoted span, and the colon
        // after it is what distinguishes a key from any other quoted text.
        let Some(open) = line.find('\'') else {
            continue;
        };
        let Some(close_rel) = line[open + 1..].find('\'') else {
            continue;
        };
        let close = open + 1 + close_rel;
        if line[close + 1..].trim_start().starts_with(':') {
            keys.push(line[open + 1..close].to_string());
        }
    }
    assert!(
        !keys.is_empty(),
        "cli/{SCRIPT}'s EXPECTED table parsed to zero active keys — the parser or \
         the table's shape changed, and every assertion resting on it is now \
         meaningless. Body read:\n{body}"
    );
    keys
}

/// `src` with both JavaScript comment forms blanked to spaces, newlines kept so
/// line structure survives for the caller's per-line parse.
///
/// String-aware, because the keys it clears the way for are paths: a naive scan
/// would mistake the `/` inside `'src/auth/paste.rs'` for the start of a comment
/// and eat the rest of the table.
///
/// All three quote forms and escapes, so that neither a `"` nor a backtick nor
/// `'it\'s'` leaves the state machine inside a string it has already left.
///
/// **Shown the `EXPECTED` table body and nothing else.** That scoping is what
/// makes it sound rather than approximately right: those four lines hold quoted
/// paths and integers, no regular expressions, and no construct where `//` or
/// `/*` means anything but a comment. It is emphatically not a JavaScript lexer —
/// an earlier draft pointed it at whole files and was wrong four ways in review
/// (see this module's header). If you need to know what an arbitrary `.mjs` file
/// means, use `node`, not this.
fn strip_js_comments(src: &str) -> String {
    let bytes: Vec<char> = src.chars().collect();
    let mut out = String::with_capacity(src.len());
    let mut i = 0;
    let mut quote: Option<char> = None;
    while i < bytes.len() {
        let c = bytes[i];
        if let Some(q) = quote {
            out.push(c);
            if c == '\\' && i + 1 < bytes.len() {
                out.push(bytes[i + 1]); // an escaped quote does not close the string
                i += 2;
                continue;
            }
            if c == q {
                quote = None;
            }
            i += 1;
            continue;
        }
        if c == '\'' || c == '"' || c == '`' {
            quote = Some(c);
            out.push(c);
            i += 1;
            continue;
        }
        if c == '/' && bytes.get(i + 1) == Some(&'/') {
            while i < bytes.len() && bytes[i] != '\n' {
                out.push(' ');
                i += 1;
            }
            continue;
        }
        if c == '/' && bytes.get(i + 1) == Some(&'*') {
            out.push_str("  ");
            i += 2;
            while i < bytes.len() && !(bytes[i] == '*' && bytes.get(i + 1) == Some(&'/')) {
                out.push(if bytes[i] == '\n' { '\n' } else { ' ' });
                i += 1;
            }
            if i < bytes.len() {
                out.push_str("  ");
                i += 2;
            }
            continue;
        }
        out.push(c);
        i += 1;
    }
    out
}
