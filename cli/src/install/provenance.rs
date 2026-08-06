//! Where an installed agent came from — the fact `update` needs and never had (#370).
//!
//! `aware agent update <id>` resolves the id in the registry and replaces whatever sits at
//! `agents/<id>/`. That is right for an agent installed FROM the registry and destructive for one
//! installed from a folder: a local fork whose `agent:` id is also a registry key was silently
//! overwritten with the registry's copy, exit 0, and the local work was unrecoverable — the staging
//! directory holds the registry payload, not yours.
//!
//! The existing hijack guard only covered the *suffix-fallback* case (a local `tekla.dev` resolving
//! to registry key `tekla`, #174). An EXACT id collision walked straight past it, because
//! `index.agents.contains_key(id)` is true and the guard's second clause never fires.
//!
//! Nothing recorded the answer, so nothing could ask. The manifest's own `provenance:` block is
//! generator metadata (`generated-by: aware-agent-builder`) — it says how the agent was AUTHORED,
//! not how it got onto this machine. This module records the second question.
//!
//! ## Where it lives, and why
//!
//! A dotfile inside the agent's own directory (`agents/<id>/.aware-install.yaml`) rather than a
//! central receipts file. It then has no lifecycle of its own: `uninstall`'s `remove_dir_all` takes
//! it, and `update`'s atomic staging-then-rename replaces it with the new one. There is no
//! `agent rename` or `agent duplicate` to carry it anywhere — those verbs are APP verbs and copy
//! `apps/`, never an agent directory. A central index would need every one of those paths to
//! remember to update it, which is the class of bug this file exists to close, not to reintroduce.
//!
//! Writing is BEST-EFFORT: an agent whose marker could not be written reads as "unknown", which
//! the guard judges conservatively. Failing an otherwise-good install over its metadata would
//! trade a recoverable ambiguity for a functional break.
//!
//! It is dot-prefixed so it cannot be mistaken for agent content: `validate_agent_on_disk` reads
//! `manifest.yaml`, skills and commands by name, and nothing walks for arbitrary files.

use std::path::Path;

use serde::{Deserialize, Serialize};

/// The filename, dot-prefixed so it reads as metadata rather than content.
pub(crate) const FILE: &str = ".aware-install.yaml";

/// How an installed agent got onto this machine.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "source", rename_all = "lowercase")]
pub enum InstallSource {
    /// Fetched from the registry. `key` is the registry entry (which may differ from the agent id —
    /// `allplan-2024` installs as `allplan-2024.0`); `version` is the key that was resolved.
    Registry { key: String, version: String },
    /// Copied from a folder on this machine. `path` is where, so a refusal can name it.
    Local { path: String },
}

/// Record where an agent was just installed from. Best-effort by design.
///
/// A failure here must not fail the install: the agent IS on disk and working, and losing the
/// marker degrades to "unknown provenance", which the reader already handles. Failing the install
/// instead would turn a metadata problem into a functional one — which is why this returns nothing
/// to check rather than a `Result` a caller might be tempted to `?`.
pub fn write(agent_dir: &Path, source: &InstallSource) {
    if let Ok(yaml) = serde_yaml::to_string(source) {
        let _ = std::fs::write(agent_dir.join(FILE), yaml);
    }
}

/// Where this agent came from, or `None` when nothing says.
///
/// `None` is not "registry" — it is the honest "this predates the marker, or it was lost". Callers
/// must decide what to do with an unknown rather than assuming the safe-for-them answer; see
/// `update_agent_from_registry`, which falls back to asking the registry whether the installed
/// VERSION is one it publishes.
pub fn read(agent_dir: &Path) -> Option<InstallSource> {
    let text = std::fs::read_to_string(agent_dir.join(FILE)).ok()?;
    serde_yaml::from_str(&text).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_written_source_reads_back_as_itself() {
        let tmp = tempfile::tempdir().unwrap();
        for source in [
            InstallSource::Registry {
                key: "tekla".into(),
                version: "2025.0.1".into(),
            },
            InstallSource::Local {
                path: "C:/work/my-fork".into(),
            },
        ] {
            write(tmp.path(), &source);
            assert_eq!(read(tmp.path()), Some(source));
        }
    }

    #[test]
    fn no_marker_reads_as_unknown_not_as_registry() {
        // The distinction the whole module turns on. An agent installed before this existed has no
        // marker, and calling that "registry" would be exactly the assumption that destroyed local
        // forks — quietly, and for the people most likely to have one.
        let tmp = tempfile::tempdir().unwrap();
        assert_eq!(read(tmp.path()), None);
    }

    #[test]
    fn an_unreadable_marker_reads_as_unknown_rather_than_erroring() {
        // A corrupt marker must not break `update` — it is metadata about an agent that is
        // otherwise fine. Unknown is the same answer as absent, and the caller's fallback covers
        // both.
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join(FILE), "source: [not\n  yaml\n").unwrap();
        assert_eq!(read(tmp.path()), None);
    }

    #[test]
    fn writing_is_best_effort_and_never_panics() {
        // Into a directory that does not exist — the install still succeeded, so this must be a
        // no-op rather than a failure.
        write(
            Path::new("no/such/directory/anywhere"),
            &InstallSource::Local { path: "x".into() },
        );
    }
}
