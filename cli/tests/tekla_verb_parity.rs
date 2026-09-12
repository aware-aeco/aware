//! Gate: the tekla manifest advertises exactly the verbs the `aware-tekla` bridge can dispatch.
//!
//! The manifest declared 24 commands; the bridge's `switch (verb)` dispatched 7. The other 19 —
//! `insert`, `part-list`, `report-create`, `uda-get`, `ifc-export`, `clash-check` and the rest —
//! were declared as ordinary runnable commands, so `aware app validate` and `aware app compile`
//! accepted a node using one and the run then died at the bridge with
//! `aware-tekla: unknown verb 'insert'`. An app author (or the terminal AI composing a workflow)
//! reads `manifest.yaml` as the contract, and that contract listed a native connection insert, UDA
//! get/set, NC/IFC export, drawing issue and clash check that do not exist — with the first signal
//! arriving after compile and the Run gate had both said yes (#520).
//!
//! The same drift, at one command, is #176 (`watch` declared, not implemented) and #161
//! (`html-report` declared a binary it did not ship). The marker that closes it already existed:
//! per-command `status: planned` (#199), which turns such a node into `E_APP_COMMAND_UNAVAILABLE`
//! at validate and compile. It was simply unused on the agent that needed it most.
//!
//! Marking the 19 fixes today. This fixes tomorrow, in BOTH directions, because each direction is
//! a different defect and neither implies the other:
//!
//!   * A **declared-but-undispatched** command without `status: planned` is the drift above — a
//!     promise the bridge cannot keep, failing at run.
//!   * A **dispatched** command marked `status: planned`, or absent from the manifest, is the
//!     inverse: working functionality nobody can reach. `list-instances` and `close` were in that
//!     state — dispatched since the bridge was written, published in no manifest.
//!
//! ## What the two parity tests pin, exactly
//!
//! Together they are an EQUALITY, which is stronger than either alone and is the real invariant:
//! [`every_declared_tekla_command_the_bridge_cannot_dispatch_is_marked_planned`] gives
//! `declared ∧ ¬planned ⇒ dispatched`, and
//! [`every_verb_the_bridge_dispatches_is_declared_and_runnable`] gives the converse. So
//! `dispatched == declared − planned`, pinned in both directions.
//!
//! That equality is also what makes a UNIFORMLY shrinking parser non-vacuous, and it pays to be
//! precise about which half each failure breaks, because it is not the intuitive one. An EMPTY
//! dispatch set fails the planned-marker test LOUDLY: all 26 commands read as undispatched, and the
//! 7 carrying no marker land in `unmarked`. What an empty set satisfies vacuously is the converse
//! test, whose two lists simply go empty.
//!
//! The genuinely dangerous failure is therefore neither: it is a SELECTIVE miss, where the parser
//! loses exactly a verb whose marker the manifest still carries — most plausibly the verb someone
//! just wired. Both tests stay green and `aware app validate` then refuses a node that would have
//! run. [`DISPATCH_ANCHORS`] is what stands against that, which is why it names every verb the
//! bridge dispatches today rather than a sample of them.
//!
//! ## Fail closed, not open
//!
//! [`dispatched_verbs`] therefore returns `Err` — never a short set — for every shape it cannot
//! read with confidence: no `switch (verb)`, no matching brace, a nested `switch`, no `default:`
//! label, a `case` label after `default:`, a non-literal `case`, a duplicate, an empty table, or a
//! preprocessor directive anywhere inside the table. Each of those is covered by a named case in
//! [`parser::missing_structure_is_an_error_naming_the_guard_that_fired`], which asserts the error
//! MESSAGE and not merely `is_err()` — two guards were previously "covered" by a case that actually
//! tripped a different guard, leaving both unprotected.
//!
//! Three of those are not hypothetical. Measured against the real `Program.cs`, the first draft of
//! this parser returned `Ok` with a WRONG set for all three: a `case` inside `#if FUTURE` read as
//! dispatched (which would have had this gate demand the marker be deleted from a verb that is not
//! even compiled in — reintroducing #520 on the gate's own advice), a `case` placed after the
//! `default:` arm went missing, and the string literal `"default:"` anywhere in the table truncated
//! it silently.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use serde_yaml::Value;

