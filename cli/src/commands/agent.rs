//! `aware agent ...` — manage installed agents.
//!
//! Phase v0.1 (read-only): `list`, `describe`, `skill`.
//! Phase v0.2 (install + validate): `install`, `uninstall`, `update`,
//!   `validate`, `publish`.

use std::time::Instant;

use clap::Subcommand;
use serde::Serialize;

use crate::context::Context;
use crate::envelope;
use crate::error::AwareError;
use crate::manifest::agent::Agent;
use crate::manifest::loader::discover_agents;
use crate::registry::catalog::{self, Catalog};
use crate::registry::fetch::fetch_catalog;
use crate::render::table::Table;

#[derive(Subcommand, Debug)]
pub enum AgentCommand {
    /// Print a table of installed agents.
    List,
    /// Print an agent's manifest summary + skill index + command list.
    ///
    /// Reads the INSTALLED agent; pass `--available` (or describe an agent that
    /// isn't installed) to read it from the registry catalog instead.
    Describe {
        /// Agent id (e.g. `tekla`, `trimble-connect`).
        agent: String,
        /// Describe a not-yet-installed agent from the registry catalog.
        #[arg(long)]
        available: bool,
    },
    /// Print a skill's content.
    Skill {
        /// Agent id.
        agent: String,
        /// Skill filename (e.g. `drawing-identity.md`) or skill name from frontmatter.
        skill: String,
    },
    /// Install an agent from the registry or a local path. (v0.2)
    Install {
        /// Agent spec (`<name>[@version]`) or a local folder path.
        spec: String,
    },
    /// Uninstall an agent. (v0.2)
    Uninstall { agent: String },
    /// Re-pull the latest matching version of an agent. (v0.2)
    ///
    /// Pass `--all` to update every installed agent instead of a single one.
    /// The `<agent>` argument is required unless `--all` is used.
    Update {
        /// Agent id to update. Omit when `--all` is set.
        agent: Option<String>,
        /// Update every installed agent.
        #[arg(long)]
        all: bool,
    },
    /// Validate an agent folder against the agent-spec. (v0.2)
    Validate {
        /// Path to an agent folder containing manifest.yaml.
        path: std::path::PathBuf,
    },
    /// Open a PR to the GitHub registry. (v0.2+)
    Publish { path: std::path::PathBuf },

    /// Browse ALL available agents from the registry catalog (not just installed).
    Catalog,
    /// Search available agents by functionality — name, description, commands, skills.
    Search {
        /// Free-text query; whitespace-separated terms (case-insensitive substring).
        query: String,
        /// Bias matching to command names/methods only ("does an agent DO this?").
        #[arg(long)]
        capability: bool,
    },
    /// Check whether an agent exposes a capability (a command/method/skill).
    ///
    /// Scriptable checkpoint: prints the matching command(s)/skill(s) and exits 0
    /// if found, non-zero if not. Reads the catalog, so it works for not-yet-
    /// installed agents.
    Has {
        /// Agent id.
        agent: String,
        /// Capability to look for (a command name, HTTP method, or skill).
        capability: String,
    },
    /// Regenerate `registry-catalog.json` from the index × on-disk manifests.
    ///
    /// Run inside an aware checkout (one containing `registry-index.json`). The
    /// catalog is the searchable sidecar powering `catalog`/`search`/`has`/
    /// `describe --available`.
    Reindex {
        /// Verify the on-disk catalog is current without writing it (CI). Exit
        /// non-zero if stale.
        #[arg(long)]
        check: bool,
    },
}

pub fn dispatch(cmd: AgentCommand, ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        AgentCommand::List => list(ctx),
        AgentCommand::Describe { agent, available } => describe(ctx, &agent, available),
        AgentCommand::Skill { agent, skill } => skill_cmd(ctx, &agent, &skill),
        AgentCommand::Install { spec } => install(ctx, &spec),
        AgentCommand::Uninstall { agent } => {
            crate::install::uninstall_agent(&agent, &ctx.paths)?;
            println!("✓ uninstalled {agent}");
            let _ = auto_regenerate_plugins(ctx);
            let _ = crate::commands::diagram::auto_regenerate(ctx);
            Ok(())
        }
        AgentCommand::Update { agent, all } => update(ctx, agent.as_deref(), all),
        AgentCommand::Validate { path } => validate_cmd(ctx, &path),
        AgentCommand::Publish { path } => publish(ctx, &path),
        AgentCommand::Catalog => catalog_cmd(ctx),
        AgentCommand::Search { query, capability } => search_cmd(ctx, &query, capability),
        AgentCommand::Has { agent, capability } => has_cmd(ctx, &agent, &capability),
        AgentCommand::Reindex { check } => reindex(ctx, check),
    }
}

