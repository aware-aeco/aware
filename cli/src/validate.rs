//! Schema + cycle + capability validation for agents and apps.
//!
//! Pure functions — no I/O except optional `skills/` directory probe when
//! the caller passes a path (used by `validate_agent_on_disk`).
//!
//! Public API consumed by:
//! - Task 6  (`agent validate` command)
//! - Task 14 (`app validate` command)
//! - install pre-flight (Task 10+)

use std::collections::{BTreeMap, HashSet};
use std::path::Path;

use crate::manifest::{Agent, App};

#[derive(Debug, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
        }
    }
}

#[derive(Debug)]
pub struct ValidationIssue {
    pub severity: Severity,
    pub code: &'static str,
    pub message: String,
}

impl ValidationIssue {
    fn error(code: &'static str, msg: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            code,
            message: msg.into(),
        }
    }
    fn warning(code: &'static str, msg: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            code,
            message: msg.into(),
        }
    }
}

pub fn has_errors(issues: &[ValidationIssue]) -> bool {
    issues.iter().any(|i| i.severity == Severity::Error)
}

/// Validate an `Agent` manifest without touching disk. Catches issues
/// `serde` can't (empty command map, missing transport, stateful-without-start, etc.).
pub fn validate_agent(agent: &Agent) -> Vec<ValidationIssue> {
    let mut out = Vec::new();
    if agent.agent.trim().is_empty() {
        out.push(ValidationIssue::error(
            "E_AGENT_EMPTY_ID",
            "agent id is empty",
        ));
    }
    if agent.commands.is_empty() {
        out.push(ValidationIssue::error(
            "E_AGENT_NO_COMMANDS",
            "agent declares zero commands",
        ));
    }
    if agent.transport.cli.is_none()
        && agent.transport.mcp.is_none()
        && agent.transport.rest.is_none()
        && agent.transport.app.is_none()
        && agent.transport.builtin.is_none()
    {
        out.push(ValidationIssue::error(
            "E_AGENT_NO_TRANSPORT",
            "agent must declare at least one transport (cli / mcp / rest / app / builtin)",
        ));
    }
    if agent.stateful {
        let has_start = agent
            .commands
            .values()
            .any(|c| matches!(c.lifecycle, crate::manifest::agent::Lifecycle::Start));
        if !has_start {
            out.push(ValidationIssue::error(
                "E_STATEFUL_NO_START",
                "stateful agent must have at least one command with lifecycle: start",
            ));
        }
    }
    out
}

/// Disk-aware: confirms every manifest-listed skill file exists, warns on orphans.
pub fn validate_agent_on_disk(agent: &Agent, agent_root: &Path) -> Vec<ValidationIssue> {
    let mut out = validate_agent(agent);
    let skills_dir = agent_root.join("skills");
    for skill in &agent.skills {
        if !skills_dir.join(skill).is_file() {
            out.push(ValidationIssue::error(
                "E_SKILL_FILE_MISSING",
                format!("manifest lists skill {skill:?} but file is missing under skills/"),
            ));
        }
    }
    if skills_dir.is_dir() {
        let listed: HashSet<_> = agent.skills.iter().cloned().collect();
        if let Ok(read) = std::fs::read_dir(&skills_dir) {
            for entry in read.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.ends_with(".md") && !listed.contains(&name) {
                    out.push(ValidationIssue::warning(
                        "W_SKILL_ORPHAN",
                        format!("skill file {name:?} on disk but not listed in manifest skills:"),
                    ));
                }
            }
        }
    }
    out
}

/// Validate an `App` manifest: cycles, dangling refs, unique node ids, inline-glue descriptions.
pub fn validate_app(app: &App) -> Vec<ValidationIssue> {
    let mut out = Vec::new();
    if app.app.trim().is_empty() {
        out.push(ValidationIssue::error("E_APP_EMPTY_ID", "app id is empty"));
    }
    if app.nodes.is_empty() {
        out.push(ValidationIssue::error(
            "E_APP_NO_NODES",
            "app declares zero nodes",
        ));
    }

    // An `exposes-as-agent: true` app must declare at least one exposed command —
    // otherwise the synthesized agent has zero commands and is uncallable
    // (app-spec § exposes-as-agent / exposed-commands).
    if app.exposes_as_agent && app.exposed_commands.as_ref().is_none_or(|m| m.is_empty()) {
        out.push(ValidationIssue::error(
            "E_APP_EXPOSES_NO_COMMANDS",
            "app sets `exposes-as-agent: true` but declares no `exposed-commands:` \
             (the synthesized agent would have zero callable commands)",
        ));
    }

    let mut seen = HashSet::new();
    for n in &app.nodes {
        if !seen.insert(n.id.as_str()) {
            out.push(ValidationIssue::error(
                "E_APP_DUPLICATE_NODE_ID",
                format!("duplicate node id: {}", n.id),
            ));
        }
    }

    let node_ids: HashSet<&str> = app.nodes.iter().map(|n| n.id.as_str()).collect();
    for c in &app.connections {
        if !node_ids.contains(c.from.as_str()) {
            out.push(ValidationIssue::error(
                "E_APP_DANGLING_FROM",
                format!("connection from unknown node: {}", c.from),
            ));
        }
        if !node_ids.contains(c.to.as_str()) {
            out.push(ValidationIssue::error(
                "E_APP_DANGLING_TO",
                format!("connection to unknown node: {}", c.to),
            ));
        }
    }

    // #208: a `{{ <node>.<field> }}` data reference is an implicit scheduling edge
    // (see `app_lock::derive_connections` + the orchestrator's `topo_order`). A circular
    // DATA dependency is therefore just as unschedulable as an explicit connection cycle,
    // so include the derived edges here — catching it at `validate` / `compile` instead
    // of only at run, matching how explicit cycles are rejected.
    let derived = crate::app_lock::derive_connections(app);
    let mut graph: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for c in app.connections.iter().chain(derived.iter()) {
        graph
            .entry(c.from.as_str())
            .or_default()
            .push(c.to.as_str());
    }
    let mut visited: HashSet<&str> = HashSet::new();
    let mut on_stack: HashSet<&str> = HashSet::new();
    for n in &app.nodes {
        if !visited.contains(n.id.as_str())
            && has_cycle(n.id.as_str(), &graph, &mut visited, &mut on_stack)
        {
            out.push(ValidationIssue::error(
                "E_APP_CYCLE",
                format!("cycle detected in connections starting from node: {}", n.id),
            ));
            break;
        }
    }

    check_inline_nodes(&app.nodes, &mut out);
    check_requires_syntax(&app.requires, &mut out);
    out
}

/// Reject `requires:` entries whose pin can't be parsed (#349).
///
/// This is a judgement about the app *file* — it needs no agent catalogue and
/// gives the same verdict on every machine — so it belongs in
/// [`validate_app`], alongside the other pure checks. Whether an installed
/// agent *satisfies* a well-formed pin is a fact about the environment and
/// lives in [`unsatisfied_pins`] instead.
/// The `requires:` syntax check on its own, for callers that do not run
/// [`validate_app`] — `aware app run` and nested exposed-app dispatch.
///
/// Needs no agent catalogue, because it judges the app *file*: an unreadable
/// pin is unreadable on every machine. That is exactly why it must run **before**
/// the `--simulate` exemption rather than inside it. Simulation is excused from
/// the catalogue checks because it stubs every node and contacts no binary —
/// a statement about the environment. Whether a constraint can be *read* is not
/// about the environment, so the same exemption must not swallow it; that is the
/// shape of #349's tenth finding, one exemption further out.
pub fn malformed_requires(app: &App) -> Vec<ValidationIssue> {
    let mut out = Vec::new();
    check_requires_syntax(&app.requires, &mut out);
    out
}

fn check_requires_syntax(requires: &[String], out: &mut Vec<ValidationIssue>) {
    for entry in requires {
        let (agent, spec) = crate::manifest::app::split_requires_entry(entry);
        // "Names no agent" is judged BEFORE the pin, and whether or not one
        // follows. Two spellings reach here — `"@1.2.3"`, which drops the id in
        // front of a pin, and `""` / `"   "`, which names nothing and pins
        // nothing — and reporting them through one rule is what stops a third
        // appearing: the earlier cut asked about the pin first, so an entry with
        // no `@` was waved through as "an unpinned agent id" without anyone
        // checking that it named an agent. Nothing downstream then said so
        // either: `unsatisfied_pins` has no version to judge, `missing_agents`
        // walks nodes rather than `requires:`, and `write_app_lockfile`'s
        // resolution is best-effort, so `agents//manifest.yaml` missed in
        // silence. [`validate_agent`] already spells "no id" as a trimmed
        // emptiness.
        if agent.is_empty() {
            out.push(ValidationIssue::error(
                "E_APP_REQUIRES_MALFORMED",
                match spec {
                    Some(_) => format!(
                        "requires entry {entry:?} pins a version but names no agent — \
                         an entry is <agent-id>@<pin>, and a pin with no id in front of \
                         the `@` constrains nothing"
                    ),
                    None => format!(
                        "requires entry {entry:?} names no agent — an entry is \
                         <agent-id> (an unpinned dependency) or <agent-id>@<pin>, and \
                         an entry with neither declares nothing"
                    ),
                },
            ));
            continue;
        }
        let Some(spec) = spec else {
            continue; // an unpinned agent id, which is legal
        };
        if VersionPin::parse(spec).is_none() {
            out.push(ValidationIssue::error(
                "E_APP_REQUIRES_MALFORMED",
                format!(
                    "requires entry {entry:?} has an unreadable version pin {spec:?} — \
                     expected {agent}@<major>.x (loose), {agent}@<major>.<minor>.x (default), \
                     or an exact {agent}@<major>.<minor>.<patch>, which may name a \
                     prerelease ({agent}@<major>.<minor>.<patch>-rc.1)"
                ),
            ));
        }
    }
}

/// A parsed `requires:` version pin.
///
/// The three forms are the ones `10-core/agent-spec.md § Versioning` publishes
/// — *"apps pin agent versions by `agent@minor.x` (default), `agent@major.x`
/// (loose), or `agent@exact-semver` (strict)"* — plus the two-component
/// `agent@<major>.<minor>` row that `10-core/app-spec.md § Versioning` lists as
/// "any compatible version newer than" it. No reference app under
/// `30-apps/_examples/` uses that last form, but the spec publishes it, so it
/// parses rather than being reported as a typo.
#[derive(Debug, PartialEq, Eq)]
enum VersionPin {
    /// `1.2.3` — this exact version. The trailing field is the prerelease
    /// suffix, so `1.2.3-rc.1` pins the release candidate and `1.2.3` pins the
    /// stable one; they are different releases and neither satisfies the other.
    Exact(u64, u64, u64, Option<String>),
    /// `1.2.x` — any patch within 1.2. The recommended default.
    Minor(u64, u64),
    /// `1.x` — any minor within major 1.
    Major(u64),
    /// `1.2` — same major, and at least minor 2.
    AtLeast(u64, u64),
}

impl VersionPin {
    /// Parse the part after the `@`. `None` means malformed.
    ///
    /// Decomposed in SemVer's own order — `<core>-<prerelease>+<build>` — for
    /// the same reason [`parse_semver`] is. The two grammars describe the same
    /// version space and have to agree: splitting the pin on `.` first meant
    /// `1.2.3-rc.1` arrived as four components and was rejected as malformed,
    /// while the *installed* parser read it happily. An app could therefore run
    /// a prerelease but never pin one, and the only way to name it was a range
    /// like `1.2.x` — which admits every other patch too, losing exactly the
    /// reproducibility the exact form exists for.
    fn parse(spec: &str) -> Option<Self> {
        // §10 keeps build metadata out of a release's identity, so `1.2.3+linux`
        // pins the same release as `1.2.3`. Split first (a build tag may contain
        // a hyphen), and validate it rather than ignoring it — a malformed pin
        // is still malformed.
        let (rest, build) = match spec.split_once('+') {
            Some((rest, build)) => (rest, Some(build)),
            None => (spec, None),
        };
        if build.is_some_and(|b| !is_dot_separated_identifiers(b, false)) {
            return None;
        }
        let (core, pre) = match rest.split_once('-') {
            Some((core, pre)) => (core, Some(pre)),
            None => (rest, None),
        };
        if pre.is_some_and(|p| !is_dot_separated_identifiers(p, true)) {
            return None;
        }
        let parts: Vec<&str> = core.split('.').collect();
        // A wildcard is only meaningful in the LAST position: `1.x.3` pins a
        // patch under an unknown minor, which denotes nothing.
        let wild = |s: &str| matches!(s, "x" | "X" | "*");
        if parts.iter().rev().skip(1).any(|p| wild(p)) {
            return None;
        }
        // `numeric_identifier`, not `parse::<u64>`, so a zero-padded component is
        // rejected rather than silently meaning something else (`01.2.x` is not
        // `1.2.x`) — the same SemVer rule the installed version is held to.
        let num = numeric_identifier;
        let ranged = match parts.as_slice() {
            [maj, min, pat] if wild(pat) => Self::Minor(num(maj)?, num(min)?),
            [maj, min, pat] => {
                return Some(Self::Exact(
                    num(maj)?,
                    num(min)?,
                    num(pat)?,
                    pre.map(str::to_owned),
                ));
            }
            [maj, min] if wild(min) => Self::Major(num(maj)?),
            [maj, min] => Self::AtLeast(num(maj)?, num(min)?),
            _ => return None,
        };
        // A prerelease names one release, so it is only meaningful on the form
        // that names one. `1.2.x-rc.1` describes no set of versions at all.
        if pre.is_some() {
            return None;
        }
        Some(ranged)
    }