/// Every verb `aware-tekla` dispatches today.
///
/// The full set, not a sample, and that costs nothing: each is checked by CONTAINMENT, so wiring an
/// eighth verb does not fail this test — only losing or renaming one of these does, which is a
/// deliberate act that should require a deliberate edit here. (An earlier version named three and
/// justified the omission by saying a full set would fight implementing a planned verb. That was
/// simply wrong about `contains`, and it left the parser free to drop any of the other four.)
///
/// This is the floor for the SELECTIVE miss described in the module header — the one failure the
/// structural errors cannot see. It must never be derived from the parser's own output.
const DISPATCH_ANCHORS: [&str; 7] = [
    "bake-scene",
    "close",
    "exec",
    "launch",
    "list-instances",
    "send-status",
    "watch",
];

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("cli crate must live directly below the repository root")
        .to_path_buf()
}

fn tekla_manifest_path() -> PathBuf {
    repository_root().join("20-agents/aeco/engineering/tekla/manifest.yaml")
}

fn bridge_source_path() -> PathBuf {
    repository_root().join("cli-tekla/Program.cs")
}

/// Read a file, naming it on failure — an unnamed `No such file or directory` from this gate reads
/// like a broken environment, which is how a real move of `cli-tekla/` would get shrugged off.
fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// Blank out `//` and `/* … */` comments, preserving everything else verbatim OUTSIDE string
/// literals.
///
/// Without this a commented-out `case "part-list":` — the exact shape a half-reverted
/// implementation leaves behind — would read as a dispatched verb and excuse a missing marker.
/// Comment bytes become spaces rather than being deleted, which preserves the newline count
/// exactly; every index taken afterwards is into this stripped copy and never into the original, so
/// the char-vs-byte shrinkage of a non-ASCII comment cannot desynchronise anything.
///
/// It is NOT string-literal aware, and that is a real limitation rather than a theoretical one:
/// `Program.cs:2343` and `:2573` carry `//` inside string literals (help text and a JSON example)
/// and are truncated by this. Harmless only because both sit ~2,000 lines below the dispatch table
/// and [`switch_body`] never reads that far. Anyone moving this scan needs to know it, because the
/// failure mode would be a silently shortened region.
fn strip_comments(source: &str) -> String {
    let chars: Vec<char> = source.chars().collect();
    let mut out = String::with_capacity(source.len());
    let mut i = 0;
    while i < chars.len() {
        match (chars[i], chars.get(i + 1).copied().unwrap_or('\0')) {
            ('/', '/') => {
                while i < chars.len() && chars[i] != '\n' {
                    out.push(' ');
                    i += 1;
                }
            }
            ('/', '*') => {
                // Consume through the terminator; an unterminated block comment swallows the
                // remainder, which makes the gate fail closed rather than read half a file.
                while i < chars.len() {
                    if chars[i] == '*' && chars.get(i + 1) == Some(&'/') {
                        out.push_str("  ");
                        i += 2;
                        break;
                    }
                    out.push(if chars[i] == '\n' { '\n' } else { ' ' });
                    i += 1;
                }
            }
            _ => {
                out.push(chars[i]);
                i += 1;
            }
        }
    }
    out
}

/// Byte index just past a string literal opening at `quote`, skipping escapes and `@""` verbatim
/// doubling. Errors rather than running to EOF, so an unbalanced literal fails the gate closed.
fn skip_string(s: &str, quote: usize) -> Result<usize, String> {
    let b = s.as_bytes();
    let verbatim = quote > 0 && b[quote - 1] == b'@';
    let mut i = quote + 1;
    while i < b.len() {
        match b[i] {
            b'"' if verbatim && b.get(i + 1) == Some(&b'"') => i += 2,
            b'"' => return Ok(i + 1),
            b'\\' if !verbatim => i += 2,
            b'\n' if !verbatim => break,
            _ => i += 1,
        }
    }
    Err("unterminated string literal in the dispatch table".into())
}

/// Byte index just past a char literal opening at `quote`.
fn skip_char(s: &str, quote: usize) -> Result<usize, String> {
    let b = s.as_bytes();
    let mut i = quote + 1;
    while i < b.len() {
        match b[i] {
            b'\\' => i += 2,
            b'\'' => return Ok(i + 1),
            b'\n' => break,
            _ => i += 1,
        }
    }
    Err("unterminated char literal in the dispatch table".into())
}

