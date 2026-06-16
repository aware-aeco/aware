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
        .unwrap_or_else(|| "aware-steel-detailer-aisc".to_string());

    let (agent_id, rules_file) = if bin_name.contains("-uk") {
        ("steel-detailer-uk", "bs-en-1993-uk.json")
    } else if bin_name.contains("-eu") {
        ("steel-detailer-eu", "bs-en-1993-eu.json")
    } else {
        ("steel-detailer-aisc", "aisc-360-22.json")
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

    let rules = db["rules"].as_array().unwrap_or_else(|| {
        eprintln!("error: rules JSON has no 'rules' array");
        process::exit(2);
    });

    let args: Vec<String> = env::args().skip(1).collect();
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("");

    match cmd {
        "lookup" => run_lookup(rules, &args[1..], &db),
        "describe" => run_describe(rules, &db),
        _ => {
            eprintln!("usage: {} <lookup|describe> [options]", bin_name);
            eprintln!("  lookup --rule <id>           look up a specific rule by id");
            eprintln!("  lookup --category <cat>      list all rules in a category");
            eprintln!("  lookup --list                list all rule ids");
            eprintln!("  lookup --json-stdin          read {{rule|category}} as JSON on stdin (AWARE cli transport)");
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
        let ids: Vec<&str> = rules.iter().filter_map(|r| r["id"].as_str()).collect();
        println!("{}", serde_json::to_string_pretty(&ids).unwrap());
        return;
    }

    if let Some(id) = inputs.rule_id.as_deref() {
        match rules.iter().find(|r| r["id"].as_str() == Some(id)) {
            Some(rule) => {
                println!("{}", serde_json::to_string_pretty(rule).unwrap());
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
                println!("{}", serde_json::to_string_pretty(&not_found).unwrap());
                process::exit(not_found_code);
            }
        }
        return;
    }

    if let Some(cat) = inputs.category.as_deref() {
        let filtered: Vec<&serde_json::Value> = rules
            .iter()
            .filter(|r| r["category"].as_str() == Some(cat))
            .collect();
        if filtered.is_empty() {
            let result = serde_json::json!({ "category": cat, "rules": [], "found": false });
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
            process::exit(not_found_code);
        } else {
            let result = serde_json::json!({ "category": cat, "rules": filtered });
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
        }
        return;
    }

    eprintln!("lookup requires --rule <id>, --category <cat>, or --list (or a JSON {{rule|category}} on stdin with --json-stdin)");
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
    println!("{}", serde_json::to_string_pretty(&info).unwrap());
}
