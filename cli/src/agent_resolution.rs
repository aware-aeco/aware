//! Resolve an approved app's agents to the exact bytes its lock approved (#626).
//!
//! One resolver, used by the run and by `aware app check`:
//!
//! * a lock with a digest for an agent (`agent-digests`, else an official
//!   `agent-bundle-pins` digest) resolves to the **store package** with those
//!   bytes — the snapshot of the current working copy if it still matches,
//!   otherwise an older snapshot left by compile or by an update;
//! * a lock without one (compiled by AWARE ≤ 0.148, not an official install)
//!   keeps today's guarantee — the current copy must match the pinned version —
//!   and is dispatched from a snapshot of that copy, labelled `version-only`.
//!
//! The run builds a [`ResolvedCatalogue`] **once**, at preflight, and every
//! run-path read — every preflight, every transport, every built-in helper and
//! every nested app-backed dispatch — reads manifests and agent roots from it
//! instead of `agents/`. `--simulate` dispatches nothing and builds none; it
//! keeps reading the working copies ([`AgentCatalogue::WorkingCopies`]).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde::Serialize;

use crate::agent_store::{self, StoredPackage};
use crate::app_lock::LockFile;
use crate::error::AwareError;
use crate::manifest::loader::DiscoveredAgent;
use crate::manifest::{Agent, App};
use crate::paths::Paths;

/// What a run's approval of an agent covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Approval {
    /// The lock names the exact bytes (`agent-digests` / official bundle pin).
    Bytes,
    /// A legacy lock that named only a version; the run uses the current copy.
    VersionOnly,
}

/// How one pinned agent resolves — the `resolution` field of `aware app check`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Resolution {
    /// The approved bytes are no longer current; a stored snapshot supplies them.
    Stored,
    /// The current working copy is the approved bytes.
    Current,
    /// Legacy version-only lock; the current copy matches the pinned version.
    CurrentVersionOnly,
    /// The agent is not installed (uninstalled after compile).
    Missing,
    /// The approved bytes exist nowhere on this machine.
    PinNotInstalled,
    /// A stored package claims the approved bytes but does not verify.
    DigestMismatch,
    /// The agent is installed but the lock never approved it.
    NeverApproved,
    /// Legacy version-only lock whose version no longer matches.
    LegacyPinMismatch,
}

impl Resolution {
    /// Whether `aware app run` accepts this resolution.
    pub fn runs(self) -> bool {
        matches!(
            self,
            Resolution::Stored | Resolution::Current | Resolution::CurrentVersionOnly
        )
    }
}

/// A store package that claims an agent's approved bytes but does not verify,
/// or whose record cannot be read. Never used; always reported (review #626-4).
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct InvalidCandidate {
    pub path: String,
    pub reason: String,
}

/// What the run recorded about one resolved agent.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ResolvedAgentInfo {
    pub version: String,
    pub digest: String,
    pub approval: Approval,
    pub resolution: Resolution,
    /// Whether ANY valid stored candidate carries a receipt that claims an
    /// official registry install — the offline precheck of
    /// `--require-verified-agents`, before the fresh index is fetched.
    #[serde(skip)]
    pub official_claim: bool,
    /// Stored packages of the approved bytes that did not verify and were
    /// skipped. Recorded in `agent-resolution` and warned about on stderr.
    pub invalid_candidates: Vec<InvalidCandidate>,
}

/// An app-backed agent's backing app, approved and resolved at preflight and
/// carried into dispatch unchanged.
#[derive(Debug)]
pub struct NestedApp {
    pub backed_by: String,
    pub app: App,
    source_text: String,
    pub catalogue: Arc<ResolvedCatalogue>,
}

impl NestedApp {
    /// A fresh owned copy of the approved backing app, parsed from the exact
    /// source text preflight hashed — never re-read from disk.
    pub fn app_owned(&self) -> Result<App, AwareError> {
        serde_yaml::from_str(&self.source_text).map_err(|error| {
            AwareError::Internal(format!(
                "re-parse approved backing app {}: {error}",
                self.backed_by
            ))
        })
    }
}

/// The agents of one approved app, resolved to store packages.
#[derive(Debug, Default)]
pub struct ResolvedCatalogue {
    agents: Vec<DiscoveredAgent>,
    info: BTreeMap<String, ResolvedAgentInfo>,
    nested: BTreeMap<String, Arc<NestedApp>>,
}

/// One executable agent the app can reach (directly, or as the leaf of an
/// app-backed agent), as strict provenance assesses it.
pub struct Reachable<'a> {
    pub id: String,
    pub agent: Option<&'a DiscoveredAgent>,
    pub info: Option<&'a ResolvedAgentInfo>,
    /// The app-backed agent this leaf is reached through, if any.
    pub via: Option<&'a str>,
}

impl Reachable<'_> {
    /// How this package is named in a refusal and keyed in the run record:
    /// the agent id, plus the app-backed agent it is reached through.
    pub fn label(&self) -> String {
        match self.via {
            Some(via) => format!("{} via {via}", self.id),
            None => self.id.clone(),
        }
    }
}

impl ResolvedCatalogue {
    /// Every resolved agent, root = its store package. Agents that are not
    /// installed are absent, so the missing-agent preflight reports them.
    pub fn agents(&self) -> &[DiscoveredAgent] {
        &self.agents
    }

    pub fn get(&self, id: &str) -> Option<&DiscoveredAgent> {
        self.agents.iter().find(|agent| agent.manifest.agent == id)
    }

    pub fn info(&self, id: &str) -> Option<&ResolvedAgentInfo> {
        self.info.get(id)
    }

    pub fn infos(&self) -> &BTreeMap<String, ResolvedAgentInfo> {
        &self.info
    }

    pub fn nested(&self, id: &str) -> Option<&Arc<NestedApp>> {
        self.nested.get(id)
    }

    pub fn nested_apps(&self) -> impl Iterator<Item = (&String, &Arc<NestedApp>)> {
        self.nested.iter()
    }

    /// The one-hop set whose transports can execute: each directly dispatched
    /// agent, and for an app-backed agent the leaf agents of its backing app
    /// (from the backing app's OWN resolution). Deduplicated by the resolved
    /// PACKAGE — agent id plus store root, which names digest and receipt key —
    /// never by id alone: a backing app may pin different bytes of an agent the
    /// app also calls directly, and strict provenance must assess both
    /// (review #626 round 3). Sorted: direct agents first, then each wrapper's.
    pub fn reachable<'a>(&'a self, app: &'a App) -> Vec<Reachable<'a>> {
        let mut seen: BTreeSet<(String, Option<PathBuf>)> = BTreeSet::new();
        let mut direct = Vec::new();
        let mut nested_out = Vec::new();
        for id in sorted_dispatchable(app) {
            let Some(agent) = self.get(id) else {
                continue; // the missing-agent preflight reports it
            };
            if let Some(nested) = self.nested.get(id) {
                for leaf in sorted_dispatchable(&nested.app) {
                    let leaf_agent = nested.catalogue.get(leaf);
                    nested_out.push(Reachable {
                        id: leaf.to_string(),
                        agent: leaf_agent,
                        info: nested.catalogue.info(leaf),
                        via: Some(id),
                    });
                }
            } else {
                direct.push(Reachable {
                    id: id.to_string(),
                    agent: Some(agent),
                    info: self.info.get(id),
                    via: None,
                });
            }
        }
        direct
            .into_iter()
            .chain(nested_out)
            .filter(|r| seen.insert((r.id.clone(), r.agent.map(|a| a.root.clone()))))
            .collect()
    }

