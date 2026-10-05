//! `aware app migrate …` (#628 PR2): plan, prepare, discard, hold, unhold.
//!
//! None of these verbs touches `<app>.lock`. `prepare` writes only the
//! candidate + evidence under `<source-dir>/.aware-migration/`; `hold` writes
//! only `<source-dir>/.aware-approvals/HOLD`. Expected outcomes — up to date,
//! needs a person, blocked, held — are `ok: true` data; `ok: false` only when
//! the command cannot run (an unknown app, a malformed `--to`, a refused
//! backing-app candidate, an unreadable file).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Instant;

use clap::Subcommand;
use serde::Serialize;

use crate::agent_resolution::PinTarget;
use crate::context::Context;
use crate::envelope;
use crate::error::AwareError;
use crate::migration::files::{self, HoldRecord};
use crate::migration::plan::{self, PlanRow, State};

#[derive(Subcommand, Debug)]
pub enum MigrateCommand {
    /// Report, per app, whether an agent update can carry its approval
    /// forward and who must decide. Writes nothing.
    Plan {
        /// An installed app id or a path to its source. Repeatable.
        #[arg(long = "app", conflicts_with = "all")]
        apps: Vec<String>,
        /// Every installed app (the default when no --app is given).
        #[arg(long)]
        all: bool,
        /// Move an agent to `<agent>@sha256:<hex>` or `<agent>@<version>`
        /// instead of to its installed copy. Repeatable.
        #[arg(long = "to")]
        to: Vec<String>,
    },
    /// Compile a candidate for one app against the new agent pins and write
    /// it, with its evidence, beside the source. The approved lock is never
    /// touched.
    Prepare {
        app: String,
        #[arg(long = "to")]
        to: Vec<String>,
    },
    /// Remove an app's candidate and evidence.
    Discard { app: String },
    /// Put an app on hold: it is sealed, certified or frozen and must not be
    /// carried forward.
    Hold {
        app: String,
        /// Who is holding it.
        #[arg(long)]
        actor: String,
        /// Why, in plain words.
        #[arg(long)]
        reason: Option<String>,
    },
    /// Lift a hold.
    Unhold {
        app: String,
        #[arg(long)]
        actor: String,
    },
}

pub fn dispatch(cmd: MigrateCommand, ctx: &Context) -> Result<(), AwareError> {
    let started = Instant::now();
    let (name, outcome) = match cmd {
        MigrateCommand::Plan { apps, all: _, to } => {
            ("app migrate plan", plan_cmd(ctx, &apps, &to))
        }
        MigrateCommand::Prepare { app, to } => ("app migrate prepare", prepare_cmd(ctx, &app, &to)),
        MigrateCommand::Discard { app } => ("app migrate discard", discard_cmd(ctx, &app)),
        MigrateCommand::Hold { app, actor, reason } => {
            ("app migrate hold", hold_cmd(ctx, &app, &actor, reason))
        }
        MigrateCommand::Unhold { app, actor } => {
            ("app migrate unhold", unhold_cmd(ctx, &app, &actor))
        }
    };
    match outcome {
        Ok(output) => {
            if ctx.json {
                envelope::print_ok(name, &output.data, started)?;
            } else {
                print!("{}", output.text);
            }
            Ok(())
        }
        Err(failure) if ctx.json => {
            let env = envelope::Envelope::<()> {
                ok: false,
                data: None,
                error: Some(envelope::EnvelopeError {
                    code: failure.code.clone(),
                    message: failure.error.to_string(),
                    details: failure.details.clone(),
                }),
                meta: envelope::meta_for(name, started),
            };
            println!("{}", serde_json::to_string(&env)?);
            use std::io::Write;
            let _ = std::io::stdout().flush();
            std::process::exit(failure.error.exit_code());
        }
        Err(failure) => Err(*failure.error),
    }
}

struct Output {
    data: serde_json::Value,
    text: String,
}

struct Failure {
    code: String,
    error: Box<AwareError>,
    details: serde_json::Value,
}

impl From<AwareError> for Failure {
    fn from(error: AwareError) -> Self {
        let code = match &error {
            AwareError::NotFound(_) => "E_MIGRATE_APP_NOT_FOUND",
            _ => "E_MIGRATE_FAILED",
        };
        Failure {
            code: code.into(),
            error: Box::new(error),
            details: serde_json::Value::Null,
        }
    }
}

fn to_json<T: Serialize>(value: &T) -> Result<serde_json::Value, Failure> {
    serde_json::to_value(value).map_err(|e| AwareError::Internal(e.to_string()).into())
}

