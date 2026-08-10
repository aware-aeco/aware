//! Claude Code plugin generator.
//!
//! Writes `<plugin_root>/aware-aeco/plugin.json` + per-agent-command markdown
//! files under `commands/`. Idempotent: re-running produces byte-identical output.

use std::path::Path;

use crate::error::AwareError;
use crate::manifest::loader::DiscoveredAgent;

/// (Re)generate the Claude Code plugin from `agents`.
///
/// Incremental by default: writes only the command files that are MISSING and removes
/// only those that are orphaned, so installing or uninstalling one agent no longer
/// rewrites every other agent's command files — with the big reflected agents that was
/// tens of thousands of file writes per install (~54s, #244). `plugin.json` is always
/// rewritten (one small file) so the command index stays exact, and orphaned files from
/// uninstalled agents are pruned, keeping the output self-healing for add/remove.
///
/// Pass `full = true` to first clear every command file, forcing a clean rewrite — used
/// on `agent update`, where an existing command's description may have changed (the
/// presence-based path would otherwise skip a file whose name is unchanged).
pub fn generate(
    agents: &[DiscoveredAgent],
    plugin_root: &Path,
    full: bool,
) -> Result<usize, AwareError> {
    use std::collections::BTreeMap;

    let aware_aeco_dir = plugin_root.join("aware-aeco");
    let commands_dir = aware_aeco_dir.join("commands");
    if full {
        // Force a clean rewrite of every command file (descriptions may have changed).
        let _ = std::fs::remove_dir_all(&commands_dir);
    }
    std::fs::create_dir_all(&commands_dir)?;

    // The exact set of command files this agent set should produce (sorted by name via
    // BTreeMap, so plugin.json's index is byte-stable). File name → content.
    let mut desired: BTreeMap<String, String> = BTreeMap::new();
    for agent in agents {
        for (cmd_name, cmd_spec) in &agent.manifest.commands {
            let file_name = format!("{}-{}.md", agent.manifest.agent, cmd_name);
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

    // Write only the missing files (presence-based — the whole point of #244).
    for (file_name, body) in &desired {
        let path = commands_dir.join(file_name);
        if !path.is_file() {
            std::fs::write(&path, body)?;
        }
    }

    // Prune orphaned command files (agents/commands that are no longer installed).
    if let Ok(rd) = std::fs::read_dir(&commands_dir) {
        for entry in rd.flatten() {
            let name = entry.file_name();
            let Some(name) = name.to_str() else { continue };
            if name.ends_with(".md") && !desired.contains_key(name) {
                let _ = std::fs::remove_file(entry.path());
            }
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
    use crate::fs_tree::copy_dir_recursive;
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
        copy_dir_recursive(&tekla_src, &aware.join("agents/tekla")).unwrap();
        tmp
    }

    #[test]
    fn generates_plugin_json_with_command_index() {
        let tmp = populate_aware_home_from_fixtures();
        let paths = Paths {
            aware_home: tmp.path().join("aware"),
        };
        let agents = discover_agents(&paths).unwrap();

        let plugin_root = tmp.path().join("plugins");
        let count = generate(&agents, &plugin_root, false).unwrap();

        // Tekla currently has 24 curated commands (grew from 23 with `bake-scene`, #235).
        assert_eq!(count, 24);
        assert!(plugin_root.join("aware-aeco/plugin.json").is_file());

        let json: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(plugin_root.join("aware-aeco/plugin.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(json["name"], "aware-aeco");
        assert_eq!(json["commands"].as_array().unwrap().len(), 24);
    }

    #[test]
    fn generates_per_command_markdown() {
        let tmp = populate_aware_home_from_fixtures();
        let paths = Paths {
            aware_home: tmp.path().join("aware"),
        };
        let agents = discover_agents(&paths).unwrap();

        let plugin_root = tmp.path().join("plugins");
        generate(&agents, &plugin_root, false).unwrap();

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
        let agents = discover_agents(&paths).unwrap();

        let plugin_root = tmp.path().join("plugins");
        generate(&agents, &plugin_root, false).unwrap();
        let first_json = std::fs::read(plugin_root.join("aware-aeco/plugin.json")).unwrap();
        let first_md =
            std::fs::read(plugin_root.join("aware-aeco/commands/tekla-watch.md")).unwrap();

        generate(&agents, &plugin_root, false).unwrap();
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
        let agents = discover_agents(&paths).unwrap();
        let plugin_root = tmp.path().join("plugins");

        generate(&agents, &plugin_root, false).unwrap();
        let commands = plugin_root.join("aware-aeco/commands");
        let kept = commands.join("tekla-watch.md");
        assert!(kept.is_file());

        // Simulate an uninstalled agent's leftover command file + mark a kept file so we
        // can prove a re-run doesn't rewrite already-present files.
        let orphan = commands.join("ghost-removed.md");
        std::fs::write(&orphan, "stale").unwrap();
        std::fs::write(&kept, "SENTINEL").unwrap(); // presence-based: must NOT be overwritten

        generate(&agents, &plugin_root, false).unwrap();

        assert!(!orphan.exists(), "orphaned command file should be pruned");
        assert_eq!(
            std::fs::read_to_string(&kept).unwrap(),
            "SENTINEL",
            "an already-present command file must not be rewritten (presence-based)"
        );
        // plugin.json never lists the orphan.
        let json = std::fs::read_to_string(plugin_root.join("aware-aeco/plugin.json")).unwrap();
        assert!(!json.contains("ghost-removed"));

        // A `full` rebuild DOES refresh content (used on `agent update`).
        generate(&agents, &plugin_root, true).unwrap();
        assert!(
            std::fs::read_to_string(&kept)
                .unwrap()
                .contains("name: tekla-watch"),
            "full rebuild should rewrite command files from the manifest"
        );
    }
}
