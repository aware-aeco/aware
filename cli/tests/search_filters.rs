//! `aware search` — the selection half of the command: which hits survive
//! `--agent` / `--curated` / `--reflected`, how `--limit` trims them, and what
//! the JSON surface reports about each one.
//!
//! `tests/search_multibyte.rs` covers the rendering half (truncating a
//! description without splitting a UTF-8 character). Nothing covered the
//! filtering: every predicate in `src/commands/search.rs::run` — the agent
//! filter, the category filter, the `limit == 0` special case, the
//! `in_name`/`in_description` provenance flags and the `total_matches` count
//! that must stay whole while the per-agent list is trimmed — could be
//! inverted or deleted without a test going red.
//!
//! The fixture is deliberately built so that the *effective* category of a
//! command is derived a different way on each agent, because
//! `Agent::category_of` has two branches and a filter that only consulted the
//! explicit `category:` key would still look correct against a one-agent
//! fixture:
//!
//! | agent         | default (from provenance) | command             | explicit | effective |
//! |---------------|---------------------------|---------------------|----------|-----------|
//! | `alpha-agent` | curated (no provenance)   | `load-model`        | —        | curated   |
//! | `alpha-agent` |                           | `raw-api-call`      | reflected| reflected |
//! | `beta-agent`  | reflected (generated-by)  | `fetch-model`       | —        | reflected |
//! | `beta-agent`  |                           | `sync-model`        | curated  | curated   |
//!
//! So each of `--curated` / `--reflected` must pick one command per agent, and
//! must reach it through a different branch on each.

use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;

/// Search term used by every test below. Matches all four fixture commands, but
/// through different columns: `load-model` by name only, `raw-api-call` by
/// description only (and only case-insensitively — the fixture spells it
/// `MODEL`), `fetch-model` by name only, `sync-model` by both.
const TERM: &str = "model";

fn write_fixture(home: &std::path::Path) {
    let alpha = "agent: alpha-agent\n\
         version: 0.1.0\n\
         description: Hand-written agent; commands default to curated.\n\
         stateful: false\n\
         license: Apache-2.0\n\
         transport:\n  \
           cli:\n    \
             binary: aware-fixture\n\
         commands:\n  \
           load-model:\n    \
             lifecycle: single\n    \
             description: Load an IFC file into the session.\n  \
           raw-api-call:\n    \
             lifecycle: single\n    \
             category: reflected\n    \
             description: Escape hatch for the raw MODEL API.\n";

    // `provenance.generated-by` is what flips this agent's default category to
    // reflected (`Agent::default_category`), so `fetch-model` below is reflected
    // without saying so itself.
    let beta = "agent: beta-agent\n\
         version: 0.1.0\n\
         description: Machine-generated agent; commands default to reflected.\n\
         stateful: false\n\
         license: Apache-2.0\n\
         provenance:\n  \
           generated-by: aware build --from-openapi\n\
         transport:\n  \
           cli:\n    \
             binary: aware-fixture\n\
         commands:\n  \
           fetch-model:\n    \
             lifecycle: single\n    \
             description: Fetch one record by id.\n  \
           sync-model:\n    \
             lifecycle: single\n    \
             category: curated\n    \
             description: Sync the model with upstream.\n";

    for (id, manifest) in [("alpha-agent", alpha), ("beta-agent", beta)] {
        let dir = home.join("agents").join(id);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("manifest.yaml"), manifest).unwrap();
    }
}

/// Run `aware search <args…>` against a fresh fixture home and return stdout.
/// The `TempDir` must outlive the run, so it is created and dropped here.
fn search(args: &[&str]) -> String {
    let tmp = tempfile::tempdir().unwrap();
    write_fixture(tmp.path());
    let out = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .arg("search")
        .args(args)
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    String::from_utf8(out).expect("stdout must be valid UTF-8")
}