fn install(ctx: &Context, spec: &str) -> Result<(), AwareError> {
    use std::path::PathBuf;
    let path = PathBuf::from(spec);
    if path.is_dir() {
        let installed = crate::install::install_agent_from_path(&path, &ctx.paths)?;
        println!("✓ installed {installed} from {}", path.display());
        // Auto-regenerate host plugins (best-effort — failures don't tear down the install)
        let _ = auto_regenerate_plugins(ctx);
        let _ = crate::commands::diagram::auto_regenerate(ctx);
        return Ok(());
    }

    // Otherwise: treat as registry id [@version] or bundle name.
    let index = crate::registry::fetch::fetch_index(&ctx.paths.cache_dir())?;
    if index.bundles.contains_key(spec) {
        let report = crate::install::install_bundle(spec, &ctx.paths, &index)?;
        println!(
            "✓ bundle {}: {} installed, {} failed",
            report.bundle,
            report.installed.len(),
            report.failed.len()
        );
        for s in &report.installed {
            println!("  ✓ {s}");
        }
        for (s, e) in &report.failed {
            println!("  ✗ {s}: {e}");
        }
        // Auto-regenerate host plugins (best-effort — failures don't tear down the install)
        let _ = auto_regenerate_plugins(ctx);
        let _ = crate::commands::diagram::auto_regenerate(ctx);
        return Ok(());
    }
    let (id, version_pin) = match spec.split_once('@') {
        Some((i, v)) => (i, Some(v)),
        None => (spec, None),
    };
    let installed =
        crate::install::install_agent_from_registry(id, version_pin, &ctx.paths, &index)?;
    println!("✓ installed {installed}");
    // Auto-regenerate host plugins (best-effort — failures don't tear down the install)
    let _ = auto_regenerate_plugins(ctx);
    let _ = crate::commands::diagram::auto_regenerate(ctx);
    Ok(())
}

fn update(ctx: &Context, id: Option<&str>, all: bool) -> Result<(), AwareError> {
    match (id, all) {
        (Some(_), true) => Err(AwareError::Validation(
            "agent update: pass either <agent> or --all, not both".into(),
        )),
        (None, false) => Err(AwareError::Validation(
            "agent update: missing <agent> (or pass --all)".into(),
        )),
        (Some(id), false) => update_one(ctx, id),
        (None, true) => update_all(ctx),
    }
}

fn update_one(ctx: &Context, id: &str) -> Result<(), AwareError> {
    let index = crate::registry::fetch::fetch_index(&ctx.paths.cache_dir())?;
    // Atomic: resolve + fetch + validate before the on-disk install is touched,
    // so a failed re-pull leaves the existing agent intact (#174).
    let installed = crate::install::update_agent_from_registry(id, &ctx.paths, &index)?;
    println!("\u{2713} updated {installed}");
    let _ = auto_regenerate_plugins(ctx);
    let _ = crate::commands::diagram::auto_regenerate(ctx);
    Ok(())
}

fn update_all(ctx: &Context) -> Result<(), AwareError> {
    let installed = discover_agents(&ctx.paths)?;
    if installed.is_empty() {
        println!("(no agents installed)");
        return Ok(());
    }
    let ids: Vec<String> = installed.iter().map(|d| d.manifest.agent.clone()).collect();
    println!("updating {} installed agents...", ids.len());
    let index = crate::registry::fetch::fetch_index(&ctx.paths.cache_dir())?;

    let mut ok = 0usize;
    let mut failed: Vec<(String, String)> = Vec::new();
    for id in &ids {
        // Atomic per-agent update: a failure leaves that agent's existing
        // install untouched rather than deleting it (#174). One transient
        // network error must not cost the user an installed agent.
        match crate::install::update_agent_from_registry(id, &ctx.paths, &index) {
            Ok(spec) => {
                println!("  \u{2713} {spec}");
                ok += 1;
            }
            Err(e) => {
                println!("  \u{2717} {id}: {e}");
                failed.push((id.clone(), e.to_string()));
            }
        }
    }

    // Refresh derived artefacts once at the end (cheaper than per-agent).
    let _ = auto_regenerate_plugins(ctx);
    let _ = crate::commands::diagram::auto_regenerate(ctx);

    println!();
    println!("{ok} updated, {} failed", failed.len());
    if !failed.is_empty() {
        return Err(AwareError::Validation(format!(
            "{} agent(s) failed to update",
            failed.len()
        )));
    }
    Ok(())
}

