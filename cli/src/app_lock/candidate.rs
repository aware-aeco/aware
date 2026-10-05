//! Candidate compilation (#628 plan §1): compile an approved app's UNCHANGED
//! source against a chosen set of agent pins, in memory, writing nothing.
//!
//! A candidate is never an approval. It reuses the compile path a person's
//! `aware app compile` uses — the same source snapshot, the same validators,
//! the same `compile_snapshot` — but resolves agents with
//! [`resolve_pins`](crate::agent_resolution::resolve_pins): the target agents
//! at their target bytes, every other agent at the base lock's own bytes, and
//! only from verified store packages (never the working copy, never a new
//! snapshot). What a person would have had to fix before compiling is reported
//! in [`Candidate::blocked`], as data.

use super::*;
use crate::agent_resolution::{PinSet, PinSource, resolve_pins};
use crate::migration::Reason;
use crate::migration::contract::PinRef;

/// The header recorded beside a candidate (in its evidence file): which
/// approval it starts from, what it moves, and which bytes it is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CandidateHeader {
    pub format: String,
    pub app: String,
    /// `sha256:` of the exact bytes of the `<app>.lock` this candidate starts from.
    pub base_lock_digest: String,
    /// The approved source hash; the candidate compiles exactly that source.
    pub base_source_hash: String,
    /// Each agent whose pin moves: from the base pin to the candidate's.
    pub targets: BTreeMap<String, PinMove>,
    /// `sha256:` of the candidate file's exact bytes.
    pub candidate_digest: String,
    /// [`plan_digest`] of the candidate lock.
    pub plan_digest: String,
}

/// The format tag of [`CandidateHeader`].
pub const CANDIDATE_FORMAT: &str = "aware.migration-candidate/v1";

/// One moved pin.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinMove {
    pub from: PinRef,
    pub to: PinRef,
}

/// One resolved pin of a candidate, without its loaded manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidatePin {
    pub agent: String,
    pub version: String,
    pub digest: String,
    pub root: std::path::PathBuf,
    pub source: PinSource,
    /// Stored copies that claimed these bytes or this version but did not
    /// verify — skipped, never used, reported.
    pub invalid_candidates: Vec<crate::agent_resolution::InvalidCandidate>,
}

/// A compiled candidate, in memory.
#[derive(Debug)]
pub struct Candidate {
    pub lock: LockFile,
    pub header: CandidateHeader,
    /// The exact bytes `aware app migrate prepare` writes as the candidate.
    pub bytes: Vec<u8>,
    /// What a person must fix before this candidate could ever be carried
    /// forward; empty when nothing blocks it.
    pub blocked: Vec<Reason>,
    /// Every dispatchable agent, as resolved for the candidate.
    pub pins: Vec<CandidatePin>,
    /// The catalogue the candidate compiled against (manifests from the store).
    pub agents: Vec<DiscoveredAgent>,
}