/// The text INSIDE the braces of the switch that `from_switch` starts with.
///
/// Brace-counted rather than cut at a substring, and literals are skipped so neither a `{` in a
/// string nor the `{verb}` hole of an interpolated one can unbalance the count. This is what makes
/// the `default:` arm's end the end of the SWITCH — an earlier version took `find('}')`, which on
/// the real source stopped at the interpolation brace inside the diagnostic, so the arm it examined
/// was 12 characters of luck and rewording the message behaviour-neutrally broke the gate.
///
/// Known limit, documented rather than guessed at: an interpolation hole containing its own string
/// literal (`$"{d["k"]}"`) ends the skip early. Nothing in this switch does that, and the effect
/// would be an unbalanced count, i.e. `Err`.
fn switch_body(from_switch: &str) -> Result<&str, String> {
    let open = from_switch
        .find('{')
        .ok_or("`switch (verb)` has no opening brace")?;
    let b = from_switch.as_bytes();
    let mut depth = 0usize;
    let mut i = open;
    while i < b.len() {
        match b[i] {
            b'{' => {
                depth += 1;
                i += 1;
            }
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(&from_switch[open + 1..i]);
                }
                i += 1;
            }
            b'"' => i = skip_string(from_switch, i)?,
            b'\'' => i = skip_char(from_switch, i)?,
            _ => i += 1,
        }
    }
    Err("`switch (verb)` has no matching closing brace".into())
}

/// Offsets of every `needle` that sits at a LABEL position — nothing but whitespace between it and
/// the start of its line.
///
/// This is what stops `"default:"` or `case "x":` appearing inside a string literal (or in any
/// trailing code) from being read as a label. Offsets are always into the full `haystack`, never
/// into a running sub-slice, so a match cannot be mistaken for line-initial because of where a
/// previous search stopped.
fn label_offsets(haystack: &str, needle: &str) -> Vec<usize> {
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(rel) = haystack[from..].find(needle) {
        let at = from + rel;
        let line_start = haystack[..at].rfind('\n').map_or(0, |n| n + 1);
        if haystack[line_start..at].trim().is_empty() {
            out.push(at);
        }
        from = at + needle.len();
    }
    out
}

/// The verbs `aware-tekla` dispatches, read out of its own `switch (verb)`.
///
/// Errors rather than returning a partial set for every shape it cannot read with confidence — see
/// "Fail closed, not open" in the module header for why that direction is the load-bearing one.
fn dispatched_verbs(source: &str) -> Result<BTreeSet<String>, String> {
    let code = strip_comments(source);
    let switch_at = code
        .find("switch (verb)")
        .ok_or("no `switch (verb)` in the bridge source")?;
    let body = switch_body(&code[switch_at..])?;

    // Whether a `case` inside `#if FUTURE` / `#if DEBUG` compiles in is a build-configuration
    // question, and answering it wrong is worse in one specific direction: reading a disabled case
    // as dispatched makes this gate report the verb's `status: planned` marker as WRONG, and a
    // maintainer following that advice deletes the marker from a verb the binary cannot dispatch —
    // #520, reintroduced by its own guard. So refuse to read the table at all.
    if let Some(directive) = body.lines().find(|l| l.trim_start().starts_with('#')) {
        return Err(format!(
            "a preprocessor directive in the dispatch table ({:?}) — whether its `case` labels \
             compile in is a build-configuration question this parser cannot answer",
            directive.trim()
        ));
    }
    // A nested switch would put foreign cases in range. Nothing nests one today (every arm is a
    // single `return Verb(parsed);`, or `return ListInstances();` which takes no argument).
    if body.contains("switch ") {
        return Err(
            "a nested `switch` inside the dispatch table — this parser cannot scope it".into(),
        );
    }

    let default_at = *label_offsets(body, "default:")
        .first()
        .ok_or("`switch (verb)` has no `default:` label")?;
    let (table, arm) = body.split_at(default_at);

    // C# allows `default:` before the last arm, and a `case` after it still dispatches. This parser
    // reads only the labels before `default:`, so such a verb would be invisible — and invisible in
    // the selective direction that both parity tests pass through.
    if let Some(stray) = label_offsets(arm, "case ").first() {
        return Err(format!(
            "a `case` label {stray} bytes after the `default:` arm — it dispatches, but this parser \
             reads only the labels before `default:`"
        ));
    }
    // Now correctly bounded to the end of the switch, so this asks what it means to ask: that the
    // region found really is the dispatch table and not something that merely reads like one.
    if !arm.contains("unknown verb") {
        return Err("the `default:` arm no longer reports an unknown verb".into());
    }

    let mut verbs = BTreeSet::new();
    for at in label_offsets(table, "case ") {
        let rest = &table[at + "case ".len()..];
        let Some(stripped) = rest.strip_prefix('"') else {
            // `case SomeConstant:` — not a literal verb. Flagged rather than skipped: a verb
            // dispatched through a constant would otherwise read as undispatched and be marked
            // planned while it works.
            return Err(format!(
                "non-literal `case` in the dispatch table: {:?}",
                rest.chars().take(40).collect::<String>()
            ));
        };
        let Some(close) = stripped.find('"') else {
            return Err("unterminated string in a `case` label".into());
        };
        let verb = &stripped[..close];
        if !stripped[close + 1..].trim_start().starts_with(':') {
            return Err(format!(
                "`case \"{verb}\"` is not followed by a label colon"
            ));
        }
        if !verbs.insert(verb.to_string()) {
            return Err(format!("`case \"{verb}\"` appears twice"));
        }
    }
    if verbs.is_empty() {
        return Err("the dispatch table contains no `case \"…\":` labels".into());
    }
    Ok(verbs)
}