    fn is_satisfied_by(&self, v: &InstalledVersion) -> bool {
        let (maj, min, pat) = v.triple;
        match *self {
            // An exact pin exists for reproducibility, so it admits only that
            // release. `1.2.0-rc.1` is a *different* release from `1.2.0` in
            // semver — it even orders before it — so a prerelease never
            // satisfies an exact pin, or `agent@1.2.0` could silently run a
            // release candidate the app never asked for. The range forms below
            // stay deliberately permissive: they are ranges, and an author who
            // wanted the strict reading had the exact form available.
            // …and an exact pin that names a prerelease admits that prerelease
            // and nothing else, which is the same rule read the other way.
            Self::Exact(a, b, c, ref pre) => {
                (maj, min, pat) == (a, b, c) && v.prerelease.as_deref() == pre.as_deref()
            }
            Self::Minor(a, b) => (maj, min) == (a, b),
            Self::Major(a) => maj == a,
            Self::AtLeast(a, b) => maj == a && min >= b,
        }
    }
}

/// An installed agent's `version:`, parsed.
struct InstalledVersion {
    triple: (u64, u64, u64),
    /// The semver prerelease suffix (`rc.1` of `1.2.0-rc.1`), if any. Kept as
    /// the identifier rather than a flag so an exact pin can name *which*
    /// prerelease it wants, not merely refuse all of them.
    /// Build metadata (`+deadbeef`) is **not** recorded: semver §10 excludes it
    /// from a release's identity and from precedence, so it cannot change which
    /// pins a version satisfies.
    prerelease: Option<String>,
}

/// Parse an installed agent's `version:` as **strict** SemVer 2.0.0.
///
/// Strictness is the point, not pedantry. The caller treats an unparseable
/// version as "cannot check this pin" and reports it; a *lenient* parse would
/// route malformed values into the satisfied path instead. `1.2.3+` (empty
/// build metadata — which `validate_agent` does not currently reject) would
/// read as the release `1.2.3` and satisfy an exact `@1.2.3` pin it is not,
/// which is the very substitution the exact form exists to prevent. Same for
/// `1.02.3`, `1.2.3-`, and `1.2.3-01`.
///
/// The triple is read from the version core, so a legitimately prereleased or
/// build-stamped version stays *checkable* rather than being reported as
/// unreadable. The prerelease flag is kept rather than discarded because it
/// changes the answer for an exact pin (see [`VersionPin::is_satisfied_by`]).
fn parse_semver(version: &str) -> Option<InstalledVersion> {
    // SemVer order is `<core>-<prerelease>+<build>`, so split build metadata
    // off first — otherwise `1.2.0+linux-gnu` would read as a prerelease.
    let (rest, build) = match version.split_once('+') {
        Some((rest, build)) => (rest, Some(build)),
        None => (version, None),
    };
    // §10: build identifiers are alphanumerics-and-hyphens, dot-separated, and
    // non-empty — but they carry no leading-zero rule, since they never take
    // part in precedence.
    if build.is_some_and(|b| !is_dot_separated_identifiers(b, false)) {
        return None;
    }
    let (core, pre) = match rest.split_once('-') {
        Some((core, pre)) => (core, Some(pre)),
        None => (rest, None),
    };
    // §9: prerelease identifiers add the rule that a purely numeric one must
    // not carry a leading zero.
    if pre.is_some_and(|p| !is_dot_separated_identifiers(p, true)) {
        return None;
    }
    let [maj, min, pat] = core.split('.').collect::<Vec<_>>()[..] else {
        return None;
    };
    Some(InstalledVersion {
        triple: (
            numeric_identifier(maj)?,
            numeric_identifier(min)?,
            numeric_identifier(pat)?,
        ),
        prerelease: pre.map(str::to_owned),
    })
}

/// A SemVer numeric identifier (§2, §9): digits only, and no leading zero
/// unless the identifier *is* `0`. Shared by the version core and the pin
/// grammar, so `01.2.x` can't silently mean `1.2.x` in either.
fn numeric_identifier(s: &str) -> Option<u64> {
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    if s.len() > 1 && s.starts_with('0') {
        return None;
    }
    s.parse().ok()
}

/// Dot-separated SemVer identifiers, per §9 (prerelease) and §10 (build).
/// `numeric_leading_zero_rule` selects the prerelease-only restriction that a
/// purely numeric identifier must not be zero-padded.
fn is_dot_separated_identifiers(s: &str, numeric_leading_zero_rule: bool) -> bool {
    !s.is_empty()
        && s.split('.').all(|id| {
            if id.is_empty() || !id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-') {
                return false;
            }
            let all_digits = id.bytes().all(|b| b.is_ascii_digit());
            !(numeric_leading_zero_rule && all_digits && id.len() > 1 && id.starts_with('0'))
        })
}

/// Report `requires:` pins that the installed agent catalogue does not satisfy (#349).
///
/// Before this, `requires:` was parsed, echoed by `app show`, and resolved into
/// the install-time `lockfile.yaml` — but nothing ever compared a declared pin
/// against the version actually installed, so an app pinned to
/// `ifc-reference-reader@0.1.x` compiled and ran clean against a `1.3.0` whose
/// contract it had never asked for. That is what makes a major bump plus a
/// `BREAKING.md` enforceable rather than advisory (`agent-spec.md § Versioning`).
///
/// Deliberately silent on an agent that is **not installed**: a pin verdict
/// needs a version to judge, and "the agent is missing" is a different finding
/// with its own gate ([`missing_agents`], #308). Reporting both would name the
/// same gap twice with two remedies.
///
/// **Fails closed on an unreadable pin.** It would be tempting to stay quiet
/// here because [`validate_app`] already reports one — but the two are not
/// always called together. `aware app run` and nested exposed-app dispatch call
/// *this* function alone; neither validates the loaded app first. Skipping an
/// unparseable constraint there would let an app installed by an older CLI, or
/// edited in place under `~/.aware/apps/`, run against any version at all —
/// silently reopening the hole this whole check exists to close, for exactly
/// the apps whose `requires:` block is already suspect. A check that cannot
/// read its constraint must never answer "satisfied".
///
/// Saying it twice is avoided by *ordering* rather than by silence: `compile`
/// and `install` both run [`validate_app`] first and return on its errors, so
/// they never reach this branch.
///
/// **Only pins agents that can actually dispatch.** A `requires:` entry whose
/// agent is reachable *only* through frozen nodes — or through no node at all —
/// is not version-checked. `app-spec.md § Frozen nodes` is explicit that a
/// frozen node "needn't have its agent installed", and gating the *version* of
/// an agent that need not exist is stricter than the contract in a way that
/// contradicts itself: an absent agent would be fine while a merely-mismatched
/// one refused a static app that had been running. The neighbouring
/// availability checks skip frozen subtrees for the same reason, so this
/// mirrors their traversal rather than inventing a second rule.
///
/// The unreadable-pin branch is deliberately *not* scoped that way: "I cannot
/// read your constraint" is a broken file, not an irrelevant constraint, and
/// nothing about it can be judged — including whether it names a dispatchable
/// agent correctly. An entry with **no id at all** (`@1.2.3`) belongs to that
/// branch rather than to the exemption: an empty id is in no `dispatchable` set,
/// so treating it as merely unreachable would silently accept an entry that
/// looks like a constraint and enforces nothing.
///
/// `severity` picks the gate, mirroring [`missing_agents`]:
/// [`Severity::Error`] (`E_APP_AGENT_PIN_UNSATISFIED`) for `compile` and `run`,
/// [`Severity::Warning`] (`W_APP_AGENT_PIN_UNSATISFIED`) for `install` — where
/// installing the app before its agents is legitimate (#170) and the right
/// agent may still be on its way.
pub fn unsatisfied_pins(
    app: &App,
    agents: &[crate::manifest::loader::DiscoveredAgent],
    severity: Severity,
) -> Vec<ValidationIssue> {
    let mut out = Vec::new();
    let mut dispatchable = HashSet::new();
    collect_dispatchable_agents(&app.nodes, &mut dispatchable);
    for entry in &app.requires {
        let (agent_id, Some(spec)) = crate::manifest::app::split_requires_entry(entry) else {
            continue; // unpinned — nothing to satisfy
        };
        if agent_id.is_empty() {
            // `@1.2.3` names no agent, so the empty id is in no catalogue and in
            // no `dispatchable` set. Falling through would hit the dispatchability
            // exemption below and skip the entry — which reads as "this pin is
            // irrelevant" when the truth is that it is unreadable. Same
            // fail-closed rule as an unparseable pin, and for the same reason:
            // `run` and nested dispatch reach here without `validate_app`, so an
            // app edited in place under `~/.aware/apps/` would otherwise run with
            // a constraint nothing ever checked.
            let msg = format!(
                "app requires {entry:?}, which pins a version but names no agent, so there \
                 is nothing to check it against — an entry is <agent-id>@<pin>"
            );
            out.push(match severity {
                Severity::Error => ValidationIssue::error("E_APP_REQUIRES_MALFORMED", msg),
                Severity::Warning => ValidationIssue::warning("W_APP_REQUIRES_MALFORMED", msg),
            });
            continue;
        }
        let Some(pin) = VersionPin::parse(spec) else {
            // Unreadable constraint — refuse rather than skip. See the fail-closed
            // note above: `run` reaches here without having called `validate_app`.
            let msg = format!(
                "app requires {entry:?}, whose version pin {spec:?} is unreadable, so no \
                 installed version can be checked against it — fix the pin to \
                 {agent_id}@<major>.x, {agent_id}@<major>.<minor>.x, or an exact \
                 {agent_id}@<major>.<minor>.<patch>, which may name a prerelease \
                 ({agent_id}@<major>.<minor>.<patch>-rc.1)"
            );
            out.push(match severity {
                Severity::Error => ValidationIssue::error("E_APP_REQUIRES_MALFORMED", msg),
                Severity::Warning => ValidationIssue::warning("W_APP_REQUIRES_MALFORMED", msg),
            });
            continue;
        };
        if !dispatchable.contains(agent_id) {
            // Declared but unreachable — frozen-only, or named in `requires:` with
            // no node behind it. Nothing will invoke it, so its version cannot
            // affect this app's behaviour.
            continue;
        }
        let Some(installed) = agents.iter().find(|d| d.manifest.agent == agent_id) else {
            continue; // not installed — `missing_agents` owns that finding
        };
        let version = installed.manifest.version.as_str();
        // This message deliberately names NO command, and that is a fix rather
        // than an omission. Five review findings landed on the command this used
        // to print, each a different spelling of the same mistake: the validator
        // does not know enough to prescribe one.
        //
        //   - a range pin is not a registry key (`Index::resolve` looks versions
        //     up literally, so `foo@0.1.x` misses even when `0.1.0` is there);
        //   - build metadata is part of a pin but not of a registry key;
        //   - `install` refuses while a copy is on disk, and this finding only
        //     fires when the agent IS installed;
        //   - `update` takes no version, so it cannot reach a pin the newest
        //     release fails;
        //   - and `uninstall` first — the last thing printed here — is actively
        //     DESTRUCTIVE for an agent installed from a local folder: `agent
        //     install id@version` is parsed as a registry spec (`commands/agent.rs`),
        //     so removing a local install and reinstalling it by version cannot
        //     work, and leaves the machine with no agent at all.
        //
        // Nothing at this point distinguishes a registry agent from a local one,
        // and no command lists the versions the registry actually holds, so any
        // sequence printed here is wrong for some real case. A wrong command is
        // worse than none — and a destructive one is worse than a dead end. State
        // the goal, which is true in every case, and leave the route to the
        // operator. #363 tracks the missing surface (version-selecting update,
        // version listing, local-install provenance); when it lands, a remedy can
        // come back with something that always works.
        let msg = match parse_semver(version) {
            Some(v) if pin.is_satisfied_by(&v) => continue,
            Some(_) => format!(
                "app requires {agent_id}@{spec}, but {version} is installed — install a version \
                 matching {spec}, or update the app's `requires:` pin if the new contract is the \
                 intended one"
            ),
            None => format!(
                "app requires {agent_id}@{spec}, but the installed version {version:?} is not \
                 semver, so the pin cannot be checked — fix the agent's `version:` field"
            ),
        };
        out.push(match severity {
            Severity::Error => ValidationIssue::error("E_APP_AGENT_PIN_UNSATISFIED", msg),
            Severity::Warning => ValidationIssue::warning("W_APP_AGENT_PIN_UNSATISFIED", msg),
        });
    }
    out
}

/// Recursively validate inline-glue nodes, descending into `for-each` `do:` bodies
/// (which the compiler flattens and the runtime executes), so unsupported inline
/// kinds are caught wherever they appear, not just at the top level (#160).
fn check_inline_nodes(nodes: &[crate::manifest::app::Node], out: &mut Vec<ValidationIssue>) {
    for n in nodes {
        if let Some(inline) = &n.inline {
            if inline.description.trim().is_empty() {
                out.push(ValidationIssue::error(
                    "E_APP_INLINE_NO_DESC",
                    format!("inline node {:?} missing description", n.id),
                ));
            }
            // The runtime executes only `predicate` inline glue today (see
            // orchestrator). Reject other kinds so an unrunnable app fails at
            // validate/compile — not after the author validated + locked it.
            if inline.kind != "predicate" {
                out.push(ValidationIssue::error(
                    "E_APP_INLINE_KIND",
                    format!(
                        "inline node {:?}: kind {:?} is not runnable yet (only 'predicate' is supported)",
                        n.id, inline.kind
                    ),
                ));
            }
        }
        if let Some(body) = &n.do_ {
            check_inline_nodes(body, out);
        }
    }
}