/// As [`search`], with `--json`, parsed and unwrapped to the `data` object.
fn search_json(args: &[&str]) -> Value {
    let mut with_json = args.to_vec();
    with_json.push("--json");
    let raw = search(&with_json);
    let env: Value = serde_json::from_str(&raw).expect("search --json must emit one JSON object");
    assert_eq!(env["ok"], true, "envelope reported failure: {raw}");
    env["data"].clone()
}

/// Command ids in the order the JSON surface lists them, flattened across
/// agents, each prefixed with its agent so a cross-agent leak is visible.
fn qualified_hits(data: &Value) -> Vec<String> {
    data["results"]
        .as_array()
        .expect("results array")
        .iter()
        .flat_map(|agent| {
            let name = agent["agent"].as_str().expect("agent id").to_string();
            agent["commands"]
                .as_array()
                .expect("commands array")
                .iter()
                .map(move |c| format!("{name}/{}", c["command"].as_str().expect("command name")))
        })
        .collect()
}

#[test]
fn unfiltered_search_finds_every_command_on_both_agents() {
    let data = search_json(&[TERM]);
    assert_eq!(
        qualified_hits(&data),
        vec![
            "alpha-agent/load-model",
            "alpha-agent/raw-api-call",
            "beta-agent/fetch-model",
            "beta-agent/sync-model",
        ],
        "baseline for every filter below"
    );
    assert_eq!(data["total_matches"], 4);
}

#[test]
fn agent_filter_excludes_the_other_agent_entirely() {
    let data = search_json(&[TERM, "--agent", "beta-agent"]);
    assert_eq!(
        qualified_hits(&data),
        vec!["beta-agent/fetch-model", "beta-agent/sync-model"],
    );
    // The count is derived after filtering, so a filter that only suppressed
    // *rendering* would still leak here.
    assert_eq!(data["total_matches"], 2);
    assert_eq!(
        data["results"].as_array().unwrap().len(),
        1,
        "alpha-agent must not appear as an empty group either"
    );
}

#[test]
fn agent_filter_matching_nothing_yields_no_results() {
    let data = search_json(&[TERM, "--agent", "no-such-agent"]);
    assert_eq!(data["total_matches"], 0);
    assert!(data["results"].as_array().unwrap().is_empty());
}

#[test]
fn curated_filter_keeps_only_curated_commands_on_both_agents() {
    // `load-model` is curated by the absent-provenance default; `sync-model` is
    // curated by an explicit key against a reflected default. Both branches of
    // `Agent::category_of` are exercised, in opposite directions.
    let data = search_json(&[TERM, "--curated"]);
    assert_eq!(
        qualified_hits(&data),
        vec!["alpha-agent/load-model", "beta-agent/sync-model"],
    );
    assert_eq!(data["total_matches"], 2);
}

#[test]
fn reflected_filter_keeps_only_reflected_commands_on_both_agents() {
    // The exact complement of the curated case — so a filter stuck on one
    // category, or one that ignored the flag entirely, fails one of the two.
    let data = search_json(&[TERM, "--reflected"]);
    assert_eq!(
        qualified_hits(&data),
        vec!["alpha-agent/raw-api-call", "beta-agent/fetch-model"],
    );
    assert_eq!(data["total_matches"], 2);
}