    /// Every resolved agent of the backing apps, keyed `<wrapper>><leaf>`, for
    /// the run record's `agent-resolution`.
    pub fn nested_infos(&self) -> Vec<(String, &ResolvedAgentInfo)> {
        self.nested
            .iter()
            .flat_map(|(wrapper, nested)| {
                nested
                    .catalogue
                    .info
                    .iter()
                    .map(move |(leaf, info)| (format!("{wrapper}>{leaf}"), info))
            })
            .collect()
    }
}

/// One warning per stored package the run skipped because it does not verify,
/// across the app and every backing app it resolved — printed by `app run`.
pub fn invalid_candidate_warnings(catalogue: &ResolvedCatalogue) -> Vec<String> {
    let mut out = Vec::new();
    for (id, info) in &catalogue.info {
        for candidate in &info.invalid_candidates {
            out.push(format!(
                "\u{26a0} agent {id}: skipped the stored package {} because it does not verify: {}",
                candidate.path, candidate.reason
            ));
        }
    }
    for nested in catalogue.nested.values() {
        out.extend(invalid_candidate_warnings(&nested.catalogue));
    }
    out
}

fn sorted_dispatchable(app: &App) -> Vec<&str> {
    let mut ids: Vec<&str> = crate::validate::dispatchable_agents(app)
        .into_iter()
        .collect();
    ids.sort_unstable();
    ids
}

/// A borrowed manifest from a resolved catalogue, or one loaded fresh from a
/// working copy (simulation).
pub enum ManifestRef<'a> {
    Borrowed(&'a Agent),
    Owned(Box<Agent>),
}

impl std::ops::Deref for ManifestRef<'_> {
    type Target = Agent;
    fn deref(&self) -> &Agent {
        match self {
            ManifestRef::Borrowed(agent) => agent,
            ManifestRef::Owned(agent) => agent,
        }
    }
}

/// Where a run's runtime reads agent manifests and roots from.
///
/// `Resolved` on every real run (`--dry-run` included): the preflight-built
/// catalogue, nothing re-read from `agents/`. `WorkingCopies` only where no
/// agent is dispatched — `--simulate` — and in unit tests, which read the
/// working copies exactly as the runtime did before #626.
#[derive(Clone, Debug)]
pub enum AgentCatalogue {
    WorkingCopies {
        agents_dir: PathBuf,
    },
    Resolved {
        agents_dir: PathBuf,
        catalogue: Arc<ResolvedCatalogue>,
    },
}

impl AgentCatalogue {
    pub fn working_copies(agents_dir: PathBuf) -> Self {
        AgentCatalogue::WorkingCopies { agents_dir }
    }

    pub fn resolved(agents_dir: PathBuf, catalogue: Arc<ResolvedCatalogue>) -> Self {
        AgentCatalogue::Resolved {
            agents_dir,
            catalogue,
        }
    }

    /// `<AWARE_HOME>/agents` — kept ONLY so `parent()` names AWARE_HOME for
    /// credentials and apps. Never join an agent id onto it on the run path.
    pub fn agents_dir(&self) -> &Path {
        match self {
            AgentCatalogue::WorkingCopies { agents_dir }
            | AgentCatalogue::Resolved { agents_dir, .. } => agents_dir,
        }
    }

    /// The manifest of agent `id`, or `NotFound`.
    pub fn manifest(&self, id: &str) -> Result<ManifestRef<'_>, AwareError> {
        match self {
            AgentCatalogue::WorkingCopies { agents_dir } => Ok(ManifestRef::Owned(Box::new(
                crate::manifest::loader::load_agent_by_id(agents_dir, id)?,
            ))),
            AgentCatalogue::Resolved { catalogue, .. } => catalogue
                .get(id)
                .map(|agent| ManifestRef::Borrowed(&agent.manifest))
                .ok_or_else(|| not_in_run(id)),
        }
    }

    /// The manifest of `id` when it is installed, `None` when it is absent, and
    /// an error only when a present manifest cannot be read.
    pub fn manifest_if_installed(&self, id: &str) -> Result<Option<ManifestRef<'_>>, AwareError> {
        match self {
            AgentCatalogue::WorkingCopies { agents_dir } => {
                let Ok(path) = crate::manifest::loader::agent_manifest_path(agents_dir, id) else {
                    return Ok(None);
                };
                match std::fs::symlink_metadata(&path) {
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
                    Err(error) => Err(std::io::Error::new(
                        error.kind(),
                        format!("{}: {error}", path.display()),
                    )
                    .into()),
                    Ok(_) => Ok(Some(ManifestRef::Owned(Box::new(
                        crate::manifest::loader::load_agent(&path)?,
                    )))),
                }
            }
            AgentCatalogue::Resolved { catalogue, .. } => Ok(catalogue
                .get(id)
                .map(|agent| ManifestRef::Borrowed(&agent.manifest))),
        }
    }

    /// The directory agent `id`'s files are read from: the store package on a
    /// resolved run, the working copy otherwise.
    pub fn root(&self, id: &str) -> Result<PathBuf, AwareError> {
        match self {
            AgentCatalogue::WorkingCopies { agents_dir } => {
                let manifest = crate::manifest::loader::agent_manifest_path(agents_dir, id)?;
                manifest
                    .parent()
                    .map(Path::to_path_buf)
                    .ok_or_else(|| AwareError::Internal("agent manifest has no parent".into()))
            }
            AgentCatalogue::Resolved { catalogue, .. } => catalogue
                .get(id)
                .map(|agent| agent.root.clone())
                .ok_or_else(|| not_in_run(id)),
        }
    }

    /// The resolved store root of `id` on a resolved run (an error when the run
    /// did not resolve it), or `Ok(None)` for the working-copy catalogue — the
    /// one place a caller may defer to the installed copy.
    pub fn resolved_root(&self, id: &str) -> Result<Option<PathBuf>, AwareError> {
        match self {
            AgentCatalogue::WorkingCopies { .. } => Ok(None),
            AgentCatalogue::Resolved { .. } => self.root(id).map(Some),
        }
    }

    /// The pre-resolved backing app of app-backed agent `id` (resolved runs only).
    pub fn nested(&self, id: &str) -> Option<Arc<NestedApp>> {
        match self {
            AgentCatalogue::WorkingCopies { .. } => None,
            AgentCatalogue::Resolved { catalogue, .. } => catalogue.nested(id).cloned(),
        }
    }
}

/// Tests hand helpers a bare `agents/` path; production must always pass the
/// catalogue it resolved, so this conversion exists only under `cfg(test)`.
#[cfg(test)]
impl From<PathBuf> for AgentCatalogue {
    fn from(agents_dir: PathBuf) -> Self {
        AgentCatalogue::working_copies(agents_dir)
    }
}

