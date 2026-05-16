//! `aware skill eval <agent> <skill-name>` — run trigger-corpus eval.

use crate::context::Context;
use crate::error::AwareError;

pub fn run(ctx: &Context, agent_id: &str, skill_name: &str) -> Result<(), AwareError> {
    let stem = skill_name.trim_end_matches(".md");
    let skills_dir = ctx.paths.agents_dir().join(agent_id).join("skills");
    let skill_path = skills_dir.join(format!("{stem}.md"));
    let eval_path = skills_dir.join(format!("{stem}.eval.md"));
    if !skill_path.is_file() {
        return Err(AwareError::NotFound(format!("skill {agent_id}/{stem}")));
    }
    if !eval_path.is_file() {
        println!(
            "No eval corpus at {}. Creating template; edit and re-run.",
            eval_path.display()
        );
        let tpl = "---\ntriggers:\n  - \"example phrase 1\"\n  - \"example phrase 2\"\n---\n\nAdd one trigger phrase per bullet above.\n";
        std::fs::write(&eval_path, tpl)?;
        return Ok(());
    }

    let skill_body = std::fs::read_to_string(&skill_path)?;
    let skill_desc = extract_frontmatter_field(&skill_body, "description").unwrap_or_default();
    let eval_body = std::fs::read_to_string(&eval_path)?;
    let triggers = extract_trigger_list(&eval_body);

    println!(
        "Evaluating {agent_id}/{stem} against {} triggers...",
        triggers.len()
    );
    let mut hits = 0usize;
    for t in &triggers {
        let pass = matches_trigger(&skill_desc, t);
        let mark = if pass {
            hits += 1;
            "\u{2713}"
        } else {
            "\u{2717}"
        };
        println!("  {mark} {t}");
    }
    println!("\n{hits} / {} triggers matched.", triggers.len());
    Ok(())
}

fn extract_frontmatter_field(body: &str, key: &str) -> Option<String> {
    let mut in_fm = false;
    for line in body.lines() {
        if line.trim() == "---" {
            if in_fm {
                return None;
            }
            in_fm = true;
            continue;
        }
        if in_fm && line.trim_start().starts_with(&format!("{key}:")) {
            return Some(
                line.trim_start()
                    .trim_start_matches(&format!("{key}:"))
                    .trim()
                    .to_string(),
            );
        }
    }
    None
}

fn extract_trigger_list(eval_body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut in_triggers = false;
    let mut in_fm = false;
    for line in eval_body.lines() {
        if line.trim() == "---" {
            in_fm = !in_fm;
            continue;
        }
        if !in_fm {
            continue;
        }
        if line.trim_start().starts_with("triggers:") {
            in_triggers = true;
            continue;
        }
        if in_triggers {
            if let Some(t) = line.trim().strip_prefix("- ") {
                out.push(t.trim().trim_matches('"').to_string());
            } else if !line.starts_with(' ') {
                in_triggers = false;
            }
        }
    }
    out
}

fn matches_trigger(description: &str, trigger: &str) -> bool {
    let d = description.to_lowercase();
    let t = trigger.to_lowercase();
    if d.contains(&t) {
        return true;
    }
    let dt: std::collections::HashSet<&str> = d.split_whitespace().collect();
    let tt: Vec<&str> = t.split_whitespace().collect();
    if tt.is_empty() {
        return false;
    }
    let overlap = tt.iter().filter(|w| dt.contains(*w)).count();
    overlap * 2 >= tt.len() // 50% threshold
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::Paths;

    fn ctx_with_skill_and_eval(tmp: &tempfile::TempDir, with_eval: bool) -> Context {
        let aware = tmp.path().join("aware");
        let skills_dir = aware.join("agents/test-agent/skills");
        std::fs::create_dir_all(&skills_dir).unwrap();
        std::fs::write(
            skills_dir.join("connection-rules.md"),
            "---\nname: test-agent-connection-rules\ndescription: Apply connection rules when assembling welded steel structures\n---\n\nbody\n",
        )
        .unwrap();
        if with_eval {
            std::fs::write(
                skills_dir.join("connection-rules.eval.md"),
                "---\ntriggers:\n  - \"welded steel\"\n  - \"connection rules\"\n  - \"unrelated python typing\"\n---\n\nNotes.\n",
            )
            .unwrap();
        }
        Context {
            paths: Paths { aware_home: aware },
            json: false,
        }
    }

    #[test]
    fn creates_eval_template_when_missing() {
        let tmp = tempfile::tempdir().unwrap();
        let ctx = ctx_with_skill_and_eval(&tmp, false);
        run(&ctx, "test-agent", "connection-rules").unwrap();
        let eval_path = tmp
            .path()
            .join("aware/agents/test-agent/skills/connection-rules.eval.md");
        assert!(eval_path.is_file());
        let body = std::fs::read_to_string(&eval_path).unwrap();
        assert!(body.contains("triggers:"));
    }

    #[test]
    fn runs_against_existing_eval_corpus() {
        let tmp = tempfile::tempdir().unwrap();
        let ctx = ctx_with_skill_and_eval(&tmp, true);
        // Just confirm it runs without error; the matcher output goes to stdout
        run(&ctx, "test-agent", "connection-rules").unwrap();
    }

    #[test]
    fn matches_substring_trigger() {
        assert!(matches_trigger("Apply welded steel rules", "welded steel"));
    }

    #[test]
    fn matches_token_overlap_50_percent() {
        // "welded structures" has 1/2 token overlap with description's "welded steel structures" → 50%
        assert!(matches_trigger(
            "welded steel structures",
            "welded structures"
        ));
    }

    #[test]
    fn does_not_match_unrelated() {
        assert!(!matches_trigger("welded steel rules", "python typing"));
    }

    #[test]
    fn missing_skill_is_not_found() {
        let tmp = tempfile::tempdir().unwrap();
        let aware = tmp.path().join("aware");
        std::fs::create_dir_all(aware.join("agents/test-agent/skills")).unwrap();
        let ctx = Context {
            paths: Paths { aware_home: aware },
            json: false,
        };
        let err = run(&ctx, "test-agent", "nope").unwrap_err();
        assert!(matches!(err, AwareError::NotFound(_)));
    }

    #[test]
    fn extract_trigger_list_parses_quoted_strings() {
        let body = "---\ntriggers:\n  - \"first\"\n  - \"second\"\n---\n";
        let t = extract_trigger_list(body);
        assert_eq!(t, vec!["first", "second"]);
    }
}