/// Every declared command, mapped to whether it carries `status: planned`.
///
/// Read as YAML rather than through the crate's `Agent` type so the assertion is about the file as
/// written. `status` defaults to `available` in the deserializer, and a gate about a MISSING marker
/// must not be reading a default that fills the marker in. It also means a misspelled `status:` key
/// reads as unmarked here (the truth) instead of being silently dropped by serde.
fn declared_commands(manifest: &str) -> BTreeMap<String, bool> {
    let manifest: Value = serde_yaml::from_str(manifest).expect("parse the tekla manifest");
    let commands = manifest
        .get("commands")
        .and_then(Value::as_mapping)
        .expect("the tekla manifest declares a `commands:` mapping");
    commands
        .iter()
        .map(|(name, body)| {
            let name = name.as_str().expect("command keys are strings").to_string();
            let planned = body.get("status").and_then(Value::as_str) == Some("planned");
            (name, planned)
        })
        .collect()
}

fn real_dispatched() -> BTreeSet<String> {
    dispatched_verbs(&read(&bridge_source_path())).expect("read the bridge's dispatch table")
}

fn real_declared() -> BTreeMap<String, bool> {
    declared_commands(&read(&tekla_manifest_path()))
}

#[test]
fn dispatch_anchors() {
    let dispatched = real_dispatched();
    for anchor in DISPATCH_ANCHORS {
        assert!(
            dispatched.contains(anchor),
            "the parser did not find `{anchor}` in the bridge's dispatch table. Either the verb was \
             removed from `cli-tekla/Program.cs` — in which case mark it `status: planned` and drop \
             it from DISPATCH_ANCHORS in the same change — or the parser is reading the table \
             wrongly, which is the selective miss both parity tests pass straight through. \
             Found: {dispatched:?}"
        );
    }
}

#[test]
fn every_declared_tekla_command_the_bridge_cannot_dispatch_is_marked_planned() {
    let dispatched = real_dispatched();
    let declared = real_declared();
    let unmarked: Vec<&str> = declared
        .iter()
        .filter(|(name, planned)| !dispatched.contains(name.as_str()) && !**planned)
        .map(|(name, _)| name.as_str())
        .collect();
    assert!(
        unmarked.is_empty(),
        "tekla declares {unmarked:?} as runnable, but `aware-tekla` dispatches none of them — an \
         app using one compiles and then fails at run with `unknown verb`. Add `status: planned` to \
         each (#199, #520), or wire the verb into `cli-tekla/Program.cs`."
    );
}

#[test]
fn every_verb_the_bridge_dispatches_is_declared_and_runnable() {
    let dispatched = real_dispatched();
    let declared = real_declared();

    let undeclared: Vec<&String> = dispatched
        .iter()
        .filter(|verb| !declared.contains_key(verb.as_str()))
        .collect();
    assert!(
        undeclared.is_empty(),
        "`aware-tekla` dispatches {undeclared:?}, which the tekla manifest declares nowhere — \
         working functionality no app author and no `aware agent describe` can see (#520)."
    );

    let wrongly_planned: Vec<&str> = declared
        .iter()
        .filter(|(name, planned)| **planned && dispatched.contains(name.as_str()))
        .map(|(name, _)| name.as_str())
        .collect();
    assert!(
        wrongly_planned.is_empty(),
        "tekla marks {wrongly_planned:?} `status: planned` while `aware-tekla` dispatches them — \
         `E_APP_COMMAND_UNAVAILABLE` will refuse a node that would have run. Delete the marker in \
         the PR that wires a verb."
    );
}

