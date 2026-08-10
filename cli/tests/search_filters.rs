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
    assert_eq!(
        text.trim(),
        "(no matches for 'zzz-no-such-concept')",
        "the empty case has its own message, not a 0-count report"
    );

    let data = search_json(&["zzz-no-such-concept"]);
    assert_eq!(data["total_matches"], 0);
    assert!(data["results"].as_array().unwrap().is_empty());
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