/// The source file of an installed app id, or of a path to a source/app dir.
fn app_source(ctx: &Context, app: &str) -> Result<PathBuf, AwareError> {
    let path = Path::new(app);
    let exists = path.try_exists().map_err(|error| {
        std::io::Error::new(error.kind(), format!("{}: {error}", path.display()))
    })?;
    if exists {
        return crate::app_lock::find_app_source(path).ok_or_else(|| {
            AwareError::NotFound(format!(
                "no app source file (.flo / .app / .flow / .aware) at {}",
                path.display()
            ))
        });
    }
    let dir = crate::manifest::loader::resolve_app_dir(&ctx.paths, app)?;
    crate::manifest::loader::find_app_manifest(&dir)
        .ok_or_else(|| AwareError::NotFound(format!("app {app} has no .flo/.app file")))
}

fn app_id(source: &Path) -> Result<String, AwareError> {
    crate::app_lock::read_app_source(source).map(|(app, _)| app.app)
}

/// Parse `--to` values: `<agent>@sha256:<hex>` or `<agent>@<version>`.
pub fn parse_targets(values: &[String]) -> Result<BTreeMap<String, PinTarget>, AwareError> {
    let mut out = BTreeMap::new();
    for value in values {
        let Some((agent, target)) = value.split_once('@') else {
            return Err(bad_target(
                value,
                "expected <agent>@sha256:<hex> or <agent>@<version>",
            ));
        };
        if !crate::manifest::loader::is_safe_segment(agent) || target.is_empty() {
            return Err(bad_target(
                value,
                "expected <agent>@sha256:<hex> or <agent>@<version>",
            ));
        }
        let target = if target.starts_with("sha256:") {
            if crate::agent_store::digest_hex(target).is_none() {
                return Err(bad_target(
                    value,
                    "a digest is sha256: followed by 64 lowercase hex characters",
                ));
            }
            PinTarget::Digest(target.to_string())
        } else {
            PinTarget::Version(target.to_string())
        };
        if out.insert(agent.to_string(), target).is_some() {
            return Err(bad_target(value, "the agent is named twice"));
        }
    }
    Ok(out)
}

fn bad_target(value: &str, why: &str) -> AwareError {
    AwareError::Validation(format!("[E_MIGRATE_BAD_TARGET] --to {value}: {why}"))
}

fn usage_failure(error: AwareError) -> Failure {
    let code = crate::migration::reason_from_error(&error).error_code();
    Failure {
        code,
        error: Box::new(error),
        details: serde_json::Value::Null,
    }
}

// ── plan ────────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct PlanData {
    apps: Vec<PlanRow>,
}

fn plan_cmd(ctx: &Context, apps: &[String], to: &[String]) -> Result<Output, Failure> {
    let targets = parse_targets(to).map_err(usage_failure)?;
    let mut sources = Vec::new();
    if apps.is_empty() {
        sources = installed_sources(ctx)?;
    } else {
        for app in apps {
            let source = app_source(ctx, app)?;
            let id = app_id(&source).unwrap_or_else(|_| app.clone());
            sources.push((id, source));
        }
    }
    let requested = (!targets.is_empty()).then_some(&targets);
    let rows = plan::plan_rows(&ctx.paths, &sources, requested);
    let mut text = String::new();
    for row in &rows {
        text.push_str(&format!("{}: {}\n", row.app, state_word(row.state)));
        for target in &row.targets {
            text.push_str(&format!(
                "  {} {} -> {} ({})\n",
                target.agent, target.from.version, target.to.version, target.bump
            ));
        }
        for reason in &row.reasons {
            text.push_str(&format!("  - {}\n", reason.text));
        }
    }
    if rows.is_empty() {
        text.push_str("no installed apps\n");
    }
    Ok(Output {
        data: to_json(&PlanData { apps: rows })?,
        text,
    })
}

/// Every installed app's source, sorted by app id. A directory with no source
/// is skipped; one whose source cannot be read is still listed, by its
/// directory name, and reported as a blocked row.
fn installed_sources(ctx: &Context) -> Result<Vec<(String, PathBuf)>, Failure> {
    let apps_dir = ctx.paths.apps_dir();
    let entries = match std::fs::read_dir(&apps_dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => {
            return Err(AwareError::from(std::io::Error::new(
                error.kind(),
                format!("{}: {error}", apps_dir.display()),
            ))
            .into());
        }
    };
    let mut out = Vec::new();
    for entry in entries {
        let entry = entry.map_err(AwareError::from)?;
        let Some(source) = crate::manifest::loader::find_app_manifest(&entry.path()) else {
            continue;
        };
        let id =
            app_id(&source).unwrap_or_else(|_| entry.file_name().to_string_lossy().into_owned());
        out.push((id, source));
    }
    out.sort();
    Ok(out)
}

fn state_word(state: State) -> &'static str {
    match state {
        State::UpToDate => "up to date",
        State::AutoUnderPolicy => "can be carried forward under policy",
        State::NeedsPerson => "needs a person",
        State::Blocked => "blocked",
        State::Held => "held",
    }
}