fn validate_cmd(_ctx: &Context, path: &std::path::Path) -> Result<(), AwareError> {
    let manifest_path = path.join("manifest.yaml");
    let agent = crate::manifest::loader::load_agent(&manifest_path)?;
    let issues = crate::validate::validate_agent_on_disk(&agent, path);
    if issues.is_empty() {
        println!("✓ {} is valid", path.display());
        return Ok(());
    }
    for i in &issues {
        let tag = match i.severity {
            crate::validate::Severity::Error => "✗",
            crate::validate::Severity::Warning => "⚠",
        };
        println!("{tag} [{}] {}", i.code, i.message);
    }
    if crate::validate::has_errors(&issues) {
        return Err(AwareError::Validation("agent failed validation".into()));
    }
    Ok(())
}

/// Standard tarball for substrate-hosted agents: every entry in the
/// aware-aeco/aware registry points at the repo's `main` archive and is
/// distinguished only by `subdir` (see `registry-index.json`).
const SUBSTRATE_TARBALL: &str =
    "https://github.com/aware-aeco/aware/archive/refs/heads/main.tar.gz";

/// `aware agent publish <path>` — validate an agent, stage its entry in the
/// registry index, and print the steps to open a PR to the GitHub registry.
///
/// Scope: agents inside an aware-substrate checkout (the registry's tarball
/// IS the repo's `main` archive, so an entry is just a `subdir` into it).
/// Publish does NOT commit or push — opening a PR is a shared-state action
/// that stays under the contributor's control; this stages the index change
/// for review.
fn publish(_ctx: &Context, path: &std::path::Path) -> Result<(), AwareError> {
    let manifest_path = path.join("manifest.yaml");
    let agent = crate::manifest::loader::load_agent(&manifest_path)?;

    let issues = crate::validate::validate_agent_on_disk(&agent, path);
    for i in &issues {
        let tag = match i.severity {
            crate::validate::Severity::Error => "✗",
            crate::validate::Severity::Warning => "⚠",
        };
        println!("{tag} [{}] {}", i.code, i.message);
    }
    if crate::validate::has_errors(&issues) {
        return Err(AwareError::Validation(
            "agent failed validation; fix the errors above before publishing".into(),
        ));
    }

    let id = agent.agent.clone();
    let version = agent.version.clone();

    let abs = path.canonicalize()?;
    let Some((index_path, rel)) = find_registry_root(&abs) else {
        println!();
        println!("This registry hosts agents inside the aware-aeco/aware repo —");
        println!("every entry's tarball is the repo's `main` archive. To publish");
        println!("{id}@{version}, run `aware agent publish` from inside an aware");
        println!("checkout (one containing registry-index.json), or host your own");
        println!("tarball + index for an external agent.");
        return Ok(());
    };

    let subdir = format!("aware-main/{}", rel.replace('\\', "/"));
    let raw = std::fs::read_to_string(&index_path)?;
    let updated = merge_publish_entry(&raw, &id, &version, SUBSTRATE_TARBALL, &subdir)?;
    std::fs::write(&index_path, &updated)?;

    println!("✓ staged {id}@{version} in {}", index_path.display());
    println!("  subdir: {subdir}");
    println!();
    println!("Review the change, then open a PR to the GitHub registry:");
    println!("  git add registry-index.json");
    println!("  git commit -m \"registry: publish {id}@{version}\"");
    println!("  gh pr create --fill        # or push your fork and open the PR on GitHub");
    Ok(())
}

/// Walk up from `start` for `registry-index.json`. Returns
/// `(index_path, agent_path_relative_to_repo_root)` when found.
fn find_registry_root(start: &std::path::Path) -> Option<(std::path::PathBuf, String)> {
    let mut dir = Some(start);
    while let Some(d) = dir {
        let candidate = d.join("registry-index.json");
        if candidate.is_file() {
            let rel = start.strip_prefix(d).ok()?;
            return Some((candidate, rel.to_string_lossy().into_owned()));
        }
        dir = d.parent();
    }
    None
}