#[test]
fn curated_and_reflected_cannot_be_combined() {
    // Declared via `conflicts_with` on `SearchArgs::reflected`. Without it the
    // pair silently resolves to `--curated`, because `run` tests that flag
    // first — a wrong answer dressed as a successful search.
    let tmp = tempfile::tempdir().unwrap();
    write_fixture(tmp.path());
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["search", TERM, "--curated", "--reflected"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--curated"));
}

#[test]
fn limit_trims_each_agent_but_not_the_reported_total() {
    let data = search_json(&[TERM, "--limit", "1"]);
    assert_eq!(
        qualified_hits(&data),
        vec!["alpha-agent/load-model", "beta-agent/fetch-model"],
        "one hit per agent, not one hit overall"
    );
    // `total_matches` and `match_count` describe what was *found*; only the
    // `commands` list is trimmed. Conflating them would under-report the corpus.
    assert_eq!(data["total_matches"], 4);
    for agent in data["results"].as_array().unwrap() {
        assert_eq!(agent["match_count"], 2, "agent: {}", agent["agent"]);
    }
}

#[test]
fn limit_zero_means_unlimited_rather_than_nothing() {
    let data = search_json(&[TERM, "--limit", "0"]);
    assert_eq!(qualified_hits(&data).len(), 4);
    assert_eq!(data["limit"], 0);

    let text = search(&[TERM, "--limit", "0"]);
    assert!(
        !text.contains("more (use --limit 0"),
        "nothing was withheld, so nothing should be advertised as withheld:\n{text}"
    );
}

#[test]
fn text_output_names_the_withheld_hits_and_the_group_sizes() {
    let trimmed = search(&[TERM, "--limit", "1"]);
    assert!(
        trimmed.contains("alpha-agent (1 of 2):"),
        "header must say how much of the group is shown:\n{trimmed}"
    );
    assert!(
        trimmed.contains("… 1 more (use --limit 0 to see all)"),
        "withheld hits must be advertised:\n{trimmed}"
    );
    assert!(
        !trimmed.contains("raw-api-call"),
        "the withheld hit must not be printed anyway:\n{trimmed}"
    );

    // With nothing withheld the header carries the bare count and no remainder
    // line — otherwise "(2 of 2)" and "… 0 more" would leak out.
    let whole = search(&[TERM]);
    assert!(
        whole.contains("alpha-agent (2):"),
        "un-trimmed group must not be rendered as an 'N of M' split:\n{whole}"
    );
    assert!(!whole.contains("more (use --limit 0"), "{whole}");
    assert!(
        whole.contains("'model' — 4 matches across 2 agent(s):"),
        "summary line must count hits and agents:\n{whole}"
    );
}

#[test]
fn match_flags_record_which_column_matched() {
    let data = search_json(&[TERM]);
    let mut seen = 0;
    for agent in data["results"].as_array().unwrap() {
        for cmd in agent["commands"].as_array().unwrap() {
            let (in_name, in_desc) = (&cmd["in_name"], &cmd["in_description"]);
            let expected = match cmd["command"].as_str().unwrap() {
                // "load-model" contains the term; its description does not.
                "load-model" => (true, false),
                // Only the description matches, and only case-insensitively —
                // the fixture spells it "MODEL".
                "raw-api-call" => (false, true),
                "fetch-model" => (true, false),
                "sync-model" => (true, true),
                other => panic!("unexpected command in results: {other}"),
            };
            assert_eq!(
                (in_name.as_bool().unwrap(), in_desc.as_bool().unwrap()),
                expected,
                "flags for {}",
                cmd["command"]
            );
            seen += 1;
        }
    }
    assert_eq!(seen, 4, "every fixture command must have been checked");
}

#[test]
fn a_term_matching_nothing_says_so_instead_of_printing_an_empty_report() {
    let text = search(&["zzz-no-such-concept"]);
    assert!(
        text.contains("(no matches for 'zzz-no-such-concept' among 2 installed agent(s))"),
        "the empty case has its own message, not a 0-count report:\n{text}"
    );

    let data = search_json(&["zzz-no-such-concept"]);
    assert_eq!(data["total_matches"], 0);
    assert!(data["results"].as_array().unwrap().is_empty());
    // A miss still reports the corpus it missed in: `total_matches: 0` alone
    // cannot tell a consumer "nothing does this" from "nothing here does this".
    assert_eq!(data["searched_agents"], 2);
    assert_eq!(data["scope"], "installed");
}

// ── Scope reporting (#495) ───────────────────────────────────────────────────
//
// `aware search` reads installed agents only; `aware agent search` reads the
// registry catalogue. #495 was filed because the first answered a capability
// question "no" without ever saying which corpus it had consulted — the
// commands it was declared missing (`outlook.mail.send`, `gmail.send`) existed
// all along, in agents that were not installed on the reporting machine.
//
// The note is asserted on the HIT path as well as the miss, because the report
// that prompted this was filed off a search that found four unrelated commands.
// A test covering only the empty case would leave the actual failure uncovered.

/// The suggested follow-up command, which must name the catalogue verb rather
/// than repeat this one.
const CATALOG_HINT: &str = "aware agent search";

#[test]
fn a_hit_still_reports_which_corpus_was_searched() {
    let text = search(&[TERM]);
    assert!(
        text.contains("Searched 2 installed agent(s)."),
        "a result that found something must still name its corpus:\n{text}"
    );
    assert!(
        text.contains("Only INSTALLED agents are searched."),
        "the scope caveat must not be conditional on an empty result:\n{text}"
    );
    assert!(
        text.contains(&format!("{CATALOG_HINT} {TERM}")),
        "the caveat must name the command that searches the catalogue:\n{text}"
    );
}

#[test]
fn a_miss_points_at_the_catalogue_instead_of_ending_the_search() {
    let text = search(&["zzz-no-such-concept"]);
    assert!(
        text.contains(&format!("{CATALOG_HINT} zzz-no-such-concept")),
        "a miss is exactly when the other corpus matters most:\n{text}"
    );
}

#[test]
fn an_empty_home_is_distinguished_from_a_genuine_miss() {
    // "nothing matched" and "there was nothing to match against" are different
    // answers. Collapsing them is how an unpopulated home reads as proof that a
    // capability does not exist anywhere.
    let tmp = tempfile::tempdir().unwrap(); // no fixture written: zero agents
    let out = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["search", TERM])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let text = String::from_utf8(out).unwrap();
    assert!(
        text.contains("no installed agents to search"),
        "an empty home must say so, not report a plain miss:\n{text}"
    );
    assert!(
        !text.contains("among 0 installed agent(s)"),
        "the zero case must not fall through to the counted phrasing:\n{text}"
    );
    assert!(text.contains(CATALOG_HINT), "{text}");
}