/// Validate the safety contract for write-mode nodes against the installed
/// agent catalogue.
///
/// Per `10-core/app-spec.md § Safety contract (write-mode nodes)`:
/// any node whose command resolves to `Mode::Write` MUST declare a `safety:`
/// block. Apps missing `safety:` on a write-mode node are rejected by
/// `aware app validate` and refused by `aware app run`.
///
/// Agents referenced by the app but not installed are skipped (not an
/// error here — caught by lockfile resolution earlier).
pub fn validate_app_safety(
    app: &App,
    agents: &[crate::manifest::loader::DiscoveredAgent],
) -> Vec<ValidationIssue> {
    use crate::manifest::agent::Mode;
    let mut out = Vec::new();
    for node in &app.nodes {
        // A frozen node never invokes its agent (the orchestrator emits its pinned output and
        // skips dispatch), so it never writes — the write-mode safety contract does not apply.
        if node.frozen.is_some() {
            continue;
        }
        let (Some(agent_id), Some(cmd_name)) = (node.agent.as_ref(), node.command.as_ref()) else {
            continue;
        };
        let Some(agent) = agents.iter().find(|d| d.manifest.agent == *agent_id) else {
            continue;
        };
        let Some(cmd) = agent.manifest.commands.get(cmd_name.as_str()) else {
            continue;
        };

        // Note: a node-level `mode:` that *conflicts* with a non-overridable
        // command is rejected by `validate_app_agents` (#165) — the agent-aware
        // validator that runs on every lock-producing + run path. Here we only
        // resolve the effective mode for the write-mode safety gate; on a
        // mode-overridable command an explicit `mode: read` legitimately makes
        // the node read-mode (so no `safety:` block is required).
        let effective = agent.manifest.effective_mode(cmd_name, cmd, node.mode);
        if effective.mode == Mode::Write && node.safety.is_none() {
            out.push(ValidationIssue::error(
                "E_APP_WRITE_WITHOUT_SAFETY",
                format!(
                    "node {:?} calls write-mode command {}.{} without a `safety:` block (required per app-spec § Safety contract)",
                    node.id, agent_id, cmd_name
                ),
            ));
        }
    }
    out
}

/// Reject nodes that reference an agent the runtime can't dispatch to — one
/// declared `status: planned` (no shipped/installable transport binary). Fails at
/// validate/compile rather than at run with "program not found" (#161). Recurses
/// into `for-each` `do:` bodies. Agents not in the catalogue are skipped here
/// (lockfile resolution / run handle the missing-agent case).
pub fn validate_app_agents(
    app: &App,
    agents: &[crate::manifest::loader::DiscoveredAgent],
) -> Vec<ValidationIssue> {
    let mut out = Vec::new();
    check_node_agents(&app.nodes, agents, app.exposes_as_agent, &app.app, &mut out);
    out
}

/// Report nodes that dispatch to an agent which is **not installed** (#308).
///
/// Split out from [`validate_app_agents`] because the two have deliberately
/// different reach: a node's agent must exist *by run time*, but need not be
/// installed when the app is installed or compiled — an app may legitimately be
/// installed before its agents are (`app_lock` compiles such a node with an
/// author-declared mode and a warning, #170). So install/compile surface these
/// as warnings while `app run` treats them as errors.
///
/// Traversal mirrors [`check_node_agents`]: frozen nodes are skipped (they emit
/// a pinned output and never dispatch, so their agent needn't exist at all) and
/// `for-each` `do:` bodies are recursed into.
/// `severity` picks the gate: [`Severity::Error`] (`E_APP_AGENT_NOT_INSTALLED`)
/// for run, [`Severity::Warning`] (`W_APP_AGENT_NOT_INSTALLED`) for
/// install/compile. The message — node id, agent id, and both remedies — is the
/// same either way.
pub fn missing_agents(
    app: &App,
    agents: &[crate::manifest::loader::DiscoveredAgent],
    severity: Severity,
) -> Vec<ValidationIssue> {
    let mut out = Vec::new();
    collect_missing_agents(&app.nodes, agents, &severity, &mut out);
    out
}

/// The agent ids this app can actually dispatch to, by the same traversal
/// [`unsatisfied_pins`] scopes itself with — frozen subtrees excluded, `do:`
/// bodies descended into.
///
/// Public so a caller that must reason about *which* agents an app reaches can
/// share this one traversal rather than writing a second one that drifts: the
/// run pre-flight uses it to find the app-backed agents whose backing apps
/// carry `requires:` blocks of their own.
pub fn dispatchable_agents(app: &App) -> HashSet<&str> {
    let mut out = HashSet::new();
    collect_dispatchable_agents(&app.nodes, &mut out);
    out
}

/// Collect the agent ids this app can actually dispatch to.
///
/// Traversal mirrors [`collect_missing_agents`] and `check_node_agents`: a
/// frozen node emits its pinned output and never invokes its agent, and the
/// orchestrator short-circuits its `do:` body along with it, so the whole
/// subtree is skipped. `do:` bodies of live nodes are descended into, since a
/// `for-each` body dispatches once per item.
fn collect_dispatchable_agents<'a>(
    nodes: &'a [crate::manifest::app::Node],
    out: &mut HashSet<&'a str>,
) {
    for n in nodes {
        if n.frozen.is_some() {
            continue;
        }
        if let Some(agent_id) = &n.agent {
            out.insert(agent_id.as_str());
        }
        if let Some(body) = &n.do_ {
            collect_dispatchable_agents(body, out);
        }
    }
}

fn collect_missing_agents(
    nodes: &[crate::manifest::app::Node],
    agents: &[crate::manifest::loader::DiscoveredAgent],
    severity: &Severity,
    out: &mut Vec<ValidationIssue>,
) {
    for n in nodes {
        // A frozen node emits its pinned value and never invokes its agent, so a
        // missing agent is harmless there — and neither does its `do:` body, which
        // the orchestrator short-circuits along with it, so skip the whole subtree
        // (app-spec § Frozen nodes; same rule as `check_node_agents`).
        if n.frozen.is_some() {
            continue;
        }
        if let Some(agent_id) = &n.agent
            && !agents.iter().any(|d| d.manifest.agent == *agent_id)
        {
            let msg = format!(
                "node {:?} references agent {:?}, which is not installed — install it with \
                 `aware agent install {agent_id}`, or check whether it exists at all with \
                 `aware agent describe {agent_id} --available`",
                n.id, agent_id
            );
            out.push(match severity {
                Severity::Error => ValidationIssue::error("E_APP_AGENT_NOT_INSTALLED", msg),
                Severity::Warning => ValidationIssue::warning("W_APP_AGENT_NOT_INSTALLED", msg),
            });
        }
        if let Some(body) = &n.do_ {
            collect_missing_agents(body, agents, severity, out);
        }
    }
}

fn check_node_agents(
    nodes: &[crate::manifest::app::Node],
    agents: &[crate::manifest::loader::DiscoveredAgent],
    app_exposes: bool,
    app_id: &str,
    out: &mut Vec<ValidationIssue>,
) {
    use crate::manifest::agent::AgentStatus;
    for n in nodes {
        // A frozen node never dispatches to its agent (it emits a pinned output and short-circuits),
        // so agent/command availability does not gate it — and it needn't have its agent installed
        // at all. Skip it (and its `do:` body, which likewise never runs while frozen).
        if n.frozen.is_some() {
            continue;
        }

        // v0 no-recursion rule (self-reference): an `exposes-as-agent` app that
        // composes its own id recurses into itself. Catch it regardless of
        // catalogue state — on first install the synthesized agent does not
        // exist yet, so the discovered-agent check below would miss it.
        if app_exposes
            && let Some(agent_id) = &n.agent
            && agent_id == app_id
        {
            out.push(ValidationIssue::error(
                "E_APP_EXPOSED_COMPOSES_EXPOSED",
                format!(
                    "node {:?} composes agent {:?}, which is this app itself; an exposes-as-agent \
                     app cannot compose itself (v0 constraint, app-spec § exposes-as-agent \
                     constraints)",
                    n.id, agent_id
                ),
            ));
        }

        if let Some(agent_id) = &n.agent
            && let Some(d) = agents.iter().find(|d| d.manifest.agent == *agent_id)
        {
            if d.manifest.status == AgentStatus::Planned {
                out.push(ValidationIssue::error(
                    "E_APP_AGENT_UNAVAILABLE",
                    format!(
                        "node {:?} references agent {:?}, which is declared but not yet runnable \
                         (no shipped/installable transport binary)",
                        n.id, agent_id
                    ),
                ));
            } else if let Some(cmd_name) = &n.command
                && d.manifest
                    .commands
                    .get(cmd_name)
                    .is_some_and(|c| c.status == AgentStatus::Planned)
            {
                // Per-command availability (#199): the agent is runnable but this
                // specific command is declared `status: planned` (not yet wired —
                // e.g. a multi-step/binary REST op). Fail fast at validate/compile
                // rather than at run.
                out.push(ValidationIssue::error(
                    "E_APP_COMMAND_UNAVAILABLE",
                    format!(
                        "node {:?} references command {:?} of agent {:?}, which is declared but \
                         not yet runnable (planned)",
                        n.id, cmd_name, agent_id
                    ),
                ));
            }

            // v0 no-recursion rule: an `exposes-as-agent` app cannot itself
            // compose another exposes-as-agent app (an app-backed synthesized
            // agent). Caught here so it fails at install/validate/compile rather
            // than only at dispatch (app-spec § exposes-as-agent constraints).
            // (The self-reference case above also covers first-install, before
            // this app's own synthesized agent exists in the catalogue.)
            if app_exposes && agent_id != app_id && d.manifest.transport.app.is_some() {
                out.push(ValidationIssue::error(
                    "E_APP_EXPOSED_COMPOSES_EXPOSED",
                    format!(
                        "node {:?} composes agent {:?}, which is itself an exposes-as-agent app; \
                         an exposed app cannot compose another exposed app (v0 constraint, \
                         app-spec § exposes-as-agent constraints)",
                        n.id, agent_id
                    ),
                ));
            }

            // #165: a node-level `mode:` is only meaningful on a
            // `mode-overridable` command (caller-determined behavior, e.g.
            // `exec`). On any other command the manifest mode is authoritative,
            // so a *conflicting* node-level declaration would be silently
            // dropped. Reject it here — in the agent-aware validator that runs
            // on every lock-producing + run path (install, compile, inspect,
            // validate, run, dry-run), not just safety pre-flight — so the
            // authoring field is never quietly ignored on any surface.
            if let Some(cmd_name) = &n.command
                && let Some(cmd) = d.manifest.commands.get(cmd_name.as_str())
                && !cmd.mode_overridable
                && let Some(declared) = n.mode
                && declared != d.manifest.mode_of(cmd_name, cmd)
            {
                out.push(ValidationIssue::error(
                    "E_APP_NODE_MODE_NOT_OVERRIDABLE",
                    format!(
                        "node {:?} declares `mode: {}` but command {}.{} is not `mode-overridable`; its manifest mode ({}) is authoritative. Remove the node-level `mode:` (or have the agent declare the command `mode-overridable` if its mode is caller-determined).",
                        n.id,
                        declared.as_str(),
                        agent_id,
                        cmd_name,
                        d.manifest.mode_of(cmd_name, cmd).as_str(),
                    ),
                ));
            }

            // RFC #223 — the fenced runtime model-extraction carve-out. A command that
            // calls a model at run time (`model-extraction: true`) is admitted into the
            // deterministic run path ONLY when it is `category: curated` AND its agent
            // declares `capabilities.runtime-model-extraction: true`. Every other path to
            // the flag — a reflected/auto-generated command, or a curated command on an
            // agent that lacks the capability — is rejected here, so no node can mint a
            // model-reader except the single curated `vision.extract`. Keyed to the flag +
            // curated + capability (structural), never to the literal agent/command name,
            // so the exception to decalog #9 stays narrow and cannot widen by accident.
            if let Some(cmd_name) = &n.command
                && let Some(cmd) = d.manifest.commands.get(cmd_name.as_str())
                && cmd.model_extraction
            {
                let curated =
                    d.manifest.category_of(cmd) == crate::manifest::agent::Category::Curated;
                let capable = d
                    .manifest
                    .capabilities
                    .as_ref()
                    .is_some_and(|c| c.runtime_model_extraction);
                if !(curated && capable) {
                    out.push(ValidationIssue::error(
                        "E_APP_RUNTIME_MODEL_FORBIDDEN",
                        format!(
                            "node {:?} resolves to {}.{}, which declares `model-extraction: true` (it \
                             calls a model at run time) but is not an admitted carve-out: a runtime model \
                             extraction is permitted ONLY on a `category: curated` command whose agent \
                             declares `capabilities.runtime-model-extraction: true` ({}). The run path is \
                             deterministic — see RFC #223 + app-spec § Runtime model extraction (the one \
                             carve-out).",
                            n.id,
                            agent_id,
                            cmd_name,
                            if !curated {
                                "this command is not category: curated"
                            } else {
                                "this agent does not declare the runtime-model-extraction capability"
                            },
                        ),
                    ));
                }
            }
        }
        if let Some(body) = &n.do_ {
            check_node_agents(body, agents, app_exposes, app_id, out);
        }
    }
}

