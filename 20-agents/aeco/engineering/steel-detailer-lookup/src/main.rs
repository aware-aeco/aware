//! Deterministic clause/section lookup for the steel-detailer-us / -uk / -eu
//! AWARE agents. One source, three `[[bin]]` names; the binary picks its rule
//! set from `argv[0]`. Built and shipped by `.github/workflows/release.yml`
//! next to `aware` in every install archive.

// CLAUDE.md §Code style: "Errors as data, not exceptions … No `unwrap()`
// outside of tests + main entry." `cli/` gates that with this exact attribute
// (see `cli/src/main.rs`); this crate is a separate cargo workspace under
// `20-agents/`, so nothing here inherited it and six `unwrap()` calls had
// accumulated in `run_lookup` / `run_describe` with every gate green — because
// until now no gate ran on this crate at all.
//
// Same form as `cli/`, for the same reason stated there: `[lints.clippy]`
// applies to every target in the package and would also fire on the unit tests
// below, where CLAUDE.md explicitly permits `unwrap()`. `cfg_attr(not(test), …)`
// encodes the carve-out exactly — the bin targets are linted, the same crate
// compiled under `cfg(test)` is not.
//
// `tests/lint_gates.rs` is this gate's negative control.
#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]

use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::process;

fn main() {
    // Determine agent identity from binary name (argv[0]) so one build serves both agents.
    let bin_name = env::current_exe()
        .ok()
        .and_then(|p| p.file_stem().map(|s| s.to_string_lossy().into_owned()))
        .unwrap_or_else(|| "aware-steel-detailer-us".to_string());

    let (agent_id, rules_file) = if bin_name.contains("-uk") {
        ("steel-detailer-uk", "bs-en-1993-uk.json")
    } else if bin_name.contains("-eu") {
        ("steel-detailer-eu", "bs-en-1993-eu.json")
    } else {
        ("steel-detailer-us", "aisc-360-22.json")
    };

    // Optional section-properties dataset (designation -> weight / depth / area),
    // merged into the rule set under the `sections` category. AISC only today; the
    // UK/EU section tables land with their own files later.
    let shapes_file: Option<&str> = match agent_id {
        "steel-detailer-us" => Some("aisc-shapes-v15.json"),
        _ => None,
    };

    // Rules live under the AWARE home: <AWARE_HOME>/agents/<agent-id>/rules/<standard>.json.
    // Honor AWARE_HOME (the CLI + tests override it for portable / non-default installs,
    // see cli/src/paths.rs); fall back to ~/.aware when it is unset.
    let aware_home: PathBuf = match env::var_os("AWARE_HOME") {
        Some(h) if !h.is_empty() => PathBuf::from(h),
        _ => {
            let home = env::var("USERPROFILE")
                .or_else(|_| env::var("HOME"))
                .unwrap_or_default();
            PathBuf::from(home).join(".aware")
        }
    };
    let rules_path = aware_home
        .join("agents")
        .join(agent_id)
        .join("rules")
        .join(rules_file);

    let rules_json = fs::read_to_string(&rules_path).unwrap_or_else(|e| {
        eprintln!(
            "error: cannot read rules from {}: {}",
            rules_path.display(),
            e
        );
        process::exit(2);
    });

    let db: serde_json::Value = serde_json::from_str(&rules_json).unwrap_or_else(|e| {
        eprintln!("error: invalid rules JSON: {}", e);
        process::exit(2);
    });

    let mut rules: Vec<serde_json::Value> = db["rules"]
        .as_array()
        .unwrap_or_else(|| {
            eprintln!("error: rules JSON has no 'rules' array");
            process::exit(2);
        })
        .clone();

    // Merge the optional section-properties dataset if installed. Missing is fine
    // (the curated rules still work); present-but-invalid is a hard error so we never
    // serve a partial or corrupt section table.
    if let Some(sf) = shapes_file {
        let shapes_path = aware_home
            .join("agents")
            .join(agent_id)
            .join("rules")
            .join(sf);
        // try_exists so a permission/IO error on stat is a hard error, not a silent skip.
        let present = shapes_path.try_exists().unwrap_or_else(|e| {
            eprintln!(
                "error: cannot stat sections file {}: {}",
                shapes_path.display(),
                e
            );
            process::exit(2);
        });
        if present {
            let shapes_json = fs::read_to_string(&shapes_path).unwrap_or_else(|e| {
                eprintln!(
                    "error: cannot read sections from {}: {}",
                    shapes_path.display(),
                    e
                );
                process::exit(2);
            });
            let shapes_db: serde_json::Value =
                serde_json::from_str(&shapes_json).unwrap_or_else(|e| {
                    eprintln!("error: invalid sections JSON: {}", e);
                    process::exit(2);
                });
            let incoming = shapes_db["rules"].as_array().unwrap_or_else(|| {
                eprintln!("error: sections JSON has no 'rules' array");
                process::exit(2);
            });
            // Validate before merging so a bad or hand-edited sections file can never
            // shadow or duplicate a curated connection rule. Owned id set so we don't
            // borrow `rules` across the following `extend`.
            let existing: HashSet<String> = rules
                .iter()
                .filter_map(|r| r["id"].as_str().map(String::from))
                .collect();
            for r in incoming {
                let id = r["id"].as_str().unwrap_or("");
                let cat = r["category"].as_str().unwrap_or("");
                if cat != "sections" || !id.starts_with("section.") {
                    eprintln!(
                        "error: sections file has a non-sections rule: id={:?} category={:?}",
                        id, cat
                    );
                    process::exit(2);
                }
                if existing.contains(id) {
                    eprintln!(
                        "error: sections file id collides with a curated rule: {}",
                        id
                    );
                    process::exit(2);
                }
            }
            rules.extend(incoming.iter().cloned());
        }
    }

    let args: Vec<String> = env::args().skip(1).collect();
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("");

    match cmd {
        "lookup" => run_lookup(&rules, &args[1..], &db),
        "describe" => run_describe(&rules, &db),
        _ => {
            eprintln!("usage: {} <lookup|describe> [options]", bin_name);
            eprintln!("  lookup --rule <id>           look up a specific rule by id");
            eprintln!("  lookup --category <cat>      list all rules in a category");
            eprintln!("  lookup --list                list all rule ids");
            eprintln!(
                "  lookup --json-stdin          read {{rule|category}} as JSON on stdin (AWARE cli transport)"
            );
            eprintln!("  describe                     show agent metadata and category list");
            process::exit(2);
        }
    }
}

