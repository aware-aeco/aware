//! Claude Code plugin generator.
//!
//! Writes `<plugin_root>/aware-aeco/plugin.json` + per-agent-command markdown
//! files under `commands/`. Idempotent: re-running produces byte-identical output.

#![allow(dead_code)]

use std::path::Path;

use crate::error::AwareError;
use crate::manifest::loader::DiscoveredAgent;

pub fn generate(agents: &[DiscoveredAgent], plugin_root: &Path) -> Result<usize, AwareError> {
    let aware_aeco_dir = plugin_root.join("aware-aeco");
    if aware_aeco_dir.exists() {
        std::fs::remove_dir_all(&aware_aeco_dir)?;
    }
    std::fs::create_dir_all(aware_aeco_dir.join("commands"))?;

    let mut commands_index: Vec<String> = Vec::new();
    for agent in agents {
        // Sort commands by name for stable byte-identical output across runs
        let mut cmd_names: Vec<&String> = agent.manifest.commands.keys().collect();
        cmd_names.sort();
        for cmd_name in cmd_names {
            let cmd_spec = &agent.manifest.commands[cmd_name];
            let file_name = format!("{}-{}.md", agent.manifest.agent, cmd_name);
            commands_index.push(format!("commands/{file_name}"));

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
            std::fs::write(aware_aeco_dir.join("commands").join(&file_name), body)?;
        }
    }

    commands_index.sort();
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

    Ok(agents.iter().map(|a| a.manifest.commands.len()).sum())
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
        copy_dir_recursive(&tekla_src, &aware.join("agents/tekla")).unwrap();
        tmp
    }

    fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
        std::fs::create_dir_all(dst)?;
        for entry in std::fs::read_dir(src)?.flatten() {
            let from = entry.path();
            let to = dst.join(entry.file_name());
            if entry.file_type()?.is_dir() {
                copy_dir_recursive(&from, &to)?;
            } else {
                std::fs::copy(&from, &to)?;
            }
        }
        Ok(())
    }

    #[test]
    fn generates_plugin_json_with_command_index() {
        let tmp = populate_aware_home_from_fixtures();
        let paths = Paths {
            aware_home: tmp.path().join("aware"),
        };
        let agents = discover_agents(&paths).unwrap();

        let plugin_root = tmp.path().join("plugins");
        let count = generate(&agents, &plugin_root).unwrap();

        // Tekla has 20 curated commands (3 original + 17 promoted in v0.16).
        assert_eq!(count, 20);
        assert!(plugin_root.join("aware-aeco/plugin.json").is_file());

        let json: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(plugin_root.join("aware-aeco/plugin.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(json["name"], "aware-aeco");
        assert_eq!(json["commands"].as_array().unwrap().len(), 20);
    }

    #[test]
    fn generates_per_command_markdown() {
        let tmp = populate_aware_home_from_fixtures();
        let paths = Paths {
            aware_home: tmp.path().join("aware"),
        };
        let agents = discover_agents(&paths).unwrap();

        let plugin_root = tmp.path().join("plugins");
        generate(&agents, &plugin_root).unwrap();

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
        generate(&agents, &plugin_root).unwrap();
        let first_json = std::fs::read(plugin_root.join("aware-aeco/plugin.json")).unwrap();
        let first_md =
            std::fs::read(plugin_root.join("aware-aeco/commands/tekla-watch.md")).unwrap();

        generate(&agents, &plugin_root).unwrap();
        let second_json = std::fs::read(plugin_root.join("aware-aeco/plugin.json")).unwrap();
        let second_md =
            std::fs::read(plugin_root.join("aware-aeco/commands/tekla-watch.md")).unwrap();

        assert_eq!(first_json, second_json);
        assert_eq!(first_md, second_md);
    }
}
