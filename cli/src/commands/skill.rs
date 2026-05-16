//! `aware skill ...` — skill commands.

use clap::Subcommand;

use crate::context::Context;
use crate::error::AwareError;

#[derive(Subcommand, Debug)]
pub enum SkillCommand {
    /// Scaffold a new skill template in an installed agent.
    Create {
        agent: String,
        #[arg(name = "skill-name")]
        skill_name: String,
    },
    /// Port a skill into an agent.
    Port {
        /// Source: `<agent>/<skill>` (installed) or path to a `.md` file.
        source: String,
        #[arg(name = "target-agent")]
        target_agent: String,
    },
    /// Open a skill in $EDITOR.
    Modify {
        agent: String,
        #[arg(name = "skill-name")]
        skill_name: String,
    },
    /// Run a trigger-match eval against the skill.
    Eval {
        agent: String,
        #[arg(name = "skill-name")]
        skill_name: String,
    },
}

pub fn dispatch(cmd: SkillCommand, ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        SkillCommand::Create { agent, skill_name } => {
            crate::skill_builder::create::run(ctx, &agent, &skill_name)
        }
        SkillCommand::Port {
            source,
            target_agent,
        } => crate::skill_builder::port::run(ctx, &source, &target_agent),
        SkillCommand::Modify { agent, skill_name } => {
            crate::skill_builder::modify::run(ctx, &agent, &skill_name)
        }
        SkillCommand::Eval { agent, skill_name } => {
            crate::skill_builder::eval::run(ctx, &agent, &skill_name)
        }
    }
}
