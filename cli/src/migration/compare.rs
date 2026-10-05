//! Fixed-state comparison (#628 plan §4, revised by §11 and §12).
//!
//! The question a migration must answer before a workflow moves without a
//! person: "does the new tool version give the same results on a fixed state?"
//! v1 has no executed method that answers it — the runtime has no `snapshot`
//! of host state and no recorded HTTP exchange to replay — so:
//!
//! * when the executable contract of a moved agent is unchanged, the verdict is
//!   [`ComparisonStatus::IdenticalInstructions`] (method `static-inspection`,
//!   `runs: 0`): both pins hand the same executor byte-identical instructions.
//!   That is a statement about INSTRUCTIONS, checked by inspection; nothing was
//!   run, and it is never reported as "pass" or "same results";
//! * when the contract changed, the verdict is
//!   [`ComparisonStatus::NotComparable`] with a transport-specific reason.
//!   Two live reads of a host are never compared: the host may have changed
//!   between them, so agreement would prove nothing and disagreement would
//!   blame the wrong thing.
//!
//! [`ComparisonStatus::Pass`] / [`ComparisonStatus::Fail`] are reserved for an
//! executed method (#628 PR4). Only [`ACCEPTED_FOR_POLICY`] statuses may ever
//! open the no-click path, and in v1 that is `Pass` alone — so the path is
//! implemented but closed until the owner ratifies `identical-instructions` or
//! an executed method ships.

use serde::Serialize;

use super::Reason;
use super::contract::{ContractDiff, Executor};

/// The method name of [`ComparisonStatus::IdenticalInstructions`].
pub const STATIC_INSPECTION: &str = "static-inspection";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComparisonStatus {
    /// Reserved: an executed fixed-state comparison found equal results.
    Pass,
    /// Reserved: an executed fixed-state comparison found different results.
    /// No v1 method executes, so nothing constructs it yet (#628 PR4).
    #[allow(dead_code)]
    Fail,
    /// Both pins hand the executor byte-identical instructions (inspection
    /// only; nothing executed). Distinct from `Pass` by design.
    IdenticalInstructions,
    /// No fixed-state comparison is possible; a person decides.
    NotComparable,
}

/// The statuses that may make a candidate eligible for the no-click path.
/// `IdenticalInstructions` is deliberately absent until the owner ratifies it
/// (#628 plan §11 Q1); enabling it is adding it here, with a test.
pub const ACCEPTED_FOR_POLICY: &[ComparisonStatus] = &[ComparisonStatus::Pass];

/// The verdict for one moved agent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AgentComparison {
    pub agent: String,
    pub status: ComparisonStatus,
    pub method: Option<&'static str>,
    pub runs: u32,
    pub reason: Reason,
}

/// The verdict for one candidate: every moved agent's, folded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Comparison {
    pub status: ComparisonStatus,
    pub method: Option<&'static str>,
    /// How many comparison runs were executed. Always 0 in v1.
    pub runs: u32,
    pub reason: Reason,
    pub per_agent: Vec<AgentComparison>,
}

impl Comparison {
    /// Whether this verdict may open the no-click path ([`ACCEPTED_FOR_POLICY`]).
    pub fn accepted_for_policy(&self) -> bool {
        ACCEPTED_FOR_POLICY.contains(&self.status)
            && self
                .per_agent
                .iter()
                .all(|a| ACCEPTED_FOR_POLICY.contains(&a.status))
    }
}

