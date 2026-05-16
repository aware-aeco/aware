//! Skill commands: create / port / modify / eval.
//!
//! Pure scaffolding + helpers. Full AI-driven skill generation is delegated
//! to the user's own agentic-CLI tooling (Claude Code, Codex, etc.); AWARE
//! provides templates + porting + a trigger-corpus eval matcher.

#![allow(dead_code)]

pub mod create;
pub mod eval;
pub mod modify;
pub mod port;