/// Insert `id@version → {tarball, subdir}` into a registry index document,
/// refreshing `updated-at`. Pure (string → string) for testability.
///
/// Merges at the JSON-value level (relying on serde_json's `preserve_order`)
/// so existing agents keep their on-disk order — a new agent is appended, and
/// a new version is appended within an existing agent — which keeps the
/// publish diff minimal and reviewable instead of re-sorting the whole index.
fn merge_publish_entry(
    index_json: &str,
    id: &str,
    version: &str,
    tarball: &str,
    subdir: &str,
) -> Result<String, AwareError> {
    let mut doc: serde_json::Value = serde_json::from_str(index_json)?;
    let agents = doc
        .get_mut("agents")
        .and_then(|a| a.as_object_mut())
        .ok_or_else(|| AwareError::Validation("registry index missing `agents` object".into()))?;
    let entry = agents
        .entry(id.to_string())
        .or_insert_with(|| serde_json::json!({ "versions": {} }));
    let versions = entry
        .get_mut("versions")
        .and_then(|v| v.as_object_mut())
        .ok_or_else(|| {
            AwareError::Validation(format!("registry entry {id} missing a `versions` object"))
        })?;
    versions.insert(
        version.to_string(),
        serde_json::json!({ "tarball": tarball, "subdir": subdir }),
    );
    doc["updated-at"] = serde_json::Value::String(crate::builder::now_iso());
    let mut out = serde_json::to_string_pretty(&doc)?;
    out.push('\n');
    Ok(out)
}

#[cfg(test)]
mod publish_tests {
    use super::*;

    const SAMPLE: &str = r#"{"version":"1.0","updated-at":"old","agents":{"tekla":{"versions":{"2025.0.1":{"tarball":"t","subdir":"s"}}}},"bundles":{}}"#;

    #[test]
    fn merge_adds_new_agent_and_preserves_existing() {
        let out = merge_publish_entry(
            SAMPLE,
            "bcf-file",
            "0.2.0",
            SUBSTRATE_TARBALL,
            "aware-main/20-agents/aeco/construction/bcf-file",
        )
        .unwrap();
        let parsed = crate::registry::Index::parse(out.as_bytes()).unwrap();
        assert!(parsed.agents.contains_key("tekla"), "existing agent kept");
        let (v, e) = parsed.resolve("bcf-file", Some("0.2.0")).unwrap();
        assert_eq!(v, "0.2.0");
        assert_eq!(e.tarball, SUBSTRATE_TARBALL);
        assert_eq!(e.subdir, "aware-main/20-agents/aeco/construction/bcf-file");
        assert_ne!(parsed.updated_at, "old", "updated-at refreshed");
    }

    #[test]
    fn merge_adds_version_to_existing_agent() {
        let out = merge_publish_entry(SAMPLE, "tekla", "2026.0.0", "tb", "sd").unwrap();
        let parsed = crate::registry::Index::parse(out.as_bytes()).unwrap();
        let tekla = parsed.agents.get("tekla").unwrap();
        assert!(tekla.versions.contains_key("2025.0.1"), "old version kept");
        assert!(tekla.versions.contains_key("2026.0.0"), "new version added");
    }

    #[test]
    fn merge_preserves_existing_agent_order_and_appends() {
        // Deliberately non-alphabetical on-disk order: zebra before alpha.
        let src = r#"{"version":"1.0","updated-at":"old","agents":{"zebra":{"versions":{"1.0.0":{"tarball":"t","subdir":"z"}}},"alpha":{"versions":{"1.0.0":{"tarball":"t","subdir":"a"}}}},"bundles":{}}"#;
        let out = merge_publish_entry(src, "middle", "1.0.0", "t", "m").unwrap();
        let zebra = out.find("\"zebra\"").unwrap();
        let alpha = out.find("\"alpha\"").unwrap();
        let middle = out.find("\"middle\"").unwrap();
        // Original order is kept (zebra, then alpha) and the new agent is appended last —
        // NOT re-sorted alphabetically.
        assert!(
            zebra < alpha,
            "existing order preserved (zebra before alpha)"
        );
        assert!(alpha < middle, "new agent appended after existing ones");
    }
}

#[derive(Serialize)]
struct AgentListRow {
    id: String,
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    sdk_target: Option<String>,
    kind: String,
    skills: usize,
    commands: usize,
}

#[derive(Serialize)]
struct AgentListData {
    agents: Vec<AgentListRow>,
}

fn describe(ctx: &Context, agent_id: &str, available: bool) -> Result<(), AwareError> {
    let started = Instant::now();
    // `--available` reads the registry catalog (a not-yet-installed agent). Without it,
    // `describe` is installed-only and keeps its not-found = exit-7 contract — but the
    // error points at `--available` so a user who meant the catalog gets unstuck.
    if available {
        return describe_from_catalog(ctx, agent_id, started);
    }
    let discovered = discover_agents(&ctx.paths)?;
    let d = discovered
        .into_iter()
        .find(|d| d.manifest.agent == agent_id)
        .ok_or_else(|| {
            AwareError::NotFound(format!(
                "agent '{agent_id}' is not installed — try \
                 `aware agent describe {agent_id} --available` to view it in the registry catalog"
            ))
        })?;
    describe_installed(ctx, &d.manifest, started)
}

