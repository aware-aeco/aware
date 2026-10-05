//! `aware app migrate plan|prepare` (#628 plan §7, revised by §11–§14): one
//! evaluation per app — what would move, what the workflow is declared to do,
//! whether the tools' run instructions changed, what could be compared, and
//! therefore who must decide.
//!
//! Every expected outcome is DATA: a state and plain-English reasons. The
//! states, in v1:
//!
//! * `up-to-date` — no agent this app dispatches would move;
//! * `needs-person` — every real candidate in v1 (no fixed-state comparison
//!   method exists yet, so the no-click path is closed: `no-click-available:
//!   false`);
//! * `blocked` — the candidate cannot be carried forward until a person edits
//!   or recompiles the workflow (the reasons say which);
//! * `held` — a person put the app on hold;
//! * `auto-under-policy` — reserved: reachable only when an accepted executed
//!   comparison method exists ([`super::compare::ACCEPTED_FOR_POLICY`]).
//!
//! An `exposes-as-agent` backing app, and any app whose candidate would move a
//! backing app's pins, is never carried forward in v1 (§14): it is
//! `needs-person` with reason `backing-app-moved`, and so is every caller of a
//! backing app with a pending move. The person path is `aware app compile`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::Serialize;

use super::compare::{self, Comparison};
use super::contract::{self, ContractDiff, DiffInput, PackageSide, PinRef};
use super::effect::{self, NodeBasis, NodeEffect, WrapperEffect};
use super::files::{self, HoldRecord};
use super::{Reason, reason_from_error};
use crate::agent_resolution::{PinSet, PinTarget, resolve_pins};
use crate::app_lock::{Candidate, LockFile};
use crate::error::AwareError;
use crate::manifest::App;
use crate::manifest::agent::{Mode, ModeBasis};
use crate::manifest::loader::DiscoveredAgent;
use crate::paths::Paths;

/// Where an app stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum State {
    UpToDate,
    /// Reserved (see the module docs): never produced in v1.
    AutoUnderPolicy,
    NeedsPerson,
    Blocked,
    Held,
}

/// One moved pin.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TargetRow {
    pub agent: String,
    pub from: PinRef,
    pub to: PinRef,
    /// `patch` | `minor` | `major` | `prerelease` | `same-version` |
    /// `downgrade` | `non-semver`.
    pub bump: &'static str,
    /// Who the new bytes came from, by their install receipt:
    /// `official-registry` | `registry` | `local` | `unknown`.
    pub publisher: &'static str,
}

/// The executable-contract verdict over every moved agent.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ContractRow {
    pub unchanged: bool,
    /// sha256 of the canonical JSON of `diffs`.
    pub diff_digest: String,
    /// Some moved agent's `probe:` changed — reported, never counted (a run
    /// never runs the probe).
    pub probe_changed: bool,
    pub diffs: Vec<ContractDiff>,
}

/// Why the workflow is, or is not, declared read-only.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EffectDetail {
    pub reasons: Vec<String>,
    /// Each dispatchable node under the old pins.
    pub old: Vec<NodeEffect>,
    /// Each dispatchable node under the new pins.
    pub new: Vec<NodeEffect>,
}

/// The candidate on disk, if any.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct CandidateRow {
    pub present: bool,
    pub candidate_digest: Option<String>,
    /// The stored candidate starts from the CURRENT approved lock and compiles
    /// to the same plan as a candidate prepared now.
    pub fresh: bool,
}

/// A run of this app that a pidfile says is in progress. Promotion never
/// moves an active run (it reads the lock once, at preflight); listed so a
/// person knows. A crashed run can leave a pidfile behind.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RunningInstance {
    pub instance: String,
    pub pid: u32,
    pub run_id: String,
    pub started_at: String,
}

