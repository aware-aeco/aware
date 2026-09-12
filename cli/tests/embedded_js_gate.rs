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
/// The single entry point both callers go through — see [`checkEmbeddedScripts`]
/// in the scan script for why the step is one function rather than three.
const ENTRY: &str = "checkEmbeddedScripts";

/// The lines of a JavaScript file that are not `//` comments.
///
/// Every assertion below reads code, never prose. An earlier draft searched whole
/// files and tripped on a rule's own explanatory comment, which quotes the
/// expression it forbids — the same "assert the text, not the behaviour" mistake
/// these tests exist to catch, made while writing one.
fn code_lines(src: &str) -> impl Iterator<Item = &str> {
    src.lines()
        .map(str::trim_start)
        .filter(|line| !line.starts_with("//"))
}

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
fn expected_keys(script: &str) -> Vec<String> {
    let start = script
        .find("const EXPECTED = {")
        .unwrap_or_else(|| panic!("cli/{SCRIPT} no longer declares `const EXPECTED = {{`"));
    let rest = &script[start + "const EXPECTED = {".len()..];
    let end = rest
        .find("};")
        .unwrap_or_else(|| panic!("cli/{SCRIPT}'s EXPECTED table is not closed by `}};`"));
    let body = strip_js_comments(&rest[..end]);

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

/// One parse check, not two.
///
/// `tests/browser/run.mjs` used to carry its own, covering the module script and
/// nothing else. Two implementations is how the classic bootstrap script came to
/// be checked by neither: each looked like the other's coverage.
#[test]
fn the_browser_gate_delegates_its_parse_check_rather_than_repeating_it() {
    let run = read("cli/tests/browser/run.mjs");

    // The IMPORT STATEMENT, not the filename. The first draft searched the whole
    // file for `parse-check-embedded-js.mjs`, which also appears in the header
    // comment explaining the delegation — so deleting the import left this green
    // while `run.mjs` reached `collect(CLI)` with `collect` undefined and aborted
    // instead of parse-checking anything (Codex review, #518).
    let import = run
        .lines()
        .find(|line| {
            let line = line.trim_start();
            line.starts_with("import") && line.contains("parse-check-embedded-js.mjs")
        })
        .unwrap_or_else(|| {
            panic!(
                "tests/browser/run.mjs has no `import … from '…/parse-check-embedded-js.mjs'` \
                 statement, so its step 0 calls functions nothing brought into scope. Two \
                 parse checks drift; none at all aborts."
            )
        });

    // The exact LOCAL binding, not a substring. `import.contains("…")` is satisfied
    // by an alias, which binds a different name and leaves the call unbound, and by
    // any longer identifier containing the substring (Codex review, #518).
    let bindings = import_local_names(import);
    assert!(
        bindings.iter().any(|b| b == ENTRY),
        "tests/browser/run.mjs imports from cli/{SCRIPT} but does not bind the local \
         name {ENTRY:?} its step 0 calls (an alias binds a different name). Local \
         names bound: {bindings:?}"
    );

    // Bound is not called. Deleting `checkEmbeddedScripts(…)` from the body while
    // leaving the import in place satisfied every assertion above, and the gate
    // would then parse whatever collection happened to find — the regression the
    // assertion exists to prevent, reachable by deleting one line (Codex review,
    // #518). So require the CALL, on a code line.
    assert!(
        code_lines(&run).any(|line| line.contains(&format!("{ENTRY}("))),
        "tests/browser/run.mjs imports {ENTRY:?} but never calls it. An import that \
         nothing invokes is not delegation; step 0 would then check nothing, or \
         whatever it reassembled locally."
    );

    // And nothing of its own. The step's primitives are deliberately NOT reachable
    // here: two rounds of findings were this file assembling them and leaving one
    // out — its own `blocks.length < 5` floor after the shared floor moved on, and
    // before that its own `new Function` parse. One call has no parts to skip;
    // these are what stop the parts coming back.
    for own in [
        "collect(",
        "shortfall(",
        "parseError(",
        "blocks.length <",
        "new Function(",
    ] {
        let line = code_lines(&run).find(|line| line.contains(own));
        assert!(
            line.is_none(),
            "tests/browser/run.mjs has reassembled the check locally ({own:?} at \
             {line:?}). The whole step lives in cli/{SCRIPT} behind {ENTRY:?} \
             precisely so this file has no floor, no parser and no discovery order \
             of its own to get wrong."
        );
    }
    assert!(
        !run.contains("new Function("),
        "tests/browser/run.mjs has reintroduced its own `new Function` parse \
         check. That is sloppy-mode Script semantics, which accepts constructs a \
         module rejects; cli/{SCRIPT} parses each block under the goal symbol a \
         browser would use."
    );
}

/// `src` with both JavaScript comment forms blanked to spaces, newlines kept so
/// line structure survives for the caller's per-line parse.
///
/// String-aware, because the keys it is clearing the way for are paths: a naive
/// scan could mistake a `/` inside `'src/auth/paste.rs'` for the start of a
/// comment. Only single quotes are tracked, which is all the table uses.
fn strip_js_comments(src: &str) -> String {
    let bytes: Vec<char> = src.chars().collect();
    let mut out = String::with_capacity(src.len());
    let mut i = 0;
    let mut in_string = false;
    while i < bytes.len() {
        let c = bytes[i];
        if in_string {
            out.push(c);
            if c == '\'' {
                in_string = false;
            }
            i += 1;
            continue;
        }
        if c == '\'' {
            in_string = true;
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

/// The local names bound by an `import { … } from '…'` line.
///
/// For `{ collect, parseError as pe }` this is `["collect", "pe"]` — the name
/// actually in scope, which is the part after `as` when a spec is aliased and the
/// whole spec otherwise. Anything outside the braces (a default or namespace
/// import) is not what step 0 relies on and is ignored.
fn import_local_names(import: &str) -> Vec<String> {
    let Some(open) = import.find('{') else {
        return Vec::new();
    };
    let Some(close_rel) = import[open + 1..].find('}') else {
        return Vec::new();
    };
    let inner = &import[open + 1..open + 1 + close_rel];
    inner
        .split(',')
        .filter_map(|spec| {
            let spec = spec.trim();
            if spec.is_empty() {
                return None;
            }
            // `orig as local` binds `local`; a bare `orig` binds `orig`.
            let local = spec.rsplit(" as ").next().unwrap_or(spec);
            Some(local.trim().to_string())
        })
        .collect()
}