/// Render an INSTALLED agent's manifest.
fn describe_installed(ctx: &Context, m: &Agent, started: Instant) -> Result<(), AwareError> {
    if ctx.json {
        #[derive(Serialize)]
        struct CommandRow {
            name: String,
            lifecycle: String,
            category: String,
            description: String,
        }
        #[derive(Serialize)]
        struct DescribeData<'a> {
            agent: &'a str,
            version: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            sdk_target: Option<&'a str>,
            display_name: Option<&'a str>,
            description: &'a str,
            stateful: bool,
            license: &'a str,
            vendor: Option<&'a str>,
            commands: Vec<CommandRow>,
            skills: &'a [String],
            skill_count: usize,
            command_count: usize,
            curated_count: usize,
            reflected_count: usize,
        }

        let cmds: Vec<CommandRow> = m
            .commands
            .iter()
            .map(|(n, c)| CommandRow {
                name: n.clone(),
                lifecycle: format!("{:?}", c.lifecycle).to_lowercase(),
                category: format!("{:?}", m.category_of(c)).to_lowercase(),
                description: c.description.clone(),
            })
            .collect();

        let data = DescribeData {
            agent: &m.agent,
            version: &m.version,
            sdk_target: m.sdk_target.as_deref(),
            display_name: m.display_name.as_deref(),
            description: &m.description,
            stateful: m.stateful,
            license: &m.license,
            vendor: m.vendor.as_deref(),
            command_count: m.command_count(),
            skill_count: m.skill_count(),
            curated_count: m.curated_count(),
            reflected_count: m.reflected_count(),
            commands: cmds,
            skills: &m.skills,
        };
        envelope::print_ok("agent describe", data, started).ok();
        return Ok(());
    }

    println!("agent:        {}", m.agent);
    println!("version:      {}", m.version);
    if let Some(sdk) = &m.sdk_target {
        println!("sdk-target:   {sdk}");
    }
    if let Some(dn) = &m.display_name {
        println!("display-name: {dn}");
    }
    println!(
        "description:  {}",
        m.description.lines().next().unwrap_or("").trim()
    );
    println!("stateful:     {}", m.stateful);
    if let Some(v) = &m.vendor {
        println!("vendor:       {v}");
    }
    println!("license:      {}", m.license);
    if let Some(t) = &m.transport.cli {
        println!("transport:    cli ({})", t.binary);
    }
    if let Some(t) = &m.transport.app {
        println!("transport:    app (backed by app {})", t.backed_by);
    }
    if m.status == crate::manifest::agent::AgentStatus::Planned {
        println!(
            "status:       \u{26a0} planned — not yet runnable (no shipped transport binary); \
             apps referencing it are rejected at validate/compile (#161)"
        );
    }
    println!();
    let curated = m.curated_count();
    let reflected = m.reflected_count();
    if reflected > 0 {
        println!("commands ({} curated · {} reflected):", curated, reflected);
    } else {
        println!("commands ({} curated):", curated);
    }
    for (name, c) in &m.commands {
        let lc = format!("{:?}", c.lifecycle).to_lowercase();
        let cat = match m.category_of(c) {
            crate::manifest::agent::Category::Curated => "★",
            crate::manifest::agent::Category::Reflected => " ",
        };
        let desc = c.description.lines().next().unwrap_or("").trim();
        println!("  {cat} {name:<18} {lc:<8} {desc}");
    }
    println!();
    println!("skills ({}):", m.skill_count());
    for s in &m.skills {
        println!("  - {s}");
    }
    Ok(())
}

fn skill_cmd(ctx: &Context, agent_id: &str, skill_name: &str) -> Result<(), AwareError> {
    let discovered = discover_agents(&ctx.paths)?;
    let d = discovered
        .into_iter()
        .find(|d| d.manifest.agent == agent_id)
        .ok_or_else(|| AwareError::NotFound(format!("agent: {agent_id}")))?;

    let filename = if skill_name.ends_with(".md") {
        skill_name.to_string()
    } else {
        format!("{skill_name}.md")
    };
    let path = d.root.join("skills").join(&filename);
    if !path.is_file() {
        return Err(AwareError::NotFound(format!(
            "skill: {agent_id}/{filename}"
        )));
    }
    let body = std::fs::read_to_string(&path)?;
    // Raw print — markdown is human-readable and AI-readable as-is.
    print!("{body}");
    Ok(())
}