/// One app's row of `aware app migrate plan --json`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct PlanRow {
    pub app: String,
    pub source: String,
    pub lock: String,
    pub state: State,
    pub targets: Vec<TargetRow>,
    /// `declared-read-only` | `not-declared-read-only`; null when nothing moves.
    pub effect: Option<&'static str>,
    pub effect_detail: Option<EffectDetail>,
    pub contract: Option<ContractRow>,
    pub comparison: Option<Comparison>,
    pub reasons: Vec<Reason>,
    pub candidate: CandidateRow,
    pub running_instances: Vec<RunningInstance>,
    pub hold: Option<HoldRecord>,
    /// This app is itself a tool other workflows run, or its candidate moves
    /// such a tool — never carried forward in v1.
    pub backing_app_moved: bool,
    /// For an `exposes-as-agent` app: the installed apps that run it.
    pub callers: Vec<String>,
    /// Things a person should know that do not change the state: stored
    /// copies that claimed the pinned bytes but did not verify (skipped).
    pub warnings: Vec<Reason>,
    /// Reserved for an urgency notice (e.g. a security advisory). Urgency is
    /// never approval: this field never changes `state`.
    pub advisory: Option<serde_json::Value>,
    /// Whether this candidate could be carried forward with no click. Always
    /// false in v1: no fixed-state comparison method is available yet.
    pub no_click_available: bool,
}

impl PlanRow {
    fn new(app: &str, source: &Path, lock: &Path) -> Self {
        Self {
            app: app.to_string(),
            source: source.display().to_string(),
            lock: lock.display().to_string(),
            state: State::UpToDate,
            targets: Vec::new(),
            effect: None,
            effect_detail: None,
            contract: None,
            comparison: None,
            reasons: Vec::new(),
            candidate: CandidateRow::default(),
            running_instances: Vec::new(),
            hold: None,
            backing_app_moved: false,
            callers: Vec::new(),
            warnings: Vec::new(),
            advisory: None,
            no_click_available: false,
        }
    }

    /// A row for an app that could not be evaluated at all.
    pub fn unevaluated(app: &str, source: &Path, error: &AwareError) -> Self {
        let mut row = Self::new(app, source, Path::new(""));
        row.lock = String::new();
        row.state = State::Blocked;
        row.reasons.push(reason_from_error(error));
        row
    }
}

/// One evaluated app: its row and, when one compiled, the candidate.
pub struct Evaluation {
    pub row: PlanRow,
    pub candidate: Option<Candidate>,
    /// The approved lock's exact bytes when they could be read.
    pub base_bytes: Option<Vec<u8>>,
}

impl Evaluation {
    fn blocked(mut self, reason: Reason) -> Self {
        self.row.reasons.push(reason);
        self.row.state = if self.row.hold.is_some() {
            State::Held
        } else {
            State::Blocked
        };
        self
    }
}

/// The text of the reason that keeps the no-click path closed in v1.
pub const NO_FIXED_STATE_METHOD: &str = "No fixed-state comparison method is available yet, so a person must approve carrying this workflow forward.";