/// Compile the source at `source` — which must be the exact source `base`
/// approved — against `pins` (the base lock's pins with targets applied).
///
/// `Err` when the candidate cannot be compiled at all: the source changed
/// since it was approved (`E_MIGRATE_SOURCE_CHANGED`), or a pin cannot be
/// resolved (`resolve_pins`'s refusals). A candidate that compiles but that a
/// person would have to fix is `Ok` with [`Candidate::blocked`] non-empty:
///
/// * a node that becomes write with no `safety:` block → `needs-source-edit`;
/// * a `requires:` pin the target version does not satisfy → `needs-source-edit`;
/// * an agent the candidate pins that this CLI cannot dispatch → `agent-unavailable`.
pub fn compile_candidate(
    source: &Path,
    paths: &Paths,
    base: &LockFile,
    base_bytes: &[u8],
    pins: &PinSet,
) -> Result<Candidate, AwareError> {
    let snapshot = read_source_snapshot(source)?;
    if !approval::source_matches(base, &snapshot.source_hash) || snapshot.app.app != base.app {
        return Err(AwareError::Validation(format!(
            "[E_MIGRATE_SOURCE_CHANGED] the workflow {} changed since it was approved (approved {}, now {}), so its approval cannot be carried forward; a person must compile it again",
            snapshot.app.app, base.source_hash, snapshot.source_hash
        )));
    }
    let app = &snapshot.app;
    let resolved = resolve_pins(paths, app, pins)?;
    let mut digests: BTreeMap<String, String> = BTreeMap::new();
    let mut candidate_pins = Vec::new();
    let mut agents = Vec::new();
    for pin in resolved {
        let id = pin.agent.manifest.agent.clone();
        digests.insert(id.clone(), pin.digest.clone());
        candidate_pins.push(CandidatePin {
            agent: id,
            version: pin.version,
            digest: pin.digest,
            root: pin.agent.root.clone(),
            source: pin.source,
            invalid_candidates: pin.invalid_candidates,
        });
        agents.push(pin.agent);
    }

    // Agents only frozen nodes reference: never dispatched, so they keep the
    // base pin. Compile against the stored base bytes when they verify; else
    // carry the base lock's pin and compiled nodes over verbatim (plan §1).
    let mut flat: Vec<FlatNode> = Vec::new();
    flatten_nodes(&app.nodes, None, &[], &mut flat);
    let referenced: BTreeSet<&str> = flat
        .iter()
        .filter_map(|(node, _, _, _)| node.agent.as_deref())
        .collect();
    let dispatched: BTreeSet<&str> = crate::validate::dispatchable_agents(app)
        .into_iter()
        .collect();
    let mut verbatim = Vec::new();
    for id in referenced.difference(&dispatched) {
        let Some(base_pin) = pins.base.get(*id) else {
            continue; // never pinned by the approval: the candidate pins nothing either
        };
        let stored = match &base_pin.digest {
            Some(digest) => crate::agent_resolution::stored_agent(paths, id, digest)?
                .filter(|agent| agent.manifest.version == base_pin.version)
                .map(|agent| (digest.clone(), agent)),
            None => None,
        };
        match stored {
            Some((digest, agent)) => {
                digests.insert((*id).to_string(), digest);
                agents.push(agent);
            }
            None => verbatim.push((*id).to_string()),
        }
    }

    let mut blocked = Vec::new();
    let errors = |issues: Vec<crate::validate::ValidationIssue>| {
        issues
            .into_iter()
            .filter(|issue| issue.severity == crate::validate::Severity::Error)
            .collect::<Vec<_>>()
    };
    for issue in errors(crate::validate::validate_app(app)) {
        blocked.push(Reason::new(
            "needs-source-edit",
            format!(
                "The workflow no longer validates ({}): {}",
                issue.code, issue.message
            ),
        ));
    }
    for issue in errors(crate::validate::validate_app_agents(app, &agents)) {
        blocked.push(Reason::new(
            "agent-unavailable",
            format!(
                "A tool version this update would use cannot run here ({}): {}",
                issue.code, issue.message
            ),
        ));
    }
    for issue in crate::validate::unsatisfied_pins(app, &agents, crate::validate::Severity::Error) {
        blocked.push(Reason::new(
            "needs-source-edit",
            format!(
                "The workflow's requires: list does not allow the new tool version, so the workflow must be edited first ({}): {}",
                issue.code, issue.message
            ),
        ));
    }
    for issue in errors(crate::validate::validate_app_safety(app, &agents)) {
        blocked.push(Reason::new(
            "needs-source-edit",
            format!(
                "Under the new tool version a step would write without a safety: block, so the workflow must be edited first ({}): {}",
                issue.code, issue.message
            ),
        ));
    }

    let mut lock = compile_snapshot(app, &agents, snapshot.source_hash, &digests)?;
    for id in &verbatim {
        carry_over_verbatim(&mut lock, base, id)?;
    }

    let mut targets = BTreeMap::new();
    for pin in &candidate_pins {
        let Some(base_pin) = pins.base.get(&pin.agent) else {
            continue;
        };
        let from = base_pin.digest.clone().unwrap_or_default();
        if from != pin.digest {
            targets.insert(
                pin.agent.clone(),
                PinMove {
                    from: PinRef {
                        version: base_pin.version.clone(),
                        digest: from,
                    },
                    to: PinRef {
                        version: pin.version.clone(),
                        digest: pin.digest.clone(),
                    },
                },
            );
        }
    }
    let comment = format!(
        "# {app}.candidate.lock — a migration CANDIDATE, not an approval.\n\
         # `aware app run` never reads this file; only a promotion can make it the approved {app}.lock.\n\n",
        app = lock.app
    );
    let bytes = render_lock(&lock, &comment)?;
    let header = CandidateHeader {
        format: CANDIDATE_FORMAT.to_string(),
        app: lock.app.clone(),
        base_lock_digest: lock_digest(base_bytes),
        base_source_hash: approval::approved_source_hash(base),
        targets,
        candidate_digest: lock_digest(&bytes),
        plan_digest: plan_digest(&lock)?,
    };
    Ok(Candidate {
        lock,
        header,
        bytes,
        blocked,
        pins: candidate_pins,
        agents,
    })
}

/// Keep `id`'s base pin and the base lock's compiled nodes for it, exactly.
fn carry_over_verbatim(lock: &mut LockFile, base: &LockFile, id: &str) -> Result<(), AwareError> {
    for (from, to) in [
        (&base.agent_pins, &mut lock.agent_pins),
        (&base.agent_digests, &mut lock.agent_digests),
        (&base.agent_bundle_pins, &mut lock.agent_bundle_pins),
    ] {
        match from.get(id) {
            Some(value) => {
                to.insert(id.to_string(), value.clone());
            }
            None => {
                to.remove(id);
            }
        }
    }
    for node in lock.nodes.iter_mut() {
        if node.agent.as_deref() != Some(id) {
            continue;
        }
        if let Some(original) = base.nodes.iter().find(|b| b.id == node.id) {
            let value = serde_yaml::to_value(original)
                .map_err(|e| AwareError::Internal(format!("copy compiled node: {e}")))?;
            *node = serde_yaml::from_value(value)
                .map_err(|e| AwareError::Internal(format!("copy compiled node: {e}")))?;
        }
    }
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests;