fn auto_regenerate_plugins(ctx: &Context) -> Result<(), AwareError> {
    let home = dirs::home_dir().ok_or_else(|| AwareError::Internal("home dir".into()))?;
    let agents = crate::manifest::loader::discover_agents(&ctx.paths)?;

    // Only regen for hosts whose plugin dir already exists (or override env var set)
    let claude_target = std::env::var_os("AWARE_PLUGINS_CLAUDE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| home.join(".claude/plugins"));
    if claude_target.exists() || std::env::var_os("AWARE_PLUGINS_CLAUDE").is_some() {
        let _ = crate::plugins::claude_code::generate(&agents, &claude_target);
    }
    // codex / opencode left as scaffolds — regen on install would write the same TODO every time

    Ok(())
}

fn list(ctx: &Context) -> Result<(), AwareError> {
    let started = Instant::now();
    let discovered = discover_agents(&ctx.paths)?;

    if ctx.json {
        let data = AgentListData {
            agents: discovered
                .iter()
                .map(|d| AgentListRow {
                    id: d.manifest.agent.clone(),
                    version: d.manifest.version.clone(),
                    sdk_target: d.manifest.sdk_target.clone(),
                    kind: d.manifest.kind(),
                    skills: d.manifest.skill_count(),
                    commands: d.manifest.command_count(),
                })
                .collect(),
        };
        envelope::print_ok("agent list", data, started).ok();
        return Ok(());
    }

    let mut t = Table::new(["ID", "VERSION", "SDK-TARGET", "KIND", "SKILLS", "COMMANDS"]);
    for d in &discovered {
        t.row([
            d.manifest.agent.clone(),
            d.manifest.version.clone(),
            d.manifest.sdk_target.clone().unwrap_or_default(),
            d.manifest.kind(),
            d.manifest.skill_count().to_string(),
            d.manifest.command_count().to_string(),
        ]);
    }
    print!("{}", t.render());
    Ok(())
}

// ── Available-agent catalog (browse / search / capability / regenerate) ───────

/// Flush stdout, then exit — so a piped checkpoint (`aware agent has …`) never
/// drops its block-buffered output when exiting non-zero.
fn flush_exit(code: i32) -> ! {
    use std::io::Write;
    let _ = std::io::stdout().flush();
    std::process::exit(code);
}

/// Fetch the catalog; when this registry has no catalog yet, print clean guidance
/// and return `Ok(None)` so the caller stops without an error.
fn load_catalog(ctx: &Context) -> Result<Option<Catalog>, AwareError> {
    match fetch_catalog(&ctx.paths.cache_dir())? {
        Some(c) => Ok(Some(c)),
        None => {
            println!(
                "No agent catalog is available for this registry yet.\n\
                 Update AWARE (`npm i -g @aware-aeco/cli@latest`), or — inside an aware\n\
                 checkout — run `aware agent reindex` to generate registry-catalog.json."
            );
            Ok(None)
        }
    }
}

/// `aware agent catalog` — every available agent (latest version) from the catalog.
fn catalog_cmd(ctx: &Context) -> Result<(), AwareError> {
    let started = Instant::now();
    let Some(catalog) = load_catalog(ctx)? else {
        return Ok(());
    };

    if ctx.json {
        #[derive(Serialize)]
        struct Row<'a> {
            id: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            display_name: Option<&'a str>,
            version: &'a str,
            status: &'a str,
            commands: usize,
            skills: usize,
            description: &'a str,
        }
        #[derive(Serialize)]
        struct Data<'a> {
            agents: Vec<Row<'a>>,
        }
        let rows: Vec<Row> = catalog
            .agents
            .iter()
            .filter_map(|(id, a)| {
                a.latest().map(|(ver, v)| Row {
                    id,
                    display_name: a.display_name.as_deref(),
                    version: ver,
                    status: &v.status,
                    commands: v.command_count,
                    skills: v.skills.len(),
                    description: &v.description,
                })
            })
            .collect();
        envelope::print_ok("agent catalog", Data { agents: rows }, started).ok();
        return Ok(());
    }

    let mut t = Table::new([
        "ID",
        "NAME",
        "VERSION",
        "STATUS",
        "CMDS",
        "SKILLS",
        "DESCRIPTION",
    ]);
    for (id, a) in &catalog.agents {
        if let Some((ver, v)) = a.latest() {
            t.row([
                id.clone(),
                a.display_name.clone().unwrap_or_default(),
                ver.clone(),
                v.status.clone(),
                v.command_count.to_string(),
                v.skills.len().to_string(),
                v.description.clone(),
            ]);
        }
    }
    print!("{}", t.render());
    println!(
        "\n{} agents available · `aware agent describe <id> --available` for details, \
         `aware agent install <id>` to install",
        catalog.agents.len()
    );
    Ok(())
}