fn not_in_run(id: &str) -> AwareError {
    AwareError::NotFound(format!(
        "agent {id} is not installed (it is not among the agents this run resolved and approved)"
    ))
}

/// How a run picks among stored packages with the approved bytes.
#[derive(Clone, Copy)]
pub enum Selection<'a> {
    /// The receipt-choice total order (offline; reads stored receipts only).
    Default,
    /// `--require-verified-agents`: walk that order and take the first package
    /// the existing assessment verifies against this fresh official index.
    Verified(&'a crate::registry::Index),
}

/// Refuse a lock whose digest fields are malformed or disagree, before anything
/// is resolved. Returns the reason as a sentence.
pub fn check_lock_consistency(lock: &LockFile) -> Result<(), String> {
    for (id, digest) in &lock.agent_digests {
        if agent_store::digest_hex(digest).is_none() {
            return Err(format!(
                "agent-digests[{id}] {digest:?} is not a sha256 digest"
            ));
        }
        if !lock.agent_pins.contains_key(id) {
            return Err(format!(
                "agent-digests names {id}, which agent-pins does not pin"
            ));
        }
        if let Some(bundle) = lock.agent_bundle_pins.get(id)
            && bundle != digest
        {
            return Err(format!(
                "agent-digests[{id}] is {digest} but agent-bundle-pins[{id}] is {bundle}"
            ));
        }
    }
    for (id, digest) in &lock.agent_bundle_pins {
        if agent_store::digest_hex(digest).is_none() {
            return Err(format!(
                "agent-bundle-pins[{id}] {digest:?} is not a sha256 digest"
            ));
        }
    }
    Ok(())
}

fn inconsistent_lock(lock: &LockFile, reason: &str) -> AwareError {
    AwareError::Validation(format!(
        "[E_APP_LOCK_INVALID] compiled approval for app {} is inconsistent: {reason}; run `aware app compile` again",
        lock.app
    ))
}

/// Build the run's catalogue for `app` under `lock`. Snapshots a matching
/// current copy that has no snapshot yet. Errors are the run's refusals.
pub fn resolve_agents(
    paths: &Paths,
    app: &App,
    lock: &LockFile,
    selection: Selection<'_>,
) -> Result<ResolvedCatalogue, AwareError> {
    resolve_inner(paths, app, lock, selection, None)
}

fn resolve_inner(
    paths: &Paths,
    app: &App,
    lock: &LockFile,
    selection: Selection<'_>,
    nested_under: Option<&str>,
) -> Result<ResolvedCatalogue, AwareError> {
    check_lock_consistency(lock).map_err(|reason| inconsistent_lock(lock, &reason))?;
    let mut catalogue = ResolvedCatalogue::default();
    for id in sorted_dispatchable(app) {
        let outcome = assess_agent(paths, id, lock, Mode::Run(selection))?;
        if let Some(refusal) = outcome.refusal {
            return Err(refusal);
        }
        let Some(chosen) = outcome.chosen else {
            continue; // not installed — the missing-agent preflight reports it
        };
        let manifest = agent_store::package_manifest(&chosen.package.root)?;
        if matches!(
            crate::runtime::invoker::effective_transport(&manifest, id),
            Ok(crate::runtime::invoker::TransportKind::App)
        ) {
            if let Some(wrapper) = nested_under {
                return Err(AwareError::Validation(format!(
                    "app-backed agent {wrapper}: nested app-backed agent {id} exceeds the v0 one-hop limit"
                )));
            }
            let nested = resolve_backing(paths, id, &manifest, selection)?;
            catalogue.nested.insert(id.to_string(), Arc::new(nested));
        }
        catalogue.info.insert(
            id.to_string(),
            ResolvedAgentInfo {
                version: manifest.version.clone(),
                digest: chosen.package.digest.clone(),
                approval: chosen.approval,
                resolution: outcome.resolution,
                official_claim: chosen.official_claim,
                invalid_candidates: outcome.invalid_candidates,
            },
        );
        catalogue.agents.push(DiscoveredAgent {
            manifest,
            root: chosen.package.root,
        });
    }
    Ok(catalogue)
}

/// Approve and resolve the backing app of app-backed agent `id`, once.
fn resolve_backing(
    paths: &Paths,
    id: &str,
    manifest: &Agent,
    selection: Selection<'_>,
) -> Result<NestedApp, AwareError> {
    let transport = manifest.transport.app.as_ref().ok_or_else(|| {
        AwareError::Validation(format!("app-backed agent {id} has no app transport"))
    })?;
    let backed_by = transport.backed_by.clone();
    if !crate::manifest::loader::is_safe_segment(&backed_by) {
        return Err(AwareError::Validation(format!(
            "app-backed agent {id} has unsafe backing app id {backed_by:?}"
        )));
    }
    let backing_dir = paths.apps_dir().join(&backed_by);
    // A directory that cannot be looked at is an error, not "not installed".
    agent_store::probe(&backing_dir)?;
    let source = crate::manifest::loader::find_app_manifest(&backing_dir).ok_or_else(|| {
        AwareError::Validation(format!(
            "app-backed agent {id}: backing app {backed_by} is not installed"
        ))
    })?;
    // Named with the hop, keeping the error's class, as the simulate-path
    // preflight does (`commands::app::nested_malformed_requires`).
    let approved = crate::app_lock::load_approved_app_snapshot(&source).map_err(|error| {
        let hop = format!("app-backed agent {id:?} (backing app {backed_by:?})");
        match error {
            AwareError::Validation(message) => AwareError::Validation(format!("{hop}: {message}")),
            AwareError::Io(io) => std::io::Error::new(io.kind(), format!("{hop}: {io}")).into(),
            other => other,
        }
    })?;
    let catalogue = resolve_inner(paths, &approved.app, &approved.lock, selection, Some(id))?;
    Ok(NestedApp {
        backed_by,
        app: approved.app,
        source_text: approved.source_text,
        catalogue: Arc::new(catalogue),
    })
}

#[derive(Clone, Copy)]
enum Mode<'a> {
    /// Resolve for a run: may snapshot a matching current copy.
    Run(Selection<'a>),
    /// `aware app check`: read-only, writes nothing.
    Check,
}

struct Chosen {
    package: StoredPackage,
    approval: Approval,
    official_claim: bool,
}

/// One agent's resolution, shared by the run (which acts on `chosen` /
/// `refusal`) and `aware app check` (which reports the rest).
pub struct AgentOutcome {
    pub agent: String,
    pub pinned_version: Option<String>,
    pub pinned_digest: Option<String>,
    pub installed_version: Option<String>,
    pub resolution: Resolution,
    pub detail: String,
    pub invalid_candidates: Vec<InvalidCandidate>,
    /// `app check` only: the manifest of the copy a run would dispatch, so the
    /// backing-app check judges THAT copy rather than a fresh re-read.
    manifest: Option<Agent>,
    chosen: Option<Chosen>,
    refusal: Option<AwareError>,
}