/// Evaluate the app whose source is `source`.
///
/// `requested`: the `--to` targets (only those the app dispatches are used);
/// `None` targets every dispatched agent whose installed copy differs from the
/// approved bytes. Writes nothing. `Err` only when the app cannot be read.
pub fn evaluate(
    paths: &Paths,
    source: &Path,
    requested: Option<&BTreeMap<String, PinTarget>>,
) -> Result<Evaluation, AwareError> {
    let (app, source_hash) = crate::app_lock::read_app_source(source)?;
    let dir = crate::fs::containing_dir(source).to_path_buf();
    let lock_path = dir.join(format!("{}.lock", app.app));
    let mut row = PlanRow::new(&app.app, source, &lock_path);
    row.hold = files::read_hold(&dir)?;
    row.running_instances = running_instances(paths, &app.app);
    let stored = files::read_candidate(&dir, &app.app)?;
    row.candidate = CandidateRow {
        present: stored.present,
        candidate_digest: stored.candidate_digest.clone(),
        fresh: false,
    };
    if row.hold.is_some() {
        row.state = State::Held;
    }
    let mut eval = Evaluation {
        row,
        candidate: None,
        base_bytes: None,
    };

    let base_bytes = match std::fs::read(&lock_path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(eval.blocked(Reason::new(
                "no-approval",
                "This workflow has no compiled approval yet, so there is nothing to carry forward; a person must compile it.",
            )));
        }
        Err(error) => {
            return Err(std::io::Error::new(
                error.kind(),
                format!("{}: {error}", lock_path.display()),
            )
            .into());
        }
    };
    eval.base_bytes = Some(base_bytes.clone());
    let base: LockFile = match serde_yaml::from_slice(&base_bytes) {
        Ok(lock) => lock,
        Err(error) => {
            return Ok(eval.blocked(Reason::new(
                "approval-invalid",
                format!(
                    "The workflow's approval {} cannot be read ({error}); a person must compile it again.",
                    lock_path.display()
                ),
            )));
        }
    };
    if base.source_hash != source_hash {
        return Ok(eval.blocked(Reason::new(
            "source-changed",
            "The workflow changed since it was approved, so its approval cannot be carried forward; a person must compile it again.",
        )));
    }

    let dispatched: BTreeSet<String> = crate::validate::dispatchable_agents(&app)
        .into_iter()
        .map(str::to_string)
        .collect();
    let targets = match requested {
        Some(requested) => requested
            .iter()
            .filter(|(id, _)| dispatched.contains(*id))
            .map(|(id, target)| (id.clone(), target.clone()))
            .collect(),
        None => default_targets(paths, &dispatched, &base)?,
    };
    let old = match PinSet::from_lock(&base, BTreeMap::new())
        .and_then(|set| resolve_pins(paths, &app, &set))
    {
        Ok(old) => old,
        Err(error) => return Ok(eval.blocked(reason_from_error(&error))),
    };
    let candidate = match PinSet::from_lock(&base, targets)
        .and_then(|set| crate::app_lock::compile_candidate(source, paths, &base, &base_bytes, &set))
    {
        Ok(candidate) => candidate,
        Err(error) => return Ok(eval.blocked(reason_from_error(&error))),
    };

    let row = &mut eval.row;
    row.candidate.fresh = stored.header.as_ref().is_some_and(|header| {
        header.base_lock_digest == candidate.header.base_lock_digest
            && header.plan_digest == candidate.header.plan_digest
    });
    let old_pins: BTreeMap<String, (String, String, PathBuf)> = old
        .iter()
        .map(|pin| {
            (
                pin.agent.manifest.agent.clone(),
                (
                    pin.version.clone(),
                    pin.digest.clone(),
                    pin.agent.root.clone(),
                ),
            )
        })
        .collect();
    let skipped = old
        .iter()
        .flat_map(|pin| {
            pin.invalid_candidates
                .iter()
                .map(move |c| (&pin.version, &pin.agent.manifest.agent, c))
        })
        .chain(candidate.pins.iter().flat_map(|pin| {
            pin.invalid_candidates
                .iter()
                .map(move |c| (&pin.version, &pin.agent, c))
        }));
    for (version, agent, invalid) in skipped {
        let warning = Reason::new(
            "invalid-stored-copy",
            format!(
                "A stored copy of {agent} {version} at {} was skipped because it does not verify: {}.",
                invalid.path, invalid.reason
            ),
        );
        if !row.warnings.contains(&warning) {
            row.warnings.push(warning);
        }
    }
    let old_agents: Vec<DiscoveredAgent> = old.into_iter().map(|pin| pin.agent).collect();
    let moved: Vec<&crate::app_lock::CandidatePin> = candidate
        .pins
        .iter()
        .filter(|pin| {
            old_pins
                .get(&pin.agent)
                .is_none_or(|(_, d, _)| *d != pin.digest)
        })
        .collect();
    let moved_ids: BTreeSet<String> = moved.iter().map(|pin| pin.agent.clone()).collect();
    for pin in &moved {
        let (from_version, from_digest, _) = old_pins.get(&pin.agent).cloned().unwrap_or_default();
        row.targets.push(TargetRow {
            agent: pin.agent.clone(),
            bump: bump(&from_version, &pin.version),
            publisher: publisher(&pin.root),
            from: PinRef {
                version: from_version,
                digest: from_digest,
            },
            to: PinRef {
                version: pin.version.clone(),
                digest: pin.digest.clone(),
            },
        });
    }
    if moved.is_empty() {
        eval.candidate = Some(candidate);
        return Ok(eval);
    }

    // §14: a backing app, or a move of one, is never carried forward in v1.
    let mut backing = Vec::new();
    if app.exposes_as_agent {
        row.callers = find_callers(paths, &app.app);
        let callers = if row.callers.is_empty() {
            "no installed workflow runs it yet".to_string()
        } else {
            format!("its callers are {}", row.callers.join(", "))
        };
        backing.push(Reason::new(
            "backing-app-moved",
            format!(
                "This workflow is itself a tool other workflows run (as {}); a backing workflow is never carried forward automatically, so compile it and its callers again ({callers}).",
                app.app
            ),
        ));
    }
    for pin in &moved {
        let wraps = |agents: &[DiscoveredAgent]| {
            agents
                .iter()
                .find(|a| a.manifest.agent == pin.agent)
                .and_then(|a| a.manifest.transport.app.as_ref())
                .map(|t| t.backed_by.clone())
        };
        if let Some(backed_by) = wraps(&old_agents).or_else(|| wraps(&candidate.agents)) {
            backing.push(Reason::new(
                "backing-app-moved",
                format!(
                    "This update would move {}, a tool that runs the workflow {backed_by}; a backing workflow is never carried forward automatically, so compile {backed_by} and this workflow again.",
                    pin.agent
                ),
            ));
        }
    }
    row.backing_app_moved = !backing.is_empty();

    // §3: declared effect under both pins.
    let old_wrappers = wrapper_effects(paths, &app, &old_agents);
    let new_wrappers = wrapper_effects(paths, &app, &candidate.agents);
    let old_effects = effect::node_effects(&app, &old_agents, &old_wrappers);
    let new_effects = effect::node_effects(&app, &candidate.agents, &new_wrappers);
    let verdict = effect::workflow_read_only(&old_effects, &new_effects, &moved_ids);
    row.effect = Some(if verdict.read_only {
        "declared-read-only"
    } else {
        "not-declared-read-only"
    });
    let effect_reasons = effect_reasons(&old_effects, &new_effects, &verdict.reasons);
    row.effect_detail = Some(EffectDetail {
        reasons: verdict.reasons.clone(),
        old: old_effects,
        new: new_effects,
    });

    // §2: the executable contract of each moved agent.
    let mut diffs = Vec::new();
    for pin in &moved {
        let Some((from_version, from_digest, from_root)) = old_pins.get(&pin.agent) else {
            continue;
        };
        let called = contract::called_commands(&app, &pin.agent);
        diffs.push(contract::diff(DiffInput {
            agent: &pin.agent,
            old: PackageSide {
                root: from_root,
                version: from_version,
                digest: from_digest,
            },
            new: PackageSide {
                root: &pin.root,
                version: &pin.version,
                digest: &pin.digest,
            },
            called: &called,
            plan_changes: contract::plan_changes(&base, &candidate.lock, &pin.agent),
        })?);
    }
    let contract_unchanged = diffs.iter().all(|d| d.unchanged);
    let diff_digest = crate::app_lock::lock_digest(
        contract::canonical_json(
            serde_json::to_value(&diffs).map_err(|e| AwareError::Internal(e.to_string()))?,
        )
        .as_bytes(),
    );
    // §4/§11: the comparison verdict.
    let comparison = compare::compare(&diffs);
    row.contract = Some(ContractRow {
        unchanged: contract_unchanged,
        diff_digest,
        probe_changed: diffs.iter().any(|d| d.probe_changed),
        diffs: diffs.clone(),
    });

    // Reasons, then state.
    let blocked = !candidate.blocked.is_empty();
    row.reasons.extend(candidate.blocked.iter().cloned());
    row.reasons.extend(backing.iter().cloned());
    if !verdict.read_only {
        row.reasons.extend(effect_reasons);
    }
    if !contract_unchanged {
        row.reasons.push(Reason::new(
            "contract-changed",
            format!(
                "What the workflow's tools would be told to do changed: {}.",
                diffs
                    .iter()
                    .filter(|d| !d.unchanged)
                    .map(contract_summary)
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
        ));
    }
    if let Some(comparison) = &comparison
        && comparison.status == compare::ComparisonStatus::NotComparable
    {
        row.reasons.push(comparison.reason.clone());
    }
    let comparison_accepted = comparison
        .as_ref()
        .is_some_and(Comparison::accepted_for_policy);
    row.no_click_available = verdict.read_only
        && contract_unchanged
        && comparison_accepted
        && backing.is_empty()
        && !blocked;
    if !row.no_click_available && !blocked {
        row.reasons
            .push(Reason::new("no-fixed-state-method", NO_FIXED_STATE_METHOD));
    }
    row.comparison = comparison;
    row.state = decide(row.hold.is_some(), true, blocked, row.no_click_available);
    eval.candidate = Some(candidate);
    Ok(eval)
}

