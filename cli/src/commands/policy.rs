//! `aware policy show|set|deny|remove` — the command policy a host writes
//! (`<AWARE_HOME>/command-policy.json`), checked at every operation dispatch
//! that carries an AWARE-resolved credential. Contract:
//! `10-core/command-policy.md`.

use std::time::Instant;

use clap::Subcommand;
use serde_json::json;

use crate::command_policy::{self, Accounts, Loaded, Rule, Selector};
use crate::context::Context;
use crate::envelope;
use crate::error::AwareError;

/// The most bytes `policy set` reads from its file.
const MAX_SET_BYTES: u64 = 256 * 1024;

#[derive(Subcommand, Debug)]
pub enum PolicyCommand {
    /// Print the command policy, or why it cannot be read.
    Show,
    /// Replace the whole command policy with a validated document.
    Set {
        /// `@<path>` to an `aware.command-policy/v1` JSON document.
        document: String,
    },
    /// Add a deny rule, or replace the rule with the same id.
    Deny {
        /// The rule id (`[a-z0-9][a-z0-9._-]*`, at most 128 characters).
        id: String,
        /// The agent whose command is denied.
        #[arg(long)]
        agent: String,
        /// The command denied, or `*` for every command of the agent.
        #[arg(long)]
        command: String,
        /// A credential slot to deny, optionally narrowed to one verified
        /// account: `<slot>` or `<slot>=<binding-id>`. Repeatable. Without
        /// any, every account is denied.
        #[arg(long = "account", value_name = "SLOT[=BINDING]")]
        accounts: Vec<String>,
        /// An account exempt from the rule: `<slot>=<binding-id>` (a slot
        /// alone is refused — it can be re-signed to another account).
        /// Repeatable.
        #[arg(long = "except", value_name = "SLOT=BINDING")]
        except: Vec<String>,
        /// Free text kept with the rule for the host; never interpreted.
        #[arg(long)]
        note: Option<String>,
    },
    /// Remove the rule with this id.
    Remove { id: String },
}

pub fn dispatch(cmd: PolicyCommand, ctx: &Context) -> Result<(), AwareError> {
    let started = Instant::now();
    let home = &ctx.paths.aware_home;
    match cmd {
        PolicyCommand::Show => show(ctx, started),
        PolicyCommand::Set { document } => {
            let bytes = read_document(&document)?;
            let policy = command_policy::parse_strict(&bytes)?;
            command_policy::set(home, &policy)?;
            report(
                ctx,
                "policy set",
                started,
                json!({ "rules": policy.rules.len() }),
                &format!("command policy replaced ({} rules)", policy.rules.len()),
            )
        }
        PolicyCommand::Deny {
            id,
            agent,
            command,
            accounts,
            except,
            note,
        } => {
            let accounts = if accounts.is_empty() {
                Accounts::All
            } else {
                Accounts::Only(
                    accounts
                        .iter()
                        .map(|a| parse_selector_arg(a))
                        .collect::<Result<_, _>>()?,
                )
            };
            let except = except
                .iter()
                .map(|a| parse_selector_arg(a))
                .collect::<Result<_, _>>()?;
            let rule = Rule {
                id: id.clone(),
                agent,
                command,
                accounts,
                except,
                note,
            };
            let data = command_policy::rule_json(&rule);
            let replaced = command_policy::upsert(home, rule)?;
            report(
                ctx,
                "policy deny",
                started,
                json!({ "rule": data, "replaced": replaced }),
                &if replaced {
                    format!("replaced deny rule {id}")
                } else {
                    format!("added deny rule {id}")
                },
            )
        }
        PolicyCommand::Remove { id } => {
            let removed = command_policy::remove(home, &id)?;
            report(
                ctx,
                "policy remove",
                started,
                json!({ "id": id, "removed": removed }),
                &if removed {
                    format!("removed rule {id}")
                } else {
                    format!("no rule {id}")
                },
            )
        }
    }
}

fn report(
    ctx: &Context,
    verb: &str,
    started: Instant,
    data: serde_json::Value,
    human: &str,
) -> Result<(), AwareError> {
    if ctx.json {
        envelope::print_ok(verb, data, started)?;
    } else {
        println!("{human}");
    }
    Ok(())
}

fn show(ctx: &Context, started: Instant) -> Result<(), AwareError> {
    let path = command_policy::policy_path(&ctx.paths.aware_home);
    let data = match command_policy::load(&ctx.paths.aware_home) {
        Loaded::Absent => json!({
            "path": path,
            "state": "absent",
            "rules": [],
        }),
        Loaded::Unreadable { why } => json!({
            "path": path,
            "state": "unreadable",
            "error": why,
            "effect": "every command that uses a credential is refused until the policy is replaced",
        }),
        Loaded::Readable { policy, broken } => json!({
            "path": path,
            "state": if broken.is_empty() { "readable" } else { "partly-unreadable" },
            "schema": command_policy::SCHEMA,
            "rules": policy.rules.iter().map(command_policy::rule_json).collect::<Vec<_>>(),
            "unreadableRules": broken
                .iter()
                .map(|(agent, id)| json!({ "agent": agent, "id": id }))
                .collect::<Vec<_>>(),
        }),
    };
    if ctx.json {
        envelope::print_ok("policy show", data, started)?;
    } else {
        println!("{}", serde_json::to_string_pretty(&data)?);
    }
    Ok(())
}

/// `@<path>`: a regular file of at most 256 KiB.
fn read_document(arg: &str) -> Result<Vec<u8>, AwareError> {
    use std::io::Read;
    let path = arg
        .strip_prefix('@')
        .filter(|p| !p.is_empty())
        .ok_or_else(|| {
            AwareError::Validation("[E_COMMAND_POLICY_INVALID] pass the policy as @<path>".into())
        })?;
    let meta = std::fs::metadata(path)?;
    if !meta.is_file() || meta.len() > MAX_SET_BYTES {
        return Err(AwareError::Validation(format!(
            "[E_COMMAND_POLICY_INVALID] {path} is not a regular file of at most {MAX_SET_BYTES} bytes"
        )));
    }
    let mut bytes = Vec::new();
    std::fs::File::open(path)?
        .take(MAX_SET_BYTES + 1)
        .read_to_end(&mut bytes)?;
    Ok(bytes)
}

/// `<slot>` or `<slot>=<binding>`.
fn parse_selector_arg(arg: &str) -> Result<Selector, AwareError> {
    let (slot, binding) = match arg.split_once('=') {
        Some((slot, binding)) => (slot, Some(binding)),
        None => (arg, None),
    };
    if let Some(binding) = binding
        && !command_policy::valid_binding(binding)
    {
        return Err(AwareError::Validation(format!(
            "[E_COMMAND_POLICY_INVALID] {binding:?} is not a binding id (a UUID)"
        )));
    }
    Ok(Selector {
        slot: slot.to_string(),
        binding: binding.map(str::to_ascii_lowercase),
    })
}