fn assess_agent(
    paths: &Paths,
    id: &str,
    lock: &LockFile,
    mode: Mode<'_>,
) -> Result<AgentOutcome, AwareError> {
    let pinned_version = lock.agent_pins.get(id).cloned();
    let pinned_digest = lock
        .agent_digests
        .get(id)
        .or_else(|| lock.agent_bundle_pins.get(id))
        .cloned();
    let mut outcome = AgentOutcome {
        agent: id.to_string(),
        pinned_version: pinned_version.clone(),
        pinned_digest: pinned_digest.clone(),
        installed_version: None,
        resolution: Resolution::Missing,
        detail: String::new(),
        invalid_candidates: Vec::new(),
        manifest: None,
        chosen: None,
        refusal: None,
    };

    // The current working copy. "Uninstalled" means there is no `agents/<id>/`
    // DIRECTORY (app-spec): today's missing-agent refusal, whatever the store
    // holds. A directory whose manifest is missing or does not parse is an
    // installed copy that cannot be used - a digest-pinned lock still runs its
    // stored approved bytes; a version-only lock is refused, naming why.
    let agents_dir = paths.agents_dir();
    let Ok(current_manifest) = crate::manifest::loader::agent_manifest_path(&agents_dir, id) else {
        outcome.detail = format!("agent {id} is not installed");
        return Ok(outcome);
    };
    let current_root = current_manifest
        .parent()
        .ok_or_else(|| AwareError::Internal("agent manifest has no parent".into()))?
        .to_path_buf();
    if agent_store::probe(&current_root)?.is_none() {
        outcome.detail = format!(
            "agent {id} is not installed; install it (`aware agent install {id}`) to run this app"
        );
        return Ok(outcome);
    }
    let mut notes: Vec<String> = Vec::new();
    let current: Option<Agent> = match crate::manifest::loader::load_agent(&current_manifest) {
        Ok(agent) => Some(agent),
        Err(AwareError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {
            notes.push(format!(
                "the installed copy at agents/{id} has no manifest.yaml"
            ));
            None
        }
        Err(error @ (AwareError::Validation(_) | AwareError::Yaml(_))) => {
            notes.push(format!(
                "the installed copy at agents/{id} has a manifest that cannot be used ({error})"
            ));
            None
        }
        // Could not READ it: a real fault, not "unusable" (review #626-2).
        Err(error) => return Err(error),
    };
    outcome.installed_version = current.as_ref().map(|agent| agent.version.clone());
    let installed = current
        .as_ref()
        .map(|agent| agent.version.clone())
        .unwrap_or_else(|| "a copy with no usable manifest".into());

    // Installed but absent from the lock: never approved.
    let Some(pinned) = pinned_version else {
        outcome.resolution = Resolution::NeverApproved;
        outcome.detail = format!(
            "agent {id} {installed} is installed but this app's compiled approval never pinned it; compile the app again"
        );
        outcome.refusal = Some(AwareError::Validation(format!(
            "[E_APP_LOCK_AGENT_PIN_MISMATCH] compiled approval pins agent {id} at no version, but the installed version is {installed}; run `aware app compile` again"
        )));
        return Ok(outcome);
    };

    let Some(required) = pinned_digest else {
        let Some(current) = current else {
            // A version-only lock can only run the current copy, and this one
            // has no usable manifest: refuse, saying exactly that.
            outcome.resolution = Resolution::LegacyPinMismatch;
            outcome.detail = format!(
                "{}; this app's approval names only a version ({pinned}), so nothing else can stand in for it — reinstall {id} or compile the app again",
                notes.join("; ")
            );
            outcome.refusal = Some(AwareError::Validation(format!(
                "[E_APP_LOCK_AGENT_PIN_MISMATCH] compiled approval pins agent {id} at {pinned}, but {}; reinstall the agent or run `aware app compile` again",
                notes.join("; ")
            )));
            return Ok(outcome);
        };
        return legacy_version_only(paths, id, &pinned, current, &current_root, mode, outcome);
    };

    // The current copy, if it still IS the approved bytes, supplies the package
    // (its own receipt kept); otherwise the store does. A copy that cannot be
    // hashed (symlink/reparse indirection, a non-UTF-8 name) is reported as
    // exactly that - never as "changed bytes"; a copy that cannot be READ is a
    // real error and propagates (the run fails with it, `app check` cannot run).
    let current_digest = if current.is_none() {
        None
    } else {
        match hash_current(&current_root)? {
            Ok(digest) => Some(digest),
            Err(reason) => {
                notes.push(format!("the installed copy cannot be hashed ({reason})"));
                None
            }
        }
    };
    let mut own: Option<StoredPackage> = None;
    let mut own_is_approved_in_check = false;
    let mut snapshot_error: Option<AwareError> = None;
    if current_digest.as_deref() == Some(required.as_str())
        && current
            .as_ref()
            .is_some_and(|agent| agent.version == pinned)
    {
        match mode {
            Mode::Run(_) => match agent_store::snapshot(paths, &current_root) {
                Ok(package) if package.digest == required && package.version == pinned => {
                    own = Some(package);
                }
                Ok(_) => notes.push("the installed copy changed while the run started".into()),
                // A filesystem fault (the package path cannot be looked at, a
                // copy cannot be written) is the real error: propagate it, as
                // `app check` does, rather than run around it.
                Err(error @ AwareError::Io(_)) => return Err(error),
                // Not a bundle mismatch: keep the error, and let a valid stored
                // package of the same bytes serve the run if there is one.
                Err(error) => {
                    notes.push(format!("snapshotting the installed copy failed ({error})"));
                    snapshot_error = Some(error);
                }
            },
            Mode::Check => {
                // Exactly what the run's snapshot would do: reuse a valid
                // package at this receipt key, or take a fresh one. A corrupt one
                // there makes the run fall through to the other candidates below.
                let key = agent_store::receipt_key(&current_root)?;
                let package = agent_store::digest_container(paths, id, &required)?.join(&key);
                // Absent only on NotFound; any other failure to look is the
                // check failing, never "not stored yet" (review #626 round 4).
                if agent_store::probe(&package)?.is_none()
                    || agent_store::verify_package(&package, id, &required, &key).is_ok()
                {
                    own_is_approved_in_check = true;
                } else {
                    notes.push(format!(
                        "snapshotting the installed copy would fail: the stored package {} does not verify",
                        package.display()
                    ));
                }
            }
        }
    }

    let mut valid: Vec<(u8, String, StoredPackage)> = Vec::new();
    let mut invalid: Vec<(PathBuf, String)> = Vec::new();
    for (key, dir) in agent_store::package_candidates(paths, id, &required)? {
        match agent_store::verify_package(&dir, id, &required, &key) {
            Ok(package) if package.version == pinned => {
                valid.push((agent_store::receipt_rank(&dir), key, package));
            }
            Ok(package) => invalid.push((
                dir,
                format!(
                    "it holds version {}, not the pinned {pinned}",
                    package.version
                ),
            )),
            Err(reason) => invalid.push((dir, reason)),
        }
    }
    valid.sort_by(|a, b| (a.0, &a.1).cmp(&(b.0, &b.1)));
    outcome.invalid_candidates = invalid
        .iter()
        .map(|(dir, reason)| InvalidCandidate {
            path: dir.display().to_string(),
            reason: reason.clone(),
        })
        .collect();
    let official_claim = valid
        .iter()
        .any(|(_, _, package)| crate::install::provenance::claims_official(&package.root))
        || own
            .as_ref()
            .is_some_and(|package| crate::install::provenance::claims_official(&package.root));
    let with_notes = |sentence: String, notes: &[String]| {
        if notes.is_empty() {
            sentence
        } else {
            format!("{sentence} ({})", notes.join("; "))
        }
    };

    if let Some(own) = own {
        let rest: Vec<StoredPackage> = valid
            .into_iter()
            .map(|(_, _, package)| package)
            .filter(|package| package.root != own.root)
            .collect();
        outcome.resolution = Resolution::Current;
        outcome.detail = format!("the installed {id} {pinned} is the approved copy");
        outcome.chosen = Some(Chosen {
            package: select(own, rest, mode),
            approval: Approval::Bytes,
            official_claim,
        });
        return Ok(outcome);
    }
    if own_is_approved_in_check {
        outcome.resolution = Resolution::Current;
        outcome.detail = format!("the installed {id} {pinned} is the approved copy");
        outcome.manifest = current;
        return Ok(outcome);
    }

    let mut ordered = valid.into_iter().map(|(_, _, package)| package);
    if let Some(first) = ordered.next() {
        outcome.resolution = Resolution::Stored;
        outcome.detail = with_notes(
            format!(
                "the approved {id} {pinned} runs from its stored copy; {installed} is installed"
            ),
            &notes,
        );
        if matches!(mode, Mode::Run(_)) {
            outcome.chosen = Some(Chosen {
                package: select(first, ordered.collect(), mode),
                approval: Approval::Bytes,
                official_claim,
            });
        } else {
            // The package the run's default order would choose; it verified
            // above, so its manifest loads.
            outcome.manifest = Some(agent_store::package_manifest(&first.root)?);
        }
        return Ok(outcome);
    }

    if let Some((dir, reason)) = invalid.into_iter().next() {
        outcome.resolution = Resolution::DigestMismatch;
        outcome.detail = with_notes(
            format!(
                "the stored copy of the approved {id} {pinned} at {} does not verify: {reason}",
                dir.display()
            ),
            &notes,
        );
        outcome.refusal = Some(bundle_mismatch(
            id,
            &required,
            &format!(
                "the stored copy at {} does not verify: {reason}",
                dir.display()
            ),
        ));
        return Ok(outcome);
    }

    // The snapshot of the matching copy failed and nothing else holds those
    // bytes: refuse with THAT error, in its own class.
    if let Some(error) = snapshot_error {
        outcome.resolution = Resolution::DigestMismatch;
        outcome.detail = error.to_string();
        outcome.refusal = Some(error);
        return Ok(outcome);
    }

    outcome.resolution = Resolution::PinNotInstalled;
    outcome.detail = if current_digest.is_none() {
        with_notes(
            format!(
                "no stored copy of the approved bytes of {id} {pinned} exists, and the installed copy cannot be compared with them"
            ),
            &notes,
        )
    } else if outcome.installed_version.as_deref() == Some(pinned.as_str()) {
        format!(
            "the approved bytes of {id} {pinned} are no longer installed - the installed copy has changed since compile; compile the app again to approve it"
        )
    } else {
        format!(
            "the approved version of {id} ({pinned}) is no longer installed; compile the app again to use {installed}"
        )
    };
    outcome.refusal = Some(AwareError::Validation(format!(
        "[E_APP_LOCK_AGENT_PIN_MISMATCH] {}; run `aware app compile` again",
        outcome.detail
    )));
    Ok(outcome)
}

/// Hash the installed copy. `Ok(Err(reason))` when the tree cannot be hashed by
/// rule (symlink/reparse indirection, a non-UTF-8 name, a non-regular entry) -
/// a fact about the copy the caller reports as data; `Err` when it cannot be
/// READ, which is a real fault the caller propagates.
fn hash_current(root: &Path) -> Result<Result<String, String>, AwareError> {
    match crate::install::integrity::tree_digest(root) {
        Ok(digest) => Ok(Ok(digest)),
        Err(AwareError::Validation(reason)) => Ok(Err(reason)),
        Err(error) => Err(error),
    }
}

/// A lock with no digest for this agent: today's version check, then the run
/// dispatches an immutable snapshot of the current copy, labelled version-only.
fn legacy_version_only(
    paths: &Paths,
    id: &str,
    pinned: &str,
    current: Agent,
    current_root: &Path,
    mode: Mode<'_>,
    mut outcome: AgentOutcome,
) -> Result<AgentOutcome, AwareError> {
    if current.version != pinned {
        outcome.resolution = Resolution::LegacyPinMismatch;
        outcome.detail = format!(
            "this app was compiled by an older AWARE that recorded only a version ({pinned}); {} is installed, so compile the app again",
            current.version
        );
        outcome.refusal = Some(AwareError::Validation(format!(
            "[E_APP_LOCK_AGENT_PIN_MISMATCH] compiled approval pins agent {id} at {pinned}, but the installed version is {}; run `aware app compile` again",
            current.version
        )));
        return Ok(outcome);
    }
    match mode {
        Mode::Run(_) => match agent_store::snapshot(paths, current_root) {
            Ok(package) if package.version == pinned => {
                outcome.resolution = Resolution::CurrentVersionOnly;
                outcome.detail = format!(
                    "the installed {id} {pinned} matches the pinned version (this lock approved a version, not bytes)"
                );
                let official_claim = crate::install::provenance::claims_official(&package.root);
                outcome.chosen = Some(Chosen {
                    package,
                    approval: Approval::VersionOnly,
                    official_claim,
                });
            }
            Ok(package) => {
                outcome.resolution = Resolution::LegacyPinMismatch;
                outcome.detail =
                    format!("{id} changed to {} while the run started", package.version);
                outcome.refusal = Some(AwareError::Validation(format!(
                    "[E_APP_LOCK_AGENT_PIN_MISMATCH] compiled approval pins agent {id} at {pinned}, but the installed version is {}; run `aware app compile` again",
                    package.version
                )));
            }
            Err(error) => {
                outcome.resolution = Resolution::DigestMismatch;
                outcome.detail = error.to_string();
                outcome.refusal = Some(error);
            }
        },
        Mode::Check => {
            // The run's snapshot would refuse a copy it cannot hash: report the
            // same refusal as data, not as a check that could not run.
            let digest = match hash_current(current_root)? {
                Ok(digest) => digest,
                Err(reason) => {
                    outcome.resolution = Resolution::DigestMismatch;
                    outcome.detail = format!(
                        "the installed {id} cannot be hashed ({reason}), so it cannot be snapshotted to run"
                    );
                    return Ok(outcome);
                }
            };
            let key = agent_store::receipt_key(current_root)?;
            let package = agent_store::digest_container(paths, id, &digest)?.join(&key);
            if agent_store::probe(&package)?.is_some()
                && let Err(reason) = agent_store::verify_package(&package, id, &digest, &key)
            {
                outcome.resolution = Resolution::DigestMismatch;
                outcome.detail = format!(
                    "the stored snapshot of the installed {id} at {} does not verify: {reason}",
                    package.display()
                );
                return Ok(outcome);
            }
            outcome.resolution = Resolution::CurrentVersionOnly;
            outcome.detail = format!(
                "the installed {id} {pinned} matches the pinned version (this lock approved a version, not bytes; compile again to approve the exact bytes)"
            );
            outcome.manifest = Some(current);
        }
    }
    Ok(outcome)
}

/// Pick a package from candidates already in the receipt-choice order
/// (`first` is the default choice, `rest` the remaining order).
fn select(first: StoredPackage, rest: Vec<StoredPackage>, mode: Mode<'_>) -> StoredPackage {
    let Mode::Run(Selection::Verified(index)) = mode else {
        return first;
    };
    let verifies = |package: &StoredPackage| {
        crate::install::provenance::assess_against_index(
            &package.root,
            &package.agent,
            &package.version,
            Some(index),
        )
        .verified
    };
    if verifies(&first) {
        return first;
    }
    // None verifying: keep the default choice; the strict check then refuses
    // with the existing assessment's reason.
    rest.into_iter().find(verifies).unwrap_or(first)
}

fn bundle_mismatch(id: &str, digest: &str, reason: &str) -> AwareError {
    AwareError::Validation(format!(
        "[E_APP_LOCK_AGENT_BUNDLE_PIN_MISMATCH] compiled approval pins agent {id} at bundle {digest}, but {reason}; nothing else will be run in its place — restore it or run `aware app compile` again"
    ))
}

// ── #628: resolve an app against a CHOSEN set of pins ──────────────────────────

/// Where a migration target pin points.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PinTarget {
    /// Exact stored bytes: `<agent>@sha256:<hex>`.
    Digest(String),
    /// A version that must map to exactly ONE stored digest: `<agent>@<version>`.
    Version(String),
}