/// `aware agent search <query>` — rank available agents by functionality.
fn search_cmd(ctx: &Context, query: &str, capability: bool) -> Result<(), AwareError> {
    let started = Instant::now();
    let Some(catalog) = load_catalog(ctx)? else {
        return Ok(());
    };
    let hits = catalog::search(&catalog, query, capability);

    if ctx.json {
        #[derive(Serialize)]
        struct Data<'a> {
            query: &'a str,
            capability: bool,
            results: &'a [catalog::SearchMatch],
        }
        envelope::print_ok(
            "agent search",
            Data {
                query,
                capability,
                results: &hits,
            },
            started,
        )
        .ok();
        return Ok(());
    }

    if hits.is_empty() {
        println!("No available agents match \"{query}\".");
        return Ok(());
    }
    println!(
        "{} match{} for \"{query}\"{}:",
        hits.len(),
        if hits.len() == 1 { "" } else { "es" },
        if capability { " (capability)" } else { "" }
    );
    for h in &hits {
        let a = catalog.agents.get(&h.id);
        let name = a.and_then(|a| a.display_name.as_deref()).unwrap_or("");
        let desc = a
            .and_then(|a| a.latest())
            .map(|(_, v)| v.description.as_str())
            .unwrap_or("");
        if name.is_empty() {
            println!("\n  {}", h.id);
        } else {
            println!("\n  {} — {name}", h.id);
        }
        if !desc.is_empty() {
            println!("    {desc}");
        }
        println!("    matched: {}", h.matched.join(", "));
        if let Some(s) = &h.snippet {
            println!("    {s}");
        }
    }
    println!("\nInstall one with `aware agent install <id>`.");
    Ok(())
}

/// `aware agent has <agent> <capability>` — scriptable capability checkpoint.
fn has_cmd(ctx: &Context, agent_id: &str, capability: &str) -> Result<(), AwareError> {
    let started = Instant::now();
    let Some(catalog) = load_catalog(ctx)? else {
        flush_exit(2); // no catalog → can't answer
    };
    let Some(agent) = catalog.agents.get(agent_id) else {
        if ctx.json {
            #[derive(Serialize)]
            struct D<'a> {
                agent: &'a str,
                found: bool,
                error: &'a str,
            }
            envelope::print_ok(
                "agent has",
                D {
                    agent: agent_id,
                    found: false,
                    error: "agent not in catalog",
                },
                started,
            )
            .ok();
        } else {
            println!("✗ '{agent_id}' is not in the registry catalog.");
        }
        flush_exit(1);
    };

    let hits = agent.capability_hits(capability);
    let found = !hits.is_empty();

    if ctx.json {
        #[derive(Serialize)]
        struct D<'a> {
            agent: &'a str,
            capability: &'a str,
            found: bool,
            hits: &'a [catalog::Hit],
        }
        envelope::print_ok(
            "agent has",
            D {
                agent: agent_id,
                capability,
                found,
                hits: &hits,
            },
            started,
        )
        .ok();
    } else if found {
        println!("✓ {agent_id} exposes '{capability}':");
        for h in &hits {
            if h.description.is_empty() {
                println!("  [{}] {}", h.kind, h.name);
            } else {
                println!("  [{}] {} — {}", h.kind, h.name, h.description);
            }
        }
    } else {
        println!("✗ {agent_id} does not expose '{capability}'.");
    }

    if found { Ok(()) } else { flush_exit(1) }
}