// ── prepare ─────────────────────────────────────────────────────────────────

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
struct PrepareData {
    app: String,
    state: State,
    /// Whether a candidate was written. False when nothing moves or the
    /// candidate is blocked (the reasons say why).
    prepared: bool,
    candidate: Option<String>,
    evidence: Option<String>,
    candidate_digest: Option<String>,
    plan_digest: Option<String>,
    evidence_digest: Option<String>,
    /// The approved lock's digest before and after: always equal.
    lock_digest: Option<String>,
    lock_unchanged: bool,
    row: PlanRow,
}

fn prepare_cmd(ctx: &Context, app: &str, to: &[String]) -> Result<Output, Failure> {
    let targets = parse_targets(to).map_err(usage_failure)?;
    let source = app_source(ctx, app)?;
    let dir = crate::fs::containing_dir(&source).to_path_buf();
    let requested = (!targets.is_empty()).then_some(&targets);
    if let Some(requested) = requested {
        // An explicit target the app does not dispatch is a misuse, not a state.
        let (parsed, _) = crate::app_lock::read_app_source(&source)?;
        let dispatched = crate::validate::dispatchable_agents(&parsed);
        if let Some(id) = requested
            .keys()
            .find(|id| !dispatched.contains(id.as_str()))
        {
            return Err(usage_failure(AwareError::Validation(format!(
                "[E_MIGRATE_TARGET_UNUSED] app {} does not dispatch agent {id}, so there is nothing to move it to",
                parsed.app
            ))));
        }
    }
    let mut eval = plan::evaluate(&ctx.paths, &source, requested)?;
    let row = &eval.row;
    // An explicit target that cannot be resolved is a misuse too.
    if requested.is_some()
        && let Some(reason) = row
            .reasons
            .iter()
            .find(|r| r.code.starts_with("migrate-target-"))
    {
        return Err(usage_failure(AwareError::Validation(format!(
            "[{}] {}",
            reason.error_code(),
            reason.text
        ))));
    }
    // §14: v1 never prepares a candidate that moves a backing app.
    if row.backing_app_moved {
        let callers = if row.callers.is_empty() {
            plan::find_callers(&ctx.paths, &row.app)
        } else {
            row.callers.clone()
        };
        return Err(Failure {
            code: "E_MIGRATE_BACKING_APP".into(),
            error: Box::new(AwareError::Validation(format!(
                "[E_MIGRATE_BACKING_APP] {}: a workflow that other workflows run as a tool is never carried forward automatically; compile it and its callers again{}",
                row.app,
                if callers.is_empty() {
                    String::new()
                } else {
                    format!(" (callers: {})", callers.join(", "))
                }
            ))),
            details: serde_json::json!({
                "callers": callers,
                "reasons": row.reasons,
            }),
        });
    }
    let lock_path = PathBuf::from(&row.lock);
    let lock_before = eval.base_bytes.as_deref().map(crate::app_lock::lock_digest);
    let writable = matches!(row.state, State::NeedsPerson | State::AutoUnderPolicy)
        || (row.state == State::Held && !row.targets.is_empty() && !has_block(row));
    let mut data = PrepareData {
        app: row.app.clone(),
        state: row.state,
        prepared: false,
        candidate: None,
        evidence: None,
        candidate_digest: None,
        plan_digest: None,
        evidence_digest: None,
        lock_digest: lock_before.clone(),
        lock_unchanged: true,
        row: row.clone(),
    };
    if writable && let Some(candidate) = eval.candidate.take() {
        let mut row = eval.row.clone();
        row.candidate = plan::CandidateRow {
            present: true,
            candidate_digest: Some(candidate.header.candidate_digest.clone()),
            fresh: true,
        };
        let evidence = files::Evidence {
            format: files::EVIDENCE_FORMAT.into(),
            app: row.app.clone(),
            prepared_at: chrono::Utc::now().to_rfc3339(),
            cli_version: env!("CARGO_PKG_VERSION").into(),
            header: candidate.header.clone(),
            row: to_json(&row)?,
        };
        let evidence_bytes = serde_json::to_vec_pretty(&evidence)
            .map_err(|e| AwareError::Internal(format!("serialize evidence: {e}")))?;
        let (candidate_file, evidence_file) =
            files::write_candidate(&dir, &row.app, &candidate.bytes, &evidence_bytes)?;
        data.prepared = true;
        data.candidate = Some(candidate_file.display().to_string());
        data.evidence = Some(evidence_file.display().to_string());
        data.candidate_digest = Some(candidate.header.candidate_digest.clone());
        data.plan_digest = Some(candidate.header.plan_digest.clone());
        data.evidence_digest = Some(crate::app_lock::lock_digest(&evidence_bytes));
        data.row = row;
    }
    let lock_after = std::fs::read(&lock_path)
        .ok()
        .map(|bytes| crate::app_lock::lock_digest(&bytes));
    data.lock_unchanged = lock_after == lock_before;
    if !data.lock_unchanged {
        return Err(AwareError::Internal(format!(
            "{} changed while a candidate was being prepared (another process wrote it); the candidate may be stale — run prepare again",
            lock_path.display()
        ))
        .into());
    }
    let text = if data.prepared {
        format!(
            "\u{2713} candidate for {} written to {}\n  the approved {} is unchanged; a person must approve before it is used\n",
            data.app,
            data.candidate.as_deref().unwrap_or(""),
            lock_path.display()
        )
    } else {
        let mut text = format!(
            "{}: {} — no candidate written\n",
            data.app,
            state_word(data.state)
        );
        for reason in &data.row.reasons {
            text.push_str(&format!("  - {}\n", reason.text));
        }
        text
    };
    Ok(Output {
        data: to_json(&data)?,
        text,
    })
}