/// One agent pin of the base lock: its version and, when the lock names
/// bytes, their digest (`agent-digests`, else an official `agent-bundle-pins`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BasePin {
    pub version: String,
    pub digest: Option<String>,
}

/// The pins a candidate resolves against: the base lock's, with some agents
/// retargeted. Every agent NOT in `targets` keeps the base lock's own bytes — a
/// Tekla candidate never drags in a newer google-workspace.
#[derive(Debug, Clone, Default)]
pub struct PinSet {
    pub base: BTreeMap<String, BasePin>,
    pub targets: BTreeMap<String, PinTarget>,
}

impl PinSet {
    /// The base lock's pins with `targets` applied over them — refused, as the
    /// run refuses it (`E_APP_LOCK_INVALID`), when the lock's digest fields are
    /// malformed or disagree. Without this, `agent-digests` would silently win
    /// over a conflicting `agent-bundle-pins` and a caller could judge a lock
    /// that `aware app run` will never execute (Codex review #628 round 3).
    pub fn from_lock(
        lock: &LockFile,
        targets: BTreeMap<String, PinTarget>,
    ) -> Result<Self, AwareError> {
        check_lock_consistency(lock).map_err(|reason| inconsistent_lock(lock, &reason))?;
        let base = lock
            .agent_pins
            .iter()
            .map(|(id, version)| {
                let digest = lock
                    .agent_digests
                    .get(id)
                    .or_else(|| lock.agent_bundle_pins.get(id))
                    .cloned();
                (
                    id.clone(),
                    BasePin {
                        version: version.clone(),
                        digest,
                    },
                )
            })
            .collect();
        Ok(PinSet { base, targets })
    }
}