fn has_cycle<'a>(
    node: &'a str,
    graph: &BTreeMap<&'a str, Vec<&'a str>>,
    visited: &mut HashSet<&'a str>,
    on_stack: &mut HashSet<&'a str>,
) -> bool {
    visited.insert(node);
    on_stack.insert(node);
    if let Some(neighbors) = graph.get(node) {
        for &next in neighbors {
            if !visited.contains(next) {
                if has_cycle(next, graph, visited, on_stack) {
                    return true;
                }
            } else if on_stack.contains(next) {
                return true;
            }
        }
    }
    on_stack.remove(node);
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::Agent;

    fn load_agent(rel: &str) -> Agent {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join(rel);
        serde_yaml::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }
    fn load_app(rel: &str) -> App {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join(rel);
        serde_yaml::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }

    #[test]
    fn real_tekla_passes_validation() {
        let a = load_agent("20-agents/aeco/engineering/tekla/manifest.yaml");
        let issues = validate_agent(&a);
        assert!(!has_errors(&issues), "issues: {issues:?}");
    }

    #[test]
    fn implicit_ref_cycle_is_rejected() {
        // #208: `a` reads `{{ b.x }}` and `b` reads `{{ a.x }}` with NO explicit
        // connections. The derived edges form a cycle, which must be rejected at
        // validate (like an explicit cycle) — not silently pass and fail at run.
        let app: App = serde_yaml::from_str(
            r#"app: cyc
version: 0.0.1
description: x
nodes:
  - id: a
    agent: x
    command: emit
    config:
      v: '{{ b.x }}'
  - id: b
    agent: x
    command: emit
    config:
      v: '{{ a.x }}'
requires: []
"#,
        )
        .unwrap();
        let issues = validate_app(&app);
        assert!(
            issues.iter().any(|i| i.code == "E_APP_CYCLE"),
            "implicit ref cycle must be flagged E_APP_CYCLE; issues: {issues:?}"
        );
    }

    #[test]
    fn real_welded_to_tc_passes_validation() {
        let a = load_app("30-apps/_examples/welded-to-tc.app");
        let issues = validate_app(&a);
        assert!(!has_errors(&issues), "issues: {issues:?}");
    }

    // ── `requires:` version pins (#349) ──────────────────────────────────────

    /// An installed agent of `id` at `version`, with the minimum manifest shape.
    fn installed(id: &str, version: &str) -> crate::manifest::loader::DiscoveredAgent {
        let yaml = format!(
            "agent: {id}\nversion: {version}\ndescription: x\nstateful: false\nlicense: MIT\n\
             transport:\n  cli:\n    binary: aware-{id}\ncommands:\n  probe:\n    \
             lifecycle: single\n    description: x\n"
        );
        crate::manifest::loader::DiscoveredAgent {
            manifest: serde_yaml::from_str(&yaml).unwrap(),
            root: std::path::PathBuf::from("."),
        }
    }

    /// An app whose only `requires:` entry is `entry`.
    fn app_requiring(entry: &str) -> App {
        serde_yaml::from_str(&format!(
            "app: pin-test\nversion: 0.1.0\ndescription: x\nrequires:\n  - {entry}\n\
             nodes:\n  - id: probe\n    agent: ifc-reference-reader\n    command: probe\n"
        ))
        .unwrap()
    }

    fn pin_codes(entry: &str, installed_version: &str) -> Vec<&'static str> {
        let agents = vec![installed("ifc-reference-reader", installed_version)];
        unsatisfied_pins(&app_requiring(entry), &agents, Severity::Error)
            .iter()
            .map(|i| i.code)
            .collect()
    }

    #[test]
    fn an_impossible_pin_is_refused_against_the_installed_version() {
        // The issue's own reproduction: `@9.9.x` compiled and ran clean against
        // an installed 1.x, because nothing compared the two.
        assert_eq!(
            pin_codes("ifc-reference-reader@9.9.x", "1.3.0"),
            ["E_APP_AGENT_PIN_UNSATISFIED"]
        );
        // …and the realistic case that motivated it: an app pinned to the OLD
        // contract, running against the agent that broke it (#343).
        assert_eq!(
            pin_codes("ifc-reference-reader@0.1.x", "1.3.0"),
            ["E_APP_AGENT_PIN_UNSATISFIED"]
        );
    }

    #[test]
    fn the_message_names_the_pin_and_the_installed_version() {
        // The whole point is that the operator can tell WHICH side is wrong.
        let agents = vec![installed("ifc-reference-reader", "1.3.0")];
        let issues = unsatisfied_pins(
            &app_requiring("ifc-reference-reader@0.1.x"),
            &agents,
            Severity::Error,
        );
        let msg = &issues[0].message;
        assert!(msg.contains("ifc-reference-reader@0.1.x"), "{msg}");
        assert!(msg.contains("1.3.0"), "{msg}");
    }

    #[test]
    fn each_pin_form_admits_exactly_the_versions_the_spec_says_it_does() {
        // `agent-spec.md § Versioning` — minor.x (default), major.x (loose),
        // exact-semver (strict); `app-spec.md § Versioning` adds `<major>.<minor>`.
        // Each row lists versions that MUST pass and versions that MUST fail, so a
        // pin that degenerated to "accept anything" fails here.
        let cases: &[(&str, &[&str], &[&str])] = &[
            // exact
            ("1.2.3", &["1.2.3"], &["1.2.4", "1.3.3", "2.2.3", "1.2.2"]),
            // minor — any patch within 1.2
            ("1.2.x", &["1.2.0", "1.2.9"], &["1.3.0", "2.2.0", "1.1.9"]),
            // major — any minor within 1
            ("1.x", &["1.0.0", "1.9.9"], &["0.9.9", "2.0.0"]),
            // two-component — same major, at least minor 2
            ("1.2", &["1.2.0", "1.9.0"], &["1.1.9", "2.2.0", "0.2.0"]),
            // real pins from `30-apps/_examples/`
            ("2026.x", &["2026.0.1"], &["2025.9.9"]),
            ("0.1.x", &["0.1.0"], &["1.3.0", "0.2.0"]),
        ];
        for (spec, pass, fail) in cases {
            for v in *pass {
                assert!(
                    pin_codes(&format!("ifc-reference-reader@{spec}"), v).is_empty(),
                    "{spec} must admit {v}"
                );
            }
            for v in *fail {
                assert_eq!(
                    pin_codes(&format!("ifc-reference-reader@{spec}"), v),
                    ["E_APP_AGENT_PIN_UNSATISFIED"],
                    "{spec} must refuse {v}"
                );
            }
        }
    }

    #[test]
    fn a_frozen_only_agent_is_not_version_gated() {
        // `app-spec.md § Frozen nodes`: a frozen node emits its pinned output, never
        // invokes its agent, and "needn't have its agent installed". Gating the
        // VERSION of an agent that need not exist is stricter than that contract —
        // and self-contradicting: an absent agent would pass while a merely
        // mismatched one refused a static app that had been running fine.
        let agents = vec![installed("ifc-reference-reader", "1.3.0")];
        let frozen_app: App = serde_yaml::from_str(
            "app: pin-test\nversion: 0.1.0\ndescription: x\nrequires:\n  \
             - ifc-reference-reader@9.9.x\nnodes:\n  - id: probe\n    \
             agent: ifc-reference-reader\n    command: probe\n    frozen:\n      ok: true\n",
        )
        .unwrap();
        assert!(
            unsatisfied_pins(&frozen_app, &agents, Severity::Error).is_empty(),
            "a frozen-only agent must not be version-gated"
        );
        // Same for an agent named in `requires:` that no node reaches at all.
        let orphan_pin: App = serde_yaml::from_str(
            "app: pin-test\nversion: 0.1.0\ndescription: x\nrequires:\n  \
             - ifc-reference-reader@9.9.x\nnodes:\n  - id: gate\n    inline:\n      \
             kind: predicate\n      description: always pass\n      code: 'true'\n",
        )
        .unwrap();
        assert!(unsatisfied_pins(&orphan_pin, &agents, Severity::Error).is_empty());

        // The exemption must not leak past the frozen subtree. A LIVE node using the
        // same agent still gates, even when a frozen one also uses it.
        let mixed: App = serde_yaml::from_str(
            "app: pin-test\nversion: 0.1.0\ndescription: x\nrequires:\n  \
             - ifc-reference-reader@9.9.x\nnodes:\n  - id: pinned\n    \
             agent: ifc-reference-reader\n    command: probe\n    frozen:\n      ok: true\n  \
             - id: live\n    agent: ifc-reference-reader\n    command: probe\n",
        )
        .unwrap();
        assert_eq!(
            unsatisfied_pins(&mixed, &agents, Severity::Error)
                .iter()
                .map(|i| i.code)
                .collect::<Vec<_>>(),
            ["E_APP_AGENT_PIN_UNSATISFIED"]
        );
    }

    #[test]
    fn dispatchability_is_judged_through_do_bodies_and_frozen_ancestors() {
        // A `for-each` body dispatches once per item, so an agent used only there is
        // live — while the body of a FROZEN node is short-circuited with it, so an
        // agent used only there is not. Getting either backwards silently changes
        // which apps the gate applies to.
        let agents = vec![installed("ifc-reference-reader", "1.3.0")];
        let live_body: App = serde_yaml::from_str(
            "app: pin-test\nversion: 0.1.0\ndescription: x\nrequires:\n  \
             - ifc-reference-reader@9.9.x\nnodes:\n  - id: loop\n    for-each: '{{ xs.rows }}'\n    \
             do:\n      - id: probe\n        agent: ifc-reference-reader\n        command: probe\n",
        )
        .unwrap();
        assert_eq!(
            unsatisfied_pins(&live_body, &agents, Severity::Error)
                .iter()
                .map(|i| i.code)
                .collect::<Vec<_>>(),
            ["E_APP_AGENT_PIN_UNSATISFIED"],
            "an agent used inside a live for-each body must still be gated"
        );

        let frozen_body: App = serde_yaml::from_str(
            "app: pin-test\nversion: 0.1.0\ndescription: x\nrequires:\n  \
             - ifc-reference-reader@9.9.x\nnodes:\n  - id: loop\n    for-each: '{{ xs.rows }}'\n    \
             frozen:\n      ok: true\n    do:\n      - id: probe\n        \
             agent: ifc-reference-reader\n        command: probe\n",
        )
        .unwrap();
        assert!(
            unsatisfied_pins(&frozen_body, &agents, Severity::Error).is_empty(),
            "a frozen node short-circuits its `do:` body, so its agents can't dispatch"
        );
    }

    #[test]
    fn an_unreadable_pin_is_reported_even_for_an_unreachable_agent() {
        // The dispatchability exemption is about an *irrelevant* constraint. An
        // unreadable one is a broken file: nothing about it can be judged, including
        // whether the id in front of the `@` is an agent this app even uses.
        let orphan: App = serde_yaml::from_str(
            "app: pin-test\nversion: 0.1.0\ndescription: x\nrequires:\n  \
             - ifc-reference-reader@not-a-version\nnodes:\n  - id: gate\n    inline:\n      \
             kind: predicate\n      description: always pass\n      code: 'true'\n",
        )
        .unwrap();
        assert_eq!(
            unsatisfied_pins(&orphan, &[], Severity::Error)
                .iter()
                .map(|i| i.code)
                .collect::<Vec<_>>(),
            ["E_APP_REQUIRES_MALFORMED"]
        );
    }

    #[test]
    fn an_entry_that_pins_a_version_but_names_no_agent_is_refused() {
        // `@1.2.3` — the id was dropped. The pin parses, so the syntax check was
        // silent; the empty id is in no `dispatchable` set, so the catalogue check
        // took the "declared but unreachable" exemption and skipped it. Between the
        // two, an entry that reads as a constraint enforced nothing, on both the
        // file path and the runtime one.
        // The whitespace spellings belong here rather than in a test of their own:
        // `" @1.2.3"` names an agent no more than `"@1.2.3"` does and dies at the
        // same place, so a check that caught only the zero-length one would just
        // move the hole one space to the right.
        for bad in [
            "@1.2.3",
            "@0.1.x",
            "@1.x",
            " @1.2.3",
            "\t@1.2.3",
            "   @1.2.3",
        ] {
            assert!(
                validate_app(&app_requiring(&format!("\"{bad}\"")))
                    .iter()
                    .any(|i| i.code == "E_APP_REQUIRES_MALFORMED"),
                "{bad} must be rejected by the syntax check"
            );
            assert_eq!(
                pin_codes(&format!("\"{bad}\""), "1.3.0"),
                ["E_APP_REQUIRES_MALFORMED"],
                "{bad} must fail closed in the catalogue check too"
            );
        }
        // Fails closed for the same reason as an unparseable pin, so it answers the
        // same way when the agent is absent, and honours the same gate selector.
        assert_eq!(
            unsatisfied_pins(&app_requiring("\"@1.2.3\""), &[], Severity::Error)
                .iter()
                .map(|i| i.code)
                .collect::<Vec<_>>(),
            ["E_APP_REQUIRES_MALFORMED"]
        );
        let agents = vec![installed("ifc-reference-reader", "1.3.0")];
        assert_eq!(
            unsatisfied_pins(&app_requiring("\"@1.2.3\""), &agents, Severity::Warning)[0].code,
            "W_APP_REQUIRES_MALFORMED"
        );
        // Reported as a missing *id*, not dressed up as a version mismatch or as an
        // unreadable pin — the pin is fine, and the operator must be sent to the
        // part that is actually wrong.
        let msg =
            &unsatisfied_pins(&app_requiring("\"@1.2.3\""), &agents, Severity::Error)[0].message;
        assert!(msg.contains("names no agent"), "{msg}");
        // A real id in front of the `@` is untouched: this must not become a
        // check that refuses every pin.
        assert!(pin_codes("ifc-reference-reader@1.3.x", "1.3.0").is_empty());
    }

    #[test]
    fn an_entry_that_names_no_agent_and_pins_nothing_is_refused_too() {
        // The last spelling of "names no agent", and the one the check above
        // walked past: it asked about the *pin* first, so an entry with no `@`
        // was accepted as "an unpinned agent id" without anyone asking whether
        // it named an agent. `requires: ["  "]` therefore validated clean while
        // `write_app_lockfile` looked for `agents//manifest.yaml`, missed, and —
        // being best-effort by design — said nothing.
        for bad in ["", " ", "   ", "\t", "\n"] {
            let issues = validate_app(&app_requiring(&format!("\"{bad}\"")));
            assert!(
                issues.iter().any(|i| i.code == "E_APP_REQUIRES_MALFORMED"),
                "{bad:?} must be rejected by the syntax check: {issues:?}"
            );
            // Sent to the part that is wrong — the missing id, not a pin. There
            // is no pin here, so a "fix the pin" remedy would be a dead end.
            let msg = &issues
                .iter()
                .find(|i| i.code == "E_APP_REQUIRES_MALFORMED")
                .unwrap()
                .message;
            assert!(msg.contains("names no agent"), "{msg}");
            assert!(!msg.contains("unreadable version pin"), "{msg}");
        }
        // Which gate this reaches, stated exactly, because the two halves differ.
        //
        // The *catalogue* gate stays silent: there is no pin, so there is no
        // version to judge, and inventing a verdict would be the fail-open this
        // PR spent twenty findings closing, run backwards.
        assert!(unsatisfied_pins(&app_requiring("\"\""), &[], Severity::Error).is_empty());
        // The *file* check refuses wherever it runs — and that includes `run` and
        // nested dispatch, which skip `validate_app` and reach this through
        // `malformed_requires`. Verified on the real binary: an app installed
        // clean and then edited in place to `requires: ["   "]` is refused by
        // `app run --simulate` with exit 3. Same treatment `"@1.2.3"` gets, and
        // deliberately so: an entry naming no agent is unreadable *as a
        // declaration*, so no gate can honestly report it satisfied.
        assert!(
            malformed_requires(&app_requiring("\"   \""))
                .iter()
                .any(|i| i.code == "E_APP_REQUIRES_MALFORMED")
        );
        // An unpinned entry that DOES name an agent is still legal, or this
        // becomes a check that refuses every unpinned dependency.
        assert!(
            !validate_app(&app_requiring("\"ifc-reference-reader\""))
                .iter()
                .any(|i| i.code == "E_APP_REQUIRES_MALFORMED")
        );
    }

    #[test]
    fn a_padded_agent_id_is_normalised_rather_than_silently_exempted() {
        // The third spelling of one bug, and the one that shows why a predicate on
        // the id's *shape* was the wrong instrument. `"probe-agent @1.2.3"` has a
        // nonempty id, so every emptiness check — `is_empty`, then
        // `trim().is_empty()` — passes it. Then `dispatchable` holds
        // `ifc-reference-reader` while the entry carries `ifc-reference-reader `,
        // the exact-equality lookup misses, and the entry takes the
        // "declared but unreachable" exemption: a live node runs the installed
        // agent with the pin apparently in force and actually unchecked.
        //
        // Normalising at the split closes this by construction. The assertions
        // below are the two halves of that: a violated pin is now *reported*, and
        // a satisfied one is still *accepted* — a fix that merely rejected padding
        // would pass the first and fail the second.
        let agents = vec![installed("ifc-reference-reader", "1.3.0")];
        for padded in [
            "ifc-reference-reader @9.9.x",
            " ifc-reference-reader@9.9.x",
            "  ifc-reference-reader  @9.9.x",
            "\tifc-reference-reader@9.9.x",
        ] {
            assert_eq!(
                unsatisfied_pins(
                    &app_requiring(&format!("\"{padded}\"")),
                    &agents,
                    Severity::Error
                )
                .iter()
                .map(|i| i.code)
                .collect::<Vec<_>>(),
                ["E_APP_AGENT_PIN_UNSATISFIED"],
                "{padded:?} must be enforced, not exempted"
            );
        }
        // …and the same padding on a pin that IS satisfied stays silent, so the
        // normalisation reads the id rather than merely refusing odd-looking ones.
        for padded in [
            "ifc-reference-reader @1.3.x",
            " ifc-reference-reader@1.3.x",
            "  ifc-reference-reader  @1.3.0",
        ] {
            assert!(
                unsatisfied_pins(
                    &app_requiring(&format!("\"{padded}\"")),
                    &agents,
                    Severity::Error
                )
                .is_empty(),
                "{padded:?} names a satisfied pin and must pass"
            );
        }
        // The file-level check agrees: a padded id is a readable entry, not a
        // malformed one. The two checks disagreeing about what an id *is* is the
        // mechanism that produced this whole family.
        assert!(
            !validate_app(&app_requiring("\"ifc-reference-reader @1.3.x\""))
                .iter()
                .any(|i| i.code == "E_APP_REQUIRES_MALFORMED")
        );
    }

    #[test]
    fn an_uninstalled_agent_gets_no_pin_verdict() {
        // A pin verdict needs a version to judge. "Not installed" is a different
        // finding with its own gate (`missing_agents`, #308) and its own remedy —
        // reporting both would name one gap twice.
        let agents = vec![installed("something-else", "1.0.0")];
        assert!(
            unsatisfied_pins(
                &app_requiring("ifc-reference-reader@9.9.x"),
                &agents,
                Severity::Error
            )
            .is_empty()
        );
    }

    #[test]
    fn an_unpinned_requires_entry_admits_any_installed_version() {
        // `requires: [- foo]` declares a dependency, not a constraint.
        assert!(pin_codes("ifc-reference-reader", "1.3.0").is_empty());
    }

    #[test]
    fn severity_selects_the_gate() {
        let agents = vec![installed("ifc-reference-reader", "1.3.0")];
        let app = app_requiring("ifc-reference-reader@9.9.x");
        let warn = unsatisfied_pins(&app, &agents, Severity::Warning);
        assert_eq!(warn[0].code, "W_APP_AGENT_PIN_UNSATISFIED");
        assert_eq!(warn[0].severity, Severity::Warning);
        let err = unsatisfied_pins(&app, &agents, Severity::Error);
        assert_eq!(err[0].code, "E_APP_AGENT_PIN_UNSATISFIED");
        assert_eq!(err[0].severity, Severity::Error);
    }

    #[test]
    fn a_non_semver_installed_version_is_reported_rather_than_silently_passing() {
        // Failing open here would reintroduce the bug for exactly the agents whose
        // versioning is already suspect.
        let codes = pin_codes("ifc-reference-reader@1.x", "latest");
        assert_eq!(codes, ["E_APP_AGENT_PIN_UNSATISFIED"]);
    }

    #[test]
    fn prerelease_and_build_metadata_stay_checkable() {
        // `1.3.0-rc.1` is within the RANGE `1.3.x`; reading the triple past the
        // suffix must not make the version unreadable (which would report it as
        // unpinnable instead of judging it).
        assert!(pin_codes("ifc-reference-reader@1.3.x", "1.3.0-rc.1").is_empty());
        assert!(pin_codes("ifc-reference-reader@1.3.x", "1.3.0+deadbeef").is_empty());
        assert_eq!(
            pin_codes("ifc-reference-reader@1.4.x", "1.3.0-rc.1"),
            ["E_APP_AGENT_PIN_UNSATISFIED"]
        );
    }

    #[test]
    fn an_exact_pin_can_name_the_prerelease_it_wants() {
        // The other half of "a prerelease is a distinct release". Refusing it
        // for `@1.2.3` is right, but the pin grammar then had no way to ask for
        // it either: `parse` split on `.` before anything else, so
        // `1.2.3-rc.1` arrived as four components and was rejected as
        // MALFORMED — while the installed-version parser read the same string
        // happily. An app could run a prerelease and never pin one, leaving a
        // range like `1.2.x` as the only way to name it, which admits every
        // other patch as well and gives up the reproducibility the exact form
        // is for.
        assert!(pin_codes("ifc-reference-reader@1.2.3-rc.1", "1.2.3-rc.1").is_empty());
        // It admits that prerelease and nothing else — not the stable release,
        // and not a different candidate.
        assert_eq!(
            pin_codes("ifc-reference-reader@1.2.3-rc.1", "1.2.3"),
            ["E_APP_AGENT_PIN_UNSATISFIED"]
        );
        assert_eq!(
            pin_codes("ifc-reference-reader@1.2.3-rc.1", "1.2.3-rc.2"),
            ["E_APP_AGENT_PIN_UNSATISFIED"]
        );
        // §10 again, now on the pin's side: build metadata is not part of a
        // release's identity, so it must not change the verdict from either
        // direction.
        assert!(pin_codes("ifc-reference-reader@1.2.3+build.1", "1.2.3").is_empty());
        assert!(pin_codes("ifc-reference-reader@1.2.3-rc.1", "1.2.3-rc.1+deadbeef").is_empty());
        // A prerelease pin is held to §9 exactly as an installed version is, so
        // a malformed one is malformed rather than quietly meaning something.
        for bad in [
            "1.2.3-",      // empty prerelease
            "1.2.3-01",    // zero-padded numeric identifier
            "1.2.3-rc..1", // empty identifier
            "1.2.3-rc_1",  // illegal character
            "1.2.3+",      // empty build metadata
        ] {
            assert!(
                VersionPin::parse(bad).is_none(),
                "{bad:?} must not parse as a pin"
            );
        }
        // A prerelease names ONE release, so it means nothing on a range form.
        for bad in ["1.2.x-rc.1", "1.x-rc.1", "1.2-rc.1"] {
            assert!(
                VersionPin::parse(bad).is_none(),
                "{bad:?} must not parse — a range cannot carry a prerelease"
            );
        }
    }

    #[test]
    fn an_exact_pin_refuses_a_prerelease_but_ignores_build_metadata() {
        // An exact pin is the reproducibility form, so it must admit only that
        // release. `1.2.3-rc.1` is a DIFFERENT release from `1.2.3` in semver
        // (§9, and it orders *before* it), so admitting it would silently run a
        // release candidate against an app that asked for the stable version —
        // exactly the substitution the exact form exists to prevent.
        assert_eq!(
            pin_codes("ifc-reference-reader@1.2.3", "1.2.3-rc.1"),
            ["E_APP_AGENT_PIN_UNSATISFIED"]
        );
        // Build metadata is excluded from identity and precedence by semver §10,
        // so it cannot change which pins a version satisfies — `1.2.3+deadbeef`
        // IS `1.2.3`. Stripping it must not be confused with stripping a
        // prerelease, including when the metadata itself contains a hyphen.
        assert!(pin_codes("ifc-reference-reader@1.2.3", "1.2.3+deadbeef").is_empty());
        assert!(pin_codes("ifc-reference-reader@1.2.3", "1.2.3+linux-gnu").is_empty());
        // A prerelease that carries build metadata too is still a prerelease.
        assert_eq!(
            pin_codes("ifc-reference-reader@1.2.3", "1.2.3-rc.1+deadbeef"),
            ["E_APP_AGENT_PIN_UNSATISFIED"]
        );
    }

    #[test]
    fn a_malformed_installed_version_never_reads_as_a_valid_release() {
        // The fail-closed branch only protects anything if malformed versions
        // actually REACH it. A lenient parse routes them into the satisfied path
        // instead: `1.2.3+` (empty build metadata, which `validate_agent` does not
        // reject today) would read as the release `1.2.3` and pass an exact
        // `@1.2.3` pin — the substitution the exact form exists to prevent.
        for bad in [
            "1.2.3+",      // empty build metadata
            "1.2.3-",      // empty prerelease
            "1.02.3",      // zero-padded core component
            "01.2.3",      // ditto, major
            "1.2.3-01",    // zero-padded numeric prerelease identifier (§9)
            "1.2.3-rc..1", // empty identifier from a repeated separator
            "1.2.3-rc.1+", // valid prerelease, empty build
            "1.2.3+a..b",  // empty build identifier
            "1.2.3-rc_1",  // underscore is not an allowed identifier character
            "1.2",         // not three components
            "1.2.3.4",
            "v1.2.3", // a leading `v` is not part of the grammar
            "",
        ] {
            assert_eq!(
                pin_codes("ifc-reference-reader@1.2.3", bad),
                ["E_APP_AGENT_PIN_UNSATISFIED"],
                "{bad:?} must be reported as uncheckable, not silently satisfied"
            );
            // Reported for the right REASON — as unreadable, not as a mismatch.
            let agents = vec![installed("ifc-reference-reader", bad)];
            let issues = unsatisfied_pins(
                &app_requiring("ifc-reference-reader@1.2.3"),
                &agents,
                Severity::Error,
            );
            assert!(
                issues[0].message.contains("is not semver"),
                "{bad:?}: {}",
                issues[0].message
            );
        }
        // …while the legitimately-suffixed versions still parse, so strictness
        // did not simply break the prerelease support added for the exact-pin fix.
        assert!(pin_codes("ifc-reference-reader@1.2.x", "1.2.3-rc.1").is_empty());
        assert!(pin_codes("ifc-reference-reader@1.2.x", "1.2.3+build.1").is_empty());
        assert!(pin_codes("ifc-reference-reader@1.2.x", "1.2.3-0.3.7+exp.sha.5114f85").is_empty());
        assert!(pin_codes("ifc-reference-reader@1.2.3", "1.2.3+linux-gnu").is_empty());
        assert!(pin_codes("ifc-reference-reader@0.2.x", "0.2.0").is_empty()); // a real `0` component
    }

    #[test]
    fn a_zero_padded_pin_component_is_rejected_rather_than_silently_reinterpreted() {
        // `01.2.x` is not `1.2.x`. Accepting it would make the pin mean something
        // its author didn't write — the same SemVer rule the installed version is
        // held to, applied to the other side of the comparison.
        for bad in ["01.2.x", "1.02.x", "1.2.03", "0.01.x"] {
            let issues = validate_app(&app_requiring(&format!("ifc-reference-reader@{bad}")));
            assert!(
                issues.iter().any(|i| i.code == "E_APP_REQUIRES_MALFORMED"),
                "{bad} must be rejected; issues: {issues:?}"
            );
        }
        // A genuine zero component is fine — the rule is about PADDING.
        assert!(
            !validate_app(&app_requiring("ifc-reference-reader@0.1.x"))
                .iter()
                .any(|i| i.code == "E_APP_REQUIRES_MALFORMED")
        );
    }

    #[test]
    fn the_remedy_states_the_goal_and_prescribes_no_command() {
        // This test used to assert the opposite for exact pins, and the history is
        // the reason it now reads this way. The printed command drew five separate
        // findings — range pins are not registry keys; build metadata is part of a
        // pin but not of a key; `install` refuses while a copy is on disk; `update`
        // takes no version — ending in a DESTRUCTIVE one: `uninstall` first, then
        // reinstall-by-version, silently assumes the registry, so for an agent
        // installed from a local folder it removed the only copy and then failed.
        //
        // The validator knows neither the agent's provenance nor what the registry
        // holds, so every sequence it could print is wrong for some real case. It
        // states the goal instead. A message that prescribes nothing cannot
        // prescribe something harmful.
        let agents = vec![installed("ifc-reference-reader", "1.3.0")];
        for pin in [
            "0.1.x",
            "0.x",
            "0.1",
            "0.1.0",
            "0.1.0+build.1",
            "0.1.0-rc.1",
            "0.1.0-rc.1+build.1",
        ] {
            let issues = unsatisfied_pins(
                &app_requiring(&format!("ifc-reference-reader@{pin}")),
                &agents,
                Severity::Error,
            );
            let msg = &issues[0].message;
            // The operator still learns what is needed…
            assert!(
                msg.contains(&format!("install a version matching {pin}")),
                "pin {pin} lost the goal: {msg}"
            );
            // …and is never handed an incantation whose success cannot be known here.
            for verb in [
                "aware agent install",
                "aware agent uninstall",
                "aware agent update",
            ] {
                assert!(!msg.contains(verb), "pin {pin} prescribed `{verb}`: {msg}");
            }
        }
    }

    #[test]
    fn a_malformed_pin_is_rejected_by_the_file_check_not_the_catalogue_one() {
        // Whether a pin is *readable* is a fact about the app file — same verdict on
        // every machine — so it belongs in `validate_app`, which is what makes
        // `aware app validate` catch it with no agents installed.
        for bad in [
            "ifc-reference-reader@not-a-version",
            "ifc-reference-reader@",
            "ifc-reference-reader@1",
            "ifc-reference-reader@1.2.3.4",
            "ifc-reference-reader@x.2.3",
            "ifc-reference-reader@1.x.3", // wildcard in a non-final position denotes nothing
            "ifc-reference-reader@-1.0.0",
        ] {
            let issues = validate_app(&app_requiring(bad));
            assert!(
                issues.iter().any(|i| i.code == "E_APP_REQUIRES_MALFORMED"),
                "{bad} must be rejected; issues: {issues:?}"
            );
        }
        // A well-formed pin the catalogue happens not to satisfy is NOT a syntax
        // error — otherwise the two findings would collapse into one wrong message.
        let issues = validate_app(&app_requiring("ifc-reference-reader@9.9.x"));
        assert!(
            !issues.iter().any(|i| i.code == "E_APP_REQUIRES_MALFORMED"),
            "issues: {issues:?}"
        );
    }

    #[test]
    fn an_unreadable_pin_fails_closed_in_the_catalogue_check_too() {
        // This test previously asserted the OPPOSITE — that `unsatisfied_pins` stays
        // quiet because `validate_app` already reported the pin. That reasoning was
        // right about the message and wrong about the mechanism: `app run` and nested
        // exposed-app dispatch call `unsatisfied_pins` WITHOUT `validate_app`, so
        // silence there let an app with an unreadable constraint run against any
        // installed version — reopening the hole for exactly the apps whose
        // `requires:` is already suspect. A check that cannot read its constraint
        // must never answer "satisfied".
        assert_eq!(
            pin_codes("ifc-reference-reader@not-a-version", "1.3.0"),
            ["E_APP_REQUIRES_MALFORMED"]
        );
        // Reported as UNREADABLE, not as a version mismatch — the operator must not
        // be sent chasing a constraint that was never parsed.
        let agents = vec![installed("ifc-reference-reader", "1.3.0")];
        let issues = unsatisfied_pins(
            &app_requiring("ifc-reference-reader@not-a-version"),
            &agents,
            Severity::Error,
        );
        assert!(
            issues[0].message.contains("is unreadable"),
            "{}",
            issues[0].message
        );
        assert!(
            !issues[0].message.contains("1.3.0 is installed"),
            "{}",
            issues[0].message
        );
        // Same fail-closed answer when the agent isn't installed at all: without a
        // readable pin there is nothing to defer to `missing_agents` about.
        assert_eq!(
            unsatisfied_pins(
                &app_requiring("ifc-reference-reader@not-a-version"),
                &[],
                Severity::Error
            )
            .iter()
            .map(|i| i.code)
            .collect::<Vec<_>>(),
            ["E_APP_REQUIRES_MALFORMED"]
        );
        // And it honours the gate selector like every other finding here.
        assert_eq!(
            unsatisfied_pins(
                &app_requiring("ifc-reference-reader@not-a-version"),
                &agents,
                Severity::Warning
            )[0]
            .code,
            "W_APP_REQUIRES_MALFORMED"
        );
    }

    #[test]
    fn every_reference_app_pin_is_well_formed_and_satisfied_by_its_own_agent() {
        // The substrate's own 8 apps are the corpus this check runs against in the
        // wild — if the grammar above were wrong, this is where it shows.
        for app_rel in [
            "30-apps/_examples/welded-to-tc.app",
            "30-apps/_examples/qa-drawings-to-tekla.app",
            "30-apps/_examples/architect-sheet-status.app",
            "30-apps/_examples/bim-monday-audit.app",
            "30-apps/_examples/designer-monday-shots.app",
            "30-apps/_examples/detailer-issue-pack.app",
            "30-apps/_examples/engineer-peer-review-delta.app",
            "30-apps/_examples/model-to-renders.app",
        ] {
            let app = load_app(app_rel);
            assert!(!app.requires.is_empty(), "{app_rel} declares no pins");
            let issues = validate_app(&app);
            assert!(
                !issues.iter().any(|i| i.code == "E_APP_REQUIRES_MALFORMED"),
                "{app_rel}: {issues:?}"
            );
            // And every pin admits the version of the agent it names — proving the
            // published pins and the shipped agents actually agree. Agents that
            // don't ship in this repo (`excel`, `think-node`) get no verdict, so
            // building the catalogue from what IS here is the whole check.
            let agents: Vec<_> = app
                .requires
                .iter()
                .filter_map(|e| crate::manifest::app::split_requires_entry(e).0.into())
                .filter_map(shipped_agent)
                .collect();
            let unsatisfied = unsatisfied_pins(&app, &agents, Severity::Error);
            assert!(
                unsatisfied.is_empty(),
                "{app_rel} pins a version no shipped agent satisfies: {:?}",
                unsatisfied.iter().map(|i| &i.message).collect::<Vec<_>>()
            );
        }
    }

    /// Load `20-agents/**/<id>/manifest.yaml` as a `DiscoveredAgent`, if that
    /// agent ships in this repo. Hand-rolled walk rather than a glob dependency
    /// for one test.
    fn shipped_agent(id: &str) -> Option<crate::manifest::loader::DiscoveredAgent> {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("20-agents");
        fn walk(dir: &std::path::Path, id: &str, out: &mut Option<std::path::PathBuf>) {
            let Ok(entries) = std::fs::read_dir(dir) else {
                return;
            };
            for e in entries.flatten() {
                let p = e.path();
                if p.is_dir() {
                    if p.file_name().and_then(|s| s.to_str()) == Some(id) {
                        let m = p.join("manifest.yaml");
                        if m.is_file() {
                            *out = Some(m);
                            return;
                        }
                    }
                    walk(&p, id, out);
                }
            }
        }
        let mut found = None;
        walk(&root, id, &mut found);
        let manifest = found?;
        Some(crate::manifest::loader::DiscoveredAgent {
            manifest: serde_yaml::from_str(&std::fs::read_to_string(&manifest).unwrap()).unwrap(),
            root: manifest.parent().unwrap().to_path_buf(),
        })
    }

    fn agent_with_status(status_line: &str) -> crate::manifest::loader::DiscoveredAgent {
        let yaml = format!(
            "agent: html-report\nversion: 0.1.0\ndescription: x\nstateful: false\n{status_line}\
             license: MIT\ntransport:\n  cli:\n    binary: aware-html-report\ncommands:\n  \
             render:\n    lifecycle: single\n    description: x\n"
        );
        crate::manifest::loader::DiscoveredAgent {
            manifest: serde_yaml::from_str(&yaml).unwrap(),
            root: std::path::PathBuf::from("."),
        }
    }

    #[test]
    fn rejects_node_referencing_planned_agent() {
        let agents = vec![agent_with_status("status: planned\n")];
        let app: App = serde_yaml::from_str(
            "app: uses-planned\nversion: 0.0.1\ndescription: |\n  uses planned agent\n\
             requires: []\nnodes:\n  - id: report\n    agent: html-report\n    command: render\n",
        )
        .unwrap();
        let issues = validate_app_agents(&app, &agents);
        assert!(
            issues.iter().any(|i| i.code == "E_APP_AGENT_UNAVAILABLE"),
            "issues: {issues:?}"
        );
    }

    #[test]
    fn available_agent_passes_agent_validation() {
        // No `status:` → defaults to available → no E_APP_AGENT_UNAVAILABLE.
        let agents = vec![agent_with_status("")];
        let app: App = serde_yaml::from_str(
            "app: uses-avail\nversion: 0.0.1\ndescription: |\n  uses available agent\n\
             requires: []\nnodes:\n  - id: report\n    agent: html-report\n    command: render\n",
        )
        .unwrap();
        let issues = validate_app_agents(&app, &agents);
        assert!(
            !issues.iter().any(|i| i.code == "E_APP_AGENT_UNAVAILABLE"),
            "issues: {issues:?}"
        );
    }

    #[test]
    fn missing_agent_is_reported_with_node_agent_and_remedy() {
        // #308: an app referencing an agent that isn't installed used to pass
        // every gate and die at run with a bare `os error 3`. The issue must
        // name the offending node, the agent, and how to fix it.
        let app: App = serde_yaml::from_str(
            "app: probe\nversion: 0.0.1\ndescription: |\n  unknown agent\n\
             requires: []\nnodes:\n  - id: n\n    agent: definitely-not-an-agent-xyz\n    command: render\n",
        )
        .unwrap();
        let issues = missing_agents(&app, &[], Severity::Error);
        let issue = issues
            .iter()
            .find(|i| i.code == "E_APP_AGENT_NOT_INSTALLED")
            .unwrap_or_else(|| panic!("expected E_APP_AGENT_NOT_INSTALLED, got {issues:?}"));
        assert!(issue.message.contains("\"n\""), "names the node: {issue:?}");
        assert!(
            issue.message.contains("definitely-not-an-agent-xyz"),
            "names the agent: {issue:?}"
        );
        assert!(
            issue
                .message
                .contains("aware agent install definitely-not-an-agent-xyz"),
            "carries the remedy: {issue:?}"
        );
    }

    #[test]
    fn missing_agent_severity_selects_the_code() {
        // install/compile warn (an app may be installed before its agents, #170);
        // run errors. Same message, different gate.
        let app: App = serde_yaml::from_str(
            "app: probe\nversion: 0.0.1\ndescription: |\n  unknown agent\n\
             requires: []\nnodes:\n  - id: n\n    agent: nope\n    command: render\n",
        )
        .unwrap();
        let warn = missing_agents(&app, &[], Severity::Warning);
        assert_eq!(warn.len(), 1);
        assert_eq!(warn[0].code, "W_APP_AGENT_NOT_INSTALLED");
        assert!(!has_errors(&warn), "warning severity must not be an error");
        let err = missing_agents(&app, &[], Severity::Error);
        assert!(has_errors(&err), "error severity must be an error");
    }

    #[test]
    fn installed_agent_is_not_reported_missing() {
        let agents = vec![agent_with_status("")];
        let app: App = serde_yaml::from_str(
            "app: ok\nversion: 0.0.1\ndescription: |\n  installed agent\n\
             requires: []\nnodes:\n  - id: report\n    agent: html-report\n    command: render\n",
        )
        .unwrap();
        assert!(
            missing_agents(&app, &agents, Severity::Error).is_empty(),
            "an installed agent must not be flagged"
        );
    }

    #[test]
    fn frozen_node_with_missing_agent_is_not_reported() {
        // A frozen node emits its pinned output and never dispatches, so its
        // agent needn't be installed at all (app-spec § Frozen nodes) — the same
        // carve-out the planned-agent gate makes.
        let app: App = serde_yaml::from_str(
            "app: frz\nversion: 0.0.1\ndescription: |\n  frozen unknown\n\
             requires: []\nnodes:\n  - id: n\n    agent: nope\n    command: render\n    frozen:\n      ok: true\n",
        )
        .unwrap();
        assert!(
            missing_agents(&app, &[], Severity::Error).is_empty(),
            "a frozen node must not require its agent to be installed"
        );
    }

    #[test]
    fn a_requires_only_agent_is_not_reported_missing() {
        // `app-spec.md § Versioning` states this, and the statement had drifted: it
        // used to promise `W_/E_APP_AGENT_NOT_INSTALLED` for any uninstalled agent
        // named in `requires:`, which was written before the dispatchability rule
        // landed in the paragraph above it and was never true for an entry with no
        // node behind it.
        //
        // The code is the side that is right. This gate already skips frozen
        // subtrees because "a missing agent is harmless there"; an agent nothing
        // dispatches to is the same case, so reporting it would treat an *absent*
        // agent more harshly than a merely mismatched one — the contradiction the
        // frozen-only exemption exists to avoid. It would also refuse apps that
        // run today, and there is no node for the message to name.
        let requires_only: App = serde_yaml::from_str(
            "app: pin-test\nversion: 0.1.0\ndescription: x\nrequires:\n  \
             - ifc-reference-reader@1.2.3\nnodes:\n  - id: gate\n    inline:\n      \
             kind: predicate\n      description: always pass\n      code: 'true'\n",
        )
        .unwrap();
        assert!(
            missing_agents(&requires_only, &[], Severity::Error).is_empty(),
            "an agent no node dispatches to must not be reported missing"
        );
        // The exemption is about dispatchability, not about `requires:`. The same
        // pin with a live node behind it is still reported, so this cannot become a
        // blanket excuse for uninstalled agents.
        let dispatched: App = serde_yaml::from_str(
            "app: pin-test\nversion: 0.1.0\ndescription: x\nrequires:\n  \
             - ifc-reference-reader@1.2.3\nnodes:\n  - id: probe\n    \
             agent: ifc-reference-reader\n    command: probe\n",
        )
        .unwrap();
        assert_eq!(
            missing_agents(&dispatched, &[], Severity::Error)
                .iter()
                .map(|i| i.code)
                .collect::<Vec<_>>(),
            ["E_APP_AGENT_NOT_INSTALLED"]
        );
    }

    #[test]
    fn frozen_for_each_body_is_not_scanned_for_missing_agents() {
        // A frozen `for-each` container short-circuits before its `do:` body runs,
        // so an uninstalled agent *inside* that body can never be dispatched to.
        // Descending into it would refuse a valid frozen composition at run.
        let app: App = serde_yaml::from_str(
            "app: frzloop\nversion: 0.0.1\ndescription: |\n  frozen for-each\n\
             requires: []\nnodes:\n  - id: each\n    for-each: '{{ inputs.items }}'\n\
             \x20   frozen:\n      ok: true\n    do:\n\
             \x20     - id: inner\n        agent: nope\n        command: render\n",
        )
        .unwrap();
        assert!(
            missing_agents(&app, &[], Severity::Error).is_empty(),
            "a frozen node's `do:` body never runs — it must not be scanned"
        );
    }

    #[test]
    fn missing_agent_is_found_inside_a_for_each_body() {
        // The traversal must recurse into `do:` bodies, or a for-each node hides
        // the missing agent from the pre-flight and it resurfaces at run.
        let app: App = serde_yaml::from_str(
            "app: loop\nversion: 0.0.1\ndescription: |\n  for-each body\n\
             requires: []\nnodes:\n  - id: each\n    for-each: '{{ inputs.items }}'\n    do:\n\
             \x20     - id: inner\n        agent: nope\n        command: render\n",
        )
        .unwrap();
        let issues = missing_agents(&app, &[], Severity::Error);
        assert!(
            issues.iter().any(|i| i.message.contains("\"inner\"")),
            "must recurse into `do:` bodies: {issues:?}"
        );
    }

    #[test]
    fn nodes_without_an_agent_are_not_reported() {
        // Inline / primitive nodes carry no `agent:` and dispatch to nothing.
        let app: App = serde_yaml::from_str(
            "app: inl\nversion: 0.0.1\ndescription: |\n  inline only\n\
             requires: []\nnodes:\n  - id: i\n    inline:\n      kind: predicate\n      description: x\n      code: 'true'\n",
        )
        .unwrap();
        assert!(
            missing_agents(&app, &[], Severity::Error).is_empty(),
            "a node with no agent has nothing to resolve"
        );
    }

    #[test]
    fn frozen_node_skips_planned_agent_gate() {
        // A frozen node never dispatches to its agent, so a planned (not-yet-runnable) agent must
        // NOT gate it — `frozen:` short-circuits the availability preflight (it needn't be installed).
        let agents = vec![agent_with_status("status: planned\n")];
        let app: App = serde_yaml::from_str(
            "app: frz\nversion: 0.0.1\ndescription: |\n  frozen planned\n\
             requires: []\nnodes:\n  - id: report\n    agent: html-report\n    command: render\n    frozen:\n      ok: true\n",
        )
        .unwrap();
        let issues = validate_app_agents(&app, &agents);
        assert!(
            !issues.iter().any(|i| i.code == "E_APP_AGENT_UNAVAILABLE"),
            "frozen node must skip the agent-availability gate; issues: {issues:?}"
        );
    }

    #[test]
    fn frozen_write_node_skips_safety_gate() {
        // A frozen node never writes (it emits a pinned value), so the write-mode safety contract
        // must not apply — a frozen write-mode node needs no `safety:` block.
        let writer = crate::manifest::loader::DiscoveredAgent {
            manifest: serde_yaml::from_str(
                "agent: writer\nversion: 0.1.0\ndescription: x\nstateful: false\nlicense: MIT\n\
                 transport:\n  cli:\n    binary: aware-writer\ncommands:\n  apply:\n    lifecycle: single\n    mode: write\n    description: x\n",
            )
            .unwrap(),
            root: std::path::PathBuf::from("."),
        };
        let app: App = serde_yaml::from_str(
            "app: frzw\nversion: 0.0.1\ndescription: |\n  frozen write\n\
             requires: []\nnodes:\n  - id: w\n    agent: writer\n    command: apply\n    frozen:\n      done: true\n",
        )
        .unwrap();
        let issues = validate_app_safety(&app, &[writer]);
        assert!(
            !issues
                .iter()
                .any(|i| i.code == "E_APP_WRITE_WITHOUT_SAFETY"),
            "frozen write-mode node must skip the safety gate; issues: {issues:?}"
        );
    }

    #[test]
    fn rejects_node_referencing_planned_command() {
        // The agent is available, but one command is `status: planned` (#199) — an
        // app using it must fail validate, while a sibling available command passes.
        let yaml = "agent: part\nversion: 0.1.0\ndescription: x\nstateful: false\n\
                    license: MIT\ntransport:\n  rest:\n    base: https://x\ncommands:\n  \
                    read-thing:\n    lifecycle: single\n    description: x\n  \
                    write-thing:\n    lifecycle: single\n    status: planned\n    description: x\n";
        let agents = vec![crate::manifest::loader::DiscoveredAgent {
            manifest: serde_yaml::from_str(yaml).unwrap(),
            root: std::path::PathBuf::from("."),
        }];

        let bad: App = serde_yaml::from_str(
            "app: uses-planned-cmd\nversion: 0.0.1\ndescription: x\nrequires: []\n\
             nodes:\n  - id: w\n    agent: part\n    command: write-thing\n",
        )
        .unwrap();
        let issues = validate_app_agents(&bad, &agents);
        assert!(
            issues.iter().any(|i| i.code == "E_APP_COMMAND_UNAVAILABLE"),
            "planned command must be rejected: {issues:?}"
        );

        let ok: App = serde_yaml::from_str(
            "app: uses-avail-cmd\nversion: 0.0.1\ndescription: x\nrequires: []\n\
             nodes:\n  - id: r\n    agent: part\n    command: read-thing\n",
        )
        .unwrap();
        let issues = validate_app_agents(&ok, &agents);
        assert!(
            !issues.iter().any(|i| i.code == "E_APP_COMMAND_UNAVAILABLE"),
            "available command must pass: {issues:?}"
        );
    }

    fn one_node_app(agent: &str, command: &str) -> App {
        serde_yaml::from_str(&format!(
            "app: a\nversion: 0.0.1\ndescription: x\nrequires: []\nnodes:\n  - id: e\n    agent: {agent}\n    command: {command}\n"
        ))
        .unwrap()
    }

    #[test]
    fn admits_curated_capable_vision_extract() {
        // RFC #223: a curated `model-extraction` command whose agent declares the
        // capability is ADMITTED into the run path (no E_APP_RUNTIME_MODEL_FORBIDDEN).
        let agents = vec![discovered(
            "agent: vision\nversion: 0.1.0\ndescription: x\nstateful: false\nlicense: MIT\n\
             capabilities:\n  runtime-model-extraction: true\ntransport:\n  builtin: {}\n\
             commands:\n  extract:\n    lifecycle: single\n    category: curated\n    mode: read\n    \
             model-extraction: true\n    description: x\n",
        )];
        let issues = validate_app_agents(&one_node_app("vision", "extract"), &agents);
        assert!(
            !issues
                .iter()
                .any(|i| i.code == "E_APP_RUNTIME_MODEL_FORBIDDEN"),
            "curated+capable vision.extract must be admitted: {issues:?}"
        );
    }

    #[test]
    fn rejects_model_extraction_without_capability() {
        // Same curated command, but the agent does NOT declare the capability → forbidden.
        let agents = vec![discovered(
            "agent: vision\nversion: 0.1.0\ndescription: x\nstateful: false\nlicense: MIT\n\
             transport:\n  builtin: {}\ncommands:\n  extract:\n    lifecycle: single\n    \
             category: curated\n    mode: read\n    model-extraction: true\n    description: x\n",
        )];
        let issues = validate_app_agents(&one_node_app("vision", "extract"), &agents);
        assert!(
            issues
                .iter()
                .any(|i| i.code == "E_APP_RUNTIME_MODEL_FORBIDDEN"),
            "model-extraction without the capability flag must be forbidden: {issues:?}"
        );
    }

    #[test]
    fn rejects_reflected_model_extraction_even_with_capability() {
        // A model-extraction command that is NOT curated (reflected) → forbidden even
        // with the capability flag, so no `aware build --from-*` output mints a reader.
        let agents = vec![discovered(
            "agent: rogue\nversion: 0.1.0\ndescription: x\nstateful: false\nlicense: MIT\n\
             provenance:\n  generated-by: reflected\ncapabilities:\n  runtime-model-extraction: true\n\
             transport:\n  builtin: {}\ncommands:\n  extract:\n    lifecycle: single\n    \
             category: reflected\n    mode: read\n    model-extraction: true\n    description: x\n",
        )];
        let issues = validate_app_agents(&one_node_app("rogue", "extract"), &agents);
        assert!(
            issues
                .iter()
                .any(|i| i.code == "E_APP_RUNTIME_MODEL_FORBIDDEN"),
            "reflected model-extraction must be forbidden: {issues:?}"
        );
    }

    #[test]
    fn real_vision_manifest_loads_and_is_admitted() {
        // The shipped 20-agents/_core/vision manifest deserializes AND its extract node
        // passes the carve-out (curated + capability) — guards the manifest against drift.
        let a = load_agent("20-agents/_core/vision/manifest.yaml");
        let agents = vec![crate::manifest::loader::DiscoveredAgent {
            manifest: a,
            root: std::path::PathBuf::from("."),
        }];
        let issues = validate_app_agents(&one_node_app("vision", "extract"), &agents);
        assert!(
            !issues
                .iter()
                .any(|i| i.code == "E_APP_RUNTIME_MODEL_FORBIDDEN"),
            "the real vision manifest must be admitted: {issues:?}"
        );
    }

    #[test]
    fn rejects_unrunnable_inline_kind_at_validate() {
        // kind: shape passes parse but the runtime only runs `predicate`; validate
        // must reject it so the failure surfaces before compile/lock (#160).
        let yaml = r#"
app: inline-shape
version: 0.0.1
description: |
  Inline shape node.
requires: []
nodes:
  - id: passthrough
    inline:
      kind: shape
      description: reshape a value
      code: "() => ({ ok: true })"
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let issues = validate_app(&app);
        assert!(has_errors(&issues), "issues: {issues:?}");
        assert!(issues.iter().any(|i| i.code == "E_APP_INLINE_KIND"));
    }

    #[test]
    fn rejects_unrunnable_inline_kind_nested_in_for_each_body() {
        // A shape node inside a for-each `do:` body is flattened + run, so it must
        // be rejected at validate too — not just top-level nodes (#160 review).
        let yaml = r#"
app: inline-shape-body
version: 0.0.1
description: |
  for-each with an inline shape in its body.
requires: []
nodes:
  - id: loop
    for-each: '{{ items }}'
    do:
      - id: reshape
        inline:
          kind: shape
          description: reshape each item
          code: "() => ({ ok: true })"
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let issues = validate_app(&app);
        assert!(
            issues.iter().any(|i| i.code == "E_APP_INLINE_KIND"),
            "issues: {issues:?}"
        );
    }

    #[test]
    fn predicate_inline_kind_passes_validate() {
        let yaml = r#"
app: inline-pred
version: 0.0.1
description: |
  Inline predicate node.
requires: []
nodes:
  - id: gate
    inline:
      kind: predicate
      description: gate on type
      code: 'e.type == "Welded"'
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let issues = validate_app(&app);
        assert!(
            !issues.iter().any(|i| i.code == "E_APP_INLINE_KIND"),
            "issues: {issues:?}"
        );
    }

    #[test]
    fn detects_cycle_in_connections() {
        let yaml = r#"
app: cyc
version: 0.0.1
description: |
  Cyclic app.
nodes:
  - id: a
  - id: b
  - id: c
connections:
  - { from: a, to: b }
  - { from: b, to: c }
  - { from: c, to: a }
requires: []
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let issues = validate_app(&app);
        assert!(has_errors(&issues));
        assert!(issues.iter().any(|i| i.code == "E_APP_CYCLE"));
    }

    #[test]
    fn detects_dangling_connection() {
        let yaml = r#"
app: dangle
version: 0.0.1
description: x
nodes:
  - id: a
connections:
  - { from: a, to: nope }
requires: []
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let issues = validate_app(&app);
        assert!(issues.iter().any(|i| i.code == "E_APP_DANGLING_TO"));
    }

    #[test]
    fn detects_duplicate_node_ids() {
        let yaml = r#"
app: dup
version: 0.0.1
description: x
nodes:
  - id: a
  - id: a
connections: []
requires: []
"#;
        let app: App = serde_yaml::from_str(yaml).unwrap();
        let issues = validate_app(&app);
        assert!(issues.iter().any(|i| i.code == "E_APP_DUPLICATE_NODE_ID"));
    }

    #[test]
    fn stateful_without_start_is_error() {
        let yaml = r#"
agent: x
version: 1.0
description: y
stateful: true
license: MIT
transport: { cli: { binary: x } }
commands:
  do:
    lifecycle: single
    description: z
"#;
        let a: Agent = serde_yaml::from_str(yaml).unwrap();
        let issues = validate_agent(&a);
        assert!(issues.iter().any(|i| i.code == "E_STATEFUL_NO_START"));
    }

    #[test]
    fn skill_file_missing_is_error() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(tmp.path().join("skills")).unwrap();
        let manifest = r#"