#[test]
fn an_agent_filter_matching_nothing_is_not_reported_as_an_empty_home() {
    // `--agent` skips every discovered agent before the counter, so
    // `searched_agents == 0` here even though two agents ARE installed. Keying
    // the empty-home message off that counter alone told a user with a typo'd
    // `--agent` that they had no agents at all. (Codex review, PR #497.)
    let text = search(&[TERM, "--agent", "no-such-agent"]);
    assert!(
        text.contains("no agent named 'no-such-agent' is installed"),
        "a missing filter target must be named as such:\n{text}"
    );
    assert!(
        text.contains("2 other agent(s) are"),
        "and must say the home is not in fact empty:\n{text}"
    );
    assert!(
        !text.contains("no installed agents to search"),
        "the empty-home diagnosis is reserved for an actually empty home:\n{text}"
    );
}

#[test]
fn a_hyphen_prefixed_term_gets_the_end_of_options_delimiter() {
    // `aware agent search` takes `query` as an ordinary positional, so clap
    // rejects a leading hyphen as an unknown option. Quoting cannot fix that —
    // the shell strips quotes before clap ever sees the argv — so the printed
    // command needs `--`. Without it the hint advertises a command that errors
    // out. (Codex review, PR #497.)
    let text = search(&["--", "--send"]);
    assert!(
        text.contains(&format!("{CATALOG_HINT} -- --send")),
        "a hyphen-prefixed term needs the end-of-options delimiter:\n{text}"
    );
    // A normal term must NOT pick up the delimiter.
    assert!(
        !search(&[TERM]).contains(&format!("{CATALOG_HINT} --")),
        "the delimiter must not leak onto ordinary terms"
    );
}