/// The state of an app from the facts that may decide it. `advisory` is not an
/// input: urgency never changes who decides.
pub fn decide(held: bool, moved: bool, blocked: bool, no_click: bool) -> State {
    if held {
        State::Held
    } else if !moved {
        State::UpToDate
    } else if blocked {
        State::Blocked
    } else if no_click {
        State::AutoUnderPolicy
    } else {
        State::NeedsPerson
    }
}

/// Evaluate several apps for `migrate plan`. An app that cannot be evaluated
/// is a `blocked` row, never a failed command. Then every caller of a backing
/// app with a pending move is marked `needs-person` (§14).
pub fn plan_rows(
    paths: &Paths,
    sources: &[(String, PathBuf)],
    requested: Option<&BTreeMap<String, PinTarget>>,
) -> Vec<PlanRow> {
    let mut rows: Vec<PlanRow> = sources
        .iter()
        .map(|(id, source)| match evaluate(paths, source, requested) {
            Ok(eval) => eval.row,
            Err(error) => PlanRow::unevaluated(id, source, &error),
        })
        .collect();
    // A backing app with a pending move carries its installed callers.
    let moving_backing: Vec<(String, BTreeSet<String>)> = rows
        .iter()
        .filter(|row| row.backing_app_moved && !row.callers.is_empty())
        .map(|row| (row.app.clone(), row.callers.iter().cloned().collect()))
        .collect();
    for (backing, callers) in moving_backing {
        for row in rows.iter_mut().filter(|row| callers.contains(&row.app)) {
            row.reasons.push(Reason::new(
                "backing-app-moved",
                format!(
                    "This workflow runs {backing}, which has a pending tool update; a backing workflow is never carried forward automatically, so compile {backing} and this workflow again."
                ),
            ));
            if row.state == State::UpToDate {
                row.state = State::NeedsPerson;
            }
        }
    }
    rows
}