/// Which side of a [`PinSet`] an agent resolved from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PinSource {
    Base,
    Target,
}

/// One agent resolved by [`resolve_pins`]: a verified store package.
#[derive(Debug)]
pub struct ResolvedPin {
    pub agent: DiscoveredAgent,
    pub version: String,
    pub digest: String,
    pub source: PinSource,
    /// Store packages or records that claimed this agent's chosen bytes or
    /// target version but do not verify — skipped, never used, reported.
    pub invalid_candidates: Vec<InvalidCandidate>,
}

/// Resolve every agent `app` dispatches to the store package `pins` names —
/// **side-effect free** (#628 plan §1): nothing is snapshotted, written or
/// repaired, and the working copy under `agents/` is never read; only verified
/// store packages are used.
///
/// * an agent in `pins.targets` resolves to the target bytes — a `Version`
///   target must map to exactly one stored digest (`E_MIGRATE_TARGET_AMBIGUOUS`
///   when it maps to several, `E_MIGRATE_TARGET_NOT_STORED` when to none);
/// * every other agent resolves to the base lock's own digest — never to a
///   newer copy that happens to be installed;
/// * a target naming an agent the app does not dispatch, an agent the base
///   lock never pinned, or a base pin with no digest (a lock compiled by AWARE
///   ≤ 0.148) is refused, naming it: none of these can be carried forward
///   without a person compiling the app again.
///
/// Frozen-only agents are not dispatchable and are not resolved here; a
/// candidate keeps their base pins (plan §1).
pub fn resolve_pins(
    paths: &Paths,
    app: &App,
    pins: &PinSet,
) -> Result<Vec<ResolvedPin>, AwareError> {
    let dispatched = sorted_dispatchable(app);
    for id in pins.targets.keys() {
        if !dispatched.contains(&id.as_str()) {
            return Err(AwareError::Validation(format!(
                "[E_MIGRATE_TARGET_UNUSED] app {} does not dispatch agent {id}, so there is nothing to move it to",
                app.app
            )));
        }
    }
    let mut out = Vec::new();
    for id in dispatched {
        // Every agent must have approved BYTES in the base lock before anything
        // is applied over it: a target moves an approval, it never creates one
        // (review #628-1).
        let base = pins.base.get(id).ok_or_else(|| {
            AwareError::Validation(format!(
                "[E_MIGRATE_PIN_MISSING] app {}'s approval never pinned agent {id}; compile the app again",
                app.app
            ))
        })?;
        let base_digest = base.digest.clone().ok_or_else(|| {
            AwareError::Validation(format!(
                "[E_MIGRATE_BASE_VERSION_ONLY] app {}'s approval names only a version of {id} ({}), not its bytes; compile the app again before migrating it",
                app.app, base.version
            ))
        })?;
        let mut invalid_candidates = Vec::new();
        let (digest, source) = match pins.targets.get(id) {
            Some(PinTarget::Digest(digest)) => (digest.clone(), PinSource::Target),
            Some(PinTarget::Version(version)) => {
                let (digest, skipped) = unique_stored_digest(paths, id, version)?;
                invalid_candidates = skipped;
                (digest, PinSource::Target)
            }
            None => (base_digest, PinSource::Base),
        };
        let (package, skipped) = match source {
            PinSource::Base => stored_package(paths, id, &digest)?,
            // A target the caller named: its absence is about THAT target, not
            // about the compiled approval, which never pinned it (review #628
            // PR2 round 1).
            PinSource::Target => stored_target(paths, id, &digest)?,
        };
        for candidate in skipped {
            if !invalid_candidates.contains(&candidate) {
                invalid_candidates.push(candidate);
            }
        }
        if source == PinSource::Base && package.version != base.version {
            return Err(bundle_mismatch(
                id,
                &digest,
                &format!(
                    "the stored copy holds version {}, not the pinned {}",
                    package.version, base.version
                ),
            ));
        }
        let manifest = agent_store::package_manifest(&package.root)?;
        out.push(ResolvedPin {
            version: package.version,
            digest: package.digest,
            source,
            invalid_candidates,
            agent: DiscoveredAgent {
                manifest,
                root: package.root,
            },
        });
    }
    Ok(out)
}

