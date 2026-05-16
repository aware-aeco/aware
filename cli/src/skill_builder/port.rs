//! `aware skill port <source> <target-agent>` — copy a skill into an agent, adapting frontmatter.

use std::path::PathBuf;

use crate::context::Context;
use crate::error::AwareError;

pub fn run(ctx: &Context, source: &str, target_agent: &str) -> Result<(), AwareError> {
    // Resolve source: try as a literal path first; if not, treat as <agent>/<skill>
    let source_path = if std::path::Path::new(source).is_file() {
        PathBuf::from(source)
    } else if let Some((sa, sn)) = source.split_once('/') {
        let stem = sn.trim_end_matches(".md");
        ctx.paths
            .agents_dir()
            .join(sa)
            .join("skills")
            .join(format!("{stem}.md"))
    } else {
        return Err(AwareError::NotFound(format!("source skill: {source}")));
    };
    if !source_path.is_file() {
        return Err(AwareError::NotFound(format!(
            "source skill: {}",
            source_path.display()
        )));
    }

    let target_dir = ctx.paths.agents_dir().join(target_agent);
    if !target_dir.is_dir() {
        return Err(AwareError::NotFound(format!("agent {target_agent}")));
    }
    let skills_dir = target_dir.join("skills");
    std::fs::create_dir_all(&skills_dir)?;
    let basename = source_path
        .file_name()
        .ok_or_else(|| AwareError::Validation("source path has no filename".into()))?;
    let target_path = skills_dir.join(basename);
    if target_path.exists() {
        return Err(AwareError::Conflict(format!(
            "target {} already exists",
            target_path.display()
        )));
    }

    let body = std::fs::read_to_string(&source_path)?;
    let stem = source_path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let new_name = format!("name: {target_agent}-{stem}");
    let adapted = adapt_frontmatter_name(&body, &new_name);

    std::fs::write(&target_path, adapted)?;
    println!(
        "\u{2713} ported {} \u{2192} {}",
        source_path.display(),
        target_path.display()
    );
    Ok(())
}

fn adapt_frontmatter_name(body: &str, new_name_line: &str) -> String {
    let mut lines: Vec<String> = body.lines().map(String::from).collect();
    let mut in_fm = false;
    for line in lines.iter_mut() {
        if line.trim() == "---" {
            in_fm = !in_fm;
            continue;
        }
        if in_fm && line.trim_start().starts_with("name:") {
            *line = new_name_line.to_string();
        }
    }
    let mut out = lines.join("\n");
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::Paths;

    fn aware_home_with_two_agents() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let aware = tmp.path().join("aware");
        for a in ["source-agent", "target-agent"] {
            std::fs::create_dir_all(aware.join("agents").join(a).join("skills")).unwrap();
            std::fs::write(
                aware.join("agents").join(a).join("manifest.yaml"),
                "agent: x\nversion: 0.0.1\ndescription: y\nstateful: false\nlicense: MIT\ntransport: { cli: { binary: x } }\ncommands: { do: { lifecycle: single, description: x } }\n",
            ).unwrap();
        }
        std::fs::write(
            aware.join("agents/source-agent/skills/my-skill.md"),
            "---\nname: source-agent-my-skill\ndescription: original\n---\n\nBody\n",
        )
        .unwrap();
        tmp
    }

    #[test]
    fn ports_from_agent_slash_skill_form() {
        let tmp = aware_home_with_two_agents();
        let ctx = Context {
            paths: Paths {
                aware_home: tmp.path().join("aware"),
            },
            json: false,
        };
        run(&ctx, "source-agent/my-skill", "target-agent").unwrap();
        let target_body = std::fs::read_to_string(
            tmp.path()
                .join("aware/agents/target-agent/skills/my-skill.md"),
        )
        .unwrap();
        assert!(target_body.contains("name: target-agent-my-skill"));
        assert!(target_body.contains("Body"));
    }

    #[test]
    fn ports_from_external_file_path() {
        let tmp = aware_home_with_two_agents();
        let ctx = Context {
            paths: Paths {
                aware_home: tmp.path().join("aware"),
            },
            json: false,
        };
        let ext = tmp.path().join("external-skill.md");
        std::fs::write(
            &ext,
            "---\nname: foo\ndescription: bar\n---\n\nExternal body\n",
        )
        .unwrap();
        run(&ctx, ext.to_str().unwrap(), "target-agent").unwrap();
        let target_body = std::fs::read_to_string(
            tmp.path()
                .join("aware/agents/target-agent/skills/external-skill.md"),
        )
        .unwrap();
        assert!(target_body.contains("name: target-agent-external-skill"));
        assert!(target_body.contains("External body"));
    }

    #[test]
    fn target_collision_is_conflict() {
        let tmp = aware_home_with_two_agents();
        let ctx = Context {
            paths: Paths {
                aware_home: tmp.path().join("aware"),
            },
            json: false,
        };
        // Pre-create the target
        std::fs::write(
            tmp.path()
                .join("aware/agents/target-agent/skills/my-skill.md"),
            "existing\n",
        )
        .unwrap();
        let err = run(&ctx, "source-agent/my-skill", "target-agent").unwrap_err();
        assert!(matches!(err, AwareError::Conflict(_)));
    }

    #[test]
    fn missing_target_agent_is_not_found() {
        let tmp = aware_home_with_two_agents();
        let ctx = Context {
            paths: Paths {
                aware_home: tmp.path().join("aware"),
            },
            json: false,
        };
        let err = run(&ctx, "source-agent/my-skill", "no-such-agent").unwrap_err();
        assert!(matches!(err, AwareError::NotFound(_)));
    }
}