// ── what the marker does, end to end ─────────────────────────────────────────────────────────────

/// A scratch `AWARE_HOME` with the REAL tekla manifest installed.
///
/// One home shared by every probe below, deliberately. With a home per probe, the accepting half of
/// the validate test passed identically when nothing was installed at all — `aware app validate`
/// calls a node whose agent is absent valid (a deliberate file-only verdict) — so it witnessed
/// nothing. Sharing the home means the refusal in the same home is the proof the manifest was read.
fn tekla_home() -> tempfile::TempDir {
    let home = tempfile::tempdir().expect("temporary AWARE_HOME");
    let installed = home.path().join("agents/tekla");
    fs::create_dir_all(&installed).expect("installed agent directory");
    fs::copy(tekla_manifest_path(), installed.join("manifest.yaml")).expect("install the manifest");
    home
}

fn app_dir(body: &str) -> tempfile::TempDir {
    let app = tempfile::tempdir().expect("temporary app directory");
    fs::write(app.path().join("probe.flo"), body).expect("write the app");
    app
}

/// stdout + stderr of `aware <args…> <app dir>` against `home`, with whether it succeeded.
fn run_aware(home: &Path, args: &[&str], app: &Path) -> (bool, String) {
    let out = Command::cargo_bin("aware")
        .expect("aware test binary")
        .env("AWARE_HOME", home)
        .args(args)
        .arg(app)
        .output()
        .expect("run aware");
    let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    (out.status.success(), text)
}

const PLANNED_PROBE: &str = "app: tekla-planned-probe\nversion: 0.1.0\n\
     description: probe a planned verb\nrequires: []\n\
     nodes:\n  - id: parts\n    agent: tekla\n    command: part-list\n";

const DISPATCHED_PROBE: &str = "app: tekla-dispatched-probe\nversion: 0.1.0\n\
     description: probe a dispatched verb\nrequires: []\n\
     nodes:\n  - id: ping\n    agent: tekla\n    command: send-status\n";

/// What the marker is FOR. The two parity tests prove it is present; this proves it bites, and that
/// it bites selectively — which is the half a missing marker and a blanket refusal have in common.
///
/// `part-list` stands in for the 19 because it is read-mode, so `E_APP_COMMAND_UNAVAILABLE` is the
/// only error raised. `insert` would satisfy the assertion too — `validate` collects every issue
/// rather than stopping at the first, so it reports the safety-contract failure AND the availability
/// one — but an assertion that passes amid other errors is a weaker witness than one on a single
/// unambiguous refusal.
#[test]
fn validate_refuses_a_planned_tekla_command_and_accepts_a_dispatched_one() {
    let home = tekla_home();

    let (ok, text) = run_aware(
        home.path(),
        &["app", "validate"],
        app_dir(PLANNED_PROBE).path(),
    );
    assert!(!ok, "a planned command must fail validate; got:\n{text}");
    assert!(
        text.contains("E_APP_COMMAND_UNAVAILABLE"),
        "expected E_APP_COMMAND_UNAVAILABLE; got:\n{text}"
    );

    let (ok, text) = run_aware(
        home.path(),
        &["app", "validate"],
        app_dir(DISPATCHED_PROBE).path(),
    );
    assert!(ok, "a dispatched command must pass validate; got:\n{text}");
    assert!(
        !text.contains("E_APP_COMMAND_UNAVAILABLE"),
        "a dispatched command must not be refused as unavailable; got:\n{text}"
    );
}

/// Compile is the surface whose silence CAUSED #520 — the issue's repro is literally "validate
/// trips on something else, compile succeeds, run dies at the bridge" — so asserting only
/// `validate` would leave the reported symptom uncovered. Install is the other gate that produces a
/// lock.
///
/// The no-lock assertion is not decoration: "refused, but wrote a lock anyway" is a distinct
/// failure from "did not refuse", and the `.lock` is the approved artifact the Run gate keys on.
#[test]
fn compile_and_install_also_refuse_a_planned_tekla_command() {
    let home = tekla_home();

    let app = app_dir(PLANNED_PROBE);
    let (ok, text) = run_aware(home.path(), &["app", "compile"], app.path());
    assert!(!ok, "a planned command must fail compile; got:\n{text}");
    assert!(
        text.contains("E_APP_COMMAND_UNAVAILABLE"),
        "expected E_APP_COMMAND_UNAVAILABLE from compile; got:\n{text}"
    );
    let lock = app.path().join("tekla-planned-probe.lock");
    assert!(
        !lock.exists(),
        "a refused compile must write no lock, but {} exists",
        lock.display()
    );

    let (ok, text) = run_aware(
        home.path(),
        &["app", "install"],
        app_dir(PLANNED_PROBE).path(),
    );
    assert!(!ok, "a planned command must fail install; got:\n{text}");
    assert!(
        text.contains("E_APP_COMMAND_UNAVAILABLE"),
        "expected E_APP_COMMAND_UNAVAILABLE from install; got:\n{text}"
    );
}

