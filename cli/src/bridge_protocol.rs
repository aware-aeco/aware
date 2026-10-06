//! Bridge protocol compatibility (#632).
//!
//! A host bridge (`aware-tekla`, `aware-revit`, …) speaks a numbered protocol
//! (verbs, request shape, receipt envelope). The installing CLI stamps that number
//! next to the bridge (`<bin>.protocol`, see [`crate::commands::sidecar`]); an agent
//! manifest may declare the range it works with:
//!
//! ```yaml
//! transport:
//!   cli:
//!     binary: aware-tekla
//!     bridge-protocol: { min: 1, max: 2 }   # inclusive; either bound may be omitted
//! ```
//!
//! This module owns the declaration grammar ([`parse`] — the only reader, so
//! `validate_agent`, `aware app check` and `aware app run` cannot disagree about what
//! a declaration means) and the verdict ([`finding_for`]).
//!
//! **Nothing refuses, it asks.** A mismatch is a [`Finding`] with plain-English
//! choices; callers warn, record it, and carry on. No declaration means no claim and
//! no output — exactly the behaviour from before the field existed.

use std::path::Path;

use serde::Serialize;
use serde_yaml::Value as Yaml;

use crate::commands::sidecar::{InstalledProtocol, installed_protocol};
use crate::manifest::Agent;

/// The protocol every bridge released before protocol numbers existed speaks, and
/// the first number a manifest may name.
pub(crate) const BASELINE: u32 = 1;

/// An inclusive protocol range; at least one bound is always present.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub(crate) struct Range {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) min: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max: Option<u32>,
}

impl Range {
    fn contains(self, n: u32) -> bool {
        self.min.is_none_or(|m| n >= m) && self.max.is_none_or(|m| n <= m)
    }

    /// The range as a person would say it.
    fn describe(self) -> String {
        match (self.min, self.max) {
            (Some(a), Some(b)) if a == b => format!("protocol {a} exactly"),
            (Some(a), Some(b)) => format!("protocols {a} to {b}"),
            (Some(a), None) => format!("protocol {a} or newer"),
            (None, Some(b)) => format!("protocol {b} or older"),
            (None, None) => "any protocol".to_string(),
        }
    }
}

/// What a manifest says about the bridge protocol it works with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Declaration {
    /// No `bridge-protocol` key (or no `cli` transport): no claim.
    Absent,
    Declared(Range),
    /// The key is present but breaks the grammar; the reason is a plain sentence.
    Invalid(String),
}

/// Read an agent's declaration. Presence matters: `bridge-protocol:` with no
/// value is [`Declaration::Invalid`], never "absent".
pub(crate) fn parse(agent: &Agent) -> Declaration {
    match agent
        .transport
        .cli
        .as_ref()
        .and_then(|cli| cli.bridge_protocol.as_ref())
    {
        None => Declaration::Absent,
        Some(value) => parse_value(value),
    }
}

fn parse_value(value: &Yaml) -> Declaration {
    let Yaml::Mapping(map) = value else {
        return Declaration::Invalid(
            "`bridge-protocol` must be a mapping such as `{ min: 1, max: 2 }`".into(),
        );
    };
    let mut range = Range {
        min: None,
        max: None,
    };
    for (key, bound) in map {
        let Some(name) = key.as_str() else {
            return Declaration::Invalid("`bridge-protocol` keys must be `min` or `max`".into());
        };
        let slot = match name {
            "min" => &mut range.min,
            "max" => &mut range.max,
            other => {
                return Declaration::Invalid(format!(
                    "`bridge-protocol` has an unknown key `{other}` (only `min` and `max` are allowed)"
                ));
            }
        };
        match bound.as_u64().and_then(|n| u32::try_from(n).ok()) {
            Some(n) if n >= BASELINE => *slot = Some(n),
            _ => {
                return Declaration::Invalid(format!(
                    "`bridge-protocol.{name}` must be a whole number, {BASELINE} or higher"
                ));
            }
        }
    }
    match (range.min, range.max) {
        (None, None) => {
            Declaration::Invalid("`bridge-protocol` must name `min`, `max` or both".into())
        }
        (Some(a), Some(b)) if a > b => Declaration::Invalid(format!(
            "`bridge-protocol.min` ({a}) is higher than `max` ({b})"
        )),
        _ => Declaration::Declared(range),
    }
}

/// How the installed bridge sits against a declared range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Status {
    Fits,
    /// The agent needs a newer bridge than the one installed.
    BridgeTooOld,
    /// The installed bridge is newer than the agent was written for.
    BridgeTooNew,
    /// AWARE cannot say which protocol the bridge that will run speaks.
    Unknown,
    DeclarationInvalid,
}

/// The pure verdict for a known installed protocol.
pub(crate) fn bridge_fit(declared: Range, installed: u32) -> Status {
    if declared.contains(installed) {
        Status::Fits
    } else if declared.min.is_some_and(|m| installed < m) {
        Status::BridgeTooOld
    } else {
        Status::BridgeTooNew
    }
}