/// Inputs for a lookup, sourced from argv flags or (under the AWARE cli
/// transport) a JSON object on stdin.
#[derive(Default)]
struct LookupInputs {
    rule_id: Option<String>,
    category: Option<String>,
    list_all: bool,
}

/// Parse `{ "rule": "...", "category": "...", "list": true }` from stdin. The
/// AWARE cli transport spawns `<binary> lookup --json-stdin` and writes the
/// manifest inputs JSON to stdin (see cli/src/runtime/invoker.rs).
fn read_stdin_inputs() -> LookupInputs {
    let mut buf = String::new();
    if std::io::stdin().read_to_string(&mut buf).is_err() || buf.trim().is_empty() {
        return LookupInputs::default();
    }
    let v: serde_json::Value = serde_json::from_str(&buf).unwrap_or_else(|e| {
        eprintln!("error: --json-stdin payload is not valid JSON: {}", e);
        process::exit(2);
    });
    LookupInputs {
        rule_id: v["rule"].as_str().map(str::to_owned),
        category: v["category"].as_str().map(str::to_owned),
        list_all: v["list"].as_bool().unwrap_or(false),
    }
}

fn run_lookup(rules: &[serde_json::Value], args: &[String], _db: &serde_json::Value) {
    let mut inputs = LookupInputs::default();
    let mut json_mode = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--rule" => {
                i += 1;
                inputs.rule_id = args.get(i).map(|s| s.to_owned());
            }
            "--category" => {
                i += 1;
                inputs.category = args.get(i).map(|s| s.to_owned());
            }
            "--list" => inputs.list_all = true,
            "--json-stdin" => json_mode = true,
            _ => {}
        }
        i += 1;
    }

    // Under the cli transport, inputs arrive as a JSON object on stdin rather
    // than as flags. Stdin values fill in anything not already given on argv.
    if json_mode {
        let stdin = read_stdin_inputs();
        inputs.rule_id = inputs.rule_id.or(stdin.rule_id);
        inputs.category = inputs.category.or(stdin.category);
        inputs.list_all = inputs.list_all || stdin.list_all;
    }

    // `found: false` is a valid typed answer, not a failure. The cli transport
    // treats any non-zero exit as a hard error and never parses stdout, so in
    // --json-stdin mode a not-found result must still exit 0. Standalone shell
    // use keeps exit 1 for not-found so it stays scriptable.
    let not_found_code = if json_mode { 0 } else { 1 };

    if inputs.list_all {
        // Built as a `Value` rather than serialized from `Vec<&str>` so it can go
        // through `print_json` with the rest — `Value::from(&str)` is infallible.
        let ids: serde_json::Value = rules
            .iter()
            .filter_map(|r| r["id"].as_str())
            .map(serde_json::Value::from)
            .collect();
        print_json(&ids);
        return;
    }

    if let Some(id) = inputs.rule_id.as_deref() {
        match find_rule(rules, id) {
            Some(rule) => {
                print_json(rule);
                // exit 0 = found
            }
            None => {
                let not_found = serde_json::json!({
                    "id": id,
                    "rule": null,
                    "value": null,
                    "units": null,
                    "citation": null,
                    "source_quote": null,
                    "found": false
                });
                print_json(&not_found);
                process::exit(not_found_code);
            }
        }
        return;
    }

    if let Some(cat) = inputs.category.as_deref() {
        let filtered = rules_in_category(rules, cat);
        if filtered.is_empty() {
            let result = serde_json::json!({ "category": cat, "rules": [], "found": false });
            print_json(&result);
            process::exit(not_found_code);
        } else {
            let result = serde_json::json!({ "category": cat, "rules": filtered });
            print_json(&result);
        }
        return;
    }

    eprintln!(
        "lookup requires --rule <id>, --category <cat>, or --list (or a JSON {{rule|category}} on stdin with --json-stdin)"
    );
    process::exit(2);
}