/// `close`'s `mode: write` is the most consequential single line this change adds, and nothing else
/// in the suite touches it: the whole `cargo test` run passes with that line deleted, and a
/// misspelled key (`mdoe: write`) is equally silent because `Command` does not deny unknown fields.
///
/// It cannot be inferred either — `is_write_by_convention` matches dotted suffixes plus the two
/// legacy names `insert` and `save-attributes`, so `close` would default to READ. Without the line,
/// any app could terminate a user's Tekla session, discarding unsaved model work, with no safety
/// contract at all.
#[test]
fn close_is_write_mode_so_a_node_calling_it_needs_a_safety_block() {
    let home = tekla_home();

    let unsafe_close = "app: tekla-close-unsafe\nversion: 0.1.0\n\
         description: close with no safety block\nrequires: []\n\
         nodes:\n  - id: shutdown\n    agent: tekla\n    command: close\n";
    let (ok, text) = run_aware(
        home.path(),
        &["app", "validate"],
        app_dir(unsafe_close).path(),
    );
    assert!(
        !ok,
        "close without `safety:` must fail validate; got:\n{text}"
    );
    assert!(
        text.contains("E_APP_WRITE_WITHOUT_SAFETY"),
        "close must be write-mode, so a node calling it without `safety:` is refused. Got:\n{text}"
    );

    let safe_close = "app: tekla-close-safe\nversion: 0.1.0\n\
         description: close with a safety block\nrequires: []\n\
         nodes:\n  - id: shutdown\n    agent: tekla\n    command: close\n    safety:\n\
         \x20     transaction-group: shutdown\n      snapshot: false\n";
    let (ok, text) = run_aware(
        home.path(),
        &["app", "validate"],
        app_dir(safe_close).path(),
    );
    assert!(
        ok,
        "close WITH a `safety:` block must validate — otherwise the test above proves only that \
         something refuses it. Got:\n{text}"
    );
}

/// CHARACTERISATION, not an endorsement: the published examples that this marker stops installing.
///
/// Marking the 19 has a consequence the repo's own artifacts contradict. `detailer-issue-pack.app`
/// is the flagship Detailer example — `30-apps/_examples/README.md` calls it the killer app,
/// `00-vision/manifesto.md` points the Detailer persona at it, and `90-onboarding/detailer.md`
/// opens with `aware app install …/detailer-issue-pack.app`, which is now refused outright.
///
/// Both apps were ALREADY unrunnable: they composed verbs the bridge cannot dispatch, so they
/// compiled and died at run. Refusing them at install is the correct new behaviour and is the whole
/// point of #520. What is not resolved is what the examples and their three references should say
/// instead, which is a content decision for the maintainer — rewrite them onto `exec`, or mark them
/// and their docs as pending implementation.
///
/// So this test documents the state rather than blessing it, per CLAUDE.md's rule for a known defect
/// that is not yet fixed: pin it so it TRIPS when the fix lands. It goes red if an example is fixed,
/// if another planned verb is composed into one, or if a verb is implemented and its marker lifted —
/// each of which should come with this list being updated deliberately.
#[test]
fn published_examples_still_blocked_by_the_planned_markers() {
    const KNOWN_BLOCKED: [(&str, [&str; 4]); 2] = [
        (
            "detailer-issue-pack.app",
            [
                "bolt-list",
                "drawing-export",
                "drawing-issue",
                "nc-export-phase",
            ],
        ),
        ("qa-drawings-to-tekla.app", ["insert", "", "", ""]),
    ];

    let declared = real_declared();
    let examples = repository_root().join("30-apps/_examples");

    for (file, expected) in KNOWN_BLOCKED {
        let app: Value = serde_yaml::from_str(&read(&examples.join(file)))
            .unwrap_or_else(|e| panic!("parse {file}: {e}"));
        let mut blocked = BTreeSet::new();
        collect_planned_tekla_commands(app.get("nodes"), &declared, &mut blocked);

        let want: BTreeSet<String> = expected
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        assert_eq!(
            blocked, want,
            "{file}: the set of planned tekla commands it composes changed. If an example was fixed \
             or a verb implemented, update KNOWN_BLOCKED — and check \
             30-apps/_examples/README.md, 00-vision/manifesto.md and 90-onboarding/detailer.md, \
             which still tell a reader to install it."
        );
    }
}

