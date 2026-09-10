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
    for file in [
        "src/render/viewer_3d.rs",
        "src/commands/report.rs",
        "src/auth/pkce.rs",
        "src/auth/paste.rs",
    ] {
        assert!(
            script.contains(&format!("'{file}'")),
            "cli/{SCRIPT}'s EXPECTED table no longer names {file:?}, so a refactor \
             that stops the scan reaching that file's script would leave its \
             syntax checked by nothing while the gate reports green."
        );
    }
}

/// One parse check, not two.
///
/// `tests/browser/run.mjs` used to carry its own, covering the module script and
/// nothing else. Two implementations is how the classic bootstrap script came to
/// be checked by neither: each looked like the other's coverage.
#[test]
fn the_browser_gate_delegates_its_parse_check_rather_than_repeating_it() {
    let run = read("cli/tests/browser/run.mjs");
    assert!(
        run.contains("parse-check-embedded-js.mjs"),
        "tests/browser/run.mjs no longer imports the shared parse check. Two \
         implementations drift, and the block one of them misses is covered by \
         nobody."
    );
    assert!(
        !run.contains("new Function("),
        "tests/browser/run.mjs has reintroduced its own `new Function` parse \
         check. That is sloppy-mode Script semantics, which accepts constructs a \
         module rejects; cli/{SCRIPT} parses each block under the goal symbol a \
         browser would use."
    );
}
