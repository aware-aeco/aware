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
    /// (from the backing app's OWN resolution). Sorted, deduplicated by id.
    pub fn reachable(&self, app: &App) -> Vec<Reachable<'_>> {
        let mut seen = BTreeSet::new();
        let mut out = Vec::new();
        for id in sorted_dispatchable(app) {
            let Some(agent) = self.get(id) else {
                continue; // the missing-agent preflight reports it
            };
            if let Some(nested) = self.nested.get(id) {
                for leaf in sorted_dispatchable(&nested.app) {
                    if seen.insert(leaf.to_string()) {
                        out.push(Reachable {
                            id: leaf.to_string(),
                            agent: nested.catalogue.get(leaf),
                            info: nested.catalogue.info(leaf),
                        });
                    }
                }
            } else if seen.insert(id.to_string()) {
                out.push(Reachable {
                    id: id.to_string(),
                    agent: Some(agent),
                    info: self.info.get(id),
                });
            }
        }
        out
    }
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
        chosen: None,
        refusal: None,
    };

    // The current working copy. Missing means uninstalled: today's
    // missing-agent refusal, whatever the store holds.
    let agents_dir = paths.agents_dir();
    let Ok(current_manifest) = crate::manifest::loader::agent_manifest_path(&agents_dir, id) else {
        outcome.detail = format!("agent {id} is not installed");
        return Ok(outcome);
    };
    match std::fs::symlink_metadata(&current_manifest) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            outcome.detail = format!(
                "agent {id} is not installed; install it (`aware agent install {id}`) to run this app"
            );
            return Ok(outcome);
        }
        Err(error) => {
            return Err(std::io::Error::new(
                error.kind(),
                format!("{}: {error}", current_manifest.display()),
            )
            .into());
        }
        Ok(_) => {}
    }
    let current = crate::manifest::loader::load_agent(&current_manifest)?;
    let current_root = current_manifest
        .parent()
        .ok_or_else(|| AwareError::Internal("agent manifest has no parent".into()))?
        .to_path_buf();
    outcome.installed_version = Some(current.version.clone());

    // Installed but absent from the lock: never approved.
    let Some(pinned) = pinned_version else {
        outcome.resolution = Resolution::NeverApproved;
        outcome.detail = format!(
            "agent {id} {} is installed but this app's compiled approval never pinned it; compile the app again",
            current.version
        );
        outcome.refusal = Some(AwareError::Validation(format!(
            "[E_APP_LOCK_AGENT_PIN_MISMATCH] compiled approval pins agent {id} at no version, but the installed version is {}; run `aware app compile` again",
            current.version
        )));
        return Ok(outcome);
    };

    let Some(required) = pinned_digest else {
        return legacy_version_only(paths, id, &pinned, &current, &current_root, mode, outcome);
    };

    // The current copy, if it still IS the approved bytes, supplies the package
    // (its own receipt kept); otherwise the store does.
    let current_digest = crate::install::integrity::tree_digest(&current_root).ok();
    let mut own: Option<StoredPackage> = None;
    if current_digest.as_deref() == Some(required.as_str()) && current.version == pinned {
        match mode {
            Mode::Run(_) => match agent_store::snapshot(paths, &current_root) {
                Ok(package) if package.digest == required && package.version == pinned => {
                    own = Some(package);
                }
                Ok(_) => {} // changed under us; the store decides below
                Err(error) => {
                    outcome.resolution = Resolution::DigestMismatch;
                    outcome.detail = error.to_string();
                    outcome.refusal = Some(bundle_mismatch(id, &required, &error.to_string()));
                    return Ok(outcome);
                }
            },
            Mode::Check => {
                let key = agent_store::receipt_key(&current_root)?;
                let package = agent_store::digest_container(paths, id, &required)?.join(&key);
                if std::fs::symlink_metadata(&package).is_ok()
                    && let Err(reason) = agent_store::verify_package(&package, id, &required, &key)
                {
                    outcome.resolution = Resolution::DigestMismatch;
                    outcome.detail = format!(
                        "the stored copy of the approved bytes at {} does not verify: {reason}",
                        package.display()
                    );
                    return Ok(outcome);
                }
                outcome.resolution = Resolution::Current;
                outcome.detail = format!("the installed {id} {pinned} is the approved copy");
                return Ok(outcome);
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
    let official_claim = valid
        .iter()
        .any(|(_, _, package)| crate::install::provenance::claims_official(&package.root))
        || own
            .as_ref()
            .is_some_and(|package| crate::install::provenance::claims_official(&package.root));

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

    let mut ordered = valid.into_iter().map(|(_, _, package)| package);
    if let Some(first) = ordered.next() {
        outcome.resolution = Resolution::Stored;
        outcome.detail = format!(
            "the approved {id} {pinned} runs from its stored copy; {} is installed",
            current.version
        );
        if matches!(mode, Mode::Run(_)) {
            outcome.chosen = Some(Chosen {
                package: select(first, ordered.collect(), mode),
                approval: Approval::Bytes,
                official_claim,
            });
        }
        return Ok(outcome);
    }

    if let Some((dir, reason)) = invalid.into_iter().next() {
        outcome.resolution = Resolution::DigestMismatch;
        outcome.detail = format!(
            "the stored copy of the approved {id} {pinned} at {} does not verify: {reason}",
            dir.display()
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

    outcome.resolution = Resolution::PinNotInstalled;
    outcome.detail = if current.version == pinned {
        format!(
            "the approved bytes of {id} {pinned} are no longer installed — the installed copy has changed since compile; compile the app again to approve it"
        )
    } else {
        format!(
            "the approved version of {id} ({pinned}) is no longer installed; compile the app again to use {}",
            current.version
        )
    };
    outcome.refusal = Some(AwareError::Validation(format!(
        "[E_APP_LOCK_AGENT_PIN_MISMATCH] {}; run `aware app compile` again",
        outcome.detail
    )));
    Ok(outcome)
}

/// A lock with no digest for this agent: today's version check, then the run
/// dispatches an immutable snapshot of the current copy, labelled version-only.
fn legacy_version_only(
    paths: &Paths,
    id: &str,
    pinned: &str,
    current: &Agent,
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
            let digest = crate::install::integrity::tree_digest(current_root)?;
            let key = agent_store::receipt_key(current_root)?;
            let package = agent_store::digest_container(paths, id, &digest)?.join(&key);
            if std::fs::symlink_metadata(&package).is_ok()
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
        let outcome = assess_agent(paths, id, &lock, Mode::Check)?;
        let runs = outcome.resolution.runs();
        let resolution = outcome.resolution;
        check.agents.push(agent_row(outcome, None));
        if !runs || resolution == Resolution::Missing {
            continue;
        }
        // An app-backed agent's backing app has an approval of its own, which the
        // run also enforces. Its manifest is read from the copy the run would use.
        let Some(manifest) = manifest_for_check(paths, id, &lock, resolution)? else {
            continue;
        };
        if !matches!(
            crate::runtime::invoker::effective_transport(&manifest, id),
            Ok(crate::runtime::invoker::TransportKind::App)
        ) {
            continue;
        }
        nested_ok &= check_backing(paths, id, &manifest, &mut check)?;
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
        via: via.map(str::to_string),
    }
}

/// The manifest the run would dispatch for an agent that resolves: the current
/// copy when it is the approved one, else the first valid stored package.
fn manifest_for_check(
    paths: &Paths,
    id: &str,
    lock: &LockFile,
    resolution: Resolution,
) -> Result<Option<Agent>, AwareError> {
    if resolution != Resolution::Stored {
        return crate::manifest::loader::load_agent_by_id(&paths.agents_dir(), id).map(Some);
    }
    let Some(required) = lock
        .agent_digests
        .get(id)
        .or_else(|| lock.agent_bundle_pins.get(id))
    else {
        return Ok(None);
    };
    for (key, dir) in agent_store::package_candidates(paths, id, required)? {
        if agent_store::verify_package(&dir, id, required, &key).is_ok() {
            return agent_store::package_manifest(&dir).map(Some);
        }
    }
    Ok(None)
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
    let source = crate::manifest::loader::is_safe_segment(&backed_by)
        .then(|| crate::manifest::loader::find_app_manifest(&paths.apps_dir().join(&backed_by)))
        .flatten();
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
