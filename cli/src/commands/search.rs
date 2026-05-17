//! `aware search <term>` — case-insensitive cross-agent fuzzy search over
//! command names + descriptions. Returns the top matches grouped by agent.
//!
//! Use this when you want to find which agents expose a concept (e.g.
//! `aware search "load IFC"` shows xeokit / thatopen / web-ifc / revit
//! variants of loading IFC).

use std::collections::BTreeMap;
use std::time::Instant;

use clap::Args;
use serde::Serialize;

use crate::context::Context;
use crate::envelope;
use crate::error::AwareError;
use crate::manifest::agent::Category;
use crate::manifest::loader::discover_agents;

#[derive(Args, Debug)]
pub struct SearchArgs {
    /// Search term (case-insensitive substring match against command names and descriptions).
    pub term: String,
    /// Maximum results to show per agent (default 5). Use 0 for unlimited.
    #[arg(long, default_value_t = 5)]
    pub limit: usize,
    /// Restrict search to a specific agent.
    #[arg(long)]
    pub agent: Option<String>,
    /// Restrict search to curated workflow verbs (hide auto-generated reflected commands).
    #[arg(long)]
    pub curated: bool,
    /// Restrict search to reflected commands (the escape hatch).
    #[arg(long, conflicts_with = "curated")]
    pub reflected: bool,
}

pub fn run(ctx: &Context, args: &SearchArgs) -> Result<(), AwareError> {
    let started = Instant::now();
    let term_lower = args.term.to_lowercase();
    let discovered = discover_agents(&ctx.paths)?;

    let mut results: BTreeMap<String, Vec<Hit>> = BTreeMap::new();

    let category_filter: Option<Category> = if args.curated {
        Some(Category::Curated)
    } else if args.reflected {
        Some(Category::Reflected)
    } else {
        None
    };

    for d in &discovered {
        if let Some(filter) = &args.agent
            && d.manifest.agent != *filter
        {
            continue;
        }
        let mut agent_hits: Vec<Hit> = Vec::new();
        for (name, cmd) in &d.manifest.commands {
            if let Some(want) = category_filter
                && d.manifest.category_of(cmd) != want
            {
                continue;
            }
            let name_match = name.to_lowercase().contains(&term_lower);
            let desc_match = cmd.description.to_lowercase().contains(&term_lower);
            if name_match || desc_match {
                agent_hits.push(Hit {
                    command: name.clone(),
                    description: cmd.description.clone(),
                    in_name: name_match,
                    in_description: desc_match,
                });
            }
        }
        if !agent_hits.is_empty() {
            results.insert(d.manifest.agent.clone(), agent_hits);
        }
    }

    let total: usize = results.values().map(|v| v.len()).sum();

    if ctx.json {
        let data = SearchData {
            term: &args.term,
            total_matches: total,
            limit: args.limit,
            results: results
                .iter()
                .map(|(agent, hits)| AgentResults {
                    agent,
                    match_count: hits.len(),
                    commands: if args.limit == 0 {
                        hits.iter().collect()
                    } else {
                        hits.iter().take(args.limit).collect()
                    },
                })
                .collect(),
        };
        envelope::print_ok("search", data, started).ok();
        return Ok(());
    }

    if results.is_empty() {
        println!("(no matches for '{}')", args.term);
        return Ok(());
    }

    println!(
        "'{}' — {} matches across {} agent(s):",
        args.term,
        total,
        results.len()
    );
    for (agent, hits) in &results {
        let shown = if args.limit == 0 {
            hits.len()
        } else {
            hits.len().min(args.limit)
        };
        let total_hits = hits.len();
        if total_hits > shown {
            println!("\n{agent} ({shown} of {total_hits}):");
        } else {
            println!("\n{agent} ({total_hits}):");
        }
        for hit in hits.iter().take(shown) {
            let desc_one = hit.description.replace('\n', " ");
            let desc_trimmed = if desc_one.len() > 100 {
                format!("{}…", &desc_one[..97])
            } else {
                desc_one
            };
            println!("  {} → {}", hit.command, desc_trimmed);
        }
        if total_hits > shown {
            println!("  … {} more (use --limit 0 to see all)", total_hits - shown);
        }
    }
    Ok(())
}

#[derive(Serialize)]
struct Hit {
    command: String,
    description: String,
    in_name: bool,
    in_description: bool,
}

#[derive(Serialize)]
struct SearchData<'a> {
    term: &'a str,
    total_matches: usize,
    limit: usize,
    results: Vec<AgentResults<'a>>,
}

#[derive(Serialize)]
struct AgentResults<'a> {
    agent: &'a str,
    match_count: usize,
    commands: Vec<&'a Hit>,
}