#[test]
fn the_scope_count_follows_the_agent_filter_rather_than_the_install_set() {
    // Counted after `--agent`, so a narrowed search cannot imply it swept
    // everything installed. Two agents exist; one was consulted.
    let text = search(&[TERM, "--agent", "beta-agent"]);
    assert!(
        text.contains("Searched 1 installed agent(s)."),
        "the count must describe what was scanned, not what is installed:\n{text}"
    );
    assert_eq!(
        search_json(&[TERM, "--agent", "beta-agent"])["searched_agents"],
        1
    );

    // The unnarrowed baseline, so a count hard-coded to 1 fails here.
    assert_eq!(search_json(&[TERM])["searched_agents"], 2);
}

#[test]
fn an_awkward_term_is_described_rather_than_shell_quoted() {
    // The repo's settled rule (#443, re-applied here as #497): never emit a
    // command line whose quoting is correct in one shell and wrong in another.
    // POSIX `'load model'` is not how cmd.exe groups, and `'it'\''s'` is not how
    // PowerShell escapes — so an awkward term gets described, not quoted.
    for awkward in ["load model", "it's", "a|b", "@team"] {
        let text = search(&[awkward]);
        assert!(
            text.contains("quote it for your shell"),
            "{awkward:?} must be described, not handed a fake-portable command:\n{text}"
        );
        assert!(
            !text.contains(&format!("{CATALOG_HINT} '{awkward}'")),
            "{awkward:?} must not be POSIX-quoted into a runnable-looking line:\n{text}"
        );
    }

    // A plain single word still gets the real, pasteable command — the
    // conservative branch must not swallow the common case. Asserted against the
    // hint LINE, not the whole output, since the summary line legitimately
    // renders the term in quotes.
    let plain = search(&[TERM]);
    assert!(
        plain.contains(&format!("{CATALOG_HINT} {TERM}")),
        "a bare term must still get a runnable command:\n{plain}"
    );
    assert!(
        !plain.contains("quote it for your shell"),
        "a bare term must not be pushed down the descriptive branch:\n{plain}"
    );
}

#[test]
fn json_reports_scope_without_the_prose() {
    let data = search_json(&[TERM]);
    assert_eq!(data["searched_agents"], 2);
    assert_eq!(data["scope"], "installed");

    // The machine surface must stay clean: the human caveat is prose for the
    // text path only, and leaking it into stdout would break JSON parsing —
    // which `search_json` already proves by parsing, so assert the fields are
    // the carrier instead.
    assert!(
        data["results"].is_array() && data["term"] == TERM,
        "unexpected JSON shape: {data}"
    );
}

/// The envelope frame `search --json` prints, including the two `meta` keys
/// whose serde renames (`cli-version`, `duration-ms`) nothing else asserts. A
/// rename regression here breaks every scripted consumer of `--json`, and the
/// unit test that used to live in `src/envelope.rs` could not catch it: it
/// hand-built an `Envelope`, serialised it and read the same literals back,
/// never touching `print_ok` or `meta_for`.
#[test]
fn json_output_is_wrapped_in_the_spec_envelope() {
    let tmp = tempfile::tempdir().unwrap();
    write_fixture(tmp.path());
    let out = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["search", TERM, "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let env: Value = serde_json::from_slice(&out).expect("one JSON object on stdout");
    assert_eq!(env["ok"], true);
    assert!(env["error"].is_null(), "success must carry a null error");
    assert_eq!(env["meta"]["command"], "search");
    assert_eq!(
        env["meta"]["cli-version"],
        env!("CARGO_PKG_VERSION"),
        "meta must report the running binary's version under the hyphenated key"
    );
    assert!(
        env["meta"]["duration-ms"].is_u64(),
        "duration-ms must be a number under the hyphenated key, got {}",
        env["meta"]["duration-ms"]
    );
    assert_eq!(env["data"]["term"], TERM);
}
