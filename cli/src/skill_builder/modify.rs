//! `aware skill modify <agent> <skill-name>` — open the skill in $EDITOR.

use crate::context::Context;
use crate::error::AwareError;

pub fn run(ctx: &Context, agent_id: &str, skill_name: &str) -> Result<(), AwareError> {
    let stem = skill_name.trim_end_matches(".md");
    let path = ctx
        .paths
        .agents_dir()
        .join(agent_id)
        .join("skills")
        .join(format!("{stem}.md"));
    if !path.is_file() {
        return Err(AwareError::NotFound(format!("skill {agent_id}/{stem}")));
    }
    let editor = std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| {
            if cfg!(windows) {
                "notepad".to_string()
            } else {
                "vi".to_string()
            }
        });
    println!("Opening {} in {editor}...", path.display());
    let status = std::process::Command::new(&editor)
        .arg(&path)
        .status()
        .map_err(|e| AwareError::Network(format!("spawn {editor}: {e}")))?;
    if !status.success() {
        return Err(AwareError::Network(format!(
            "{editor} exited {:?}",
            status.code()
        )));
    }
    println!("\u{2713} saved {}", path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::Paths;

    fn ctx_with_skill(tmp: &tempfile::TempDir) -> (Context, std::path::PathBuf) {
        let aware = tmp.path().join("aware");
        std::fs::create_dir_all(aware.join("agents/test-agent/skills")).unwrap();
        let skill = aware.join("agents/test-agent/skills/example.md");
        std::fs::write(
            &skill,
            "---\nname: test-agent-example\ndescription: x\n---\n\nbody\n",
        )
        .unwrap();
        (
            Context {
                paths: Paths { aware_home: aware },
                json: false,
            },
            skill,
        )
    }

    #[test]
    fn missing_skill_is_not_found() {
        let tmp = tempfile::tempdir().unwrap();
        let (ctx, _) = ctx_with_skill(&tmp);
        let err = run(&ctx, "test-agent", "nope").unwrap_err();
        assert!(matches!(err, AwareError::NotFound(_)));
    }

    // We don't unit-test the actual editor spawn here — it'd require a working editor in test env.
    // Manual smoke covers it.
}