/// Every dispatched agent whose installed copy is not the approved bytes,
/// targeted at the installed copy's digest.
fn default_targets(
    paths: &Paths,
    dispatched: &BTreeSet<String>,
    base: &LockFile,
) -> Result<BTreeMap<String, PinTarget>, AwareError> {
    let mut out = BTreeMap::new();
    for id in dispatched {
        let Some(approved) = base
            .agent_digests
            .get(id)
            .or_else(|| base.agent_bundle_pins.get(id))
        else {
            continue; // version-only: resolve_pins names the refusal
        };
        if !crate::manifest::loader::is_safe_segment(id) {
            continue;
        }
        let dir = paths.agents_dir().join(id);
        if crate::agent_store::probe(&dir)?.is_none() {
            continue; // uninstalled: nothing to move to
        }
        match crate::install::integrity::tree_digest(&dir) {
            Ok(current) if current != *approved => {
                out.insert(id.clone(), PinTarget::Digest(current));
            }
            Ok(_) | Err(AwareError::Validation(_)) => {}
            Err(error) => return Err(error),
        }
    }
    Ok(out)
}

/// The inherited effect of every app-backed agent command `app` calls, each
/// judged against its backing app's own approved pins. A backing app that
/// cannot be loaded or resolved is left out — its caller nodes are then never
/// eligible as read ("not evaluated"), the safe side.
fn wrapper_effects(
    paths: &Paths,
    app: &App,
    agents: &[DiscoveredAgent],
) -> BTreeMap<String, WrapperEffect> {
    let mut out = BTreeMap::new();
    for agent in agents {
        let Some(transport) = agent.manifest.transport.app.as_ref() else {
            continue;
        };
        let backed_by = &transport.backed_by;
        if !crate::manifest::loader::is_safe_segment(backed_by) {
            continue;
        }
        let Some(source) =
            crate::manifest::loader::find_app_manifest(&paths.apps_dir().join(backed_by))
        else {
            continue;
        };
        let Ok(approved) = crate::app_lock::load_approved_app_snapshot(&source) else {
            continue;
        };
        let Ok(backing_agents) = PinSet::from_lock(&approved.lock, BTreeMap::new())
            .and_then(|set| resolve_pins(paths, &approved.app, &set))
        else {
            continue;
        };
        let backing_agents: Vec<DiscoveredAgent> =
            backing_agents.into_iter().map(|pin| pin.agent).collect();
        for command in contract::called_commands(app, &agent.manifest.agent).keys() {
            out.insert(
                effect::wrapper_key(&agent.manifest.agent, command),
                effect::wrapper_effect(&approved.app, command, &backing_agents),
            );
        }
    }
    out
}