/// The verified store package of `id` with bytes `digest`, loaded — or `None`
/// when no stored copy of those bytes verifies. Side-effect free, like
/// [`resolve_pins`]. A candidate uses it for an agent only FROZEN nodes
/// reference (#628 plan §1): such an agent is never dispatched, so its absence
/// is not a refusal — the candidate keeps the base lock's pin verbatim.
pub fn stored_agent(
    paths: &Paths,
    id: &str,
    digest: &str,
) -> Result<Option<DiscoveredAgent>, AwareError> {
    let (valid, _) = verified_candidates(paths, id, digest)?;
    let Some(package) = valid.into_iter().next() else {
        return Ok(None);
    };
    let manifest = agent_store::package_manifest(&package.root)?;
    Ok(Some(DiscoveredAgent {
        manifest,
        root: package.root,
    }))
}

/// Every store package of `id` claiming bytes `digest`, verified: the valid
/// ones in the receipt order a run uses, and the ones that do not verify.
fn verified_candidates(
    paths: &Paths,
    id: &str,
    digest: &str,
) -> Result<(Vec<StoredPackage>, Vec<InvalidCandidate>), AwareError> {
    let mut valid = Vec::new();
    let mut invalid = Vec::new();
    for (key, dir) in agent_store::package_candidates(paths, id, digest)? {
        match agent_store::verify_package(&dir, id, digest, &key) {
            Ok(package) => valid.push((agent_store::receipt_rank(&dir), key, package)),
            Err(reason) => invalid.push(InvalidCandidate {
                path: dir.display().to_string(),
                reason,
            }),
        }
    }
    valid.sort_by(|a, b| (a.0, &a.1).cmp(&(b.0, &b.1)));
    Ok((valid.into_iter().map(|(_, _, p)| p).collect(), invalid))
}

/// The verified store package of `id` with bytes `digest`, chosen by the same
/// receipt order a run uses, plus the candidates that did not verify. Refuses
/// when none verifies.
fn stored_package(
    paths: &Paths,
    id: &str,
    digest: &str,
) -> Result<(StoredPackage, Vec<InvalidCandidate>), AwareError> {
    let (valid, invalid) = verified_candidates(paths, id, digest)?;
    match valid.into_iter().next() {
        Some(package) => Ok((package, invalid)),
        None if invalid.is_empty() => Err(AwareError::Validation(format!(
            "[E_MIGRATE_PIN_NOT_STORED] no stored copy of agent {id} {digest} exists on this machine"
        ))),
        None => Err(bundle_mismatch(
            id,
            digest,
            &format!(
                "no stored copy verifies ({})",
                invalid
                    .iter()
                    .map(|c| format!("{}: {}", c.path, c.reason))
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
        )),
    }
}

/// [`stored_package`] for a migration TARGET: no verified copy is
/// `E_MIGRATE_TARGET_NOT_STORED`, naming the copies that did not verify.
fn stored_target(
    paths: &Paths,
    id: &str,
    digest: &str,
) -> Result<(StoredPackage, Vec<InvalidCandidate>), AwareError> {
    let (valid, invalid) = verified_candidates(paths, id, digest)?;
    match valid.into_iter().next() {
        Some(package) => Ok((package, invalid)),
        None if invalid.is_empty() => Err(AwareError::Validation(format!(
            "[E_MIGRATE_TARGET_NOT_STORED] no stored copy of agent {id} {digest} exists on this machine"
        ))),
        None => Err(AwareError::Validation(format!(
            "[E_MIGRATE_TARGET_NOT_STORED] no stored copy of agent {id} {digest} verifies on this machine ({})",
            invalid
                .iter()
                .map(|c| format!("{}: {}", c.path, c.reason))
                .collect::<Vec<_>>()
                .join("; ")
        ))),
    }
}

/// The ONE stored digest of `id` at `version`. The store records only
/// nominate digests; each is VERIFIED before it counts, so a corrupt or forged
/// record claiming the version can neither make a target ambiguous nor be
/// chosen (review #628-2). Records that do not verify — and store entries
/// whose record cannot be read at all — are returned for reporting.
fn unique_stored_digest(
    paths: &Paths,
    id: &str,
    version: &str,
) -> Result<(String, Vec<InvalidCandidate>), AwareError> {
    let listing = agent_store::stored_versions(paths, id);
    let nominated: BTreeSet<&str> = listing
        .stored
        .iter()
        .filter(|stored| stored.version == version)
        .map(|stored| stored.digest.as_str())
        .collect();
    let mut invalid = listing.unreadable.clone();
    let mut verified: Vec<&str> = Vec::new();
    for digest in nominated {
        let (valid, bad) = verified_candidates(paths, id, digest)?;
        invalid.extend(bad);
        if valid.iter().any(|package| package.version == version) {
            verified.push(digest);
        }
    }
    match verified.as_slice() {
        [one] => Ok(((*one).to_string(), invalid)),
        [] if invalid.is_empty() => Err(AwareError::Validation(format!(
            "[E_MIGRATE_TARGET_NOT_STORED] no stored copy of agent {id} {version} exists on this machine"
        ))),
        [] => Err(AwareError::Validation(format!(
            "[E_MIGRATE_TARGET_NOT_STORED] no stored copy of agent {id} {version} verifies on this machine ({})",
            invalid
                .iter()
                .map(|c| format!("{}: {}", c.path, c.reason))
                .collect::<Vec<_>>()
                .join("; ")
        ))),
        many => Err(AwareError::Validation(format!(
            "[E_MIGRATE_TARGET_AMBIGUOUS] agent {id} {version} is stored as {} different verified byte sets ({}); name one as {id}@sha256:<hex>",
            many.len(),
            many.join(", ")
        ))),
    }
}

// ── `aware app check` ──────────────────────────────────────────────────────────

/// Whether the app's compiled approval could be read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LockState {
    Valid,
    Missing,
    Invalid,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AgentCheck {
    pub agent: String,
    pub pinned_version: Option<String>,
    pub pinned_digest: Option<String>,
    pub installed_version: Option<String>,
    pub resolution: Resolution,
    pub detail: String,
    /// Stored packages of the approved bytes that do not verify (skipped by
    /// the run, which warns about them).
    pub invalid_candidates: Vec<InvalidCandidate>,
    /// For a leaf of an app-backed agent: that agent's id. Absent otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via: Option<String>,
}

/// The approval of one app-backed agent's backing app.
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct NestedCheck {
    pub agent: String,
    pub app: String,
    pub lock: LockState,
    pub source_current: bool,
    pub detail: String,
}

/// `aware app check --json` `data`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AppCheck {
    pub app: String,
    pub approval_current: bool,
    pub approval_kind: Option<Approval>,
    pub source_current: bool,
    pub lock: LockState,
    pub lock_detail: Option<String>,
    pub agents: Vec<AgentCheck>,
    pub nested_apps: Vec<NestedCheck>,
}

