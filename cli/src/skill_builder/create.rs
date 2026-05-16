//! `aware skill create <agent> <skill-name>` — write a skill template.

use crate::context::Context;
use crate::error::AwareError;

pub fn run(ctx: &Context, agent_id: &str, skill_name: &str) -> Result<(), AwareError> {
    let agent_dir = ctx.paths.agents_dir().join(agent_id);
    if !agent_dir.is_dir() {
        return Err(AwareError::NotFound(format!("agent {agent_id}")));
    }
    let skills_dir = agent_dir.join("skills");
    std::fs::create_dir_all(&skills_dir)?;
    let stem = skill_name.trim_end_matches(".md");
    let path = skills_dir.join(format!("{stem}.md"));
    if path.exists() {
        return Err(AwareError::Conflict(format!(
            "skill {stem}.md already exists"
        )));
    }
    let template = format!(
        "---\nname: {agent_id}-{stem}\ndescription: TODO \u{2014} one paragraph; what task triggers this skill?\n---\n\n# {stem}\n\nTODO: replace this with the skill content. Lead with the rule, then explain.\n\nWhat triggers loading: TODO.\n\nWhy it matters: TODO.\n"
    );
    std::fs::write(&path, template)?;
    println!("\u{2713} created {}", path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::Paths;

    fn make_ctx_with_agent(tmp: &tempfile::TempDir, agent_id: &str) -> Context {
        let aware = tmp.path().join("aware");
        std::fs::create_dir_all(aware.join("agents").join(agent_id).join("skills")).unwrap();
        // Minimal manifest so the agent is discoverable (loader may not load it here, but
        // the create command checks dir existence, not manifest validity)
        std::fs::write(
            aware.join("agents").join(agent_id).join("manifest.yaml"),
            "agent: stub\nversion: 0.0.1\ndescription: x\nstateful: false\nlicense: MIT\ntransport: { cli: { binary: x } }\ncommands: { do: { lifecycle: single, description: x } }\n",
        ).unwrap();
        Context {
            paths: Paths { aware_home: aware },
            json: false,
        }
    }

    #[test]
    fn creates_skill_template() {
        let tmp = tempfile::tempdir().unwrap();
        let ctx = make_ctx_with_agent(&tmp, "tekla");
        run(&ctx, "tekla", "new-skill").unwrap();
        let path = tmp.path().join("aware/agents/tekla/skills/new-skill.md");
        assert!(path.is_file());
        let body = std::fs::read_to_string(&path).unwrap();
        assert!(body.contains("name: tekla-new-skill"));
        assert!(body.contains("TODO"));
    }

    #[test]
    fn missing_agent_is_not_found() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("aware/agents")).unwrap();
        let ctx = Context {
            paths: Paths {
                aware_home: tmp.path().join("aware"),
            },
            json: false,
        };
        let err = run(&ctx, "nope", "anything").unwrap_err();
        assert!(matches!(err, AwareError::NotFound(_)));
    }

    #[test]
    fn existing_skill_is_conflict() {
        let tmp = tempfile::tempdir().unwrap();
        let ctx = make_ctx_with_agent(&tmp, "tekla");
        run(&ctx, "tekla", "dup").unwrap();
        let err = run(&ctx, "tekla", "dup").unwrap_err();
        assert!(matches!(err, AwareError::Conflict(_)));
    }
}