/// The installed apps whose workflow dispatches agent `backing` (the agent an
/// `exposes-as-agent` app installs as is named after the app).
pub fn find_callers(paths: &Paths, backing: &str) -> Vec<String> {
    let mut out = BTreeSet::new();
    let Ok(entries) = std::fs::read_dir(paths.apps_dir()) else {
        return Vec::new();
    };
    for entry in entries.flatten() {
        let Some(source) = crate::manifest::loader::find_app_manifest(&entry.path()) else {
            continue;
        };
        let Ok((app, _)) = crate::app_lock::read_app_source(&source) else {
            continue;
        };
        if app.app != backing && crate::validate::dispatchable_agents(&app).contains(backing) {
            out.insert(app.app);
        }
    }
    out.into_iter().collect()
}

/// Runs a pidfile records under `apps/<app>/instances/*/`.
fn running_instances(paths: &Paths, app: &str) -> Vec<RunningInstance> {
    if !crate::manifest::loader::is_safe_segment(app) {
        return Vec::new();
    }
    let Ok(entries) = std::fs::read_dir(paths.apps_dir().join(app).join("instances")) else {
        return Vec::new();
    };
    let mut out: Vec<RunningInstance> = entries
        .flatten()
        .filter_map(|entry| crate::runtime::pidfile::read(&entry.path()).ok())
        .map(|pid| RunningInstance {
            instance: pid.instance,
            pid: pid.pid,
            run_id: pid.run_id,
            started_at: pid.started_at,
        })
        .collect();
    out.sort_by(|a, b| a.instance.cmp(&b.instance));
    out
}

fn bump(from: &str, to: &str) -> &'static str {
    let (Some(a), Some(b)) = (
        crate::validate::parse_semver(from),
        crate::validate::parse_semver(to),
    ) else {
        return "non-semver";
    };
    if b.triple < a.triple {
        "downgrade"
    } else if b.triple.0 != a.triple.0 {
        "major"
    } else if b.triple.1 != a.triple.1 {
        "minor"
    } else if b.triple.2 != a.triple.2 {
        "patch"
    } else if a.prerelease != b.prerelease {
        "prerelease"
    } else {
        "same-version"
    }
}

fn publisher(root: &Path) -> &'static str {
    use crate::install::provenance::InstallSource;
    match crate::install::provenance::read(root) {
        Some(InstallSource::Registry {
            official_source: true,
            ..
        }) => "official-registry",
        Some(InstallSource::Registry { .. }) => "registry",
        Some(InstallSource::Local { .. }) => "local",
        None => "unknown",
    }
}