/// Answer "would `aware app run` refuse this app with an `E_APP_LOCK_*`
/// code?" with the run's own resolver, writing nothing. Every expected drift is
/// data; `Err` only when the check itself cannot run (unreadable source, an
/// unreadable agent manifest, an unreadable AWARE_HOME).
pub fn check_app(paths: &Paths, source: &Path) -> Result<AppCheck, AwareError> {
    let (app, source_hash) = crate::app_lock::read_app_source(source)?;
    let lock_path = source
        .parent()
        .ok_or_else(|| AwareError::Internal("source path has no parent".into()))?
        .join(format!("{}.lock", app.app));
    let mut check = AppCheck {
        app: app.app.clone(),
        approval_current: false,
        approval_kind: None,
        source_current: false,
        lock: LockState::Missing,
        lock_detail: None,
        agents: Vec::new(),
        nested_apps: Vec::new(),
    };
    let lock = match read_lock(&lock_path) {
        Ok(lock) => lock,
        Err((state, detail)) => {
            check.lock = state;
            check.lock_detail = Some(detail);
            return Ok(check);
        }
    };
    check.source_current = lock.source_hash == source_hash;
    if let Err(reason) = check_lock_consistency(&lock) {
        check.lock = LockState::Invalid;
        check.lock_detail = Some(reason);
        return Ok(check);
    }
    check.lock = LockState::Valid;

    let mut nested_ok = true;
    for id in sorted_dispatchable(&app) {
        let mut outcome = assess_agent(paths, id, &lock, Mode::Check)?;
        let runs = outcome.resolution.runs();
        let manifest = outcome.manifest.take();
        check.agents.push(agent_row(outcome, None));
        if !runs {
            continue;
        }
        // An app-backed agent's backing app has an approval of its own, which
        // the run also enforces — judged on the copy the resolver chose.
        nested_ok &= fold_backing(paths, id, manifest.as_ref(), &mut check)?;
    }
    let agents_ok = check.agents.iter().all(|row| row.resolution.runs());
    check.approval_current = check.source_current && agents_ok && nested_ok;
    if check.approval_current {
        check.approval_kind = Some(
            if check
                .agents
                .iter()
                .any(|row| row.resolution == Resolution::CurrentVersionOnly)
            {
                Approval::VersionOnly
            } else {
                Approval::Bytes
            },
        );
    }
    Ok(check)
}

fn agent_row(outcome: AgentOutcome, via: Option<&str>) -> AgentCheck {
    AgentCheck {
        agent: outcome.agent,
        pinned_version: outcome.pinned_version,
        pinned_digest: outcome.pinned_digest,
        installed_version: outcome.installed_version,
        resolution: outcome.resolution,
        detail: outcome.detail,
        invalid_candidates: outcome.invalid_candidates,
        via: via.map(str::to_string),
    }
}

/// Fold the backing-app check of one runnable agent into `check`, from the
/// manifest of the copy the resolver chose. Without that manifest the backing
/// app cannot be judged, which fails the check rather than passing it.
fn fold_backing(
    paths: &Paths,
    id: &str,
    manifest: Option<&Agent>,
    check: &mut AppCheck,
) -> Result<bool, AwareError> {
    let Some(manifest) = manifest else {
        check.nested_apps.push(NestedCheck {
            agent: id.to_string(),
            app: String::new(),
            lock: LockState::Missing,
            source_current: false,
            detail: format!(
                "could not determine which copy of {id} the run would use, so any app behind it was not checked"
            ),
        });
        return Ok(false);
    };
    if !matches!(
        crate::runtime::invoker::effective_transport(manifest, id),
        Ok(crate::runtime::invoker::TransportKind::App)
    ) {
        return Ok(true);
    }
    check_backing(paths, id, manifest, check)
}

fn check_backing(
    paths: &Paths,
    id: &str,
    manifest: &Agent,
    check: &mut AppCheck,
) -> Result<bool, AwareError> {
    let Some(backed_by) = manifest
        .transport
        .app
        .as_ref()
        .map(|transport| transport.backed_by.clone())
    else {
        return Ok(true);
    };
    let mut row = NestedCheck {
        agent: id.to_string(),
        app: backed_by.clone(),
        lock: LockState::Missing,
        source_current: false,
        detail: String::new(),
    };
    let source = if crate::manifest::loader::is_safe_segment(&backed_by) {
        let backing_dir = paths.apps_dir().join(&backed_by);
        // As in the run (`resolve_backing`): a directory that cannot be looked
        // at fails the check, it is not "not installed" (review #626 round 4).
        agent_store::probe(&backing_dir)?;
        crate::manifest::loader::find_app_manifest(&backing_dir)
    } else {
        None
    };
    let Some(source) = source else {
        row.detail = format!("backing app {backed_by} is not installed");
        check.nested_apps.push(row);
        return Ok(false);
    };
    let (backing, hash) = crate::app_lock::read_app_source(&source)?;
    let lock_path = source
        .parent()
        .ok_or_else(|| AwareError::Internal("source path has no parent".into()))?
        .join(format!("{}.lock", backing.app));
    let lock = match read_lock(&lock_path) {
        Ok(lock) => lock,
        Err((state, detail)) => {
            row.lock = state;
            row.detail = detail;
            check.nested_apps.push(row);
            return Ok(false);
        }
    };
    row.source_current = lock.source_hash == hash;
    if let Err(reason) = check_lock_consistency(&lock) {
        row.lock = LockState::Invalid;
        row.detail = reason;
        check.nested_apps.push(row);
        return Ok(false);
    }
    row.lock = LockState::Valid;
    let mut ok = row.source_current;
    row.detail = if row.source_current {
        format!("backing app {backed_by} is approved")
    } else {
        format!("backing app {backed_by} changed since it was compiled")
    };
    check.nested_apps.push(row);
    for leaf in sorted_dispatchable(&backing) {
        let outcome = assess_agent(paths, leaf, &lock, Mode::Check)?;
        ok &= outcome.resolution.runs();
        check.agents.push(agent_row(outcome, Some(id)));
    }
    Ok(ok)
}

fn read_lock(lock_path: &Path) -> Result<LockFile, (LockState, String)> {
    let text = match std::fs::read_to_string(lock_path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Err((
                LockState::Missing,
                format!("no compiled approval at {}", lock_path.display()),
            ));
        }
        Err(error) => {
            return Err((
                LockState::Invalid,
                format!("cannot read {}: {error}", lock_path.display()),
            ));
        }
    };
    serde_yaml::from_str(&text).map_err(|error| {
        (
            LockState::Invalid,
            format!("{} is invalid: {error}", lock_path.display()),
        )
    })
}

#[cfg(test)]
mod tests;
