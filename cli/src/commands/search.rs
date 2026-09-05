//! `aware search <term>` — case-insensitive cross-agent fuzzy search over
//! command names + descriptions. Returns the top matches grouped by agent.
//!
//! Use this when you want to find which agents expose a concept (e.g.
//! `aware search "load IFC"` shows xeokit / thatopen / web-ifc / revit
//! variants of loading IFC).
//!
//! **Scope, and why every result says so.** This verb reads
//! `<aware_home>/agents/` — the INSTALLED agents — and nothing else. The
//! registry catalogue of not-yet-installed agents is a separate corpus behind
//! `aware agent search`. Two commands whose names differ by one word search
//! two different worlds, and the narrower one is the shorter to type.
//!
//! Left unsaid, that produced a confident false negative (#495): `aware search
//! mail` returned four unrelated hits from the agents that happened to be
//! installed, and the absence of `outlook.mail.send` / `gmail.send` from the
//! output was read as those commands not existing. They do exist, and always
//! did — `microsoft-365` and `google-workspace` were simply not installed on
//! the machine that ran the search. A capability question was answered "no"
//! by a command that had never looked at the corpus holding the answer.
//!
//! So the scope is printed on every text result, hit or miss, and carried in
//! `--json` as `searched_agents` / `scope`. The miss case is not the only one
//! that misleads: the report above was filed off a search that *found*
//! something, which is why the note is not conditional on an empty result.

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::time::Instant;

use clap::Args;
use serde::Serialize;

use crate::context::Context;
use crate::envelope;
use crate::error::AwareError;
use crate::manifest::agent::Category;
use crate::manifest::loader::discover_agents;
use crate::text;

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

    // Agents actually scanned, counted after the `--agent` filter rather than
    // before it: a `--agent foo` run must report the one agent it consulted,
    // not imply it swept the whole installed set.
    let mut searched_agents = 0usize;

    for d in &discovered {
        if let Some(filter) = &args.agent
            && d.manifest.agent != *filter
        {
            continue;
        }
        searched_agents += 1;
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
            searched_agents,
            scope: SCOPE,
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
        // Three distinct answers, not one. "Nothing matched", "there was nothing
        // to match against", and "the agent you named isn't installed" send the
        // reader somewhere different, and collapsing them is how an empty result
        // reads as proof that a capability does not exist.
        //
        // `searched_agents == 0` alone does NOT mean an empty home: a `--agent`
        // that matches nothing skips every discovered agent before the counter,
        // so the install set has to be consulted separately to tell the two
        // apart.
        match (searched_agents, discovered.is_empty(), &args.agent) {
            (0, false, Some(filter)) => println!(
                "(no agent named '{filter}' is installed — {} other agent(s) are)",
                discovered.len()
            ),
            (0, _, _) => println!(
                "(no matches for '{}' — no installed agents to search)",
                args.term
            ),
            _ => println!(
                "(no matches for '{}' among {searched_agents} installed agent(s))",
                args.term
            ),
        }
        print_scope_note(&args.term);
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
            // Characters, not bytes. This was `&desc_one[..97]`, which aborts the
            // process when byte 97 lands inside a multi-byte character — four
            // descriptions already in `20-agents/` do exactly that (see
            // `crate::text`). Descriptions are author-written prose; typographic
            // quotes and accents in them are ordinary, not exotic.
            println!("  {} → {}", hit.command, text::ellipsize(&desc_one, 97));
        }
        if total_hits > shown {
            println!("  … {} more (use --limit 0 to see all)", total_hits - shown);
        }
    }
    println!("\nSearched {searched_agents} installed agent(s).");
    print_scope_note(&args.term);
    Ok(())
}

/// What this verb did NOT look at, and the command that does.
///
/// Printed on hits as well as misses. A result that lists four commands looks
/// complete in a way an empty one does not, so the case that most needs the
/// caveat is the one that found something — see the module header.
fn print_scope_note(term: &str) {
    // A term starting with `-` needs the `--` end-of-options delimiter, not
    // quoting: `aware agent search` takes `query` as an ordinary positional, so
    // clap reads a leading hyphen as an option and rejects it before any value
    // parsing happens. Quotes are consumed by the shell and never reach clap, so
    // they cannot fix this — the delimiter is what makes the printed command
    // pasteable. (`aware search -- --send` is itself reachable the same way.)
    let delimiter = if term.starts_with('-') { "-- " } else { "" };
    println!(
        "Only INSTALLED agents are searched. To search every agent in the registry\n\
         catalog, including ones not installed here:\n  \
         aware agent search {delimiter}{}",
        shell_quote(term)
    );
}

/// Quote `term` so the suggested command can be pasted into a POSIX shell
/// verbatim.
///
/// `aware search "load IFC"` is a documented multi-word use, and
/// `aware agent search` takes the query as ONE positional — so echoing a
/// multi-word term bare would print advice that fails when followed. Only
/// wraps when the term is not already a plain word, to keep the common
/// single-word hint free of noise.
fn shell_quote(term: &str) -> Cow<'_, str> {
    let plain = !term.is_empty()
        && term
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '/' | ':' | '@'));
    if plain {
        return Cow::Borrowed(term);
    }
    // POSIX single-quoting: everything is literal inside '…', and an embedded
    // single quote is spelled by closing, escaping one, and reopening.
    Cow::Owned(format!("'{}'", term.replace('\'', r"'\''")))
}

#[derive(Serialize)]
struct Hit {
    command: String,
    description: String,
    in_name: bool,
    in_description: bool,
}

/// What corpus this verb reads, reported verbatim in `--json`.
///
/// A machine consumer (floless.app builds its agent surface off `--json`) has
/// the same false-negative problem a human does: `total_matches: 0` alone
/// cannot distinguish "no agent does this" from "no agent that does this is
/// installed here". This names the corpus so the caller can tell.
const SCOPE: &str = "installed";

// Fields stay snake_case: `total_matches` is already the published key that
// `--json` consumers read, so kebab-casing this struct to match `meta` would
// silently break them. New keys follow the surface that exists, not the one
// that would have been tidier.
#[derive(Serialize)]
struct SearchData<'a> {
    term: &'a str,
    total_matches: usize,
    /// Installed agents actually scanned, after any `--agent` narrowing.
    searched_agents: usize,
    scope: &'static str,
    limit: usize,
    results: Vec<AgentResults<'a>>,
}

#[derive(Serialize)]
struct AgentResults<'a> {
    agent: &'a str,
    match_count: usize,
    commands: Vec<&'a Hit>,
}