agent: x
version: 1.0
description: y
stateful: false
license: MIT
transport: { cli: { binary: x } }
commands: { do: { lifecycle: single, description: z } }
skills:
  - foo.md
"#;
        let a: Agent = serde_yaml::from_str(manifest).unwrap();
        let issues = validate_agent_on_disk(&a, tmp.path());
        assert!(issues.iter().any(|i| i.code == "E_SKILL_FILE_MISSING"));
    }

    /// Build a `DiscoveredAgent` for the safety-contract tests below.
    fn discovered(agent_yaml: &str) -> crate::manifest::loader::DiscoveredAgent {
        let a: Agent = serde_yaml::from_str(agent_yaml).unwrap();
        crate::manifest::loader::DiscoveredAgent {
            manifest: a,
            root: std::path::PathBuf::from("/dev/null"),
        }
    }

    #[test]
    fn safety_check_rejects_write_node_without_safety_block() {
        let agent = discovered(
            r#"
agent: tekla
version: 1.0
description: x
stateful: false
license: Apache-2.0
transport: { cli: { binary: aware-tekla } }
commands:
  insert:
    lifecycle: single
    category: curated
    description: Insert a connection.
"#,
        );
        let app: App = serde_yaml::from_str(
            r#"
app: write-no-safety
version: 0.0.1
description: x
nodes:
  - id: t
    agent: tekla
    command: insert
connections: []
requires: []
"#,
        )
        .unwrap();

        let issues = validate_app_safety(&app, &[agent]);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].code, "E_APP_WRITE_WITHOUT_SAFETY");
    }

    #[test]
    fn safety_check_passes_when_write_node_has_safety_block() {
        let agent = discovered(
            r#"
agent: tekla
version: 1.0
description: x
stateful: false
license: Apache-2.0
transport: { cli: { binary: aware-tekla } }
commands:
  insert:
    lifecycle: single
    category: curated
    description: Insert a connection.
"#,
        );
        let app: App = serde_yaml::from_str(
            r#"
app: write-with-safety
version: 0.0.1
description: x
nodes:
  - id: t
    agent: tekla
    command: insert
    safety:
      transaction-group: my-tx
      snapshot: true
      worksharing:
        check: true
        fail-if-other-user: true
      audit-stamp:
        uda-prefix: AWARE_
connections: []
requires: []
"#,
        )
        .unwrap();

        let issues = validate_app_safety(&app, &[agent]);
        assert!(issues.is_empty(), "unexpected issues: {issues:?}");
    }

    #[test]
    fn safety_check_skips_read_mode_nodes() {
        let agent = discovered(
            r#"
agent: ex
version: 1.0
description: x
stateful: false
license: Apache-2.0
transport: { cli: { binary: aware-ex } }
commands:
  list:
    lifecycle: single
    category: curated
    description: List items.
"#,
        );
        let app: App = serde_yaml::from_str(
            r#"
app: read-only
version: 0.0.1
description: x
nodes:
  - id: l
    agent: ex
    command: list
connections: []
requires: []
"#,
        )
        .unwrap();

        let issues = validate_app_safety(&app, &[agent]);
        assert!(issues.is_empty(), "unexpected issues: {issues:?}");
    }

    #[test]
    fn safety_check_honors_explicit_mode_write_field() {
        let agent = discovered(
            r#"
agent: x
version: 1.0
description: x
stateful: false
license: MIT
transport: { cli: { binary: aware-x } }
commands:
  unusual-name-not-matching-convention:
    lifecycle: single
    mode: write
    description: Writes despite the name not matching the convention.
"#,
        );
        let app: App = serde_yaml::from_str(
            r#"
app: explicit-write
version: 0.0.1
description: x
nodes:
  - id: n
    agent: x
    command: unusual-name-not-matching-convention
connections: []
requires: []
"#,
        )
        .unwrap();

        let issues = validate_app_safety(&app, &[agent]);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].code, "E_APP_WRITE_WITHOUT_SAFETY");
    }

    #[test]
    fn safety_check_honors_explicit_mode_read_override() {
        // A command that names itself like a write (.insert suffix) but
        // declares mode: read explicitly is treated as read-mode. Lets
        // curators override the naming-convention default.
        let agent = discovered(
            r#"
agent: x
version: 1.0
description: x
stateful: false
license: MIT
transport: { cli: { binary: aware-x } }
commands:
  thing.insert:
    lifecycle: single
    mode: read
    description: Looks like a write but actually a read.
"#,
        );
        let app: App = serde_yaml::from_str(
            r#"
app: misleading-name
version: 0.0.1
description: x
nodes:
  - id: n
    agent: x
    command: thing.insert
connections: []
requires: []
"#,
        )
        .unwrap();

        let issues = validate_app_safety(&app, &[agent]);
        assert!(issues.is_empty(), "unexpected issues: {issues:?}");
    }

    #[test]
    fn safety_check_passes_read_override_on_mode_overridable_exec() {
        // The #165 install repro: `exec` is declared write-mode but
        // `mode-overridable`. A node-level `mode: read` makes it read-mode, so
        // no `safety:` block is required and install is NOT refused.
        let agent = discovered(
            r#"
agent: tekla
version: 1.0
description: x
stateful: true
license: MIT
transport: { cli: { binary: aware-tekla } }
commands:
  exec:
    lifecycle: single
    mode: write
    mode-overridable: true
    description: runs arbitrary C#
"#,
        );
        let app: App = serde_yaml::from_str(
            r#"
app: exec-mode-repro
version: 0.0.1
description: x
nodes:
  - id: probe
    agent: tekla
    command: exec
    mode: read
    config:
      code: return new { ok = true };
connections: []
requires: []
"#,
        )
        .unwrap();

        let issues = validate_app_safety(&app, &[agent]);
        assert!(
            issues.is_empty(),
            "read-only exec must install without safety: ; issues: {issues:?}"
        );
    }

    #[test]
    fn safety_check_still_requires_safety_on_unannotated_mode_overridable_exec() {
        // Without a node-level mode, mode-overridable exec keeps its
        // conservative write default — safety: is still required.
        let agent = discovered(
            r#"
agent: tekla
version: 1.0
description: x
stateful: true
license: MIT
transport: { cli: { binary: aware-tekla } }
commands:
  exec:
    lifecycle: single
    mode: write
    mode-overridable: true
    description: runs arbitrary C#
"#,
        );
        let app: App = serde_yaml::from_str(
            r#"
app: exec-unannotated
version: 0.0.1
description: x
nodes:
  - id: probe
    agent: tekla
    command: exec
    config:
      code: Model.CommitChanges();
connections: []
requires: []
"#,
        )
        .unwrap();

        let issues = validate_app_safety(&app, &[agent]);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].code, "E_APP_WRITE_WITHOUT_SAFETY");
    }

    #[test]
    fn agents_check_rejects_conflicting_node_mode_on_non_overridable_command() {
        // A node-level `mode: read` on a genuinely-writing, non-overridable
        // command must be rejected loudly — never silently dropped (#165).
        // The check lives in `validate_app_agents` so it fires on every
        // agent-aware path (install, compile, inspect, validate, run, dry-run),
        // not just safety pre-flight.
        let agent = discovered(
            r#"
agent: tekla
version: 1.0
description: x
stateful: true
license: MIT
transport: { cli: { binary: aware-tekla } }
commands:
  uda-set:
    lifecycle: single
    mode: write
    description: writes a UDA
"#,
        );
        let app: App = serde_yaml::from_str(
            r#"
app: sneaky-write
version: 0.0.1
description: x
nodes:
  - id: n
    agent: tekla
    command: uda-set
    mode: read
connections: []
requires: []
"#,
        )
        .unwrap();

        let issues = validate_app_agents(&app, &[agent]);
        assert!(
            issues
                .iter()
                .any(|i| i.code == "E_APP_NODE_MODE_NOT_OVERRIDABLE"),
            "conflicting node mode on non-overridable command must be rejected; issues: {issues:?}"
        );
    }

    #[test]
    fn agents_check_rejects_conflicting_node_mode_in_nested_body() {
        // The conflict check recurses into `for-each`/`sweep` `do:` bodies —
        // a nested node cannot smuggle a dropped `mode:` past validation.
        let agent = discovered(
            r#"
agent: tekla
version: 1.0
description: x
stateful: true
license: MIT
transport: { cli: { binary: aware-tekla } }
commands:
  uda-set:
    lifecycle: single
    mode: write
    description: writes a UDA
"#,
        );
        let app: App = serde_yaml::from_str(
            r#"
app: nested-sneaky
version: 0.0.1
description: x
nodes:
  - id: loop
    for-each: '{{ inputs.items }}'
    do:
      - id: inner
        agent: tekla
        command: uda-set
        mode: read
connections: []
requires: []
"#,
        )
        .unwrap();

        let issues = validate_app_agents(&app, &[agent]);
        assert!(
            issues
                .iter()
                .any(|i| i.code == "E_APP_NODE_MODE_NOT_OVERRIDABLE"),
            "nested conflicting node mode must be rejected; issues: {issues:?}"
        );
    }

    #[test]
    fn agents_check_allows_redundant_matching_node_mode_on_non_overridable_command() {
        // Restating the manifest mode (no conflict) is redundant but allowed —
        // it must not error in either the agent-aware or safety validators.
        let agent = discovered(
            r#"
agent: tekla
version: 1.0
description: x
stateful: true
license: MIT
transport: { cli: { binary: aware-tekla } }
commands:
  bolt-list:
    lifecycle: single
    mode: read
    description: reads a bolt schedule
"#,
        );
        let app: App = serde_yaml::from_str(
            r#"
app: redundant-mode
version: 0.0.1
description: x
nodes:
  - id: n
    agent: tekla
    command: bolt-list
    mode: read
connections: []
requires: []
"#,
        )
        .unwrap();

        assert!(
            validate_app_agents(&app, std::slice::from_ref(&agent)).is_empty(),
            "redundant matching mode must not error in validate_app_agents"
        );
        assert!(
            validate_app_safety(&app, &[agent]).is_empty(),
            "redundant matching read mode must not error in validate_app_safety"
        );
    }

    // ── #178: exposes-as-agent validation ──────────────────────────────────

    #[test]
    fn exposes_as_agent_without_commands_is_error() {
        let app: App = serde_yaml::from_str(
            r#"
app: empty-expose
version: 0.0.1
description: x
exposes-as-agent: true
nodes:
  - id: n
    inline:
      kind: predicate
      description: pass
      code: 'true'
requires: []
"#,
        )
        .unwrap();
        let issues = validate_app(&app);
        assert!(
            issues.iter().any(|i| i.code == "E_APP_EXPOSES_NO_COMMANDS"),
            "issues: {issues:?}"
        );
    }

    #[test]
    fn exposes_as_agent_with_commands_passes() {
        let app: App = serde_yaml::from_str(
            r#"
app: ok-expose
version: 0.0.1
description: x
exposes-as-agent: true
exposed-commands:
  run:
    lifecycle: single
nodes:
  - id: n
    inline:
      kind: predicate
      description: pass
      code: 'true'
requires: []
"#,
        )
        .unwrap();
        let issues = validate_app(&app);
        assert!(
            !issues.iter().any(|i| i.code == "E_APP_EXPOSES_NO_COMMANDS"),
            "issues: {issues:?}"
        );
    }

    #[test]
    fn exposed_app_composing_another_exposed_app_is_rejected() {
        // `inner` is an app-backed (synthesized) agent — it declares an `app`
        // transport. A `mid` app that is ITSELF exposes-as-agent and composes
        // `inner` violates the v0 no-recursion rule.
        let inner = discovered(
            r#"
agent: inner
version: 1.0
description: x
stateful: false
license: app-exposed
transport:
  app:
    backed-by: inner
commands:
  run:
    lifecycle: single
    description: x
"#,
        );
        let mid: App = serde_yaml::from_str(
            r#"
app: mid
version: 0.0.1
description: x
exposes-as-agent: true
exposed-commands:
  run:
    lifecycle: single
nodes:
  - id: call-inner
    agent: inner
    command: run
connections: []
requires: []
"#,
        )
        .unwrap();
        let issues = validate_app_agents(&mid, std::slice::from_ref(&inner));
        assert!(
            issues
                .iter()
                .any(|i| i.code == "E_APP_EXPOSED_COMPOSES_EXPOSED"),
            "an exposed app composing another exposed app must be rejected; issues: {issues:?}"
        );

        // A NON-exposed app composing the same app-backed agent is allowed
        // (one-level composition by a normal consumer).
        let outer: App = serde_yaml::from_str(
            r#"
app: outer
version: 0.0.1
description: x
nodes:
  - id: call-inner
    agent: inner
    command: run
connections: []
requires: []
"#,
        )
        .unwrap();
        let issues = validate_app_agents(&outer, &[inner]);
        assert!(
            !issues
                .iter()
                .any(|i| i.code == "E_APP_EXPOSED_COMPOSES_EXPOSED"),
            "a normal consumer composing an exposed app must be allowed; issues: {issues:?}"
        );
    }

    #[test]
    fn exposed_app_referencing_itself_is_rejected_even_on_first_install() {
        // An exposes-as-agent app whose node references its OWN id recurses into
        // itself. On first install the synthesized agent does not exist yet, so
        // this must be caught from the app id alone — not only via the catalogue.
        let selfie: App = serde_yaml::from_str(
            r#"
app: selfie
version: 0.0.1
description: x
exposes-as-agent: true
exposed-commands:
  run:
    lifecycle: single
nodes:
  - id: loop-self
    agent: selfie
    command: run
connections: []
requires: []
"#,
        )
        .unwrap();
        // Empty catalogue — the synth agent for `selfie` is not yet installed.
        let issues = validate_app_agents(&selfie, &[]);
        assert!(
            issues
                .iter()
                .any(|i| i.code == "E_APP_EXPOSED_COMPOSES_EXPOSED"),
            "self-referential exposed app must be rejected at first install; issues: {issues:?}"
        );
    }
}
