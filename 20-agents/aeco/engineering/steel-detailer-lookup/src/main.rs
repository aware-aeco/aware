use std::collections::HashSet;
use std::env;
use std::fs;
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
    } else {
        ("steel-detailer-aisc", "aisc-360-22.json")
    };

    // Build path to rules file: ~/.aware/agents/<agent-id>/rules/<standard>.json
    let home = env::var("USERPROFILE")
        .or_else(|_| env::var("HOME"))
        .unwrap_or_default();
    let rules_path: PathBuf = [&home, ".aware", "agents", agent_id, "rules", rules_file]
        .iter()
        .collect();

    let rules_json = fs::read_to_string(&rules_path).unwrap_or_else(|e| {
        eprintln!(
            "error: cannot read rules from {}: {}",
            rules_path.display(),
            e
        );
        process::exit(2);
    });

    let db: serde_json::Value =
        serde_json::from_str(&rules_json).unwrap_or_else(|e| {
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
            eprintln!(
                "usage: {} <lookup|describe> [options]",
                bin_name
            );
            eprintln!("  lookup --rule <id>           look up a specific rule by id");
            eprintln!("  lookup --category <cat>      list all rules in a category");
            eprintln!("  lookup --list                list all rule ids");
            eprintln!("  describe                     show agent metadata and category list");
            process::exit(2);
        }
    }
}

fn run_lookup(rules: &[serde_json::Value], args: &[String], _db: &serde_json::Value) {
    let mut rule_id: Option<&str> = None;
    let mut category: Option<&str> = None;
    let mut list_all = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--rule" => {
                i += 1;
                rule_id = args.get(i).map(|s| s.as_str());
            }
            "--category" => {
                i += 1;
                category = args.get(i).map(|s| s.as_str());
            }
            "--list" => {
                list_all = true;
            }
            _ => {}
        }
        i += 1;
    }

    if list_all {
        let ids: Vec<&str> = rules.iter().filter_map(|r| r["id"].as_str()).collect();
        println!("{}", serde_json::to_string_pretty(&ids).unwrap());
        return;
    }

    if let Some(id) = rule_id {
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
                process::exit(1); // exit 1 = not found
            }
        }
        return;
    }

    if let Some(cat) = category {
        let filtered: Vec<&serde_json::Value> = rules
            .iter()
            .filter(|r| r["category"].as_str() == Some(cat))
            .collect();
        if filtered.is_empty() {
            let result = serde_json::json!({ "category": cat, "rules": [], "found": false });
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
            process::exit(1);
        } else {
            let result = serde_json::json!({ "category": cat, "rules": filtered });
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
        }
        return;
    }

    eprintln!("lookup requires --rule <id>, --category <cat>, or --list");
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
