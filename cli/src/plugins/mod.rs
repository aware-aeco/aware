//! Host plugin generators for claude-code / codex / opencode.
//!
//! `claude_code` is a real generator. Codex and OpenCode have no settled plugin
//! format yet, so both write the same placeholder marker — one body in
//! `scaffold`, parameterized by the host's name, rather than a file each.

pub mod claude_code;
pub mod scaffold;