/// One agent's bridge verdict, as `aware app check` and the run record carry it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Finding {
    pub(crate) binary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) declared: Option<Range>,
    /// The installed bridge's protocol; null when AWARE cannot say.
    pub(crate) installed_protocol: Option<u32>,
    /// True when the number is the baseline assumed for a bridge installed before
    /// protocol stamps existed, not a stamp.
    pub(crate) assumed: bool,
    pub(crate) status: Status,
    pub(crate) detail: String,
    /// What the person can do about it; every list ends with "carry on anyway",
    /// because nothing here blocks a run.
    pub(crate) choices: Vec<String>,
}

impl Finding {
    /// Whether the run should say something: a known mismatch or an unreadable
    /// declaration. `Fits` and `Unknown` have nothing for anyone to act on.
    pub(crate) fn needs_attention(&self) -> bool {
        matches!(
            self.status,
            Status::BridgeTooOld | Status::BridgeTooNew | Status::DeclarationInvalid
        )
    }
}

/// The verdict for `agent`, judged on `manifest` — the copy the run's resolver
/// chose (a pinned approval's stored package, not the installed working copy).
/// `None` when the manifest makes no claim: no `cli` transport (the `cli` block is
/// the one dispatch uses whenever it is present) or no `bridge-protocol` key.
pub(crate) fn finding_for(manifest: &Agent, bridges_dir: &Path) -> Option<Finding> {
    let cli = manifest.transport.cli.as_ref()?;
    let binary = cli.binary.clone();
    let declared = match parse(manifest) {
        Declaration::Absent => return None,
        Declaration::Invalid(reason) => {
            return Some(Finding {
                binary,
                declared: None,
                installed_protocol: None,
                assumed: false,
                status: Status::DeclarationInvalid,
                detail: format!(
                    "{} {} declares the bridge protocol it works with, but the declaration cannot be read ({reason}), so no bridge check was made.",
                    manifest.agent, manifest.version
                ),
                choices: vec![
                    "Ask the tool's author for a corrected release.".into(),
                    CARRY_ON.into(),
                ],
            });
        }
        Declaration::Declared(range) => range,
    };
    let who = format!("{} {}", manifest.agent, manifest.version);
    let (n, assumed) = match installed_protocol(&binary, bridges_dir) {
        InstalledProtocol::Stamped(n) => (n, false),
        InstalledProtocol::AssumedBaseline => (BASELINE, true),
        InstalledProtocol::Unknown(why) => {
            return Some(Finding {
                detail: format!(
                    "{who} works with {}, but AWARE cannot tell which protocol the {binary} that will run speaks: {why}.",
                    declared.describe()
                ),
                binary,
                declared: Some(declared),
                installed_protocol: None,
                assumed: false,
                status: Status::Unknown,
                choices: Vec::new(),
            });
        }
    };
    let bridge_id = crate::commands::sidecar::bridge_id_for_binary(&binary);
    let (status, detail, choices) = match bridge_fit(declared, n) {
        Status::Fits => (
            Status::Fits,
            format!(
                "{who} works with {}; the installed {binary} speaks protocol {n}.",
                declared.describe()
            ),
            Vec::new(),
        ),
        Status::BridgeTooOld => (
            Status::BridgeTooOld,
            format!(
                "{who} needs {}, but the installed {binary} speaks protocol {n}.",
                declared.describe()
            ),
            vec![
                match bridge_id {
                    Some(id) => format!(
                        "Update the bridge: run `aware sidecar install {id}` (if its protocol number does not change, update aware itself first)."
                    ),
                    None => "Update the bridge program to a newer release.".into(),
                },
                CARRY_ON.into(),
            ],
        ),
        _ => (
            Status::BridgeTooNew,
            format!(
                "{who} was written for {}, but the installed {binary} speaks the newer protocol {n}.",
                declared.describe()
            ),
            vec![
                format!(
                    "Update the tool, then compile the workflow again: run `aware agent update {}`.",
                    manifest.agent
                ),
                CARRY_ON.into(),
            ],
        ),
    };
    Some(Finding {
        binary,
        declared: Some(declared),
        installed_protocol: Some(n),
        assumed,
        status,
        detail,
        choices,
    })
}

const CARRY_ON: &str =
    "Carry on anyway: the run goes ahead and may fail inside the bridge if the two really differ.";

/// The one-paragraph warning `aware app run` prints for a finding that needs
/// attention; the run carries on.
pub(crate) fn warning(agent: &str, finding: &Finding) -> String {
    let mut text = format!("\u{26a0} agent {agent}: {}", finding.detail);
    for (i, choice) in finding.choices.iter().enumerate() {
        text.push_str(&format!("\n    {}. {choice}", i + 1));
    }
    text
}

#[cfg(test)]
mod tests;