fn contract_summary(diff: &ContractDiff) -> String {
    let mut parts = Vec::new();
    if !diff.agent_level.changed.is_empty() {
        parts.push(format!("settings {}", diff.agent_level.changed.join(", ")));
    }
    let commands: Vec<&str> = diff
        .commands
        .iter()
        .filter(|c| !c.unchanged)
        .map(|c| c.command.as_str())
        .collect();
    if !commands.is_empty() {
        parts.push(format!("commands {}", commands.join(", ")));
    }
    let files = diff.executable_files.added.len()
        + diff.executable_files.removed.len()
        + diff.executable_files.changed.len();
    if files > 0 {
        parts.push(format!("{files} executable file(s)"));
    }
    if !diff.plan_changes.is_empty() {
        parts.push("the compiled plan".to_string());
    }
    format!(
        "{} {} -> {} ({})",
        diff.agent,
        diff.from.version,
        diff.to.version,
        parts.join("; ")
    )
}

/// One reason per kind of non-read node, naming the nodes; plus the verdict's
/// own sentences when no single node explains it (a node on one side only, a
/// moved app-backed tool).
fn effect_reasons(old: &[NodeEffect], new: &[NodeEffect], verdict: &[String]) -> Vec<Reason> {
    let mut by_code: BTreeMap<&'static str, BTreeSet<String>> = BTreeMap::new();
    for node in old.iter().chain(new) {
        if effect::eligible_read(node).is_ok() {
            continue;
        }
        let label = match (&node.agent, &node.command) {
            (Some(agent), Some(command)) => format!("node {} ({agent} {command})", node.node),
            _ => format!("node {} ({})", node.node, node.kind),
        };
        by_code.entry(effect_code(node)).or_default().insert(label);
    }
    let mut out: Vec<Reason> = by_code
        .into_iter()
        .map(|(code, nodes)| {
            let nodes = nodes.into_iter().collect::<Vec<_>>().join(", ");
            let why = match code {
                "mode-overridable" => {
                    "the tool lets each workflow choose whether this command reads or writes (mode-overridable), so the tool itself does not declare it read-only"
                }
                "mode-inferred" => {
                    "it is read only by inference from the command's name; the tool does not declare it"
                }
                "mode-unknown" => {
                    "the command is not in the tool's manifest, so it is treated as a write"
                }
                "runtime-model" => "it calls a model at run time",
                "effect-primitive" => {
                    "it is a sweep, approve, snapshot or model-lock step, which is never treated as read-only"
                }
                "agent-not-resolved" => "its tool could not be resolved, so its effect is unknown",
                "inherited-not-read-only" => {
                    "it runs another workflow that is not declared read-only throughout"
                }
                _ => "it is declared to write",
            };
            Reason::new(
                code,
                format!("The workflow is not declared read-only: {nodes} — {why}."),
            )
        })
        .collect();
    if out.is_empty() && !verdict.is_empty() {
        out.push(Reason::new(
            "not-declared-read-only",
            format!(
                "The workflow is not declared read-only: {}.",
                verdict.join("; ")
            ),
        ));
    }
    out
}

fn effect_code(node: &NodeEffect) -> &'static str {
    if node.runtime_model {
        return "runtime-model";
    }
    match node.basis {
        NodeBasis::EffectPrimitive => "effect-primitive",
        NodeBasis::AgentNotResolved => "agent-not-resolved",
        NodeBasis::ReadPrimitive | NodeBasis::Container => "writes",
        NodeBasis::Agent(basis) => match (node.mode, basis) {
            (_, ModeBasis::OverridableDefault) => "mode-overridable",
            (Mode::Read, ModeBasis::NodeOverride) => "mode-overridable",
            (_, ModeBasis::Inherited) => "inherited-not-read-only",
            (_, ModeBasis::FallbackWrite) => "mode-unknown",
            (Mode::Read, ModeBasis::InferredName) => "mode-inferred",
            _ => "writes",
        },
    }
}

#[cfg(test)]
mod tests;