/// `aware agent reindex` — regenerate registry-catalog.json from the index × manifests.
fn reindex(ctx: &Context, check: bool) -> Result<(), AwareError> {
    let _ = ctx;
    let index_path = find_index_path().ok_or_else(|| {
        AwareError::Validation(
            "no registry-index.json found — run `aware agent reindex` inside an aware checkout"
                .into(),
        )
    })?;
    let repo_root = index_path
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .to_path_buf();
    let index = crate::registry::Index::parse(std::fs::File::open(&index_path)?)?;

    let (cat, errors) = catalog::build_catalog(&index, crate::builder::now_iso(), |subdir| {
        let rel = subdir.strip_prefix("aware-main/").unwrap_or(subdir);
        let manifest = repo_root.join(rel).join("manifest.yaml");
        crate::manifest::loader::load_agent(&manifest)
    });

    let mut out = serde_json::to_string_pretty(&cat)
        .map_err(|e| AwareError::Validation(format!("serialize catalog: {e}")))?;
    out.push('\n');
    let catalog_path = repo_root.join("registry-catalog.json");

    if check {
        let current = std::fs::read_to_string(&catalog_path).unwrap_or_default();
        if current == out {
            println!(
                "✓ registry-catalog.json is up to date ({} agents)",
                cat.agents.len()
            );
            return Ok(());
        }
        return Err(AwareError::Validation(
            "registry-catalog.json is stale — run `aware agent reindex` and commit the result"
                .into(),
        ));
    }

    std::fs::write(&catalog_path, &out)?;
    println!(
        "✓ wrote {} ({} agents)",
        catalog_path.display(),
        cat.agents.len()
    );
    if !errors.is_empty() {
        eprintln!(
            "⚠ {} agent(s) skipped (manifest load failed):",
            errors.len()
        );
        for (id, e) in &errors {
            eprintln!("  ✗ {id}: {e}");
        }
        return Err(AwareError::Validation(format!(
            "{} agent(s) failed to load",
            errors.len()
        )));
    }
    Ok(())
}

/// `aware agent describe <agent> --available` — describe a not-installed agent from the catalog.
fn describe_from_catalog(
    ctx: &Context,
    agent_id: &str,
    started: Instant,
) -> Result<(), AwareError> {
    let Some(catalog) = load_catalog(ctx)? else {
        return Ok(());
    };
    let agent = catalog.agents.get(agent_id).ok_or_else(|| {
        AwareError::NotFound(format!(
            "agent '{agent_id}' is not installed and not in the registry catalog"
        ))
    })?;
    let (ver, v) = agent.latest().ok_or_else(|| {
        AwareError::NotFound(format!("agent '{agent_id}' has no versions in the catalog"))
    })?;

    if ctx.json {
        #[derive(Serialize)]
        struct D<'a> {
            agent: &'a str,
            installed: bool,
            version: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            display_name: Option<&'a str>,
            description: &'a str,
            status: &'a str,
            stateful: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            vendor: Option<&'a str>,
            transport: &'a str,
            /// Total commands (curated + reflected); `commands` lists curated only.
            command_count: usize,
            commands: &'a [catalog::CatalogCommand],
            skills: &'a [String],
        }
        let data = D {
            agent: agent_id,
            installed: false,
            version: ver,
            display_name: agent.display_name.as_deref(),
            description: &v.description,
            status: &v.status,
            stateful: v.stateful,
            vendor: agent.vendor.as_deref(),
            transport: &v.transport,
            command_count: v.command_count,
            commands: &v.commands,
            skills: &v.skills,
        };
        envelope::print_ok("agent describe", data, started).ok();
        return Ok(());
    }

    println!("agent:        {agent_id}  (from registry catalog — not installed)");
    println!("version:      {ver}");
    if let Some(dn) = &agent.display_name {
        println!("display-name: {dn}");
    }
    println!("description:  {}", v.description);
    println!("status:       {}", v.status);
    println!("stateful:     {}", v.stateful);
    if let Some(vd) = &agent.vendor {
        println!("vendor:       {vd}");
    }
    println!("transport:    {}", v.transport);
    println!();
    let reflected = v.command_count.saturating_sub(v.commands.len());
    if reflected > 0 {
        println!(
            "commands ({} total · {} curated · {} reflected not listed — `aware agent install {agent_id}` then `describe` for the full surface):",
            v.command_count,
            v.commands.len(),
            reflected
        );
    } else {
        println!("commands ({}):", v.command_count);
    }
    for c in &v.commands {
        let star = if c.category == "curated" { "★" } else { " " };
        println!(
            "  {star} {:<20} {:<8} {}",
            c.name, c.lifecycle, c.description
        );
    }
    println!();
    println!("skills ({}):", v.skills.len());
    for s in &v.skills {
        println!("  - {s}");
    }
    println!();
    println!("Install with `aware agent install {agent_id}`.");
    Ok(())
}

/// Walk up from the cwd for `registry-index.json`; return its path if found.
fn find_index_path() -> Option<std::path::PathBuf> {
    let mut dir: Option<std::path::PathBuf> = std::env::current_dir().ok();
    while let Some(d) = dir {
        let candidate = d.join("registry-index.json");
        if candidate.is_file() {
            return Some(candidate);
        }
        dir = d.parent().map(|p| p.to_path_buf());
    }
    None
}
