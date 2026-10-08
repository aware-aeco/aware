//! Claude Code plugin generator.
//!
//! Writes `<plugin_root>/aware-aeco/plugin.json` + per-agent-command markdown
//! files under `commands/`. Idempotent: re-running produces byte-identical output.

use std::collections::{BTreeMap, HashSet};
use std::path::Path;

use crate::error::AwareError;
use crate::manifest::loader::DiscoveredAgent;

/// Which command files [`generate`] writes even though they already exist.
pub enum Refresh<'a> {
    /// Only missing files are written (install / uninstall).
    Missing,
    /// The named agents' files are also checked against their manifests and rewritten
    /// where the content differs — `agent update`, where one agent's descriptions may have
    /// changed. Every other agent's files are left exactly as they are, so the cost follows
    /// the agent that changed, not the number of agents installed (#671).
    Agents(&'a [String]),
    /// Clear every command file and rewrite them all (`aware plugins regenerate`, the
    /// drift-repair path). Cost grows with everything installed — never on a per-agent verb.
    All,
}

/// (Re)generate the Claude Code plugin from `agents`.
///
/// Incremental by default: writes only the command files that are MISSING and removes
/// only those that are orphaned, so installing or uninstalling one agent no longer
/// rewrites every other agent's command files — with the big reflected agents that was
/// tens of thousands of file writes per install (~54s, #244). `plugin.json` is always
/// rewritten (one small file) so the command index stays exact, and orphaned files from
/// uninstalled agents are pruned, keeping the output self-healing for add/remove.
///
/// `refresh` says which already-present files are re-checked. `agent update` passes the
/// updated agent (an existing command's description may have changed, which the
/// presence-based path would skip because the file name is unchanged); it used to clear and
/// rewrite every agent's files, tens of thousands of writes in a full store (#671).
pub fn generate(
    agents: &[DiscoveredAgent],
    plugin_root: &Path,
    refresh: Refresh<'_>,
) -> Result<usize, AwareError> {
    let aware_aeco_dir = plugin_root.join("aware-aeco");
    let commands_dir = aware_aeco_dir.join("commands");
    if matches!(refresh, Refresh::All) {
        // Force a clean rewrite of every command file (descriptions may have changed).
        let _ = std::fs::remove_dir_all(&commands_dir);
    }
    std::fs::create_dir_all(&commands_dir)?;
    // One directory listing instead of a stat per desired file, and the same set drives the
    // orphan prune below.
    let existing: HashSet<String> = std::fs::read_dir(&commands_dir)?
        .flatten()
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect();

    // The exact set of command files this agent set should produce (sorted by name via
    // BTreeMap, so plugin.json's index is byte-stable). File name → content.
    let mut desired: BTreeMap<String, String> = BTreeMap::new();
    // The file names whose content is re-checked even when present.
    let mut recheck: HashSet<String> = HashSet::new();
    for agent in agents {
        let rechecked =
            matches!(&refresh, Refresh::Agents(ids) if ids.contains(&agent.manifest.agent));
        for (cmd_name, cmd_spec) in &agent.manifest.commands {
            let file_name = format!("{}-{}.md", agent.manifest.agent, cmd_name);
            if rechecked {
                recheck.insert(file_name.clone());
            }
            let description_first_line = cmd_spec.description.lines().next().unwrap_or("").trim();
            let body = format!(
                "---\n\
                name: {agent_id}-{cmd_name}\n\
                description: {description}\n\
                ---\n\
                \n\
                Invokes `{agent_id}/{cmd_name}` via the AWARE CLI.\n\
                \n\
                Run an app that uses this command:\n\
                ```bash\n\
                aware app run <app-name> --instance default\n\
                ```\n",
                agent_id = agent.manifest.agent,
                cmd_name = cmd_name,
                description = description_first_line,
            );
            desired.insert(file_name, body);
        }
    }

    // Write the missing files (presence-based — the whole point of #244), and the
    // re-checked agents' files whose content differs.
    for (file_name, body) in &desired {
        let path = commands_dir.join(file_name);
        let stale = !existing.contains(file_name)
            || (recheck.contains(file_name)
                && std::fs::read_to_string(&path).ok().as_deref() != Some(body));
        if stale {
            std::fs::write(&path, body)?;
        }
    }

    // Prune orphaned command files (agents/commands that are no longer installed).
    for name in &existing {
        if name.ends_with(".md") && !desired.contains_key(name) {
            let _ = std::fs::remove_file(commands_dir.join(name));
        }
    }

    // plugin.json — always rewritten; reflects exactly the desired command set.
    let commands_index: Vec<String> = desired.keys().map(|f| format!("commands/{f}")).collect();
    let plugin_json = serde_json::json!({
        "name": "aware-aeco",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "AWARE agents and apps bundle. Composes AECO software into AI-runnable scripts.",
        "homepage": "https://github.com/aware-aeco/aware",
        "commands": commands_index,
    });
    std::fs::write(
        aware_aeco_dir.join("plugin.json"),
        serde_json::to_string_pretty(&plugin_json)? + "\n",
    )?;

    Ok(desired.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::loader::discover_agents;
    use crate::paths::Paths;

    fn populate_aware_home_from_fixtures() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let aware = tmp.path().join("aware");
        std::fs::create_dir_all(aware.join("agents")).unwrap();

        let repo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        // Copy tekla agent for the test
        let tekla_src = repo.join("20-agents/aeco/engineering/tekla");
        crate::fs::copy_dir_recursive(&tekla_src, &aware.join("agents/tekla")).unwrap();
        tmp
    }

    #[test]
    fn generates_plugin_json_with_command_index() {
        let tmp = populate_aware_home_from_fixtures();
        let paths = Paths {
            aware_home: tmp.path().join("aware"),
        };
        let agents = discover_agents(&paths).unwrap().agents;

        let plugin_root = tmp.path().join("plugins");
        let count = generate(&agents, &plugin_root, Refresh::Missing).unwrap();

        // Tekla currently has 27 curated commands (grew from 23 with `bake-scene`, #235, from 24
        // when #520 declared the dispatched-but-unpublished `list-instances` and `close`, and from
        // 26 when #617 added the read-only `model-info` probe verb).
        //
        // The generator indexes every declared command, `status: planned` ones included — so 19 of
        // these 27 are slash commands for verbs `aware-tekla` cannot dispatch. That is unchanged by
        // #520 (those 19 were already in the 24) and is not specific to tekla: `file` and
        // `google-workspace` carry planned commands too. Whether a planned command belongs in a
        // generated plugin index is a separate question from whether the manifest may advertise it
        // as runnable, which is what #520 settled.
        assert_eq!(count, 27);
        assert!(plugin_root.join("aware-aeco/plugin.json").is_file());

        let json: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(plugin_root.join("aware-aeco/plugin.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(json["name"], "aware-aeco");
        assert_eq!(json["commands"].as_array().unwrap().len(), 27);
    }

    #[test]
    fn generates_per_command_markdown() {
        let tmp = populate_aware_home_from_fixtures();
        let paths = Paths {
            aware_home: tmp.path().join("aware"),
        };
        let agents = discover_agents(&paths).unwrap().agents;

        let plugin_root = tmp.path().join("plugins");
        generate(&agents, &plugin_root, Refresh::Missing).unwrap();

        // Tekla commands: insert, save-attributes, watch
        let dir = plugin_root.join("aware-aeco/commands");
        assert!(dir.join("tekla-watch.md").is_file());
        assert!(dir.join("tekla-insert.md").is_file());
        assert!(dir.join("tekla-save-attributes.md").is_file());

        let body = std::fs::read_to_string(dir.join("tekla-watch.md")).unwrap();
        assert!(body.contains("name: tekla-watch"));
        assert!(body.contains("aware app run"));
    }

    #[test]
    fn idempotent_byte_identical_output() {
        let tmp = populate_aware_home_from_fixtures();
        let paths = Paths {
            aware_home: tmp.path().join("aware"),
        };
        let agents = discover_agents(&paths).unwrap().agents;

        let plugin_root = tmp.path().join("plugins");
        generate(&agents, &plugin_root, Refresh::Missing).unwrap();
        let first_json = std::fs::read(plugin_root.join("aware-aeco/plugin.json")).unwrap();
        let first_md =
            std::fs::read(plugin_root.join("aware-aeco/commands/tekla-watch.md")).unwrap();

        generate(&agents, &plugin_root, Refresh::Missing).unwrap();
        let second_json = std::fs::read(plugin_root.join("aware-aeco/plugin.json")).unwrap();
        let second_md =
            std::fs::read(plugin_root.join("aware-aeco/commands/tekla-watch.md")).unwrap();

        assert_eq!(first_json, second_json);
        assert_eq!(first_md, second_md);
    }

    #[test]
    fn incremental_prunes_orphans_and_keeps_existing() {
        let tmp = populate_aware_home_from_fixtures();
        let paths = Paths {
            aware_home: tmp.path().join("aware"),
        };
        let agents = discover_agents(&paths).unwrap().agents;
        let plugin_root = tmp.path().join("plugins");

        generate(&agents, &plugin_root, Refresh::Missing).unwrap();
        let commands = plugin_root.join("aware-aeco/commands");
        let kept = commands.join("tekla-watch.md");
        assert!(kept.is_file());

        // Simulate an uninstalled agent's leftover command file + mark a kept file so we
        // can prove a re-run doesn't rewrite already-present files.
        let orphan = commands.join("ghost-removed.md");
        std::fs::write(&orphan, "stale").unwrap();
        std::fs::write(&kept, "SENTINEL").unwrap(); // presence-based: must NOT be overwritten

        generate(&agents, &plugin_root, Refresh::Missing).unwrap();

        assert!(!orphan.exists(), "orphaned command file should be pruned");
        assert_eq!(
            std::fs::read_to_string(&kept).unwrap(),
            "SENTINEL",
            "an already-present command file must not be rewritten (presence-based)"
        );
        // plugin.json never lists the orphan.
        let json = std::fs::read_to_string(plugin_root.join("aware-aeco/plugin.json")).unwrap();
        assert!(!json.contains("ghost-removed"));

        // A full rebuild DOES refresh content (`aware plugins regenerate`).
        generate(&agents, &plugin_root, Refresh::All).unwrap();
        assert!(
            std::fs::read_to_string(&kept)
                .unwrap()
                .contains("name: tekla-watch"),
            "full rebuild should rewrite command files from the manifest"
        );
    }
    /// A second installed agent, `probe`, next to the tekla fixture.
    fn add_probe_agent(aware: &Path) {
        let dir = aware.join("agents/probe");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("manifest.yaml"),
            "agent: probe\nversion: 0.0.1\ndescription: a probe\nstateful: false\nlicense: MIT\n\
             transport:\n  cli:\n    binary: aware-probe\ncommands:\n  ping:\n    lifecycle: single\n    \
             description: ping the probe\n    mode: read\n",
        )
        .unwrap();
    }

    /// #671: `agent update` used to clear and rewrite EVERY installed agent's command
    /// files, so one update cost as much as the whole store (~28 s with 23,000 command
    /// files). Re-checking one agent must leave every other agent's files alone — a
    /// sentinel in another agent's file survives, a stale file of the named agent is fixed.
    #[test]
    fn refreshing_one_agent_leaves_every_other_agents_files_alone() {
        let tmp = populate_aware_home_from_fixtures();
        add_probe_agent(&tmp.path().join("aware"));
        let paths = Paths {
            aware_home: tmp.path().join("aware"),
        };
        let agents = discover_agents(&paths).unwrap().agents;
        let plugin_root = tmp.path().join("plugins");
        generate(&agents, &plugin_root, Refresh::Missing).unwrap();
        let commands = plugin_root.join("aware-aeco/commands");

        let other = commands.join("tekla-watch.md");
        let named = commands.join("probe-ping.md");
        std::fs::write(&other, "SENTINEL").unwrap();
        std::fs::write(&named, "STALE").unwrap();

        let ids = ["probe".to_string()];
        generate(&agents, &plugin_root, Refresh::Agents(&ids)).unwrap();

        assert_eq!(
            std::fs::read_to_string(&other).unwrap(),
            "SENTINEL",
            "an update of one agent must not rewrite another agent's command files"
        );
        assert!(
            std::fs::read_to_string(&named)
                .unwrap()
                .contains("name: probe-ping"),
            "the named agent's stale command file is rewritten from its manifest"
        );
    }

    #[test]
    fn refreshing_an_agent_prunes_the_commands_it_dropped() {
        let tmp = populate_aware_home_from_fixtures();
        add_probe_agent(&tmp.path().join("aware"));
        let paths = Paths {
            aware_home: tmp.path().join("aware"),
        };
        let agents = discover_agents(&paths).unwrap().agents;
        let plugin_root = tmp.path().join("plugins");
        generate(&agents, &plugin_root, Refresh::Missing).unwrap();
        let commands = plugin_root.join("aware-aeco/commands");

        // The previous version of `probe` had a command the new one no longer declares.
        let dropped = commands.join("probe-removed.md");
        std::fs::write(&dropped, "old").unwrap();

        let ids = ["probe".to_string()];
        generate(&agents, &plugin_root, Refresh::Agents(&ids)).unwrap();

        assert!(!dropped.exists(), "a dropped command's file is pruned");
        let json = std::fs::read_to_string(plugin_root.join("aware-aeco/plugin.json")).unwrap();
        assert!(json.contains("probe-ping") && !json.contains("probe-removed"));
    }
}