/// A held app's candidate is still worth recording unless something blocks it.
fn has_block(row: &PlanRow) -> bool {
    row.reasons.iter().any(|r| {
        matches!(
            r.code.as_str(),
            "needs-source-edit"
                | "agent-unavailable"
                | "no-approval"
                | "approval-invalid"
                | "source-changed"
        ) || r.code.starts_with("migrate-")
            || r.code.starts_with("app-lock-")
            || r.code == "cannot-evaluate"
    })
}

// ── discard / hold / unhold ─────────────────────────────────────────────────

fn discard_cmd(ctx: &Context, app: &str) -> Result<Output, Failure> {
    let source = app_source(ctx, app)?;
    let id = app_id(&source)?;
    let dir = crate::fs::containing_dir(&source);
    let discarded = files::discard_candidate(dir, &id)?;
    Ok(Output {
        data: serde_json::json!({ "app": id, "discarded": discarded }),
        text: if discarded {
            format!("\u{2713} discarded the candidate for {id}\n")
        } else {
            format!("{id} has no candidate\n")
        },
    })
}

fn hold_cmd(
    ctx: &Context,
    app: &str,
    actor: &str,
    reason: Option<String>,
) -> Result<Output, Failure> {
    let actor = require_actor(actor)?;
    let source = app_source(ctx, app)?;
    let id = app_id(&source)?;
    let dir = crate::fs::containing_dir(&source);
    let record = HoldRecord {
        format: 1,
        held_by: actor,
        held_at: chrono::Utc::now().to_rfc3339(),
        reason,
    };
    let path = files::write_hold(dir, &record)?;
    Ok(Output {
        data: serde_json::json!({ "app": id, "held": true, "hold": record, "path": path.display().to_string() }),
        text: format!(
            "\u{2713} {id} is on hold; it will not be carried forward until someone runs `aware app migrate unhold {id}`\n"
        ),
    })
}

fn unhold_cmd(ctx: &Context, app: &str, actor: &str) -> Result<Output, Failure> {
    let actor = require_actor(actor)?;
    let source = app_source(ctx, app)?;
    let id = app_id(&source)?;
    let dir = crate::fs::containing_dir(&source);
    let was_held = files::remove_hold(dir)?;
    Ok(Output {
        data: serde_json::json!({ "app": id, "held": false, "was-held": was_held, "by": actor }),
        text: if was_held {
            format!("\u{2713} lifted the hold on {id}\n")
        } else {
            format!("{id} was not on hold\n")
        },
    })
}

fn require_actor(actor: &str) -> Result<String, Failure> {
    let actor = actor.trim();
    if actor.is_empty() {
        return Err(usage_failure(AwareError::Validation(
            "[E_MIGRATE_NO_ACTOR] --actor must name who is acting".into(),
        )));
    }
    Ok(actor.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn targets_parse_as_digest_or_version_and_refuse_the_rest() {
        let hex = "a".repeat(64);
        let parsed = parse_targets(&[format!("tekla@sha256:{hex}"), "gw@1.2.3".into()]).unwrap();
        assert_eq!(parsed["tekla"], PinTarget::Digest(format!("sha256:{hex}")));
        assert_eq!(parsed["gw"], PinTarget::Version("1.2.3".into()));
        for bad in [
            "tekla",
            "@1.0.0",
            "tekla@",
            "tekla@sha256:ABC",
            "../x@1.0.0",
        ] {
            let error = parse_targets(&[bad.to_string()]).unwrap_err().to_string();
            assert!(error.contains("E_MIGRATE_BAD_TARGET"), "{bad}: {error}");
        }
        let twice = parse_targets(&["a@1.0.0".into(), "a@2.0.0".into()]).unwrap_err();
        assert!(twice.to_string().contains("named twice"));
    }
}