fn run_describe(rules: &[serde_json::Value], db: &serde_json::Value) {
    let categories: HashSet<&str> = rules
        .iter()
        .filter_map(|r| r["category"].as_str())
        .collect();
    let mut cat_list: Vec<&str> = categories.into_iter().collect();
    cat_list.sort_unstable();

    let info = serde_json::json!({
        "agent": db["agent"],
        "standard": db["standard"],
        "version": db["version"],
        "last_verified": db["last_verified"],
        "rule_count": rules.len(),
        "categories": cat_list
    });
    print_json(&info);
}

/// Print `value` as pretty JSON on stdout, or report the failure and exit 2.
///
/// Every result this binary emits goes through here. `to_string_pretty` returns
/// a `Result`, and the six call sites used to `unwrap()` it — a panic message
/// with no context on stderr, and exit code 101 rather than the 2 this binary
/// uses for every other hard error, which the cli transport reads as a failed
/// invocation with an unparseable reason (see `cli/src/runtime/invoker.rs`).
/// Serializing a `Value` is not expected to fail; "not expected to" is exactly
/// the claim the gate at the top of this file no longer takes on trust.
fn print_json(value: &serde_json::Value) {
    match serde_json::to_string_pretty(value) {
        Ok(text) => println!("{text}"),
        Err(e) => {
            eprintln!("error: cannot serialize result to JSON: {e}");
            process::exit(2);
        }
    }
}

/// Find a rule by its exact `id`. Pure — the curated connection rules and the merged
/// `sections` table live in one slice, so this serves both. Unit-tested below.
fn find_rule<'a>(rules: &'a [serde_json::Value], id: &str) -> Option<&'a serde_json::Value> {
    rules.iter().find(|r| r["id"].as_str() == Some(id))
}

/// All rules in a category — mirrors `lookup --category`. Pure — unit-tested.
fn rules_in_category<'a>(rules: &'a [serde_json::Value], cat: &str) -> Vec<&'a serde_json::Value> {
    rules
        .iter()
        .filter(|r| r["category"].as_str() == Some(cat))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Mimics the post-merge slice: curated connection rules + section rules together.
    fn merged() -> Vec<serde_json::Value> {
        vec![
            json!({"id": "bolt.spacing.min", "category": "bolts", "value": "2.67d"}),
            json!({"id": "section.W16X26", "category": "sections",
                   "value": "26 lb/ft; depth d = 15.7 in; area A = 7.68 in²",
                   "properties": {"type": "W", "weight_plf": 26.0, "depth_in": 15.7}}),
            json!({"id": "section.HSS6X6X3/8", "category": "sections",
                   "properties": {"type": "HSS", "weight_plf": 27.48, "wall_in": 0.349}}),
        ]
    }

    #[test]
    fn finds_section_by_designation_id() {
        let rules = merged();
        let r = find_rule(&rules, "section.W16X26").expect("section present");
        assert_eq!(r["properties"]["weight_plf"], json!(26.0));
        assert_eq!(r["category"], json!("sections"));
    }

    #[test]
    fn finds_hss_weight_not_in_the_designation() {
        // the whole point: HSS weight is not encoded in the name, so it must be looked up
        let rules = merged();
        let r = find_rule(&rules, "section.HSS6X6X3/8").expect("hss present");
        assert_eq!(r["properties"]["weight_plf"], json!(27.48));
    }

    #[test]
    fn missing_rule_is_none() {
        assert!(find_rule(&merged(), "section.NOPE").is_none());
    }

    #[test]
    fn curated_and_section_rules_coexist_after_merge() {
        let rules = merged();
        let cats: HashSet<&str> = rules
            .iter()
            .filter_map(|r| r["category"].as_str())
            .collect();
        assert!(
            cats.contains("bolts"),
            "curated connection rules survive the merge"
        );
        assert!(cats.contains("sections"), "section rules are added");
    }

    #[test]
    fn category_filter_selects_only_that_category() {
        let rules = merged();
        let secs = rules_in_category(&rules, "sections");
        assert_eq!(secs.len(), 2);
        assert!(
            secs.iter()
                .all(|r| r["id"].as_str().unwrap().starts_with("section."))
        );
        assert!(rules_in_category(&rules, "nonexistent").is_empty());
    }
}
