//! `aware skill ...` — skill-builder commands.
//!
//! Phase v0.5. Wraps Anthropic's `skill-creator` with AWARE conventions.
//! See `20-agents/_core/aware-skill-builder/` for the pipeline spec.

use clap::Subcommand;

use crate::error::AwareError;

#[derive(Subcommand, Debug)]
pub enum SkillCommand {
    /// Author a new skill in a target agent.
    Create {
        /// Target agent id (e.g. `tekla`, `trimble-connect`).
        agent: String,
        /// Short filename without extension (e.g. `coordinate-systems`).
        skill_name: String,
        /// One-paragraph subject of the skill.
        #[arg(long)]
        topic: String,
        /// Optional notes file to seed the skill.
        #[arg(long)]
        notes_file: Option<std::path::PathBuf>,
        /// Tier classification: 1 = judgment/gotcha, 2 = API reference.
        #[arg(long, default_value_t = 1)]
        tier: u8,
    },
    /// Port an existing skill from an external source.
    Port {
        /// Path to the source markdown file.
        source: std::path::PathBuf,
        /// Which AWARE agent receives the skill.
        target_agent: String,
        /// Optional override for the skill filename.
        #[arg(long)]
        target_name: Option<String>,
        /// Source kind: floless | vendor-doc | contributor-notes | other.
        #[arg(long, default_value = "other")]
        source_kind: String,
    },
    /// Refine an existing skill (re-runs skill-creator + eval).
    Modify {
        agent: String,
        skill_name: String,
        /// What's wrong? (e.g. "doesn't trigger on X", "missing gotcha Y").
        #[arg(long)]
        issue: String,
    },
    /// Run skill-creator's eval pass on an existing skill.
    Eval {
        agent: String,
        skill_name: String,
        /// Path to a YAML file with test-prompts (see commands/eval.md).
        #[arg(long)]
        test_prompts: Option<std::path::PathBuf>,
    },
}

pub fn dispatch(cmd: SkillCommand) -> Result<(), AwareError> {
    match cmd {
        SkillCommand::Create { .. } => Err(AwareError::NotYetImplemented("skill create")),
        SkillCommand::Port { .. } => Err(AwareError::NotYetImplemented("skill port")),
        SkillCommand::Modify { .. } => Err(AwareError::NotYetImplemented("skill modify")),
        SkillCommand::Eval { .. } => Err(AwareError::NotYetImplemented("skill eval")),
    }
}