/// Walk `nodes:` and every `do:` body, collecting tekla commands the manifest marks planned. Mirrors
/// the traversal in `cli/src/validate.rs`, minus the frozen-node skip — a frozen node is not refused,
/// but neither example freezes one, and counting it would understate what a reader hits.
fn collect_planned_tekla_commands(
    nodes: Option<&Value>,
    declared: &BTreeMap<String, bool>,
    out: &mut BTreeSet<String>,
) {
    for node in nodes.and_then(Value::as_sequence).into_iter().flatten() {
        if node.get("agent").and_then(Value::as_str) == Some("tekla")
            && let Some(command) = node.get("command").and_then(Value::as_str)
            && declared.get(command) == Some(&true)
        {
            out.insert(command.to_string());
        }
        collect_planned_tekla_commands(node.get("do"), declared, out);
    }
}

// ── the parser's own negative controls ───────────────────────────────────────────────────────────
//
// Each is a way `dispatched_verbs` could report a wrong set while all three real-file tests stayed
// green. They run against a synthetic fixture, so they constrain the PARSER; `dispatch_anchors` is
// what ties it to the real `Program.cs`.

mod parser {
    use super::*;

    /// A dispatch table plus the SHAPES `Program.cs` has around it: a string switch BEFORE it
    /// (excluded by the start offset) and another AFTER it (excluded by the `default:` cut). Both
    /// matter — with only the leading one, removing the `default:` truncation entirely left every
    /// parser test green and the cut was pinned by nothing.
    ///
    /// One substitution: `case "insert":` / `case "part-list":` are planted in the foreign switches
    /// to collide with real declared commands. No such collision exists today (the real foreign
    /// labels are `all`/`welded`/`bolted`/`assembly` and the four `--flag`s), which is exactly why
    /// the worst case has to be constructed rather than sampled.
    const SOURCE: &str = r#"
        static int Classify(string filter)
        {
            switch ((filter ?? "all").Trim())
            {
                case "all": return 1;
                case "insert": return 2;
            }
        }

        static int Run(string[] args)
        {
            switch (verb)
            {
                case "send-status":
                    return SendStatus(parsed);
                // case "part-list":
                //     return PartList(parsed);
                case "exec":
                    return Exec(parsed);
                default:
                    Console.Error.WriteLine($"aware-tekla: unknown verb '{verb}'. Try --help.");
                    return 2;
            }
        }

        static int Export(string format)
        {
            switch (format)
            {
                case "pdf": return 1;
                case "part-list": return 2;
                default: return 0;
            }
        }
    "#;

    fn verbs(source: &str) -> BTreeSet<String> {
        dispatched_verbs(source).expect("parse the sample")
    }