/// Compare every moved agent from its contract diff. `None` when nothing moved.
pub fn compare(diffs: &[ContractDiff]) -> Option<Comparison> {
    if diffs.is_empty() {
        return None;
    }
    let per_agent: Vec<AgentComparison> = diffs.iter().map(compare_agent).collect();
    let not_comparable: Vec<&AgentComparison> = per_agent
        .iter()
        .filter(|a| a.status != ComparisonStatus::IdenticalInstructions)
        .collect();
    let comparison = if not_comparable.is_empty() {
        Comparison {
            status: ComparisonStatus::IdenticalInstructions,
            method: Some(STATIC_INSPECTION),
            runs: 0,
            reason: Reason::new(
                "identical-instructions",
                "The tools' run instructions for this workflow are byte-identical under the old and the new versions (checked by inspection; nothing was run).",
            ),
            per_agent,
        }
    } else {
        let text = not_comparable
            .iter()
            .map(|a| a.reason.text.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        Comparison {
            status: ComparisonStatus::NotComparable,
            method: None,
            runs: 0,
            reason: Reason::new("not-comparable", text),
            per_agent,
        }
    };
    Some(comparison)
}

fn compare_agent(diff: &ContractDiff) -> AgentComparison {
    let agent = diff.agent.clone();
    let versions = format!("{} {} -> {}", agent, diff.from.version, diff.to.version);
    if diff.unchanged {
        return AgentComparison {
            reason: Reason::new(
                "identical-instructions",
                format!(
                    "{versions}: the run instructions this workflow uses are byte-identical (checked by inspection; nothing was run)."
                ),
            ),
            agent,
            status: ComparisonStatus::IdenticalInstructions,
            method: Some(STATIC_INSPECTION),
            runs: 0,
        };
    }
    let reason = match &diff.executor {
        Executor::Cli { .. } => Reason::new(
            "no-fixed-state",
            format!(
                "{versions}: the run instructions changed, and this tool works on live host state; AWARE has no fixed snapshot of that state to run both versions against (two live reads are never compared)."
            ),
        ),
        Executor::AwareCli { .. } => Reason::new(
            "no-recorded-exchange",
            format!(
                "{versions}: the run instructions changed, and this tool calls a web service; AWARE has no recorded exchange to replay against both versions."
            ),
        ),
        Executor::App { backed_by } => Reason::new(
            "nested-generated-tool",
            format!(
                "{versions}: the run instructions changed, and this tool runs the workflow {backed_by}; comparing a nested workflow is not supported."
            ),
        ),
        Executor::None => Reason::new(
            "no-executor",
            format!(
                "{versions}: the run instructions changed, and the tool has no transport AWARE can run, so nothing can be compared."
            ),
        ),
    };
    AgentComparison {
        agent,
        status: ComparisonStatus::NotComparable,
        method: None,
        runs: 0,
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::migration::contract::{
        AgentLevel, CONTRACT_DIFF_FORMAT, CliResolution, FileDiff, Ignored, PinRef,
    };

    fn diff(agent: &str, unchanged: bool, executor: Executor) -> ContractDiff {
        ContractDiff {
            format: CONTRACT_DIFF_FORMAT,
            agent: agent.into(),
            from: PinRef {
                version: "0.1.5".into(),
                digest: "sha256:a".into(),
            },
            to: PinRef {
                version: "0.1.6".into(),
                digest: "sha256:b".into(),
            },
            unchanged,
            agent_level: AgentLevel::default(),
            commands: Vec::new(),
            executable_files: FileDiff::default(),
            probe_changed: false,
            executor,
            plan_changes: Vec::new(),
            ignored: Ignored::default(),
        }
    }

    fn cli() -> Executor {
        Executor::Cli {
            binary: "b".into(),
            program: "/x/b".into(),
            resolution: CliResolution::FixedPath,
            sha256: Some("sha256:c".into()),
            detail: None,
        }
    }

    #[test]
    fn nothing_moved_means_no_comparison() {
        assert_eq!(compare(&[]), None);
    }

    #[test]
    fn an_unchanged_contract_is_identical_instructions_never_pass() {
        let c = compare(&[diff("tekla", true, cli())]).unwrap();
        assert_eq!(c.status, ComparisonStatus::IdenticalInstructions);
        assert_ne!(c.status, ComparisonStatus::Pass);
        assert_eq!(c.method, Some(STATIC_INSPECTION));
        assert_eq!(c.runs, 0);
        assert!(c.reason.text.contains("nothing was run"), "{c:?}");
        assert!(!c.reason.text.to_lowercase().contains("same result"));
        // v1: inspection is NOT accepted for the no-click path.
        assert!(!c.accepted_for_policy());
    }

    #[test]
    fn a_changed_contract_is_not_comparable_with_a_transport_reason() {
        let cases = [
            (cli(), "no-fixed-state"),
            (
                Executor::AwareCli {
                    version: "0.149.0".into(),
                },
                "no-recorded-exchange",
            ),
            (
                Executor::App {
                    backed_by: "inner".into(),
                },
                "nested-generated-tool",
            ),
            (Executor::None, "no-executor"),
        ];
        for (executor, code) in cases {
            let c = compare(&[diff("t", false, executor)]).unwrap();
            assert_eq!(c.status, ComparisonStatus::NotComparable);
            assert_eq!(c.per_agent[0].reason.code, code);
            assert_eq!(c.method, None);
            assert!(!c.accepted_for_policy());
        }
    }

    #[test]
    fn one_changed_agent_makes_the_whole_candidate_not_comparable() {
        let c = compare(&[diff("a", true, cli()), diff("b", false, cli())]).unwrap();
        assert_eq!(c.status, ComparisonStatus::NotComparable);
        assert!(c.reason.text.starts_with("b 0.1.5 -> 0.1.6"), "{c:?}");
        assert_eq!(c.per_agent.len(), 2);
    }

    #[test]
    fn only_an_executed_pass_is_accepted_for_policy() {
        let mut c = compare(&[diff("a", true, cli())]).unwrap();
        c.status = ComparisonStatus::Pass;
        assert!(
            !c.accepted_for_policy(),
            "every agent must pass, not the fold alone"
        );
        c.per_agent[0].status = ComparisonStatus::Pass;
        assert!(c.accepted_for_policy());
        assert!(!ACCEPTED_FOR_POLICY.contains(&ComparisonStatus::IdenticalInstructions));
    }
}