    fn expect_set(names: &[&str]) -> BTreeSet<String> {
        names.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn reads_only_the_verb_switch() {
        assert_eq!(
            verbs(SOURCE),
            expect_set(&["exec", "send-status"]),
            "a case from another switch, or a commented-out one, was counted as a dispatched verb"
        );
    }

    #[test]
    fn a_commented_out_case_is_not_dispatched() {
        // `// case "part-list":` sits INSIDE the verb switch, so scoping alone does not exclude it.
        // Only comment-stripping does, and without it the planned marker on `part-list` would read
        // as a bug.
        assert!(!verbs(SOURCE).contains("part-list"));
        let block = SOURCE.replace(
            "// case \"part-list\":",
            "/* case \"part-list\": return PartList(parsed); */",
        );
        assert!(!verbs(&block).contains("part-list"));
    }

    #[test]
    fn a_case_from_a_foreign_switch_is_not_dispatched() {
        // `insert` precedes the verb switch and `part-list` follows it — the two geometries the real
        // file has, and both planted to collide with a real planned command.
        let found = verbs(SOURCE);
        assert!(!found.contains("insert"), "leading foreign switch leaked");
        assert!(!found.contains("all"), "leading foreign switch leaked");
        assert!(
            !found.contains("part-list"),
            "trailing foreign switch leaked"
        );
        assert!(!found.contains("pdf"), "trailing foreign switch leaked");
    }

    /// The string literal `"default:"` anywhere in the table must not truncate it. The earlier
    /// version cut the table at the first `"default:"` SUBSTRING, so this returned `Ok` with
    /// `watch` — the last arm — silently missing, and no anchor covered the table's tail. Labels are
    /// now recognised only at a line-initial position, so the literal is inert and the real label is
    /// still found.
    #[test]
    fn a_default_colon_inside_a_string_literal_does_not_truncate_the_table() {
        let planted = SOURCE.replace(
            "return SendStatus(parsed);",
            "return SendStatus(parsed, \"default:\");",
        );
        assert_ne!(planted, SOURCE, "the fixture moved — fix this test");
        assert_eq!(verbs(&planted), expect_set(&["exec", "send-status"]));
    }

    /// A reworded diagnostic must NOT break the gate. The earlier version scoped the `default:` arm
    /// with `find('}')`, which stopped at the `{verb}` interpolation brace — so moving `unknown
    /// verb` after the hole made all three real-file tests fail on a change that altered nothing.
    #[test]
    fn rewording_the_unknown_verb_message_does_not_break_the_gate() {
        let reworded = SOURCE.replace(
            r#"unknown verb '{verb}'. Try --help."#,
            r#"'{verb}' is an unknown verb. Try --help."#,
        );
        assert_ne!(
            reworded, SOURCE,
            "the fixture's diagnostic moved — fix this test"
        );
        assert_eq!(verbs(&reworded), expect_set(&["exec", "send-status"]));
    }

    /// Measured against the real `Program.cs`, the first draft returned `Ok` with a WRONG set for
    /// the first three of these rather than erroring.
    #[test]
    fn missing_structure_is_an_error_naming_the_guard_that_fired() {
        // Asserting the MESSAGE, not just `is_err()`: two cases here previously tripped a different
        // guard than the one they were named for (a missing `switch (verb)` fell through to the
        // nested-switch check), so deleting either guard left the whole file green.
        let cases: [(&str, String, &str); 8] = [
            (
                "a `case` inside `#if`",
                SOURCE.replace(
                    "                case \"exec\":",
                    "#if FUTURE\n                case \"exec\":",
                ),
                "preprocessor directive",
            ),
            (
                "a `case` after the `default:` arm",
                SOURCE.replace(
                    "                    return 2;",
                    "                    return 2;\n                case \"part-list\":\n                    return PartList(parsed);",
                ),
                "after the `default:` arm",
            ),
            (
                "no verb switch",
                SOURCE.replace("switch (verb)", "switch (other)"),
                "no `switch (verb)`",
            ),
            (
                "no default label",
                SOURCE.replace("                default:", "                case \"x\":"),
                "no `default:` label",
            ),
            (
                "no unknown-verb diagnostic",
                SOURCE.replace("unknown verb", "something else"),
                "no longer reports an unknown verb",
            ),
            (
                "an empty table",
                SOURCE
                    .replace("                case \"send-status\":", "")
                    .replace("                case \"exec\":", ""),
                "no `case",
            ),
            (
                "a nested switch",
                SOURCE.replace(
                    "                case \"exec\":",
                    "                switch (inner) { case \"exec\":",
                ),
                "nested `switch`",
            ),
            (
                "a non-literal case",
                SOURCE.replace("                case \"exec\":", "                case ExecVerb:"),
                "non-literal `case`",
            ),
        ];
        for (label, source, expected) in cases {
            assert_ne!(source, SOURCE, "{label}: the mutation changed nothing");
            match dispatched_verbs(&source) {
                Ok(found) => panic!("{label}: expected an error, got {found:?}"),
                Err(message) => assert!(
                    message.contains(expected),
                    "{label}: expected the {expected:?} guard to fire, got {message:?}"
                ),
            }
        }
    }

    #[test]
    fn a_planned_marker_is_read_from_the_file_not_a_default() {
        let declared = declared_commands(
            "commands:\n  wired:\n    lifecycle: single\n    description: x\n  \
             unwired:\n    lifecycle: single\n    status: planned\n    description: y\n",
        );
        assert_eq!(declared.get("wired"), Some(&false));
        assert_eq!(declared.get("unwired"), Some(&true));
        // `status: available` spelled out is not `planned`, and neither is some third value arriving
        // later — which must not be silently read as "marked".
        let explicit = declared_commands(
            "commands:\n  wired:\n    lifecycle: single\n    status: available\n    description: x\n",
        );
        assert_eq!(explicit.get("wired"), Some(&false));
    }
}
